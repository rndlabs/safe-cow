use std::sync::Arc;

use clap::Parser;
use eyre::Error;

use ethers::prelude::*;

use safe_cow::{order, safe, sign, Commands, Opts, SupportedChains};

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

    // connect to the safe
    let mut safe = safe::Safe::new(&opts).await?;
    println!("Connected to safe: {}", safe.address);

    // separator
    println!();

    let subcommand = opts.subcommand.clone();

    // run the subcommand
    match subcommand {
        // Create an order for a safe
        Commands::CreateOrder(order) => {
            order::create_order(order, &opts, &mut safe, provider, chain).await?;
        }
        // Cancel an order for a safe
        Commands::CancelOrder(order) => {
            order::cancel_order(order, &opts, &safe, provider, chain).await?;
        }
        // Sign a message with a safe
        Commands::SignMessage(message) => {
            sign::run(message, &safe).await?;
        }
    }
    Ok(())
}

/// Prints the banner
fn banner() {
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
