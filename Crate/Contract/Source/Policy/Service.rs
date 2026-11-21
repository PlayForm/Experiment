//! v6.1 service-to-route allowlist.
//!
//! `policy/v61-roles.json:13-51` maps 18 trusted services to the
//! internal paths each may call. Internal admission and decision services must
//! re-evaluate authorization; a caller's `verdict: allow` is never trusted
//! (`docs/23-interface-inventory.md:70-73`).

/// One service route grant.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Service {
    /// Service principal name.
    pub name: &'static str,
    /// Internal paths it may call.
    pub paths: &'static [&'static str],
}

impl Service {
    /// Build one service descriptor.
    #[must_use]
    pub const fn of(name: &'static str, paths: &'static [&'static str]) -> Self {
        Self { name, paths }
    }

    /// All declared service grants.
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
        Service::of("audit-service", &["/internal/v1/audit/denial"]),
        Service::of("context-projector", &["/internal/v1/research/view/compile"]),
        Service::of("coordinator", &["/internal/v1/research/candidates/register", "/internal/v1/research/track"]),
        Service::of("custody-broker", &["/internal/v1/custody/commit", "/internal/v1/custody/preflight"]),
        Service::of("custody-policy", &["/internal/v1/artifacts/policy/bind"]),
        Service::of("experiment-compiler", &["/internal/v1/experiments/spec/compile"]),
        Service::of("feedback-projector", &["/internal/v1/feedback/project"]),
        Service::of("holder-supervisor", &["/internal/v1/holders/attest"]),
        Service::of("ledger", &["/internal/v1/facts/append"]),
        Service::of("license-registry", &["/internal/v1/licenses/register"]),
        Service::of("policy-service", &["/internal/v1/custody/decide", "/internal/v1/embargoes/register", "/internal/v1/knowledge/compatibility", "/internal/v1/trade/restrict"]),
        Service::of("publication-bridge", &["/internal/v1/knowledge/release"]),
        Service::of("source-adapter", &["/internal/v1/repos/admit"]),
        Service::of("storage-broker", &["/internal/v1/custody/reserve"]),
        Service::of("supervisor", &["/internal/v1/research/boundary/record", "/internal/v1/research/intent/ingest", "/internal/v1/research/task-context", "/internal/v1/workers/neutral-receipt"]),
        Service::of("curriculum-service", &["/internal/v1/curriculum/register", "/internal/v1/curriculum/study/assign"]),
        Service::of("study-evaluator", &["/internal/v1/curriculum/study/outcome"]),
        Service::of("resource-scheduler", &["/internal/v1/resources/allocate"]),
        ]
    }

    /// Whether this service may call a given internal path.
    #[must_use]
    pub fn may_call(&self, path: &str) -> crate::Permission {
        match self.paths.iter().any(|allowed| *allowed == path) {
            true => crate::Permission::Allow,
            false => crate::Permission::Deny,
        }
    }
}
