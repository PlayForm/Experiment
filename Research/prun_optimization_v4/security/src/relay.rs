//! Narrow mTLS relay. Application frames are length-prefixed JSON over TLS 1.3.
//! It verifies sender/recipient identities but never holds HPKE private keys.
//! This is not an HTTP endpoint; a REST/HTTPS adapter can be added in a later
//! phase.
use std::{
	fs,
	io::BufReader,
	net::SocketAddr,
	path::{Path, PathBuf},
	sync::Arc,
};

use anyhow::{Context, Result, anyhow, bail};
use serde::{Deserialize, Serialize};
use tokio::{
	io::{AsyncReadExt, AsyncWriteExt},
	net::{TcpListener, TcpStream},
};
use tokio_rustls::{TlsAcceptor, TlsConnector, rustls};

use crate::{
	crypto::{self, EncryptedEnvelope},
	storage,
};

const MAX_FRAME:usize = 9 * 1024 * 1024;
#[derive(Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Request {
	Publish { envelope:EncryptedEnvelope },
	Fetch { limit:usize },
	Ack { id:String },
}
#[derive(Serialize, Deserialize)]
pub struct Response {
	pub ok:bool,
	pub reason:String,
	pub messages:Vec<EncryptedEnvelope>,
}
fn certs(path:&Path) -> Result<Vec<rustls::pki_types::CertificateDer<'static>>> {
	let f = fs::File::open(path)?;
	Ok(rustls_pemfile::certs(&mut BufReader::new(f)).collect::<std::result::Result<Vec<_>, _>>()?)
}
fn key(path:&Path) -> Result<rustls::pki_types::PrivateKeyDer<'static>> {
	let f = fs::File::open(path)?;
	rustls_pemfile::private_key(&mut BufReader::new(f))?.ok_or_else(|| anyhow!("TLS key not found"))
}
fn roots(state:&Path) -> Result<rustls::RootCertStore> {
	let mut roots = rustls::RootCertStore::empty();
	for cert in certs(&state.join("public/tls-root.cert.pem"))? {
		roots.add(cert)?;
	}
	Ok(roots)
}
fn tls_server(state:&Path, id:&str) -> Result<Arc<rustls::ServerConfig>> {
	let verifier = rustls::server::WebPkiClientVerifier::builder(Arc::new(roots(state)?)).build()?;
	let cfg = rustls::ServerConfig::builder()
		.with_client_cert_verifier(verifier)
		.with_single_cert(
			certs(&state.join("identities").join(id).join("tls.cert.pem"))?,
			key(&state.join("identities").join(id).join("tls.key.pem"))?,
		)?;
	Ok(Arc::new(cfg))
}
fn tls_client(state:&Path, id:&str) -> Result<Arc<rustls::ClientConfig>> {
	let cfg = rustls::ClientConfig::builder()
		.with_root_certificates(roots(state)?)
		.with_client_auth_cert(
			certs(&state.join("identities").join(id).join("tls.cert.pem"))?,
			key(&state.join("identities").join(id).join("tls.key.pem"))?,
		)?;
	Ok(Arc::new(cfg))
}
async fn read_frame<S:AsyncReadExt + Unpin>(s:&mut S) -> Result<Vec<u8>> {
	let size = s.read_u32().await? as usize;
	if size == 0 || size > MAX_FRAME {
		bail!("frame outside size limit")
	}
	let mut bytes = vec![0u8; size];
	s.read_exact(&mut bytes).await?;
	Ok(bytes)
}
async fn write_frame<S:AsyncWriteExt + Unpin>(s:&mut S, bytes:&[u8]) -> Result<()> {
	if bytes.len() > MAX_FRAME {
		bail!("response exceeds frame limit")
	}
	s.write_u32(bytes.len() as u32).await?;
	s.write_all(bytes).await?;
	s.flush().await?;
	Ok(())
}
fn mailbox(root:&Path, id:&str) -> PathBuf { root.join("mailboxes").join(id) }
fn message_name(id:&str) -> Result<()> {
	if id.len() != 32 || !id.bytes().all(|x| x.is_ascii_hexdigit()) {
		bail!("invalid message ID")
	}
	Ok(())
}
fn handle(state:&Path, relay_id:&str, peer:&str, req:Request) -> Result<Response> {
	crypto::validate_identity(state, peer)?;
	match req {
		Request::Publish { envelope } => {
			crypto::verify(state, &envelope)?;
			if envelope.protected.sender != peer {
				bail!("sender differs from authenticated TLS peer")
			}
			let recipient = &envelope.protected.recipient;
			let id = &envelope.protected.message_id;
			message_name(id)?;
			let dir = mailbox(state, recipient);
			fs::create_dir_all(&dir)?;
			let already_archived = state.join("archive").join(recipient).join(format!("{id}.json")).exists();
			if already_archived {
				bail!("message already delivered")
			}
			storage::public(&dir.join(format!("{id}.json")), &serde_json::to_vec(&envelope)?)?;
			storage::relay_event(
				state,
				relay_id,
				"PUBLISH",
				peer,
				recipient,
				&crypto::sha(&serde_json::to_vec(&envelope)?),
			)?;
			Ok(Response { ok:true, reason:"accepted_ciphertext".into(), messages:vec![] })
		},
		Request::Fetch { limit } => {
			let dir = mailbox(state, peer);
			fs::create_dir_all(&dir)?;
			let mut items:Vec<_> = fs::read_dir(dir)?.filter_map(|e| e.ok()).collect();
			items.sort_by_key(|e| e.file_name());
			let mut out = vec![];
			for item in items.into_iter().take(limit.min(16)) {
				let env:EncryptedEnvelope = serde_json::from_slice(&fs::read(item.path())?)?;
				crypto::verify(state, &env)?;
				out.push(env);
			}
			storage::relay_event(
				state,
				relay_id,
				"FETCH",
				peer,
				peer,
				&crypto::sha(serde_json::to_vec(&out)?.as_slice()),
			)?;
			Ok(Response { ok:true, reason:"ciphertext_only".into(), messages:out })
		},
		Request::Ack { id } => {
			message_name(&id)?;
			let src = mailbox(state, peer).join(format!("{id}.json"));
			if !src.is_file() {
				bail!("unknown message")
			}
			let dst_dir = state.join("archive").join(peer);
			fs::create_dir_all(&dst_dir)?;
			fs::rename(src, dst_dir.join(format!("{id}.json")))?;
			storage::relay_event(state, relay_id, "ACK", peer, peer, &crypto::sha(id.as_bytes()))?;
			Ok(Response { ok:true, reason:"archived".into(), messages:vec![] })
		},
	}
}
pub async fn serve(state:PathBuf, identity:String, bind:SocketAddr) -> Result<()> {
	let _ = crypto::validate_identity(&state, &identity)?;
	let acceptor = TlsAcceptor::from(tls_server(&state, &identity)?);
	let serialized_filesystem = Arc::new(tokio::sync::Mutex::new(()));
	let listener = TcpListener::bind(bind).await?;
	eprintln!("mTLS opaque relay bound to {bind} as {identity}");
	loop {
		let (socket, _) = listener.accept().await?;
		let acceptor = acceptor.clone();
		let state = state.clone();
		let identity = identity.clone();
		let serialized_filesystem = serialized_filesystem.clone();
		tokio::spawn(async move {
			let res:Result<()> = async {
				let mut tls = acceptor.accept(socket).await?;
				let peer_certs = tls
					.get_ref()
					.1
					.peer_certificates()
					.ok_or_else(|| anyhow!("missing TLS peer certificate"))?;
				let peer_cert = peer_certs.first().ok_or_else(|| anyhow!("empty TLS peer certificate chain"))?;
				let fp = crypto::sha(peer_cert.as_ref());
				// Use the attested certificate fingerprint, not a user-supplied
				// identity header.
				let mut peer_id = None;
				for entry in fs::read_dir(state.join("registry"))? {
					let entry = entry?;
					let name = match entry.path().file_stem().and_then(|s| s.to_str()) {
						Some(s) => s.to_owned(),
						None => continue,
					};
					if let Ok(record) = crypto::validate_identity(&state, &name) {
						if record.subject.tls_certificate_sha256 == fp {
							peer_id = Some(name);
							break;
						}
					}
				}
				let peer = peer_id.ok_or_else(|| anyhow!("TLS credential not registered or revoked"))?;
				let request:Request = serde_json::from_slice(&read_frame(&mut tls).await?)?;
				// Expected validation errors return no candidate data and do
				// not execute anything.
				let _lock = serialized_filesystem.lock().await;
				let response = match handle(&state, &identity, &peer, request) {
					Ok(r) => r,
					Err(e) => Response { ok:false, reason:e.to_string(), messages:vec![] },
				};
				write_frame(&mut tls, &serde_json::to_vec(&response)?).await?;
				Ok(())
			}
			.await;
			if let Err(e) = res {
				eprintln!("relay connection rejected: {e}");
			}
		});
	}
}
pub async fn call(state:&Path, identity:&str, host:&str, addr:&str, request:Request) -> Result<Response> {
	let _ = crypto::validate_identity(state, identity)?;
	let connector = TlsConnector::from(tls_client(state, identity)?);
	let socket = TcpStream::connect(addr).await?;
	let servername = rustls::pki_types::ServerName::try_from(host.to_string()).context("invalid relay DNS name")?;
	let mut tls = connector
		.connect(servername, socket)
		.await
		.context("server TLS identity rejected")?;
	write_frame(&mut tls, &serde_json::to_vec(&request)?).await?;
	let response:Response = serde_json::from_slice(&read_frame(&mut tls).await?)?;
	if !response.ok {
		bail!("relay rejected request: {}", response.reason)
	}
	Ok(response)
}
