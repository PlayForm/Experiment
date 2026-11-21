//! Recipient-only HPKE; independent Ed25519 signatures; CA-attested key
//! bindings.
use std::{fs, path::Path};

use anyhow::{Context, Result, anyhow, bail};
use base64::{Engine, engine::general_purpose::STANDARD as B64};
use chrono::Utc;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use hpke::{
	Deserializable,
	Kem,
	OpModeR,
	OpModeS,
	Serializable,
	aead::ChaCha20Poly1305,
	kdf::HkdfSha256,
	kem::X25519HkdfSha256,
	setup_receiver,
	setup_sender,
};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const INFO:&[u8] = b"prun-lab/v4/agent-message/hpke-x25519-hkdfsha256-chacha20poly1305";
const SIG_DOMAIN:&[u8] = b"prun-lab/v4/agent-message/ed25519\0";
const REG_DOMAIN:&[u8] = b"prun-lab/v4/identity-attestation/ed25519\0";
type K = X25519HkdfSha256;
type A = ChaCha20Poly1305;
type D = HkdfSha256;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicIdentity {
	pub identity:String,
	pub role:String,
	pub sign_public_hex:String,
	pub encrypt_public_b64:String,
	pub tls_certificate_sha256:String,
	pub allowed_message_types:Vec<String>,
	pub created_unix:i64,
	pub expires_unix:i64,
	pub issuance_id:String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttestedIdentity {
	pub subject:PublicIdentity,
	pub authority_signature_b64:String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProtectedHeader {
	pub version:u32,
	pub message_id:String,
	pub sender:String,
	pub recipient:String,
	pub kind:String,
	pub campaign:String,
	pub created_unix:i64,
	pub expires_unix:i64,
	pub nonce_hex:String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptedEnvelope {
	pub protected:ProtectedHeader,
	pub encapsulation_b64:String,
	pub ciphertext_b64:String,
	pub signature_b64:String,
}
#[derive(Serialize)]
struct SignedParts<'a> {
	protected:&'a ProtectedHeader,
	encapsulation_b64:&'a str,
	ciphertext_b64:&'a str,
}

fn keyed_payload(prefix:&[u8], bytes:&[u8]) -> Vec<u8> {
	let mut v = Vec::with_capacity(prefix.len() + bytes.len());
	v.extend_from_slice(prefix);
	v.extend_from_slice(bytes);
	v
}
fn fixed_32(input:&str) -> Result<[u8; 32]> {
	let bytes = hex::decode(input)?;
	bytes.try_into().map_err(|_| anyhow!("expected 32-byte key"))
}
fn signing_key(path:&Path) -> Result<SigningKey> {
	Ok(SigningKey::from_bytes(&fixed_32(fs::read_to_string(path)?.trim())?))
}
fn authority_public(state:&Path) -> Result<VerifyingKey> {
	Ok(VerifyingKey::from_bytes(&fixed_32(
		&fs::read_to_string(state.join("public/message-authority.hex"))?.trim(),
	)?)?)
}
pub fn generate_sign_key(path:&Path) -> Result<String> {
	let sk = SigningKey::generate(&mut OsRng);
	super::storage::secret(path, hex::encode(sk.to_bytes()).as_bytes())?;
	Ok(hex::encode(sk.verifying_key().to_bytes()))
}
pub fn generate_hpke_key(path:&Path) -> Result<String> {
	let (sk, pk) = K::gen_keypair();
	super::storage::secret(path, B64.encode(sk.to_bytes()).as_bytes())?;
	Ok(B64.encode(pk.to_bytes()))
}
pub fn attest(state:&Path, identity:PublicIdentity) -> Result<AttestedIdentity> {
	let sk = signing_key(&state.join("offline/message-authority.hex"))?;
	let bytes = serde_json::to_vec(&identity)?;
	let signature = sk.sign(&keyed_payload(REG_DOMAIN, &bytes));
	Ok(AttestedIdentity { subject:identity, authority_signature_b64:B64.encode(signature.to_bytes()) })
}
pub fn validate_identity(state:&Path, name:&str) -> Result<AttestedIdentity> {
	super::storage::identity_name(name)?;
	let entry:AttestedIdentity =
		serde_json::from_slice(&fs::read(state.join("registry").join(format!("{name}.json")))?)?;
	if entry.subject.identity != name {
		bail!("identity record name mismatch")
	}
	if state.join("revoked").join(name).exists() {
		bail!("identity revoked")
	}
	let msg = keyed_payload(REG_DOMAIN, &serde_json::to_vec(&entry.subject)?);
	let signature = Signature::from_slice(&B64.decode(&entry.authority_signature_b64)?)?;
	authority_public(state)?
		.verify(&msg, &signature)
		.context("invalid CA identity attestation")?;
	let now = Utc::now().timestamp();
	if now < entry.subject.created_unix || now >= entry.subject.expires_unix {
		bail!("identity outside validity window")
	}
	Ok(entry)
}
fn signed_bytes(e:&EncryptedEnvelope) -> Result<Vec<u8>> {
	let p = SignedParts {
		protected:&e.protected,
		encapsulation_b64:&e.encapsulation_b64,
		ciphertext_b64:&e.ciphertext_b64,
	};
	Ok(keyed_payload(SIG_DOMAIN, &serde_json::to_vec(&p)?))
}
pub fn seal(
	state:&Path,
	sender:&str,
	receiver:&str,
	kind:&str,
	campaign:&str,
	plaintext:&[u8],
) -> Result<EncryptedEnvelope> {
	if plaintext.len() > 8 * 1024 * 1024 {
		bail!("message too large")
	}
	let author = validate_identity(state, sender)?;
	let recipient = validate_identity(state, receiver)?;
	if !author.subject.allowed_message_types.iter().any(|x| x == kind) {
		bail!("sender not permitted to send this kind")
	}
	let pk_bytes = B64.decode(recipient.subject.encrypt_public_b64)?;
	let pk = <K as Kem>::PublicKey::from_bytes(&pk_bytes).map_err(|_| anyhow!("recipient HPKE key invalid"))?;
	let now = Utc::now().timestamp();
	let mut nonce = [0u8; 16];
	use rand::RngCore;
	OsRng.fill_bytes(&mut nonce);
	let header = ProtectedHeader {
		version:4,
		message_id:hex::encode(nonce),
		sender:sender.into(),
		recipient:receiver.into(),
		kind:kind.into(),
		campaign:campaign.into(),
		created_unix:now,
		expires_unix:now + 3600,
		nonce_hex:hex::encode(nonce),
	};
	let aad = serde_json::to_vec(&header)?;
	let (enc, mut ctx) =
		setup_sender::<A, D, K>(&OpModeS::Base, &pk, INFO).map_err(|_| anyhow!("HPKE setup failed"))?;
	let ciphertext = ctx.seal(plaintext, &aad).map_err(|_| anyhow!("HPKE encryption failed"))?;
	let mut envelope = EncryptedEnvelope {
		protected:header,
		encapsulation_b64:B64.encode(enc.to_bytes()),
		ciphertext_b64:B64.encode(ciphertext),
		signature_b64:String::new(),
	};
	let author_key = signing_key(&state.join("identities").join(sender).join("sign.secret.hex"))?;
	if hex::encode(author_key.verifying_key().to_bytes()) != author.subject.sign_public_hex {
		bail!("sender key mismatch")
	}
	envelope.signature_b64 = B64.encode(author_key.sign(&signed_bytes(&envelope)?).to_bytes());
	Ok(envelope)
}
pub fn verify(state:&Path, e:&EncryptedEnvelope) -> Result<()> {
	let h = &e.protected;
	if h.version != 4 {
		bail!("unknown envelope version")
	}
	if h.message_id != h.nonce_hex || hex::decode(&h.nonce_hex)?.len() != 16 {
		bail!("invalid nonce/message identity")
	}
	if h.created_unix > Utc::now().timestamp() + 60
		|| Utc::now().timestamp() >= h.expires_unix
		|| h.expires_unix - h.created_unix > 3600
	{
		bail!("message expired or invalid time window")
	}
	let author = validate_identity(state, &h.sender)?;
	validate_identity(state, &h.recipient)?;
	if !author.subject.allowed_message_types.iter().any(|x| x == &h.kind) {
		bail!("message kind outside sender authorization")
	}
	let pubkey = VerifyingKey::from_bytes(&fixed_32(&author.subject.sign_public_hex)?)?;
	let sig = Signature::from_slice(&B64.decode(&e.signature_b64)?)?;
	pubkey.verify(&signed_bytes(e)?, &sig).context("invalid message signature")?;
	if B64.decode(&e.ciphertext_b64)?.len() > 8 * 1024 * 1024 + 64 {
		bail!("ciphertext too large")
	}
	Ok(())
}
pub fn open(state:&Path, recipient:&str, e:&EncryptedEnvelope, replay:bool) -> Result<Vec<u8>> {
	verify(state, e)?;
	if recipient != e.protected.recipient {
		bail!("recipient mismatch")
	}
	let sk_bytes =
		B64.decode(fs::read_to_string(state.join("identities").join(recipient).join("encrypt.secret.b64"))?.trim())?;
	let sk = <K as Kem>::PrivateKey::from_bytes(&sk_bytes).map_err(|_| anyhow!("invalid private HPKE key"))?;
	let enc_bytes = B64.decode(&e.encapsulation_b64)?;
	let enc = <K as Kem>::EncappedKey::from_bytes(&enc_bytes).map_err(|_| anyhow!("invalid encapsulated key"))?;
	let mut ctx =
		setup_receiver::<A, D, K>(&OpModeR::Base, &sk, &enc, INFO).map_err(|_| anyhow!("HPKE open failed"))?;
	let plaintext = ctx
		.open(&B64.decode(&e.ciphertext_b64)?, &serde_json::to_vec(&e.protected)?)
		.map_err(|_| anyhow!("authenticated decryption rejected ciphertext"))?;
	if replay {
		let dir = state.join("replay").join(recipient);
		fs::create_dir_all(&dir)?;
		// This atomic operation rejects duplicates across simultaneous readers
		// on a shared filesystem.
		super::storage::mark_once(&dir.join(&e.protected.message_id))?;
	}
	Ok(plaintext)
}
pub fn sha(bytes:&[u8]) -> String { hex::encode(Sha256::digest(bytes)) }

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn domain_separation_differs() {
		assert_ne!(SIG_DOMAIN, REG_DOMAIN);
	}
	#[test]
	fn invalid_key_length_rejected() {
		assert!(fixed_32("aabb").is_err());
	}
}
