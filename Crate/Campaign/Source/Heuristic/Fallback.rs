//! Named fallback strategies and reference selection.
//!
//! Grounded in LAW-008 (`REQUIREMENTS.md:73-76`), `campaign.v2.json:133`
//! (`fallback_strategy: reference`) and REQ-HEU-09 / REQ-HEU-10 (`:229-231`).
//! An unknown strategy degrades to the conservative choice rather than being
//! reinterpreted.

use serde::{Deserialize, Serialize};

use crate::heuristic::{Analysis, Class};

/// The fallback a planner records when it cannot safely choose a strategy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Fallback {
    /// Use the verified reference strategy.
    Reference,
    /// Use a conservative plan that changes no observable ordering.
    Conservative,
    /// Stop and report; never fabricate a decision.
    FailClosed,
}

impl Fallback {
    /// Token used in the campaign document and explain-plan output.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Reference => "reference",
            Self::Conservative => "conservative",
            Self::FailClosed => "fail_closed",
        }
    }

    /// Recover a fallback from a document token; unknown values degrade to
    /// `FailClosed` (REQ-FEA-06).
    #[must_use]
    pub fn from_key(key: &str) -> Self {
        match key {
            "reference" => Self::Reference,
            "conservative" => Self::Conservative,
            _ => Self::FailClosed,
        }
    }

    /// Choose a fallback from an analysis. Unknown or mixed classifications
    /// always use the reference (LAW-008).
    #[must_use]
    pub fn decide(analysis: &Analysis) -> Self {
        match analysis.classify() {
            Class::Unknown | Class::Mixed => Self::Reference,
            Class::KnownCpu | Class::LikelyIo | Class::ShortSpawn | Class::LongRunning => {
                Self::Conservative
            }
        }
    }
}
