//! Authorized correspondence transfers.
//!
//! `events/correspondence-catalog.json:3-207` declares 29
//! finite, typed producer/consumer relationships across the control and
//! product planes (`docs/05-event-correspondence.md:9-41`). This is not a
//! generic agent-to-agent chat bus: the trusted parent and children have no
//! chat sockets to one another. Every transfer records its medium, object and
//! guarantee as data; no channel is opened.

/// One authorized correspondence channel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Correspondence {
    /// Authorized sender.
    pub sender: &'static str,
    /// Authorized recipient.
    pub recipient: &'static str,
    /// Transport medium.
    pub transport: &'static str,
    /// Record or object crossing the channel.
    pub record: &'static str,
    /// Guarantee attached to the transfer.
    pub meaning: &'static str,
}

impl Correspondence {
    /// Build one transfer descriptor.
    #[must_use]
    pub const fn of(
        sender: &'static str,
        recipient: &'static str,
        transport: &'static str,
        record: &'static str,
        meaning: &'static str,
    ) -> Self {
        Self { sender, recipient, transport, record, meaning }
    }

    /// The complete finite correspondence network.
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
        Correspondence::of(
            "Owner", "Public API", "mTLS HTTP",
            "command", "Operator-authenticated command document; only declarative allowed operations",
        ),
        Correspondence::of(
            "Public API", "Coordinator", "mTLS HTTP",
            "command", "Validated owner command; no arbitrary shell",
        ),
        Correspondence::of(
            "Coordinator", "Event ledger", "transactional append",
            "audit-event", "Signed state transition and hash chain",
        ),
        Correspondence::of(
            "Ledger", "Projection service", "authenticated outbox",
            "audit-event", "At-least-once delivery; idempotent projection",
        ),
        Correspondence::of(
            "Projection service", "GPUI", "mTLS HTTP/SSE",
            "page/status/audit-event", "Metadata-only public authorized view",
        ),
        Correspondence::of(
            "Coordinator", "Supervisor", "mTLS claim",
            "task-bundle", "Scoped work order; model is never API recipient",
        ),
        Correspondence::of(
            "Supervisor", "LLM worker", "read-only file mailbox",
            "task-bundle (filtered)", "Only source/task/content; no topology, IDs can be opaque",
        ),
        Correspondence::of(
            "LLM worker", "Supervisor", "scratch output directory",
            "patch/research-claim/test-proposal", "Untrusted data only; cannot run trusted control calls",
        ),
        Correspondence::of(
            "Supervisor", "Conversation CAS", "encrypted artifact commit",
            "agent-turn/tool-call", "Visible turn transcript, local tool args/results stored encrypted",
        ),
        Correspondence::of(
            "Supervisor", "Ingestion gateway", "mTLS HTTP",
            "task-receipt/patch-proposal", "Hash checks, scope checks, size checks",
        ),
        Correspondence::of(
            "Ingestion gateway", "Novelty gate", "trusted internal dispatch",
            "patch-proposal", "Exact identity, structural similarity, feature conflicts",
        ),
        Correspondence::of(
            "Coordinator", "Builder", "mTLS HTTP",
            "build-request", "Immutable source/toolchain/dependency closure",
        ),
        Correspondence::of(
            "Builder", "Artifact CAS", "mTLS HTTP",
            "build-result", "Executable digest and build log artifacts",
        ),
        Correspondence::of(
            "Coordinator", "Oracle", "mTLS HTTP",
            "oracle-request", "Independent frozen test suite",
        ),
        Correspondence::of(
            "Oracle", "Event ledger", "mTLS HTTP",
            "oracle-result", "Tests/counterexamples attested by oracle",
        ),
        Correspondence::of(
            "Coordinator", "Benchmark scheduler", "trusted dispatch",
            "benchmark-request", "Frozen comparative protocol",
        ),
        Correspondence::of(
            "Scheduler", "Benchmark host", "fenced lease",
            "lease", "Exclusive CPU and host resource reservation",
        ),
        Correspondence::of(
            "Benchmark runner", "Event ledger", "mTLS HTTP",
            "benchmark-result", "Raw paired samples and environment attestations",
        ),
        Correspondence::of(
            "Evaluator", "Event ledger", "mTLS HTTP",
            "evaluation-result", "Predeclared statistics; no model verdict authority",
        ),
        Correspondence::of(
            "Feedback authority", "Supervisor", "mTLS HTTP/CAS",
            "feedback-capsule", "Scoped results, not holdout internals",
        ),
        Correspondence::of(
            "Supervisor", "LLM worker", "read-only file mailbox",
            "feedback-capsule (filtered)", "Next iteration context supplied offline",
        ),
        Correspondence::of(
            "Research workers", "Supervisor", "scratch output directory",
            "research-claim", "Private authored opinion; no result authority",
        ),
        Correspondence::of(
            "Publisher", "Publication authority", "mTLS HTTP",
            "research-paper", "Sanitized derivative, evidence map and release policy",
        ),
        Correspondence::of(
            "Publication authority", "Evidence ledger", "mTLS HTTP",
            "attestation", "Release decision and signature",
        ),
        Correspondence::of(
            "Integrator", "Supervisor", "file mailbox",
            "research-case", "Transfer hypothesis only, not automatic promotion",
        ),
        Correspondence::of(
            "PKI issuer", "Evidence ledger", "mTLS HTTP",
            "certificate-event", "Issuance/renewal/revocation counts and reasons",
        ),
        Correspondence::of(
            "Trusted services", "Control OTLP", "OTLP/gRPC mTLS",
            "trace-record/metric-record", "Metadata, no content",
        ),
        Correspondence::of(
            "SUT diagnostic binary", "Product OTLP", "OTLP/gRPC mTLS",
            "trace-record/metric-record", "Diagnostic mode only",
        ),
        Correspondence::of(
            "Control/Product telemetry stores", "Projection service", "trusted read",
            "trace/metric projections", "Correlated by IDs only, no storage authority",
        ),
        ]
    }

    /// Whether this transfer crosses into an offline worker mailbox.
    #[must_use]
    pub fn is_offline_mailbox(&self) -> bool {
        self.transport.contains("file mailbox") || self.transport.contains("scratch output")
    }
}
