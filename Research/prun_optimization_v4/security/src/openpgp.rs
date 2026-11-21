//! Optional OpenPGP interoperability tools. Not used as the native agent
//! message protocol. Import/verify public keys through the CA identity registry
//! before relying on their signatures.
use std::{fs, path::PathBuf};

use anyhow::Result;
use pgp_compat as pgp;

pub async fn encrypt(pubkey:PathBuf, input:PathBuf, output:PathBuf) -> Result<()> {
	let pk = pgp::read_pkey_from_path(pubkey).await?;
	let plaintext = fs::read(input)?;
	fs::write(output, pgp::encrypt(vec![pk], plaintext).await?)?;
	Ok(())
}
pub async fn decrypt(secretkey:PathBuf, passphrase:&str, input:PathBuf, output:PathBuf) -> Result<()> {
	let sk = pgp::read_skey_from_file(secretkey).await?;
	let message = fs::read(input)?;
	fs::write(output, pgp::decrypt(sk, passphrase, message).await?)?;
	Ok(())
}
pub async fn sign(secretkey:PathBuf, passphrase:&str, input:PathBuf, signature:PathBuf) -> Result<()> {
	let sk = pgp::read_skey_from_file(secretkey).await?;
	let input = fs::read(input)?;
	fs::write(signature, pgp::sign(sk, passphrase, input).await?)?;
	Ok(())
}
pub async fn verify(pubkey:PathBuf, input:PathBuf, signature:PathBuf) -> Result<()> {
	let pk = pgp::read_pkey_from_path(pubkey).await?;
	let sig = pgp::read_sig_from_bytes(fs::read(signature)?).await?;
	pgp::verify(pk, sig, fs::read(input)?).await?;
	Ok(())
}
