# v4 Rust trust-fabric prototype (not production-ready)

This source is an experimental identity and encrypted-messaging CLI with local
CA initialization, identity issuance, Ed25519 signed HPKE envelopes,
offline-root staging, OpenSSH certificate support, optional OpenPGP helper,
Hickory private DNS zone, and a Rustls mTLS framed-message relay. It is **not**
an audited production CA or a public HTTPS server. The relay wire protocol is
length-prefixed JSON over TLS, not HTTP. Docker Compose is a local illustration
only.

**Important:** This execution environment did not have `cargo`, `rustc` or
Docker. Compilation, TLS interoperability, SSH/OpenPGP compatibility, fuzz
testing, rate limits and operational security have not been verified. Review and
test cryptographic implementation and file permissions before using with any
real secret.

For a separate v5 research economy overlay see
`../../prun_optimization_v5/architecture/V4_TRUST_BRIDGE.md` in the combined
bundle. The v4 source will need additions for signed bid/claim/paper payload
formats, purpose-specific capabilities, declassification and treasury
signatures.

Run from the `security` directory on a host with Rust and Docker:

```bash
cargo test
cargo run -- init /path/to/private/ca-state
# Only in a development workspace and while controlling access to CA secrets.
```

Do not mount `/path/to/private/ca-state/offline` into any relay or agent
container. The helper `stage` command is intended for a relay-only restricted
state view; audit resulting contents before deployment. The current Compose file
is not a validated end-to-end cluster launcher.
