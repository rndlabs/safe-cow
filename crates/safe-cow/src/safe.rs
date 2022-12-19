use std::{str::FromStr, sync::Arc};

use eyre::Result;

use ethers::{
    abi::AbiEncode,
    prelude::{k256::ecdsa::SigningKey, *},
    types::transaction::eip712::{EIP712Domain, EIP712WithDomain, Eip712},
    utils,
};
use safe_sdk::rpc::{
    common::{Operations, Paginated},
    msig_history::MsigTxResponse,
    propose::{MetaTransactionData, SafeGasConfig, SafeTransactionData},
};

use crate::{
    contracts::{
        erc1271_signature_validator::ERC1271SignatureValidator,
        erc20::{self, ERC20},
        gnosis_safe::GnosisSafe,
    },
    order::TokenAmount,
    Opts, SupportedChains,
};

/// Updated magic number from https://github.com/safe-global/safe-contracts/blob/main/contracts/handler/CompatibilityFallbackHandler.sol
/// EIP-1271 published magic number is [0x16, 0x26, 0xba, 0x7e];
const UPDATED_MAGIC_NUMBER: [u8; 4] = [0x20, 0xc1, 0x3b, 0x0b];

// The EIP-712 struct definition for a Safe Message
#[derive(Eip712, EthAbiType, Clone)]
#[eip712()]
pub struct SafeMessage {
    message: Bytes,
}

pub struct Safe {
    pub address: H160,
    pub contract: GnosisSafe<Provider<Http>>,
    pub owners: Vec<H160>,
    pub threshold: u32,
    pks: Option<Vec<String>>,
    pub provider: Arc<Provider<Http>>,
    pub chain: SupportedChains,
    pub base_url: String,
}

impl Safe {
    pub async fn new(opts: &Opts) -> Result<Self> {
        let provider = Provider::<Http>::try_from(opts.rpc_url.as_str())?;
        let address = *opts.safe.as_address().unwrap();
        let contract = GnosisSafe::new(address, provider.clone().into());

        let version_call = contract.version();
        let threshold_call = contract.get_threshold();
        let owners_call = contract.get_owners();

        let mut multicall = Multicall::new(provider.clone(), None)
            .await?
            .version(MulticallVersion::Multicall3);

        multicall
            .add_call(version_call, false)
            .add_call(threshold_call, false)
            .add_call(owners_call, false);

        let (version, threshold, owners): ((bool, String), (bool, U256), (bool, Vec<H160>)) =
            multicall.call().await?;

        if version.1 != "1.3.0" || threshold.1 == 0.into() {
            return Err(eyre::eyre!("Invalid Safe contract"));
        }

        // default to no private keys
        let pks = None;

        let chain = SupportedChains::get_chain(provider.clone().into()).await?;

        // generate the api url
        let base_url = format!(
            "https://safe-transaction-{}.safe.global/api/v1/safes/{}",
            chain.get_api_name_v2(),
            utils::to_checksum(&address, None)
        );

        Ok(Self {
            address,
            contract,
            owners: owners.1,
            threshold: threshold.1.as_u32(),
            pks,
            provider: provider.into(),
            chain,
            base_url,
        })
    }

    /// Prompt the user for private keys
    pub fn prompt_private_keys(&mut self) -> Result<()> {
        // if there are no owners, return an error
        if self.owners.is_empty() {
            return Err(eyre::eyre!("No owners provided"));
        }

        // if the keys are already set, return ok
        if self.pks.is_some() {
            return Ok(());
        }

        // print a blank line
        println!("Enter private keys for Safe owners:");

        let mut private_keys = vec![];
        loop {
            // Prompt the user for a private key
            let private_key = prompt_key(format!(
                "Private key #{} / {} ({} total owners): ",
                (private_keys.len() + 1),
                self.threshold,
                self.owners.len()
            ));

            // check to make sure that the private key corresponds to an owner
            let sk = SigningKey::from_bytes(&hex::decode(private_key.clone()).unwrap()).unwrap();
            let address = Wallet::from(sk).address();
            if !self.owners.contains(&address) {
                println!("Private key does not correspond to an owner");
                continue;
            }

            // check to make sure that the private key is not already in the list
            if private_keys.contains(&private_key) {
                println!("Private key already entered");
                continue;
            }

            // Add the private key to the list
            private_keys.push(private_key);

            // If we have enough private keys, we are done
            if private_keys.len() >= self.threshold as usize {
                break;
            }
        }

        // print a blank line for readability
        println!();

        // sort the private keys by their public key address
        // signatures are required to be ordered for consideration by the smart contract
        private_keys.sort_by(|a, b| {
            let a = SigningKey::from_bytes(&hex::decode(a).unwrap()).unwrap();
            let a = Wallet::from(a).address();
            let b = SigningKey::from_bytes(&hex::decode(b).unwrap()).unwrap();
            let b = Wallet::from(b).address();
            a.cmp(&b)
        });

        // set the private keys
        self.pks = Some(private_keys);

        Ok(())
    }

    /// Check if the Safe has any pending transactions that have not been executed
    /// This is done by querying for transactions with a nonce greater than the current nonce
    /// in the Safe transaction service API.
    pub async fn has_pending_txs(&self) -> Result<bool> {
        // get the current nonce
        let nonce: U256 = self.contract.nonce().call().await?;

        // get the transactions from the Safe transaction service API
        let url = format!(
            "{}/multisig-transactions/?nonce__gte={}",
            self.base_url, nonce
        );
        let results: Paginated<MsigTxResponse> = reqwest::get(&url).await?.json().await?;

        Ok(results.count > 0)
    }

    /// Check if the Safe has enough balance to cover the transaction
    pub async fn has_min_balance(&self, token: &TokenAmount) -> Result<bool> {
        let contract = ERC20::new(
            H160::from_str(token.token.address.as_str()).unwrap(),
            self.provider.clone(),
        );

        let balance = contract.balance_of(self.address).call().await?;

        Ok(balance >= token.amount)
    }

    /// Check if the nominated address has enough approval to cover the transaction
    pub async fn has_min_approval(&self, token: &TokenAmount, spender: H160) -> Result<bool> {
        let contract = ERC20::new(
            H160::from_str(token.token.address.as_str()).unwrap(),
            self.provider.clone(),
        );

        let allowance = contract.allowance(self.address, spender).call().await?;

        Ok(allowance >= token.amount)
    }

    /// Approve the nominated address to spend the token amount
    pub async fn approve(&self, token: &TokenAmount, spender: H160) -> Result<()> {
        // encode the call
        let call = erc20::ApproveCall {
            spender,
            amount: token.amount,
        };
        let call = Bytes::from(call.encode());

        let core = MetaTransactionData {
            to: safe_sdk::rpc::common::ChecksumAddress(H160::from_str(&token.token.address)?),
            value: 0,
            data: Some(call.clone()),
            operation: Some(Operations::Call),
        };

        let gas = SafeGasConfig {
            safe_tx_gas: 0,
            base_gas: 0,
            gas_price: 0,
            gas_token: safe_sdk::rpc::common::ChecksumAddress(H160::from_str(
                "0x0000000000000000000000000000000000000000",
            )?),
            refund_receiver: safe_sdk::rpc::common::ChecksumAddress(H160::from_str(
                "0x0000000000000000000000000000000000000000",
            )?),
        };

        // get the nonce
        let nonce = self.contract.nonce().call().await?.as_u64();

        let tx = SafeTransactionData {
            core: core.clone(),
            gas,
            nonce,
        };

        // sign the typed data and assemble all the signatures
        let mut signatures = Vec::new();
        for pk in self.pks.as_ref().unwrap() {
            let wallet = pk.parse::<LocalWallet>()?;
            let signature = tx
                .sign(&wallet, self.address, self.chain.get_chain_id())
                .await?;

            signatures.push(signature);
        }

        // join the signatures into a single bytes array
        let mut packed = Vec::new();
        for signature in signatures {
            packed.extend_from_slice(&signature.signature().to_vec());
        }

        // execute the transaction
        let to = H160::from_str(&token.token.address)?;
        let value = 0;
        // extract the number from the operation enum
        let operation = match core.operation.unwrap() {
            Operations::Call => 0,
            Operations::DelegateCall => 1,
        };

        let safe_tx_gas = 0;
        let base_gas = 0;
        let gas_price = 0;
        let gas_token = H160::from_str("0x0000000000000000000000000000000000000000")?;
        let refund_receiver = H160::from_str("0x0000000000000000000000000000000000000000")?;
        let signatures = Bytes::from(packed);

        // Prompt the user for a private key
        let private_key = prompt_key("Private key for account to submit transaction:".to_string());

        let provider = Arc::new({
            SignerMiddleware::new(
                self.provider.clone(),
                private_key
                    .parse::<LocalWallet>()?
                    .with_chain_id(self.chain.get_chain_id()),
            )
        });

        let contract = GnosisSafe::new(self.address, provider.clone());
        let tx = contract.exec_transaction(
            to,
            value.into(),
            call,
            operation,
            safe_tx_gas.into(),
            base_gas.into(),
            gas_price.into(),
            gas_token,
            refund_receiver,
            signatures,
        );
        let tx = tx.send().await?;

        print!(
            "Transaction hash {} submitted, waiting for 1 confirmation...",
            tx.tx_hash()
        );

        let receipt = tx.confirmations(1).await?;
        match receipt {
            Some(_receipt) => {
                println!("Transaction mined");
            }
            None => {
                println!("Transaction mining failed");
            }
        }

        // print a blank line
        println!();

        Ok(())
    }

    pub async fn sign(&self, message: &Bytes) -> Result<([u8; 32], Vec<u8>)> {
        // if there are no private keys, return an error
        if self.pks.is_none() {
            return Err(eyre::eyre!("No private keys provided"));
        }

        let safe_message = SafeMessage {
            message: message.clone(),
        };
        let safe_message = EIP712WithDomain::new(safe_message)?.set_domain(EIP712Domain {
            name: None,
            version: None,
            chain_id: Some(self.provider.get_chainid().await?),
            verifying_contract: Some(self.address),
            salt: None,
        });

        // sign the typed data and assemble all the signatures
        let mut signatures = Vec::new();
        for pk in self.pks.as_ref().unwrap() {
            let wallet = pk.parse::<LocalWallet>()?;
            let signature = wallet.sign_typed_data(&safe_message).await?;

            signatures.push(signature);
        }

        // join the signatures into a single bytes array
        let mut packed = Vec::new();
        for signature in signatures {
            packed.extend_from_slice(&signature.to_vec());
        }

        Ok((safe_message.encode_eip712().unwrap(), packed))
    }

    /// Verify the signature of a hash against a contract that implements the
    /// EIP-1271 interface.
    pub async fn verify_signature(&self, data: &Bytes, signature: &[u8]) -> Result<bool> {
        let contract = ERC1271SignatureValidator::new(self.address, self.provider.clone());

        // convert digest to bytes32 for the contract call
        let eip1271_result: [u8; 4] = contract
            .is_valid_signature(data.clone(), Bytes::from(signature.to_owned()))
            .call()
            .await?;

        // check the result against the magic number
        Ok(eip1271_result == UPDATED_MAGIC_NUMBER)
    }

    pub async fn get_safe_app_url(&self) -> Result<String> {
        let chain = SupportedChains::get_chain(self.provider.clone()).await?;

        Ok(match chain {
            SupportedChains::Mainnet => format!("{}/{}:", self.base_url, "eth"),
            SupportedChains::Goerli => format!("{}/{}:", self.base_url, "gor"),
            SupportedChains::Gnosis => format!("{}/{}:", self.base_url, "gno"),
        })
    }
}

fn prompt_key(msg: String) -> String {
    let private_key = loop {
        // Prompt the user for a private key
        let pk = rpassword::prompt_password(&msg).unwrap();

        // Validate the private key
        if pk.len() != 64 {
            println!("Invalid private key length");
            continue;
        }

        if let Err(e) = hex::decode(&pk) {
            println!("Invalid private key: {e}");
            continue;
        }

        break pk;
    };

    private_key
}
