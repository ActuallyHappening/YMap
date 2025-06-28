use alloy::signers::Signer as _;
use alloy_primitives::Signature;

use crate::prelude::*;

pub struct EthPrivateKey(alloy::signers::local::PrivateKeySigner);

impl EthPrivateKey {
	pub fn new(str: &str) -> color_eyre::Result<Self> {
		Ok(Self(str.parse().wrap_err(
			"Couldn't parse string as hex encoded ETH private key",
		)?))
	}

	pub async fn sign(&self, msg: &[u8]) -> color_eyre::Result<Signature> {
		let (sig, recovery_id) = self.0.clone().into_credential().sign_recoverable(&msg)?;
		eprintln!("Sig: {sig:?}, recovery: {recovery_id:?}");
		todo!()
	}
}

#[tokio::test]
async fn sig_experimenting() -> color_eyre::Result<()> {
	let private_key = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/private-key"));
	let signer = EthPrivateKey::new(private_key)?;

	let message = b"Hello world!";
	signer.sign(&message[..]).await?;

	Ok(())
}

pub trait Signer {
	async fn sign(&self, msg: Vec<u8>) -> Signature;
	async fn verify(&self, msg: Vec<u8>, sig: Signature) -> color_eyre::Result<()>;
}
