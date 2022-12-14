use eyre::Result;
use reqwest::Client;

use ethers::{
    prelude::*,
    types::transaction::eip712::{EIP712Domain, EIP712WithDomain},
};

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
