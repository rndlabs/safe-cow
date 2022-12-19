use async_trait::async_trait;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Select};
use ethers::{prelude::*, utils};
use eyre::Result;
use reqwest::Client;
use std::{collections::HashMap, fmt, str::FromStr, sync::Arc};
use token_list::{Token, TokenList};

use model::order::{BuyTokenDestination, OrderBuilder, OrderCreation, OrderKind, SellTokenSource};

use crate::{
    contracts::erc20::ERC20, get_cowswap_api_url, get_cowswap_explorer_url, safe::Safe,
    CancelOrder, CowswapApiError, CreateOrder, Invertible, Opts, SettlementContract,
    SupportedChains, VaultRelayerContract,
};

pub enum OrderTokens {
    TokenList(TokenList),
    Custom,
}
/// Create a new order
pub async fn create_order<M>(
    config: CreateOrder,
    opts: &Opts,
    safe: &mut Safe,
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

    // Get the respective tokens annd amounts for the swap
    let token_amount0 = get_token_amount(
        &usable_tokens,
        order_kind.to_string(),
        provider.clone(),
        &opts,
    )
    .await?;

    let token_amount1 = get_token_amount(
        &usable_tokens,
        order_kind.invert().to_string(),
        provider.clone(),
        &opts,
    )
    .await?;

    // if the order is a sell order, we need to invert the token amounts
    let (buy_token_amount, sell_token_amount) = match order_kind {
        OrderKind::Buy => (token_amount0.clone(), token_amount1.clone()),
        OrderKind::Sell => (token_amount1.clone(), token_amount0.clone()),
    };

    // print a blank line for readability
    println!();

    // check that the user has enough balance for the sell token
    if !safe.has_min_balance(&sell_token_amount).await? {
        println!("You don't have enough balance for this order");
        return Ok(());
    }

    // check that the vaultrelayer has enough allowance for the sell token
    let vault_relayer = VaultRelayerContract::get_by_chain(&chain).get_address();
    let do_approve_tx = !safe
        .has_min_approval(&sell_token_amount, vault_relayer)
        .await?;

    if do_approve_tx && safe.has_pending_txs().await? {
        println!("You have pending transactions in the Safe API. Please execute these first.");
        println!(
            "{}{}{}",
            safe.get_safe_app_url().await?,
            utils::to_checksum(&safe.address, None),
            "/transactions/queue"
        );
        return Ok(());
    }

    // if the user doesn't have enough allowance, prompt them to approve the vault relayer
    if do_approve_tx {
        // prompt the user to approve the vault relayer
        if dialoguer::Confirm::new()
            .with_prompt(format!(
                "Approve the GPv2VaultRelayer ({}) for {}?",
                Address::from(vault_relayer).to_string(),
                &sell_token_amount
            ))
            .interact()?
        {
            // if the prompt private keys fails, the user has cancelled the order
            if !safe.prompt_private_keys().is_ok() {
                println!("Order creation cancelled");
                return Ok(());
            }

            // approve the vault relayer
            safe.approve(&sell_token_amount, vault_relayer).await?;
        } else {
            println!("Order creation cancelled");
            return Ok(());
        }
    }

    // confirm the order
    if !dialoguer::Confirm::new()
        .with_prompt(format!(
            "Confirm {} {} for {}?",
            order_kind, token_amount0, token_amount1
        ))
        .interact()?
    {
        println!("Order creation cancelled");
        return Ok(());
    } else {
        // blank line for readability
        println!();
        println!("Creating order...");
    }

    // if the prompt private keys fails, the user has cancelled the order
    if !safe.prompt_private_keys().is_ok() {
        println!("Order creation cancelled");
        return Ok(());
    }

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

    let (_safe_msg_digest, signature) = safe.sign(&digest).await?;

    let valid = safe.verify_signature(&digest, &signature).await?;

    // if the signature is invalid, we should exit
    if !valid {
        println!("Signature is invalid");
        return Ok(());
    }

    // Triple check confirmation here
    if !dialoguer::Confirm::new()
        .with_prompt("Are you sure you want to submit this order?")
        .interact()?
    {
        println!("Order creation cancelled");
        return Ok(());
    } else {
        print!("Submitting order...");
    }

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
    let response = client.post(&url).json(&order).send().await?;

    if !response.status().is_success() {
        // decode the error message from the response
        let error: CowswapApiError = response.json().await?;
        println!("Error: {error}");
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
    safe: &Safe,
    provider: Arc<Provider<M>>,
    chain: SupportedChains,
) -> Result<()>
where
    M: JsonRpcClient + Send + Sync + 'static,
{
    unimplemented!()
}

/// An analogue for the similar CurrencyAmount popularised by Uniswap's sdk-core.
#[derive(Debug, Clone)]
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
        // // format the amount with the token's decimals
        ethers::utils::format_units(self.amount, u32::from(self.token.decimals))
            .map_err(|_| fmt::Error)
            .and_then(|amount| write!(f, "{} {}", amount, self.token.symbol))
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
            address: utils::to_checksum(&address, None),
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

/// Prompt the user to input a token address and return the token
/// If the user inputs an address, we validate that it is a valid address
/// If the user inputs an invalid symbol or address, we return an error
pub fn get_token_input() -> Result<Address> {
    let token = dialoguer::Input::<String>::new()
        .with_prompt("ERC20 Token address")
        .interact()?;

    Ok(Address::from_str(&token)?)
}

pub async fn get_token_amount<M>(
    usable_tokens: &OrderTokens,
    msg: String,
    provider: Arc<Provider<M>>,
    opts: &Opts,
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

            Token::from_address(t?, provider.clone()).await?
        }
    };

    let amount = get_amount(&token, provider.clone(), &opts).await;

    Ok(TokenAmount::new(token, amount))
}

/// Prompt the user to input a token amount. Enforce that the amount is valid.
pub async fn get_amount<M>(token: &Token, provider: Arc<Provider<M>>, opts: &Opts) -> U256
where
    M: JsonRpcClient + Send + Sync + 'static,
{
    // Get the balance of the token
    let contract = ERC20::new(token.into_address(), provider.clone());
    let balance = contract
        .balance_of(*opts.safe.as_address().unwrap())
        .call()
        .await
        .unwrap();
    let balance = ethers::utils::format_units(balance, i32::from(token.decimals)).unwrap();

    let mut amount = dialoguer::Input::<String>::new()
        .with_prompt(format!("Amount of {} (Balance: {balance})", token.symbol))
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
