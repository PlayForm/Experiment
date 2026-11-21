//! OpenSSH-compatible short-lived user certificates issued with pure-Rust
//! ssh-key. This issues and checks SSH certs; it is not an interactive SSH
//! server.
use std::{
	fs,
	path::Path,
	time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Result, bail};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use ssh_key::{
	Algorithm,
	LineEnding,
	PrivateKey,
	PublicKey,
	certificate::{Builder, CertType},
};

use crate::{crypto, storage};

pub fn initialize(state:&Path) -> Result<()> {
	let p = state.join("offline/ssh-ca.key");
	if p.exists() {
		bail!("SSH CA exists")
	}
	let key = PrivateKey::random(&mut OsRng, Algorithm::Ed25519)?;
	storage::secret(&p, key.to_openssh(LineEnding::LF)?.as_bytes())?;
	storage::public(&state.join("public/ssh-ca.pub"), key.public_key().to_openssh()?.as_bytes())?;
	storage::audit(
		state,
		"SSH_CA_CREATED",
		"ssh-ca",
		"admin_bootstrap",
		&crypto::sha(key.public_key().to_openssh()?.as_bytes()),
	)
}
pub fn issue(state:&Path, identity:&str, subject_pub:&Path, output:&Path, reason:&str) -> Result<()> {
	storage::identity_name(identity)?;
	crypto::validate_identity(state, identity)?;
	let signer = PrivateKey::read_openssh_file(&state.join("offline/ssh-ca.key"))?;
	let subject = PublicKey::read_openssh_file(subject_pub)?;
	let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
	// Example principal is a fixed dedicated account, NOT root.
	let mut builder =
		Builder::new_with_random_nonce(&mut OsRng, subject.key_data().clone(), now.saturating_sub(10), now + 600)?;
	let mut serial_hash = Sha256::new();
	serial_hash.update(identity.as_bytes());
	serial_hash.update(now.to_be_bytes());
	let digest = serial_hash.finalize();
	let serial = u64::from_be_bytes(digest[..8].try_into()?);
	builder.serial(serial)?;
	builder.key_id(format!("run-lab:{identity}:{serial}"))?;
	builder.cert_type(CertType::User)?;
	builder.valid_principal("lab-maintenance")?;
	// No forwarding, no agent forwarding, no unrestricted shell / PTY
	// extensions. Actual SSH server must separately enforce the principal and
	// command allow-list.
	let cert = builder.sign(&signer)?;
	let public = signer.public_key();
	cert.validate(&[public.fingerprint(ssh_key::HashAlg::Sha256)])?;
	storage::public(output, cert.to_openssh()?.as_bytes())?;
	storage::audit(
		state,
		"SSH_CERT_ISSUED",
		identity,
		reason,
		&crypto::sha(cert.to_openssh()?.as_bytes()),
	)?;
	Ok(())
}
