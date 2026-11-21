//! Non-LLM statistical evaluator: the only source of numeric verdicts.
//!
//! Grounded in `ARCHITECTURE.md:58-59,88-89`, `REQUIREMENTS_V3.md` EXP-005/
//! EXP-009 and `playbooks/fanout-5.yaml:80-83`. It consumes validated raw
//! samples under a versioned protocol and emits a typed verdict. It preserves
//! inconclusive and invalid outcomes instead of rewriting them as wins.
//!
//! Exception declared: `Observation` is the typed input of this one evaluator
//! and has no independent lifecycle outside `Evaluator`.

use crate::evaluate::verdict::Verdict;
use crate::evaluate::protocol::Protocol;
use crate::gate::reason::Reason;

/// A validated observation presented to the evaluator.
#[derive(Clone, Debug, PartialEq)]
pub enum Observation {
    /// A valid paired sample block with its estimate.
    Valid {
        ratio: f64,
        ci_low: f64,
        ci_high: f64,
        attempts: u32,
    },
    /// The sample or manifest failed integrity checks.
    Invalid { reason: Reason },
    /// Host contamination was detected.
    Contaminated,
    /// The trial block was incomplete.
    Incomplete,
    /// Confidence could not be established within budget.
    NoConfidence,
}

/// A deterministic evaluator bound to one protocol version.
#[derive(Clone, Debug)]
pub struct Evaluator {
    /// The versioned protocol used for every classification.
    pub protocol: Protocol,
}

impl Evaluator {
    /// Classify one observation without ever consulting a model.
    #[must_use]
    pub fn classify(&self, observation: Observation) -> Verdict {
        match observation {
            Observation::Invalid { .. }
            | Observation::Contaminated
            | Observation::Incomplete => Verdict::Invalid,
            Observation::NoConfidence => Verdict::Inconclusive,
            Observation::Valid {
                ratio,
                ci_low,
                ci_high,
                ..
            } => match (
                self.protocol.is_win(ratio, ci_low),
                self.protocol.is_regression(ci_high),
            ) {
                (true, _) => Verdict::Win,
                (_, true) => Verdict::Loss,
                (false, false) => Verdict::Tie,
            },
        }
    }

    /// The protocol version this verdict will cite.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.protocol.version
    }
}
