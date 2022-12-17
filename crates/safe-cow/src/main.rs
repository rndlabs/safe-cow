use std::sync::Arc;

use clap::Parser;
use eyre::Error;

use ethers::prelude::*;

use safe_cow::{
    Opts,
    Commands,
    SupportedChains,
    order,
    safesigner,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    banner();

    // parse the command line options
    let opts = Opts::parse();

    // connect to the RPC for re-use
    let provider = Arc::new(Provider::<Http>::try_from(&opts.rpc_url)?);
    let chain = SupportedChains::get_chain(provider.clone()).await?;

    println!(
        "Connected to chain: {} ({})",
        chain.get_name(),
        opts.rpc_url
    );

    // check if the safe address is valid
    let is_valid = safesigner::verify_is_safe(*opts.safe.as_address().unwrap(), provider.clone()).await;
    match is_valid {
        Ok(is_valid) => {
            if !is_valid {
                println!("Safe address is invalid");
                return Ok(());
            } else {
                println!(
                    "Transacting with safe: {}",
                    *opts.safe.as_address().unwrap()
                );
            }
        }
        Err(e) => {
            println!("Safe address is invalid: {}", e);
            return Ok(());
        }
    }

    // separator
    println!();

    let subcommand = opts.subcommand.clone();

    // run the subcommand
    match subcommand {
        // Create an order for a safe
        Commands::CreateOrder(order) => {
            order::create_order(order, &opts, provider, chain).await?;
        }
        // Cancel an order for a safe
        Commands::CancelOrder(order) => {
            order::cancel_order(order, &opts, provider, chain).await?;
        }
        // Sign a message with a safe
        Commands::SignMessage(message) => {
            safesigner::run(message, &opts, provider).await?;
        }
    }
    Ok(())
}

/// Prints the banner
fn banner() -> () {
    println!(
        r#"
             /( ,,,,, )\
            _\,;;;;;;;,/_
         .-"; ;;;;;;;;; ;"-.
         '.__/`_ / \ _`\__.'
            | (')| |(') |
            | .--' '--. |
            |/ o     o \|
            |           |
           / \ _..=.._ / \
          /:. '._____.'   \
         ;::'    / \      .;
         |     _|_ _|_   ::|            ███████  █████  ███████ ███████      ██████  ██████  ██     ██ 
       .-|     '==o=='    '|-.          ██      ██   ██ ██      ██          ██      ██    ██ ██     ██ 
      /  |  . /       \    |  \         ███████ ███████ █████   █████       ██      ██    ██ ██  █  ██ 
      |  | ::|         |   | .|              ██ ██   ██ ██      ██          ██      ██    ██ ██ ███ ██
      |  (  ')         (.  )::|         ███████ ██   ██ ██      ███████      ██████  ██████   ███ ███
      |: |   |; U U U ;|:: | `|
      |' |   | \ U U / |'  |  |
      ##V|   |_/`"""`\_|   |V##
jgs      ##V##         ##V##

"#
    )
}
