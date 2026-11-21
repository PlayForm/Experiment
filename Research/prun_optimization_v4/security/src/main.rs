//! CLI examples (developer mode): init, issue, seal, verify, open, revoke,
//! audit, relay, send, fetch and ack. Root secret is never loaded by the relay
//! code path.
mod crypto;
#[cfg(feature = "openpgp")]
mod openpgp;
mod relay;
mod ssh_ca;
mod storage;
use std::{
	env,
	fs,
	net::SocketAddr,
	path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
fn help() -> &'static str {
	"Usage:\nprun-lab-trust init STATE\nprun-lab-trust issue STATE NAME ROLE KINDS_CSV REASON\nprun-lab-trust revoke \
	 STATE NAME REASON\nprun-lab-trust audit STATE\nprun-lab-trust audit-relay STATE\nprun-lab-trust ssh-init \
	 STATE\nprun-lab-trust ssh-issue STATE ID SUBJECT_PUBLIC_KEY OUTPUT REASON\nprun-lab-trust stage STATE \
	 OUTPUT_DIRECTORY LOCAL_IDENTITY\nprun-lab-trust seal STATE SENDER RECEIVER KIND CAMPAIGN INPUT \
	 OUTPUT\nprun-lab-trust verify STATE ENVELOPE\nprun-lab-trust open STATE RECEIVER ENVELOPE OUTPUT\nprun-lab-trust \
	 relay STATE RELAY_ID BIND_ADDRESS\nprun-lab-trust send STATE AGENT RELAY_HOST RELAY_ADDRESS \
	 ENVELOPE\nprun-lab-trust fetch STATE AGENT RELAY_HOST RELAY_ADDRESS OUTPUT_DIRECTORY\nprun-lab-trust ack STATE \
	 AGENT RELAY_HOST RELAY_ADDRESS MESSAGE_ID\nwith --features openpgp:\nprun-lab-trust pgp-encrypt PUBLIC_KEY INPUT \
	 OUTPUT\nprun-lab-trust pgp-decrypt SECRET_KEY PASSPHRASE INPUT OUTPUT\nprun-lab-trust pgp-sign SECRET_KEY \
	 PASSPHRASE INPUT SIGNATURE\nprun-lab-trust pgp-verify PUBLIC_KEY INPUT SIGNATURE\n"
}
fn path(x:&str) -> &Path { Path::new(x) }
#[tokio::main]
async fn main() -> Result<()> {
	let args:Vec<String> = env::args().skip(1).collect();
	let a:Vec<&str> = args.iter().map(String::as_str).collect();
	match a.as_slice() {
		#[cfg(feature = "openpgp")]
		["pgp-encrypt", pk, input, output] => openpgp::encrypt(pk.into(), input.into(), output.into()).await,
		#[cfg(feature = "openpgp")]
		["pgp-decrypt", sk, pass, input, output] => openpgp::decrypt(sk.into(), pass, input.into(), output.into()).await,
		#[cfg(feature = "openpgp")]
		["pgp-sign", sk, pass, input, sig] => openpgp::sign(sk.into(), pass, input.into(), sig.into()).await,
		#[cfg(feature = "openpgp")]
		["pgp-verify", pk, input, sig] => openpgp::verify(pk.into(), input.into(), sig.into()).await,
		["init", state] => storage::init(path(state)),
		["issue", state, name, role, kinds, reason] => {
			storage::issue(path(state), name, role, kinds.split(',').map(str::to_string).collect(), reason)
		},
		["revoke", state, name, reason] => storage::revoke(path(state), name, reason),
		["audit", state] => storage::audit_summary(path(state)),
		["audit-relay", state] => {
			let events = storage::read_relay_events(path(state))?;
			println!("verified relay events={}", events.len());
			for e in events {
				println!(
					"#{} {} {} {} {}",
					e.body.seq, e.body.when_unix, e.body.operation, e.body.sender, e.body.ciphertext_sha256
				);
			}
			Ok(())
		},
		["stage", state, output, id] => storage::stage_runtime(path(state), path(output), id),
		["ssh-init", state] => ssh_ca::initialize(path(state)),
		["ssh-issue", state, id, pubkey, output, reason] => {
			ssh_ca::issue(path(state), id, path(pubkey), path(output), reason)
		},
		["seal", state, sender, receiver, kind, campaign, input, output] => {
			let data = fs::read(input).context("failed to read plaintext input")?;
			let env = crypto::seal(path(state), sender, receiver, kind, campaign, &data)?;
			storage::public(path(output), &serde_json::to_vec_pretty(&env)?)?;
			println!("sealed {} -> {} id={}", sender, receiver, env.protected.message_id);
			Ok(())
		},
		["verify", state, file] => {
			let e:crypto::EncryptedEnvelope = serde_json::from_slice(&fs::read(file)?)?;
			crypto::verify(path(state), &e)?;
			println!(
				"verified signature, CA identity, message type, expiry, recipient and ciphertext binding: {}",
				e.protected.message_id
			);
			Ok(())
		},
		["open", state, name, file, output] => {
			let e:crypto::EncryptedEnvelope = serde_json::from_slice(&fs::read(file)?)?;
			let bytes = crypto::open(path(state), name, &e, true)?;
			storage::secret(path(output), &bytes)?;
			println!("decrypted for {name} id={}", e.protected.message_id);
			Ok(())
		},
		["relay", state, id, bind] => {
			relay::serve(PathBuf::from(state), id.to_string(), bind.parse::<SocketAddr>()?).await
		},
		["send", state, id, host, addr, file] => {
			let e = serde_json::from_slice(&fs::read(file)?)?;
			let res = relay::call(path(state), id, host, addr, relay::Request::Publish { envelope:e }).await?;
			println!("{}", res.reason);
			Ok(())
		},
		["fetch", state, id, host, addr, dir] => {
			fs::create_dir_all(dir)?;
			let res = relay::call(path(state), id, host, addr, relay::Request::Fetch { limit:16 }).await?;
			for env in res.messages {
				let f = Path::new(dir).join(format!("{}.json", env.protected.message_id));
				storage::public(&f, &serde_json::to_vec_pretty(&env)?)?;
				println!("{}", f.display());
			}
			Ok(())
		},
		["ack", state, id, host, addr, msg] => {
			let res = relay::call(path(state), id, host, addr, relay::Request::Ack { id:msg.to_string() }).await?;
			println!("{}", res.reason);
			Ok(())
		},
		_ => bail!("{}", help()),
	}
}
