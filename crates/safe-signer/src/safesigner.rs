use std::sync::Arc;

use eyre::Result;

use ethers::{
    prelude::*,
    types::transaction::eip712::{Eip712, EIP712WithDomain, EIP712Domain}
};

use crate::{SignMessage, Opts};
// const MAGIC_NUMBER: [u8; 4] = [0x16, 0x26, 0xba, 0x7e];
const MAGIC_NUMBER: [u8; 4] = [0x20, 0xc1, 0x3b, 0x0b];


// The EIP-712 struct definition for a Safe Message
#[derive(Eip712, EthAbiType, Clone)]
#[eip712()]
pub struct SafeMessage {
    message: Bytes
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

/// Run the sign message command
pub async fn run(config: SignMessage, opts: &Opts) -> Result<()> {
    // test if the message is a valid hex string otherwise encode string as bytes
    let message = if config.message.starts_with("0x") {
        hex::decode(&config.message[2..])?
    } else {
        config.message.as_bytes().to_vec()
    };

    let message = Bytes::from(message);
    println!("Message to sign with the safe: {message:#}");

    // get the safe signature
    let (digest, safe_signature) = safe_signature_of_message(
        &message,
        opts
    ).await?;

    // check if the signature is valid
    let valid = verify_signature(
        &message,
        &safe_signature,
        opts
    ).await?;

    // print the signature
    println!("Signature: 0x{}", hex::encode(safe_signature));

    if valid {
        println!("Signature is valid");
    } else {
        println!("Signature is invalid");
    }

    Ok(())
}

pub async fn safe_signature_of_message(
    message: &Bytes,
    opts: &Opts,
) -> Result<([u8; 32], Vec<u8>)> {
    let provider = Provider::<Http>::try_from(&opts.rpc_url)?;
    let safe_message = SafeMessage { message: message.clone() };
    let safe_message = EIP712WithDomain::new(safe_message)?.set_domain(EIP712Domain {
        name: None,
        version: None,
        chain_id: Some(provider.get_chainid().await?),
        verifying_contract: Some(*opts.safe.as_address().unwrap()),
        salt: None,
    });

    let pks = prompt_private_keys()?;

    // sign the typed data and assemble all the signatures
    let mut signatures = Vec::new();
    for pk in pks {
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
pub async fn verify_signature(
    data: &Bytes,
    signature: &Vec<u8>,
    opts: &Opts,
) -> Result<bool> {
    let provider = Provider::<Http>::try_from(opts.rpc_url.as_str())?;
    let contract = ERC1271SignatureValidator::new(*opts.safe.as_address().unwrap(), provider.into());

    // convert digest to bytes32 for the contract call
    let eip1271_result: [u8; 4] = contract
        .is_valid_signature(data.clone(), Bytes::from(signature.clone()))
        .call()
        .await?;

    // check the result against the magic number
    Ok(eip1271_result == MAGIC_NUMBER)
}

/// Verify with a high certainty that the address is a Safe contract
pub async fn verify_is_safe<M>(
    safe: Address,
    provider: Arc<Provider<M>>,
) -> Result<bool> 
where M: JsonRpcClient + 'static {
    let contract = GnosisSafe::new(safe, provider.clone());

    // check that the version = 1.3.0 and the threshold is 1 or greater
    let version: String = contract.version().call().await?;
    let threshold: U256 = contract.get_threshold().call().await?;

    Ok(version == "1.3.0" && threshold > 0.into())
}

/// Prompt the user for private keys
pub fn prompt_private_keys() -> Result<Vec<String>> {
    let mut private_keys = vec![];
    loop {
        // Prompt the user for a private key
        let private_key = rpassword::prompt_password(format!(
            "Enter private key #{} (leave blank to stop): ",
            (private_keys.len() + 1)
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

        // Add the private key to the list
        private_keys.push(private_key);
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
    Ok(private_keys)
}