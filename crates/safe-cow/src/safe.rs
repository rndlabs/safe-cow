use std::{str::FromStr, sync::Arc};

use eyre::Result;

use ethers::{
    prelude::{k256::ecdsa::SigningKey, *},
    types::transaction::eip712::{EIP712Domain, EIP712WithDomain, Eip712},
    utils,
};
use safe_sdk::rpc::{common::Paginated, msig_history::MsigTxResponse};

use crate::{order::TokenAmount, Opts, SupportedChains, contracts::{erc20::{ERC20, self}, erc1271_signature_validator::ERC1271SignatureValidator, gnosis_safe::GnosisSafe}};

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
            let private_key = rpassword::prompt_password(format!(
                "Private key #{} / {} ({} total owners): ",
                (private_keys.len() + 1),
                self.threshold,
                self.owners.len()
            ))?;

            // If the private key is empty, we are done
            if private_key.is_empty() {
                break;
            }

            // Validate the private key
            if private_key.len() != 64 {
                println!("Invalid private key length");
                continue;
            }

            if let Err(e) = hex::decode(&private_key) {
                println!("Invalid private key: {e}");
                continue;
            }

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

    pub async fn approve(&self, token: &TokenAmount, spender: H160) -> Result<()> {
        unimplemented!("Approval not yet implemented");
        // let contract = ERC20::new(
        //     H160::from_str(token.token.address.as_str()).unwrap(),
        //     self.provider.clone(),
        // );

        // let tx = contract
        //     .approve(spender, token.amount)
        //     .nonce(self.provider.get_nonce(self.address).await?)
        //     .gas_price(self.provider.get_gas_price().await?)
        //     .gas(100_000)
        //     .send()
        //     .await?;

        // println!("Waiting for approval transaction to be mined...");
        // self.provider.wait_for_transaction(tx).await?;

        // Ok(())
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
    pub async fn verify_signature(&self, data: &Bytes, signature: &Vec<u8>) -> Result<bool> {
        let contract = ERC1271SignatureValidator::new(self.address, self.provider.clone());

        // convert digest to bytes32 for the contract call
        let eip1271_result: [u8; 4] = contract
            .is_valid_signature(data.clone(), Bytes::from(signature.clone()))
            .call()
            .await?;

        // check the result against the magic number
        Ok(eip1271_result == UPDATED_MAGIC_NUMBER)
    }

    pub async fn get_safe_app_url(&self) -> Result<String> {
        let chain = SupportedChains::get_chain(self.provider.clone().into()).await?;

        Ok(match chain {
            SupportedChains::Mainnet => format!("{}/{}:", self.base_url, "eth"),
            SupportedChains::Goerli => format!("{}/{}:", self.base_url, "gor"),
            SupportedChains::Gnosis => format!("{}/{}:", self.base_url, "gno"),
        })
    }
}
