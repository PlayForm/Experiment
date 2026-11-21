//! A predeclared, versioned comparison protocol.
//! Grounded in `AGENT_CONTRACT.md:63-75` (predeclare thresholds, compare only
//! protocol-compatible references, include planner overhead and startup) and
//! `RESEARCH_GATE.md:83-88` (paired ABBA/BAAB, warmup, sample counts,
//! multiple-testing threshold).

/// The declared threshold a candidate must clear.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Protocol {
    /// Protocol name and version token.
    pub name: String,
    /// Practical-effect threshold in percent.
    pub effect_threshold_percent: u32,
    /// Number of paired blocks required.
    pub blocks: u32,
}

impl Protocol {
    /// Declare a protocol before any sample is observed.
    #[must_use]
    pub fn declare(name: &str, effect_threshold_percent: u32, blocks: u32) -> Self {
        Self {
            name: name.to_owned(),
            effect_threshold_percent,
            blocks,
        }
    }

    /// Whether an observed effect clears the predeclared threshold.
    #[must_use]
    pub const fn met(&self, observed_percent: u32) -> bool {
        observed_percent >= self.effect_threshold_percent
    }

    /// Whether the protocol is fully specified.
    #[must_use]
    pub fn is_declared(&self) -> bool {
        !self.name.is_empty() && self.blocks > 0
    }
}
