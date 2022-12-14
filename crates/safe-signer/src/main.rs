use eyre::Error;
use clap::Parser;

use ethers::prelude::*;

use safe_signer::{order, Commands, Opts, safesigner};

#[tokio::main]
async fn main() -> Result<(), Error> {
    banner();

    // parse the command line options
    let opts = Opts::parse();

    // connect to the RPC    
    let provider = Provider::<Http>::try_from(opts.rpc_url.clone())?;

    // determine the name of chain the RPC is connected to
    let chain_id = provider.get_chainid().await?;

    match opts.subcommand {
        Commands::CreateOrder(order) => {
            order::run(order, &opts).await?;
        }
        Commands::CancelOrder(mut order) => {
            // not implemented yet
            unimplemented!();
            // order.safe.private_keys = prompt_private_keys(order.safe.private_keys)?;
            // order::cancel(order).await?;
        }
        Commands::SignMessage(message) => {
            // opts.private_keys = prompt_private_keys(&opts.private_keys)?;
            safesigner::run(message, &opts).await?;
        }
    }
    Ok(())
}

/// If a vector of private keys is not provided, prompt the user to enter them
/// one by one.
/// 
/// # Examples
/// 
/// ```
/// use safe_signer::prompt_private_keys;
/// 
/// let private_keys = prompt_private_keys().unwrap();
/// ```
/// 
/// # Safety
/// 
/// This function is not thread-safe.
/// 
fn prompt_private_keys(pks: &Option<Vec<String>>) -> Result<Option<Vec<String>>, Error> {
    let mut private_keys = pks.clone().unwrap_or_default();
    
    if private_keys.is_empty() {
        private_keys = safe_signer::prompt_private_keys()?
    }

    Ok(Some(private_keys))
}

// Prints an ASCII banner
fn banner() -> () {
    println!(r#"
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

"#)
}