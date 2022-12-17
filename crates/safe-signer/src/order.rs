use eyre::Result;
use reqwest::Client;
use ethers::prelude::*;
use std::{str::FromStr, sync::Arc};
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Select};
use token_list::TokenList;

use crate::{
    get_chain_id, get_cowswap_api_url, get_cowswap_explorer_url, safesigner::{verify_signature, safe_signature_of_message},
    SettlementContract, CreateOrder, SupportedChains, Opts,
};

use model::order::{BuyTokenDestination, OrderBuilder, OrderCreation, OrderKind, SellTokenSource};

/// Run the sign order command
pub async fn run(config: CreateOrder, opts: Opts) -> Result<()> {
    // 1. Create the cowswap order
    let order = OrderBuilder::default()
        .with_sell_token(*config.sell_token.as_address().unwrap())
        .with_sell_amount(U256::from_dec_str(&config.sell_amount).unwrap())
        .with_buy_token(*config.buy_token.as_address().unwrap())
        .with_buy_amount(U256::from_dec_str(&config.buy_amount).unwrap())
        // make order valid to current time + 20 minutes
        .with_valid_to(chrono::Utc::now().timestamp() as u32 + config.valid_to.unwrap_or(1200))
        // by setting fee amount to 0, we default to limit orders
        .with_fee_amount(U256::from(0))
        // partially fillable orders aren't supported yet
        // .with_partially_fillable(false)
        .with_sell_token_balance(SellTokenSource::Erc20)
        .with_buy_token_balance(BuyTokenDestination::Erc20)
        .with_kind(OrderKind::Sell)
        .build();

    // 2. Get the chain, chain id and contract address for the signing domain
    let chain =
        SupportedChains::get_by_chain_id(get_chain_id(&opts.rpc_url).await?).unwrap();
    let contract_address = SettlementContract::get_by_chain(&chain).get_address();
    let chain_id = chain.get_chain_id();

    // 3. Calculate the digest of the order
    let domain_separator = model::DomainSeparator::new(chain_id, contract_address);

    println!("Domain separator: {domain_separator:#?}");

    let digest32 =
        model::signature::hashed_eip712_message(&domain_separator, &order.data.hash_struct());
    let digest: Bytes = digest32.into();

    println!("EIP-712 Typed digest: {digest}");

    let (_safe_msg_digest, signature) = safe_signature_of_message(
        &digest,
        &opts
    ).await?;

    let valid = verify_signature(
        &digest,
        &signature,
        &opts
    )
    .await?;

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

pub fn get_token(usable_tokens: &OrderTokens, msg: String) -> Result<(H160, ethers::types::U256)> {
    let token = match usable_tokens {
        OrderTokens::TokenList(token_list) => {
            let token_names: Vec<String> = token_list
                .tokens
                .iter()
                .map(|token| format!("{} ({})", token.symbol, token.name))
                .collect();

            let token = FuzzySelect::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("{} Token", msg))
                .items(&token_names)
                .default(0)
                .interact()?;

            Address::from_str(&token_list.tokens[token].address).unwrap()
        }
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

            t.unwrap()
        }
    };

    let amount = get_token_amount(&usable_tokens, &token)?;
    
    Ok((token, amount))
}

/// Prompt the user for the amount of the token with decimals and a custom msg
/// If the user inputs an invalid amount, we return an error
/// If the user inputs a valid amount, we return the amount in U256
pub fn get_token_amount(usable_tokens: &OrderTokens, token: &Address) -> Result<U256> {
    // Get the token record from the token list if it exists
    let token_record = match usable_tokens {
        OrderTokens::TokenList(token_list) => token_list
            .tokens
            .iter()
            .find(|t| Address::from_str(&t.address).unwrap() == *token),
        OrderTokens::Custom => None,
    };

    // if token_record doesn't exist, set the symbol to the address
    let binding = token.to_string();
    let symbol = token_record.map(|token| &token.symbol).unwrap_or(&binding);
    let decimals = token_record.map(|token| token.decimals).unwrap_or(0);

    let mut amount = dialoguer::Input::<String>::new()
        .with_prompt(format!("Amount of {symbol}"))
        .interact()?;

    amount = loop {
        if amount.is_empty() || !ethers::utils::parse_units(&amount, i32::from(decimals)).is_ok() {
            println!("Invalid amount");
            amount = dialoguer::Input::<String>::new()
                .with_prompt(format!("Amount of {symbol}"))
                .interact()?;
            continue;
        } else {
            break amount;
        }
    };

    let amount = ethers::utils::parse_units(amount, i32::from(decimals))?;

    Ok(amount.into())
}
