//! Trust domains and their declared network reachability.
//!
//! `policy/access-matrix.json:3-45` declares six domains. `llm-worker` and
//! `private-root-ca` have no `may_connect` entries and no network: the worker
//! sees only a scoped filesystem mailbox, and the root CA stays offline.

/// One trust domain descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Domain {
    /// Domain name.
    pub name: &'static str,
    /// Endpoints this domain may connect to.
    pub may_connect: &'static [&'static str],
    /// Accepted credential kinds.
    pub credentials: &'static [&'static str],
    /// Network scope.
    pub network: &'static str,
}

impl Domain {
    /// Build one domain descriptor.
    #[must_use]
    pub const fn of(
        name: &'static str,
        may_connect: &'static [&'static str],
        credentials: &'static [&'static str],
        network: &'static str,
    ) -> Self {
        Self { name, may_connect, credentials, network }
    }

    /// The six declared domains, in access-matrix order.
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
        Domain::of("human-observation", &["public-api"], &["human-client-mtls"], "loopback or explicitly authenticated remote"),
        Domain::of("trusted-core", &["internal-api", "event-bus", "artifact-store", "control-otlp"], &["scoped-service-mtls"], "trusted-internal"),
        Domain::of("product-diagnostics", &["product-otlp"], &["diagnostic-collector-mtls"], "isolated-diagnostics"),
        Domain::of("build-execution", &["artifact-staging"], &["job-specific-write-token"], "none during compilation/execution"),
        Domain::of("llm-worker", &[], &[], "none"),
        Domain::of("private-root-ca", &[], &["offline-key"], "none"),
        ]
    }

    /// Whether this domain has any network route at all.
    #[must_use]
    pub fn is_isolated(&self) -> bool {
        match self.network {
            "none" => true,
            _ => false,
        }
    }
}
