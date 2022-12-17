use std::sync::Arc;

use clap::Parser;
use eyre::Error;

use ethers::prelude::*;

use safe_cow::{
    order,
    safesigner::{self, verify_is_safe},
    Commands, Opts, SupportedChains,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    banner();

    // parse the command line options
    let opts = Opts::parse();

    // connect to the RPC
    let provider = Arc::new(Provider::<Http>::try_from(&opts.rpc_url)?);
    let chain = SupportedChains::get_chain(provider.clone()).await?;

    println!(
        "Connected to chain: {} ({})",
        chain.get_name(),
        opts.rpc_url
    );

    // check if the safe address is valid
    let is_valid = verify_is_safe(*opts.safe.as_address().unwrap(), provider.clone()).await;
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

    // print a space
    println!("");

    let subcommand = opts.subcommand.clone();

    match subcommand {
        Commands::CreateOrder(order) => {
            order::run(order, &opts, provider, chain).await?;
        }
        Commands::CancelOrder(mut order) => {
            // not implemented yet
            unimplemented!();
            // order.safe.private_keys = prompt_private_keys(order.safe.private_keys)?;
            // order::cancel(order).await?;
        }
        Commands::SignMessage(message) => {
            // opts.private_keys = prompt_private_keys(&opts.private_keys)?;
            safesigner::run(message, &opts, provider).await?;
        }
    }
    Ok(())
}

// Prints an ASCII banner
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
