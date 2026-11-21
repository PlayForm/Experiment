//! rustls TLS 1.3 configuration descriptors.
//!
//! Grounded in `security/src/relay.rs:40-73`: `roots` loads
//! `public/tls-root.cert.pem` into a root store, `tls_server` builds a rustls
//! `ServerConfig` with `WebPkiClientVerifier` (mandatory client certificates),
//! `tls_client` builds a `ClientConfig` with a client auth certificate, and
//! `call` validates the relay DNS name via `ServerName::try_from`
//! (`relay.rs:223`). This type describes the configuration; it opens no socket
//! and loads no certificate.

use crate::permission::Permission;

/// A typed rustls configuration for one relay endpoint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tls {
    /// Protocol version the wire protocol expects (`relay.rs:1-2`).
    pub version: &'static str,
    /// Whether a client certificate is required (`relay.rs:56`).
    pub client_auth: Permission,
    /// Trusted root certificate path (`relay.rs:50`).
    pub root_cert: &'static str,
    /// This endpoint's certificate path (`relay.rs:60,69`).
    pub leaf_cert: &'static str,
    /// This endpoint's private key path (`relay.rs:61,70`).
    pub leaf_key: &'static str,
    /// Whether the server DNS name is validated (`relay.rs:223`).
    pub verify_server_name: Permission,
}

impl Tls {
    /// The server-side descriptor (`relay.rs:55-64`).
    #[must_use]
    pub const fn server() -> Self {
        Self {
            version: "1.3",
            client_auth: Permission::Allow,
            root_cert: "public/tls-root.cert.pem",
            leaf_cert: "identities/<id>/tls.cert.pem",
            leaf_key: "identities/<id>/tls.key.pem",
            verify_server_name: Permission::Deny,
        }
    }

    /// The client-side descriptor (`relay.rs:65-73`).
    #[must_use]
    pub const fn client() -> Self {
        Self {
            version: "1.3",
            client_auth: Permission::Allow,
            root_cert: "public/tls-root.cert.pem",
            leaf_cert: "identities/<id>/tls.cert.pem",
            leaf_key: "identities/<id>/tls.key.pem",
            verify_server_name: Permission::Allow,
        }
    }

    /// Whether a peer certificate chain is mandatory.
    #[must_use]
    pub const fn requires_peer_certificate(&self) -> Permission {
        self.client_auth
    }
}
