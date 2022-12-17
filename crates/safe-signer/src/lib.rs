use clap::{Args, Parser, Subcommand};
use eyre::Result;
use ethers::{
    prelude::*,
    core::k256::ecdsa::SigningKey,
};

pub mod order;
pub mod safesigner;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Create a token swap order and submit it to the Cowswap API
    CreateOrder(CreateOrder),
    /// Cancel an existing order on the Cowswap API
    CancelOrder(CancelOrder),
    /// Sign an arbitrary message off-line using EIP-1271
    SignMessage(SignMessage),
}

#[derive(Parser, Debug, Clone)]
#[clap(name = "safe-cow", version, author, about)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcommand: Commands,
    #[clap(
        long,
        value_parser,
        default_value = "http://erigon.dappnode:8545/",
        env = "RPC_URL",
        help = "The RPC to connect to"
    )]
    pub rpc_url: String,
    #[clap(
        help = "The Safe address the order is from",
        value_parser = parse_name_or_address,
        value_name = "SAFE",
        env = "SAFE_ADDRESS")
    ]
    pub safe: NameOrAddress,
}

#[derive(Args, Debug, Clone)]
pub struct CreateOrder {
    #[clap(
        help = "The address of the token to buy",
        value_parser = parse_name_or_address,
        value_name = "BUY_TOKEN"
    )]
    pub buy_token: Option<NameOrAddress>,
    #[clap(help = "The amount of buy token to buy", value_name = "BUY_AMOUNT")]
    pub buy_amount: Option<String>,
    #[clap(
        help = "The address of the token to sell",
        value_parser = parse_name_or_address,
        value_name = "SELL_TOKEN",
    )]
    pub sell_token: Option<NameOrAddress>,
    #[clap(help = "The amount of sell token to sell", value_name = "SELL_AMOUNT")]
    pub sell_amount: Option<String>,
    #[clap(
        help = "How long the order is valid for in seconds",
        value_parser,
        value_name = "VALID_TO",
        default_value = "1200"
    )]
    pub valid_to: Option<u32>,
    #[clap(
        help = "Use the Cowswap staging environment?",
        value_name = "STAGING",
        default_value = "true"
    )]
    pub staging: Option<bool>,
}

#[derive(Args, Debug, Clone)]
pub struct CancelOrder {
    #[clap(
        help = "The order to cancel",
        value_name = "ORDER",
        value_parser = parse_order_uid,
    )]
    pub order_uid: Vec<u8>,
}

#[derive(Args, Debug, Clone)]
pub struct SignMessage {
    #[clap(
        help = "The message to sign. If prepended with 0x, it will be interpreted as hex, otherwise as a string.",
        value_name = "MESSAGE"
    )]
    pub message: String,
}

/// A `clap` `value_parser` that parses a `NameOrAddress` from a string
pub fn parse_name_or_address(s: &str) -> Result<NameOrAddress> {
    Ok(if s.starts_with("0x") {
        NameOrAddress::Address(s.parse()?)
    } else {
        NameOrAddress::Name(s.to_string())
    })
}

/// A `clap` `value_parser` that parses an order UID from a string
pub fn parse_order_uid(s: &str) -> Result<[u8; 56]> {
    let mut order_uid = [0u8; 56];
    order_uid.copy_from_slice(&hex::decode(s)?);
    Ok(order_uid)
}

/// A `clap` `value_parser` that removes a `0x` prefix if it exists
pub fn strip_0x_prefix(s: &str) -> Result<String, &'static str> {
    Ok(s.strip_prefix("0x").unwrap_or(s).to_string())
}

/// Supported chains
pub enum SupportedChains {
    Mainnet,
    Goerli,
    Gnosis,
}

/// Utility functions for SupportedChains
impl SupportedChains {
    pub async fn get_chain<M>(provider: Arc<Provider<M>>) -> Result<SupportedChains>
    where
        M: JsonRpcClient,
    {
        match provider.get_chainid().await {
            Ok(chain_id) => match SupportedChains::get_by_chain_id(chain_id.as_u64()) {
                Some(chain) => Ok(chain),
                None => panic!("Unsupported chain ID: {}", chain_id),
            },
            Err(_) => panic!("Failed to get chain ID"),
        }
    }

    pub fn get_chain_id(&self) -> u64 {
        match self {
            SupportedChains::Mainnet => 1,
            SupportedChains::Goerli => 5,
            SupportedChains::Gnosis => 100,
        }
    }

    pub fn get_by_chain_id(chain_id: u64) -> Option<SupportedChains> {
        match chain_id {
            1 => Some(SupportedChains::Mainnet),
            5 => Some(SupportedChains::Goerli),
            100 => Some(SupportedChains::Gnosis),
            _ => None,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            SupportedChains::Mainnet => "Ethereum Mainnet".to_string(),
            SupportedChains::Goerli => "Görli Testnet".to_string(),
            SupportedChains::Gnosis => "Gnosis Testnet".to_string(),
        }
    }

    pub fn get_api_name(&self) -> String {
        match self {
            SupportedChains::Mainnet => "mainnet".to_string(),
            SupportedChains::Goerli => "goerli".to_string(),
            SupportedChains::Gnosis => "xdai".to_string(),
        }
    }
}

// Get the Cowswap API URL for a given chain and if it is a staging environment
pub fn get_cowswap_api_url(chain: &SupportedChains, staging: bool) -> String {
    let api_name = chain.get_api_name();
    if !staging {
        format!("https://api.cow.fi/{api_name}/api/v1")
    } else {
        format!("https://barn.api.cow.fi/{api_name}/api/v1")
    }
}

// Get the Cowswap Explorer URL for a given chain and if it is a staging environment
pub fn get_cowswap_explorer_url(chain: &SupportedChains, staging: bool) -> String {
    let api_name = chain.get_api_name();
    if !staging {
        format!("https://explorer.cow.fi/{api_name}")
    } else {
        format!("https://barn.explorer.cow.fi/{api_name}")
    }
}

// store settlement contracts for each chain
pub enum SettlementContract {
    Mainnet,
    Goerli,
    Gnosis,
}

impl SettlementContract {
    pub fn get_address(&self) -> Address {
        match self {
            SettlementContract::Mainnet => "0x9008D19f58AAbD9eD0D60971565AA8510560ab41"
                .parse()
                .unwrap(),
            SettlementContract::Goerli => "0x9008D19f58AAbD9eD0D60971565AA8510560ab41"
                .parse()
                .unwrap(),
            SettlementContract::Gnosis => "0x9008D19f58AAbD9eD0D60971565AA8510560ab41"
                .parse()
                .unwrap(),
        }
    }

    pub fn get_by_chain(chain: &SupportedChains) -> SettlementContract {
        match chain {
            SupportedChains::Mainnet => SettlementContract::Mainnet,
            SupportedChains::Goerli => SettlementContract::Goerli,
            SupportedChains::Gnosis => SettlementContract::Gnosis,
        }
    }
}

/// Connect to an RPC node and get the chain id
pub async fn get_chain_id(rpc_url: &str) -> Result<u64> {
    let provider =
        Provider::<Http>::try_from(rpc_url).expect("Could not instantiate HTTP provider");

    let chain_id = provider.get_chainid().await?;
    Ok(chain_id.as_u64())
}