//! Full Pareto frontier over a hardware-classified cohort.
//!
//! Grounded in `docs/00-system-constitution.md:14-15` and
//! `docs/02-pitfalls-and-verification.md:14` ("preserve full cohort/hardware
//! matrix and Pareto frontier, including regressions and cases with no
//! benefit"). `docs/10-benchmark-mesh.md:35-39` allows targeted implementations
//! when general promotion fails. A dominated candidate is retained as
//! specialized if it targets a declared hardware class, so no winner is
//! inferred from one fixture alone.

use crate::{mesh::Candidate, Permission};

/// The preserved frontier for one optimization campaign.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Pareto {
    /// All admitted candidates, including non-frontier and regressing ones.
    pub admitted: Vec<Candidate>,
    /// Digests of candidates selected as specialized targets.
    pub targeted: Vec<crate::Digest>,
}

impl Pareto {
    /// Create an empty frontier.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            admitted: Vec::new(),
            targeted: Vec::new(),
        }
    }

    /// Admit a correct candidate; a failed or inconclusive one is refused.
    #[must_use]
    pub fn admit(&mut self, candidate: Candidate) -> Result<Permission, crate::Warning> {
        match candidate.verdict {
            crate::evidence::Verdict::Pass => {
                self.admitted.push(candidate);
                Ok(Permission::Allow)
            }
            crate::evidence::Verdict::Fail
            | crate::evidence::Verdict::Contaminated
            | crate::evidence::Verdict::Inconclusive => Err(crate::Warning::Inconclusive {
                subject: format!("candidate {:?}", candidate.binary.as_str()),
            }),
        }
    }

    /// Non-dominated candidates, each in its own hardware class.
    #[must_use]
    pub fn frontier(&self) -> Vec<&Candidate> {
        self.admitted
            .iter()
            .filter(|point| {
                !self
                    .admitted
                    .iter()
                    .any(|other| other.dominates(point) && other != *point)
            })
            .collect()
    }

    /// Mark a candidate as a targeted implementation for its hardware class.
    #[must_use]
    pub fn target(&mut self, binary: crate::Digest) -> Result<Permission, crate::Warning> {
        let known = self.admitted.iter().any(|point| point.binary == binary);
        match known {
            true => {
                self.targeted.push(binary);
                Ok(Permission::Allow)
            }
            false => Err(crate::Warning::MissingArtifact {
                digest: binary.as_str().to_owned(),
            }),
        }
    }
}

