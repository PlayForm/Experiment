//! The 102-line autonomous-week YAML as a typed playbook document.
//!
//! Grounded in every section of `playbooks/autonomous-week.yaml`: version and
//! mission (`:1-7`), resource policy (`:8-22`), ten phases (`:23-83`),
//! recovery (`:84-91`) and reporting (`:92-102`). Building this value emits no
//! YAML; the playbook remains a future-facing contract.

use crate::{
    week::{Budget, Kind, Phase, Recovery, Reporting},
    Warning,
};

/// A typed copy of the autonomous-week playbook.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Playbook {
    pub version: u32,
    pub id: String,
    pub mission: String,
    pub base_commit: String,
    pub budget: Budget,
    pub phases: Vec<Phase>,
    pub recovery: Vec<Recovery>,
    pub reporting: Reporting,
}

impl Playbook {
    /// The only documented playbook version (`autonomous-week.yaml:2`).
    pub const VERSION: u32 = 1;

    /// Construct the playbook from the design values. No file is read.
    #[must_use]
    pub fn from_design() -> Self {
        Self {
            version: Self::VERSION,
            id: "autonomous-week".to_owned(),
            mission: "Improve PlayForm/Run total runtime without changing contract semantics. \
                      Explore one conceptual source mutation per branch; keep full lineage."
                .to_owned(),
            base_commit: "Current".to_owned(),
            budget: Budget::default(),
            phases: Phase::all(),
            recovery: Recovery::ALL.to_vec(),
            reporting: Reporting::default(),
        }
    }

    /// Find a phase by its typed identifier.
    #[must_use]
    pub fn phase(&self, kind: Kind) -> Option<&Phase> {
        self.phases.iter().find(|phase| phase.kind == kind)
    }

    /// Validate the typed document, reporting missing evidence rather than
    /// inventing it.
    pub fn validate(&self, warnings: &mut Vec<Warning>) {
        match self.version {
            Self::VERSION => {}
            found => warnings.push(Warning::Version {
                found: found.to_string(),
                expected: Self::VERSION.to_string(),
            }),
        }
        self.budget.validate(warnings);
        self.phases.iter().for_each(|phase| phase.validate(warnings));
        self.reporting.validate(warnings);
        match self.recovery.is_empty() {
            true => warnings.push(Warning::Empty {
                domain: "playbook.recovery",
            }),
            false => {}
        }
        match self.phases.len() {
            10 => {}
            found => warnings.push(Warning::Bound {
                domain: "playbook.phases",
                key: format!("expected 10, found {found}"),
            }),
        }
    }
}
