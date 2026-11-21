//! Repository-agnostic admission and the limits of transfer.
//!
//! Grounded in `docs/13-repository-agnostic.md:32-45` and
//! `docs/25-failure-analysis.md` F18: an idea transferred from one repository
//! is a hypothesis with source evidence, not a portable speedup. A new target
//! repository or host requires a fresh build, oracle and benchmark experiment
//! and a separate attestation. This type makes the no-overgeneralization rule a
//! value a caller must consult.

/// A transfer between repository trust domains.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Agnostic {
    /// Source repository identity the idea came from.
    pub source: String,
    /// Target repository identity the idea is proposed for.
    pub target: String,
    /// Whether the target has run its own oracle and benchmark.
    pub fresh_experiment: crate::Permission,
}

impl Agnostic {
    /// Record a proposed transfer; a source equal to target is not a transfer.
    #[must_use]
    pub fn of(
        source: String,
        target: String,
        fresh_experiment: crate::Permission,
    ) -> Result<Self, crate::Warning> {
        match source == target {
            true => Err(crate::Warning::InvalidValue {
                field: "transfer",
                detail: "source and target repository are identical".to_owned(),
            }),
            false => Ok(Self {
                source,
                target,
                fresh_experiment,
            }),
        }
    }

    /// A transferred gain is never a confirmation in the target repository.
    #[must_use]
    pub const fn confirms_performance(&self) -> crate::Permission {
        crate::Permission::Deny
    }

    /// Whether this transfer may enter fresh target experiments at all.
    #[must_use]
    pub const fn admissible(&self) -> crate::Permission {
        self.fresh_experiment
    }
}
