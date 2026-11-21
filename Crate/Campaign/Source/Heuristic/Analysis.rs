//! Typed analyzer result and workload classification.
//!
//! Grounded in REQ-HEU-01 and REQ-HEU-03 (`REQUIREMENTS.md:216-226`) and the
//! feature vector at `:244-261`. Unknowns stay explicit; a program name alone
//! never establishes safe mutability (REQ-HEU-04).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// A classification label with confidence (`REQUIREMENTS.md:220-223`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Class {
    KnownCpu,
    LikelyIo,
    ShortSpawn,
    LongRunning,
    Mixed,
    /// Low confidence or an unseen case - must fall back to reference.
    Unknown,
}

/// The analyzer's return value, modelled as data.
// `f64` is not `Eq`, so comparison is deliberately partial.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Analysis {
    pub features: BTreeMap<String, String>,
    pub constraints: Vec<String>,
    pub confidence: f64,
    pub evidence: Vec<String>,
    #[serde(rename = "elapsed_ns")]
    pub elapsed_ns: u64,
    pub unknowns: Vec<String>,
}

impl Analysis {
    /// REQ-HEU-01's six fields in declaration order.
    pub const KEYS: [&'static str; 6] = [
        "features",
        "constraints",
        "confidence",
        "evidence",
        "elapsed_ns",
        "unknowns",
    ];

    /// Whether the analysis is confident enough to plan from.
    #[must_use]
    pub fn is_known(&self) -> bool {
        self.confidence.is_finite() && self.unknowns.is_empty() && self.confidence > 0.0
    }

    /// The classification, defaulting to `Unknown` on any malformed value.
    #[must_use]
    pub fn classify(&self) -> Class {
        match (self.is_known(), self.features.get("class").map(String::as_str)) {
            (false, _) => Class::Unknown,
            (true, Some("cpu_bound")) => Class::KnownCpu,
            (true, Some("io_bound")) => Class::LikelyIo,
            (true, Some("short_spawn")) => Class::ShortSpawn,
            (true, Some("long_running")) => Class::LongRunning,
            (true, Some("mixed")) => Class::Mixed,
            (true, _) => Class::Unknown,
        }
    }
}
