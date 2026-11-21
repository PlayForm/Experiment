//! Preserved inconclusive result; absence of confidence is not a win.
//!
//! Grounded in `REQUIREMENTS_V3.md` INV-010 (rejected and inconclusive
//! experiments are discoverable with a reason and cannot be rewritten as
//! untested duplicate wins), `RESEARCH_GATE.md:94-95` (negative evidence
//! matters) and `feedback-capsule.v3.schema.json:37-45` (`inconclusive`
//! evaluation status).

use crate::gate::reason::Reason;

/// An inconclusive evaluation that must stay discoverable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Inconclusive {
    /// Workload or trial ids that lacked confidence.
    pub trials: Vec<String>,
    /// Digest of the raw samples retained for future reanalysis.
    pub raw_samples_sha256: String,
    /// Typed reasons why confidence was not established.
    pub reasons: Vec<Reason>,
    /// Analysis protocol version used.
    pub protocol_version: String,
}

impl Inconclusive {
    /// An inconclusive result can never be promoted as a win.
    #[must_use]
    pub const fn may_promote() -> bool {
        false
    }

    /// Whether the result may be reanalyzed under a newer protocol.
    #[must_use]
    pub fn is_reanalyzable(&self) -> bool {
        !self.raw_samples_sha256.is_empty()
    }

    /// The capsule status token for this result.
    #[must_use]
    pub const fn status() -> &'static str {
        "inconclusive"
    }
}
