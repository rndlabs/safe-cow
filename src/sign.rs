use eyre::Result;

use ethers::types::Bytes;

use crate::{safe::Safe, SignMessage};

/// Run the sign message command
pub async fn run(config: SignMessage, safe: &Safe) -> Result<()> {
    // test if the message is a valid hex string otherwise encode string as bytes
    let message = if config.message.starts_with("0x") {
        hex::decode(&config.message[2..])?
    } else {
        config.message.as_bytes().to_vec()
    };

    let message = Bytes::from(message);
    println!("Message to sign with the safe: {message:#}");

    // get the safe signature
    let (_digest, safe_signature) = safe.sign(&message).await?;

    // check if the signature is valid
    let valid = safe.verify_signature(&message, &safe_signature).await?;

    // print the signature
    println!("Signature: 0x{}", hex::encode(safe_signature));

    if valid {
        println!("Signature is valid");
    } else {
        println!("Signature is invalid");
    }

    Ok(())
}
