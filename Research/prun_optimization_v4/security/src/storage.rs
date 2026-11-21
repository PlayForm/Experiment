//! File-backed developer PKI with tamper-evident signed issuance journal.
//! NOT a hardened production CA: root keys must be held offline, outside the
//! runtime container.
#[cfg(unix)]
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::{
	fs::{self, OpenOptions},
	io::Write,
	path::Path,
};

use anyhow::{Result, anyhow, bail};
use chrono::Utc;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::crypto;
const AUDIT_DOMAIN:&[u8] = b"prun-lab/v4/ca-audit/ed25519\0";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditBody {
	pub seq:u64,
	pub previous_sha256:String,
	pub when_unix:i64,
	pub event:String,
	pub subject:String,
	pub reason:String,
	pub evidence_sha256:String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRecord {
	pub body:AuditBody,
	pub signature_hex:String,
}

pub fn identity_name(name:&str) -> Result<()> {
	if name.len() < 2 || name.len() > 64 || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_') {
		bail!("invalid identity name: use 2-64 ASCII letters/digits/_/-")
	}
	Ok(())
}
pub fn secret(path:&Path, data:&[u8]) -> Result<()> {
	if let Some(parent) = path.parent() {
		fs::create_dir_all(parent)?;
	}
	let mut opts = OpenOptions::new();
	opts.write(true).create_new(true);
	#[cfg(unix)]
	opts.mode(0o600);
	let mut file = opts.open(path)?;
	file.write_all(data)?;
	file.sync_all()?;
	Ok(())
}
pub fn public(path:&Path, data:&[u8]) -> Result<()> {
	if let Some(parent) = path.parent() {
		fs::create_dir_all(parent)?;
	}
	let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
	file.write_all(data)?;
	file.sync_all()?;
	Ok(())
}
pub fn mark_once(path:&Path) -> Result<()> { secret(path, b"seen") }
fn audit_key(state:&Path) -> Result<SigningKey> {
	let b = hex::decode(fs::read_to_string(state.join("offline/message-authority.hex"))?.trim())?;
	Ok(SigningKey::from_bytes(
		&b.try_into().map_err(|_| anyhow!("bad audit secret length"))?,
	))
}
fn audit_public(state:&Path) -> Result<VerifyingKey> {
	let b = hex::decode(fs::read_to_string(state.join("public/message-authority.hex"))?.trim())?;
	Ok(VerifyingKey::from_bytes(
		&b.try_into().map_err(|_| anyhow!("bad audit public length"))?,
	)?)
}
fn audit_bytes(body:&AuditBody) -> Result<Vec<u8>> {
	let mut bytes = AUDIT_DOMAIN.to_vec();
	bytes.extend(serde_json::to_vec(body)?);
	Ok(bytes)
}
fn audit_digest(record:&AuditRecord) -> Result<String> { Ok(crypto::sha(&serde_json::to_vec(record)?)) }
pub fn read_audit(state:&Path) -> Result<Vec<AuditRecord>> {
	let data = fs::read_to_string(state.join("audit/issuance.jsonl"))?;
	let mut last = "0".repeat(64);
	let public = audit_public(state)?;
	let mut events = Vec::new();
	for (index, line) in data.lines().enumerate() {
		let record:AuditRecord = serde_json::from_str(line)?;
		if record.body.seq != (index + 1) as u64 || record.body.previous_sha256 != last {
			bail!("broken journal sequence or previous hash at {}", index + 1)
		}
		let sig = Signature::from_slice(&hex::decode(&record.signature_hex)?)?;
		public.verify(&audit_bytes(&record.body)?, &sig)?;
		last = audit_digest(&record)?;
		events.push(record);
	}
	Ok(events)
}
pub fn audit(state:&Path, event:&str, subject:&str, reason:&str, evidence:&str) -> Result<()> {
	// One writer is required for this developer file-backed issuer. Production
	// uses DB transactions.
	let entries = read_audit(state)?;
	let previous_sha256 = match entries.last() {
		Some(last) => audit_digest(last)?,
		None => "0".repeat(64),
	};
	let body = AuditBody {
		seq:(entries.len() + 1) as u64,
		previous_sha256,
		when_unix:Utc::now().timestamp(),
		event:event.into(),
		subject:subject.into(),
		reason:reason.into(),
		evidence_sha256:evidence.into(),
	};
	let sig = audit_key(state)?.sign(&audit_bytes(&body)?);
	let record = AuditRecord { body, signature_hex:hex::encode(sig.to_bytes()) };
	let mut file = OpenOptions::new().append(true).open(state.join("audit/issuance.jsonl"))?;
	writeln!(file, "{}", serde_json::to_string(&record)?)?;
	file.sync_all()?;
	Ok(())
}
pub fn init(state:&Path) -> Result<()> {
	if state.exists() {
		bail!("refusing to reuse existing CA directory")
	}
	fs::create_dir_all(state.join("offline"))?;
	#[cfg(unix)]
	fs::set_permissions(state.join("offline"), fs::Permissions::from_mode(0o700))?;
	fs::create_dir_all(state.join("public"))?;
	fs::create_dir_all(state.join("audit"))?;
	fs::create_dir_all(state.join("identities"))?;
	fs::create_dir_all(state.join("registry"))?;
	fs::create_dir_all(state.join("revoked"))?;
	// Separate CA signing key for message identity attestations and issuance
	// ledger.
	let pubhex = crypto::generate_sign_key(&state.join("offline/message-authority.hex"))?;
	public(&state.join("public/message-authority.hex"), pubhex.as_bytes())?;
	let mut ca = rcgen::CertificateParams::default();
	ca.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
	ca.distinguished_name
		.push(rcgen::DnType::CommonName, "Run Laboratory Dev Root CA");
	ca.key_usages = vec![
		rcgen::KeyUsagePurpose::KeyCertSign,
		rcgen::KeyUsagePurpose::CrlSign,
		rcgen::KeyUsagePurpose::DigitalSignature,
	];
	let ca_key = rcgen::KeyPair::generate()?;
	let cert = ca.self_signed(&ca_key)?;
	secret(&state.join("offline/tls-root.key.pem"), ca_key.serialize_pem().as_bytes())?;
	public(&state.join("public/tls-root.cert.pem"), cert.pem().as_bytes())?;
	public(&state.join("audit/issuance.jsonl"), b"")?;
	audit(
		state,
		"ROOT_INITIALIZED",
		"dev-root",
		"operator_bootstrap",
		&crypto::sha(cert.der()),
	)?;
	Ok(())
}
pub fn issue(state:&Path, name:&str, role:&str, kinds:Vec<String>, reason:&str) -> Result<()> {
	identity_name(name)?;
	identity_name(role)?;
	if kinds.is_empty() || kinds.iter().any(|k| k.len() > 64 || k.is_empty()) {
		bail!("invalid message kinds")
	}
	if state.join("registry").join(format!("{name}.json")).exists() {
		bail!("identity already exists")
	}
	let identity_dir = state.join("identities").join(name);
	fs::create_dir(&identity_dir)?;
	#[cfg(unix)]
	fs::set_permissions(&identity_dir, fs::Permissions::from_mode(0o700))?;
	let sign_public_hex = crypto::generate_sign_key(&identity_dir.join("sign.secret.hex"))?;
	let encrypt_public_b64 = crypto::generate_hpke_key(&identity_dir.join("encrypt.secret.b64"))?;
	let ca_key = rcgen::KeyPair::from_pem(&fs::read_to_string(state.join("offline/tls-root.key.pem"))?)?;
	let ca_cert_pem = fs::read_to_string(state.join("public/tls-root.cert.pem"))?;
	let issuer = rcgen::Issuer::from_ca_cert_pem(&ca_cert_pem, &ca_key)?;
	// Both client+server EKUs are for this demo. Production issues
	// purpose-separated leaf certs.
	let mut params = rcgen::CertificateParams::new(vec![format!("{name}.lab.internal")])?;
	params
		.distinguished_name
		.push(rcgen::DnType::CommonName, format!("run-agent:{name}"));
	params.extended_key_usages = vec![
		rcgen::ExtendedKeyUsagePurpose::ClientAuth,
		rcgen::ExtendedKeyUsagePurpose::ServerAuth,
	];
	let cert_key = rcgen::KeyPair::generate()?;
	let cert = params.signed_by(&cert_key, &issuer)?;
	secret(&identity_dir.join("tls.key.pem"), cert_key.serialize_pem().as_bytes())?;
	public(&identity_dir.join("tls.cert.pem"), cert.pem().as_bytes())?;
	let now = Utc::now().timestamp();
	let subject = crypto::PublicIdentity {
		identity:name.into(),
		role:role.into(),
		sign_public_hex,
		encrypt_public_b64,
		tls_certificate_sha256:crypto::sha(cert.der()),
		allowed_message_types:kinds,
		created_unix:now - 10,
		expires_unix:now + 86400,
		issuance_id:crypto::sha(format!("{name}:{now}:{}", crypto::sha(cert.der())).as_bytes()),
	};
	let signed = crypto::attest(state, subject)?;
	public(
		&state.join("registry").join(format!("{name}.json")),
		&serde_json::to_vec_pretty(&signed)?,
	)?;
	audit(
		state,
		"IDENTITY_ISSUED",
		name,
		reason,
		&crypto::sha(&serde_json::to_vec(&signed)?),
	)?;
	Ok(())
}
pub fn revoke(state:&Path, name:&str, reason:&str) -> Result<()> {
	crypto::validate_identity(state, name)?;
	// Journal the intent first, then make the revocation effective. Fail closed
	// on partial revocations.
	audit(state, "IDENTITY_REVOKED", name, reason, &crypto::sha(name.as_bytes()))?;
	public(&state.join("revoked").join(name), reason.as_bytes())?;
	Ok(())
}
pub fn audit_summary(state:&Path) -> Result<()> {
	let entries = read_audit(state)?;
	let issued = entries.iter().filter(|x| x.body.event == "IDENTITY_ISSUED").count();
	let revoked = entries.iter().filter(|x| x.body.event == "IDENTITY_REVOKED").count();
	println!(
		"verified entries={} issued={} revoked={} note=local-signed-hash-chain",
		entries.len(),
		issued,
		revoked
	);
	for entry in &entries {
		println!(
			"#{} {} {} reason={} hash={}",
			entry.body.seq, entry.body.when_unix, entry.body.event, entry.body.reason, entry.body.evidence_sha256
		);
	}
	Ok(())
}

/// Create a runtime view with public trust and exactly one service's TLS
/// credentials. The offline CA, other identities' secret keys and replay
/// secrets are never copied.
pub fn stage_runtime(state:&Path, output:&Path, local_identity:&str) -> Result<()> {
	identity_name(local_identity)?;
	crypto::validate_identity(state, local_identity)?;
	if output.exists() {
		bail!("refusing to overwrite runtime bundle")
	}
	fs::create_dir_all(output.join("public"))?;
	fs::create_dir_all(output.join("registry"))?;
	fs::create_dir_all(output.join("revoked"))?;
	fs::create_dir_all(output.join("identities").join(local_identity))?;
	for f in ["tls-root.cert.pem", "message-authority.hex", "ssh-ca.pub"] {
		let from = state.join("public").join(f);
		if from.is_file() {
			fs::copy(from, output.join("public").join(f))?;
		}
	}
	for e in fs::read_dir(state.join("registry"))? {
		let e = e?;
		if e.path().is_file() {
			fs::copy(e.path(), output.join("registry").join(e.file_name()))?;
		}
	}
	for e in fs::read_dir(state.join("revoked"))? {
		let e = e?;
		if e.path().is_file() {
			fs::copy(e.path(), output.join("revoked").join(e.file_name()))?;
		}
	}
	let agent = state.join("identities").join(local_identity);
	// Relay needs its own signed event key but never any HPKE decryption key.
	for name in ["tls.cert.pem", "tls.key.pem", "sign.secret.hex"] {
		fs::copy(agent.join(name), output.join("identities").join(local_identity).join(name))?;
	}
	fs::create_dir(output.join("mailboxes"))?;
	fs::create_dir(output.join("archive"))?;
	#[cfg(unix)]
	{
		fs::set_permissions(
			output.join("identities").join(local_identity).join("tls.key.pem"),
			fs::Permissions::from_mode(0o600),
		)?;
	}
	Ok(())
}

/// Observer-visible metadata only. Each event is signed by the relay's
/// CA-attested Ed25519 key; the journal can be verified without opening any
/// agent messages.
#[derive(Serialize, Deserialize)]
pub struct RelayEventBody {
	pub seq:u64,
	pub previous_sha256:String,
	pub when_unix:i64,
	pub relay_identity:String,
	pub operation:String,
	pub sender:String,
	pub recipient:String,
	pub ciphertext_sha256:String,
}
#[derive(Serialize, Deserialize)]
pub struct RelayEvent {
	pub body:RelayEventBody,
	pub signature_hex:String,
}
fn relay_event_bytes(body:&RelayEventBody) -> Result<Vec<u8>> {
	let mut data = b"prun-lab/v4/relay-audit/ed25519\0".to_vec();
	data.extend_from_slice(&serde_json::to_vec(body)?);
	Ok(data)
}
pub fn read_relay_events(state:&Path) -> Result<Vec<RelayEvent>> {
	let log = state.join("audit/relay-events.jsonl");
	if !log.exists() {
		return Ok(vec![]);
	}
	let text = fs::read_to_string(log)?;
	let mut events = vec![];
	let mut previous = "0".repeat(64);
	for (i, line) in text.lines().enumerate() {
		let ev:RelayEvent = serde_json::from_str(line)?;
		if ev.body.seq != (i + 1) as u64 || ev.body.previous_sha256 != previous {
			bail!("broken relay event chain")
		}
		let id = crypto::validate_identity(state, &ev.body.relay_identity)?;
		let pk:[u8; 32] = hex::decode(&id.subject.sign_public_hex)?
			.try_into()
			.map_err(|_| anyhow!("invalid relay public key"))?;
		let public = VerifyingKey::from_bytes(&pk)?;
		let signature = Signature::from_slice(&hex::decode(&ev.signature_hex)?)?;
		public.verify(&relay_event_bytes(&ev.body)?, &signature)?;
		previous = crypto::sha(&serde_json::to_vec(&ev)?);
		events.push(ev);
	}
	Ok(events)
}
pub fn relay_event(
	state:&Path,
	identity:&str,
	operation:&str,
	sender:&str,
	recipient:&str,
	cipher_hash:&str,
) -> Result<()> {
	crypto::validate_identity(state, identity)?;
	let events = read_relay_events(state)?;
	let previous = match events.last() {
		Some(x) => crypto::sha(&serde_json::to_vec(x)?),
		None => "0".repeat(64),
	};
	let body = RelayEventBody {
		seq:(events.len() + 1) as u64,
		previous_sha256:previous,
		when_unix:Utc::now().timestamp(),
		relay_identity:identity.into(),
		operation:operation.into(),
		sender:sender.into(),
		recipient:recipient.into(),
		ciphertext_sha256:cipher_hash.into(),
	};
	let key_bytes:[u8; 32] =
		hex::decode(fs::read_to_string(state.join("identities").join(identity).join("sign.secret.hex"))?.trim())?
			.try_into()
			.map_err(|_| anyhow!("invalid relay signing key"))?;
	let signer = SigningKey::from_bytes(&key_bytes);
	let signature_hex = hex::encode(signer.sign(&relay_event_bytes(&body)?).to_bytes());
	let ev = RelayEvent { body, signature_hex };
	fs::create_dir_all(state.join("audit"))?;
	let mut file = OpenOptions::new()
		.append(true)
		.create(true)
		.open(state.join("audit/relay-events.jsonl"))?;
	writeln!(file, "{}", serde_json::to_string(&ev)?)?;
	file.sync_all()?;
	Ok(())
}
