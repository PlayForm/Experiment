//! The Rust GPUI research exchange cockpit.
//!
//! Grounded in `TELEMETRY.md:30-46` (rows A-F, read-only server-policy
//! projections, WebView never an authorization bypass, no private decryption
//! keys by default) and `:24-25` (the append-only ledger remains authority even
//! if an exporter drops records). This is a typed panel specification; no UI
//! is rendered.

/// The operator observatory specification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cockpit;

impl Cockpit {
    /// Rows A-F of the research circuit panel (`TELEMETRY.md:33-39`).
    pub const ROWS: [&'static str; 8] = [
        "TOP NAV: Tenants | Campaigns | Research Exchange | Evidence | Papers | Trust | Settings",
        "ROW A: Trust health | Treasury balance | Network throughput | Active experiments",
        "ROW B: Researchers by stance / budget / token speed | Dissent map | Bids, accepts, rejects",
        "ROW C: Case lineage: parent -> patches -> oracles -> paired trials -> paper -> transfer",
        "ROW D: Resource market: token burn, CPU seconds, exclusive runner slots, unused reserves",
        "ROW E: Scientific quality: calibrated forecasts, cited claims, null results, replications",
        "ROW F: Sealed publication: release state, redaction, signed attestations, audit timeline",
        "OBSERVABILITY: control and research planes plus isolated product measurement",
    ];

    /// The cockpit is a read-only projection, not an authority
    /// (`TELEMETRY.md:42-46`).
    #[must_use]
    pub const fn is_authority() -> crate::Permission {
        crate::Permission::Deny
    }

    /// No private ciphertext key is delivered to the GUI by default
    /// (`TELEMETRY.md:45-46`).
    #[must_use]
    pub const fn may_receive_private_key() -> crate::Permission {
        crate::Permission::Deny
    }

    /// A dropped exporter does not erase the ledger authority
    /// (`TELEMETRY.md:24`).
    #[must_use]
    pub const fn ledger_survives_exporter_loss() -> crate::Permission {
        crate::Permission::Allow
    }
}
