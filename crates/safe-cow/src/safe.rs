use std::sync::Arc;

use eyre::Result;

use ethers::{
    prelude::{k256::ecdsa::SigningKey, *},
    types::transaction::eip712::{EIP712Domain, EIP712WithDomain, Eip712},
};

use crate::Opts;

/// Updated magic number from https://github.com/safe-global/safe-contracts/blob/main/contracts/handler/CompatibilityFallbackHandler.sol
/// EIP-1271 published magic number is [0x16, 0x26, 0xba, 0x7e];
const UPDATED_MAGIC_NUMBER: [u8; 4] = [0x20, 0xc1, 0x3b, 0x0b];

// The EIP-712 struct definition for a Safe Message
#[derive(Eip712, EthAbiType, Clone)]
#[eip712()]
pub struct SafeMessage {
    message: Bytes,
}

// Generate the type-safe contract bindings for the EIP-1271 interface
abigen!(
    ERC1271SignatureValidator,
    "./abi/ERC1271SignatureValidator.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

abigen!(
    GnosisSafe,
    "./abi/GnosisSafe.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

pub struct Safe {
    pub address: H160,
    pub contract: GnosisSafe<Provider<Http>>,
    pub owners: Vec<H160>,
    pub threshold: u32,
    pks: Option<Vec<String>>,
    pub provider: Arc<Provider<Http>>,
}

impl Safe {
    pub async fn new(opts: &Opts) -> Result<Self> {
        let provider = Provider::<Http>::try_from(opts.rpc_url.as_str())?;
        let contract = GnosisSafe::new(*opts.safe.as_address().unwrap(), provider.clone().into());

        // check that the version = 1.3.0 and the threshold is 1 or greater
        // TODO: Assemble this into a single call to the contract
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

        Ok(Self {
            address: *opts.safe.as_address().unwrap(),
            contract,
            owners: owners.1,
            threshold: threshold.1.as_u32(),
            pks,
            provider: provider.into(),
        })
    }

    /// Prompt the user for private keys
    pub fn prompt_private_keys(&mut self) -> Result<()> {
        // if there are no owners, return an error
        if self.owners.is_empty() {
            return Err(eyre::eyre!("No owners provided"));
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
}
