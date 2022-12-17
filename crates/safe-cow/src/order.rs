use async_trait::async_trait;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Select};
use ethers::prelude::*;
use eyre::Result;
use reqwest::Client;
use std::{collections::HashMap, str::FromStr, sync::Arc, fmt};
use token_list::{Token, TokenList};

use model::order::{BuyTokenDestination, OrderBuilder, OrderCreation, OrderKind, SellTokenSource};

use crate::{
    get_cowswap_api_url, get_cowswap_explorer_url,
    safesigner::{safe_signature_of_message, verify_signature},
    CancelOrder, CreateOrder, Invertible, Opts, SettlementContract, SupportedChains,
};

abigen!(
    ERC20,
    "./abi/ERC20.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

pub enum OrderTokens {
    TokenList(TokenList),
    Custom,
}
/// Create a new order
pub async fn create_order<M>(
    config: CreateOrder,
    opts: &Opts,
    provider: Arc<Provider<M>>,
    chain: SupportedChains,
) -> Result<()>
where
    M: JsonRpcClient + Send + Sync + 'static,
{
    // set order_tokens to TokenList or Custom depending on the user's choice
    let usable_tokens = match dialoguer::Confirm::new()
        .with_prompt("Use a token list for selecting tokens?")
        .interact()?
    {
        true => OrderTokens::TokenList(chain.get_token_list().await?),
        false => OrderTokens::Custom,
    };

    let order_kinds = vec![OrderKind::Buy, OrderKind::Sell];

    // Is this a buy or sell order?
    let order_kind = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Order direction")
        .items(&order_kinds)
        .default(0)
        .interact()?;
    let order_kind = order_kinds[order_kind];

    let token_amount0 =
        get_token_amount(&usable_tokens, order_kind.to_string(), provider.clone()).await?;

    let token_amount1 = get_token_amount(
        &usable_tokens,
        order_kind.invert().to_string(),
        provider.clone(),
    )
    .await?;

    // if the order is a sell order, we need to invert the token amounts
    let (buy_token_amount, sell_token_amount) = match order_kind {
        OrderKind::Buy => (
            token_amount0.clone(),
            token_amount1.clone()
        ),
        OrderKind::Sell => (
            token_amount1.clone(),
            token_amount0.clone()
        ),
    };

    // output the order details
    println!(
        "{} {} for {}",
        order_kind,
        token_amount0.clone(),
        token_amount1.clone()
    );

    // 1. Create the cowswap order
    let order = OrderBuilder::default()
        .with_buy_token(buy_token_amount.into_address())
        .with_buy_amount(buy_token_amount.amount)
        .with_sell_token(sell_token_amount.into_address())
        .with_sell_amount(sell_token_amount.amount)
        // make order valid to current time + 20 minutes
        .with_valid_to(chrono::Utc::now().timestamp() as u32 + config.valid_to.unwrap_or(1200))
        // by setting fee amount to 0, we default to limit orders
        .with_fee_amount(U256::from(0))
        // partially fillable orders aren't supported yet
        // .with_partially_fillable(false)
        .with_sell_token_balance(SellTokenSource::Erc20)
        .with_buy_token_balance(BuyTokenDestination::Erc20)
        .with_kind(order_kind)
        .build();

    // 2. Get the chain, chain id and contract address for the signing domain
    let contract_address = SettlementContract::get_by_chain(&chain).get_address();

    // 3. Calculate the digest of the order
    let domain_separator = model::DomainSeparator::new(chain.get_chain_id(), contract_address);

    let digest =
        model::signature::hashed_eip712_message(&domain_separator, &order.data.hash_struct())
            .into();

    let (_safe_msg_digest, signature) = safe_signature_of_message(&digest, &opts, provider).await?;

    let valid = verify_signature(&digest, &signature, &opts).await?;

    println!("Signature is valid: {valid:?}");

    // 8. Submit the order to the API
    let order = OrderCreation {
        data: order.data,
        from: Some(*opts.safe.as_address().unwrap()),
        signature: model::signature::Signature::Eip1271(signature),
        quote_id: None,
    };

    let client = Client::new();
    let url = format!(
        "{}/orders",
        get_cowswap_api_url(&chain, config.staging.unwrap_or(false))
    );
    println!("API URL: {url}");
    let response = client.post(&url).json(&order).send().await?;

    if !response.status().is_success() {
        println!("Error: {:?}", response.text().await?);
        return Ok(());
    } else {
        println!("Order submitted successfully");

        // get the order id which is a JSON encoded string in the response
        let order_id: String = response.json().await?;
        println!("Order ID: {order_id}");
        println!(
            "Cow.FI explorer URL: {}/orders/{}",
            get_cowswap_explorer_url(&chain, config.staging.unwrap_or(false)),
            order_id
        )
    }

    Ok(())
}

/// Cancel an order by its ID
pub async fn cancel_order<M>(
    config: CancelOrder,
    opts: &Opts,
    provider: Arc<Provider<M>>,
    chain: SupportedChains,
) -> Result<()>
where
    M: JsonRpcClient + Send + Sync + 'static,
{
    unimplemented!()
}

/// An analogue for the similar CurrencyAmount popularised by Uniswap's sdk-core.
#[derive(Clone)]
pub struct TokenAmount {
    pub token: Token,
    pub amount: U256,
}

impl TokenAmount {
    pub fn new(token: Token, amount: U256) -> Self {
        Self { token, amount }
    }
}

impl fmt::Display for TokenAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.token.symbol)
    }
}

pub trait FromTokenList {
    fn from_list_with_prompt(token_list: &TokenList, msg: String) -> Result<Self>
    where
        Self: Sized;
}

impl FromTokenList for Token {
    fn from_list_with_prompt(token_list: &TokenList, msg: String) -> Result<Self> {
        let token_names = token_list
            .tokens
            .iter()
            .map(|token| format!("{} ({})", token.symbol, token.name))
            .collect::<Vec<_>>();

        let token = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("{} Token", msg))
            .items(&token_names)
            .default(0)
            .interact()?;

        Ok(token_list.tokens[token].clone())
    }
}

/// A trait for retrieving an object from the chain by address.
#[async_trait]
pub trait FromChain {
    async fn from_address<M>(address: H160, provider: Arc<Provider<M>>) -> Result<Self>
    where
        Self: Sized,
        M: JsonRpcClient + Send + Sync + 'static;
}

/// Retrieve a token from the chain by address.
#[async_trait]
impl FromChain for Token {
    async fn from_address<M>(address: H160, provider: Arc<Provider<M>>) -> Result<Self>
    where
        M: JsonRpcClient + Send + Sync + 'static,
    {
        let contract = ERC20::new(address, provider.clone());

        let name_call = contract.name();
        let symbol_call = contract.symbol();
        let decimals_call = contract.decimals();

        let mut multicall = Multicall::new(provider.clone(), None)
            .await?
            .version(MulticallVersion::Multicall3);
        multicall
            .add_call(name_call, false)
            .add_call(symbol_call, false)
            .add_call(decimals_call, false);

        let (name, symbol, decimals): ((bool, String), (bool, String), (bool, U256)) =
            multicall.call().await?;

        Ok(Token {
            chain_id: provider.get_chainid().await?.as_u32(),
            name: name.1,
            symbol: symbol.1,
            decimals: decimals.1.as_u32().try_into()?,
            address: Bytes::from(address.as_bytes().to_vec()).to_string(),
            logo_uri: None,
            tags: Vec::new(),
            extensions: HashMap::new(),
        })
    }
}

// Define a trait to convert a token to an address
pub trait IntoAddress {
    fn into_address(&self) -> Address;
}

impl IntoAddress for Token {
    fn into_address(&self) -> Address {
        Address::from_str(&self.address).unwrap()
    }
}

impl IntoAddress for TokenAmount {
    fn into_address(&self) -> Address {
        self.token.into_address()
    }
}

/// Prompt the user to input a token symbol or address and return the token address
/// If the user inputs a symbol, we query the token list to get the address
/// If the user inputs an address, we validate that it is a valid address
/// If the user inputs an invalid symbol or address, we return an error

pub fn get_token_input() -> Result<Address> {
    let token = dialoguer::Input::<String>::new()
        .with_prompt("Token symbol or address")
        .interact()?;

    Ok(Address::from_str(&token)?)
}

pub async fn get_token_amount<M>(
    usable_tokens: &OrderTokens,
    msg: String,
    provider: Arc<Provider<M>>,
) -> Result<TokenAmount>
where
    M: JsonRpcClient + Send + Sync + 'static,
{
    let token = match usable_tokens {
        OrderTokens::TokenList(token_list) => Token::from_list_with_prompt(token_list, msg)?,
        OrderTokens::Custom => {
            // prompt for custom token and make sure there is no error
            let mut t = get_token_input();

            loop {
                if t.is_err() {
                    println!("Invalid token address");
                    t = get_token_input();
                } else {
                    break;
                }
            }

            Token::from_address(t?, provider).await?
        }
    };

    let amount = get_amount(&token);

    Ok(TokenAmount::new(token, amount))
}

/// Prompt the user to input a token amount. Enforce that the amount is valid.
pub fn get_amount(token: &Token) -> U256 {
    let mut amount = dialoguer::Input::<String>::new()
        .with_prompt(format!("Amount of {}", token.symbol))
        .interact();

    amount = loop {
        if amount.as_ref().is_ok_and(|x| {
            x.is_empty() || !ethers::utils::parse_units(&x, i32::from(token.decimals)).is_ok()
        }) {
            println!("Invalid amount");
            amount = dialoguer::Input::<String>::new()
                .with_prompt(format!("Amount of {}", token.symbol))
                .interact();
            continue;
        } else {
            break amount;
        }
    };

    ethers::utils::parse_units(amount.unwrap(), i32::from(token.decimals))
        .unwrap()
        .into()
}
