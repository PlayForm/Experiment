//! The v1 plan representation and its explicit, non-reinterpreting migration.
//!
//! Grounded in `README.md:7-9` ("extends ... no existing keys are implicitly
//! reinterpreted"), `:37-38` ("Schema v2 reader and migration from v1 plan
//! representation (without silently reinterpreting v1 config)"), LAW-017
//! (`REQUIREMENTS.md:101-103`) and REQ-ID-07 (`REQUIREMENTS.md:369-370`).
//!
//! A `Mapping` is a named transition, never an inference: v1 axes are carried
//! by explicit name, v2 dimensions are declared as new, and nothing is guessed.
//!
//! Exception declared: `Mapping` is declared beside `Plan` because it is the
//! transition table of this one type and has no independent lifecycle.

use crate::Warning;

/// One explicit migration step between the v1 plan and v2 campaign.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mapping {
    /// A v1 axis is carried into a named v2 field under the same meaning.
    Carry {
        from: &'static str,
        to: &'static str,
    },
    /// A v2 dimension has no v1 counterpart and is introduced, not derived.
    Extend { to: &'static str },
    /// A v1 field has no v2 counterpart and is retained only as evidence.
    Retain { from: &'static str },
}

impl Mapping {
    /// Render the step for a migration report.
    #[must_use]
    pub fn line(&self) -> String {
        match self {
            Self::Carry { from, to } => format!("carry {from} -> {to} (meaning unchanged)"),
            Self::Extend { to } => format!("extend: {to} is new in v2, not derived from v1"),
            Self::Retain { from } => format!("retain {from} as v1 evidence only"),
        }
    }
}

/// The v1 `prun-bench.json` plan axes the v2 campaign extends.
///
/// These are the three v0 axes (refs, builds, feature_sets) plus the fixture
/// and workload lists, consumed by path/type name from
/// `benchmark::config::Manifest` - this crate does not edit or import that
/// crate's types.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Plan {
    pub refs: Vec<String>,
    pub builds: Vec<String>,
    pub feature_sets: Vec<String>,
    pub fixtures: Vec<String>,
    pub workloads: Vec<String>,
}

impl Plan {
    /// Recognised v1 plan keys.
    pub const KEYS: [&'static str; 5] = ["refs", "builds", "feature_sets", "fixtures", "workloads"];

    /// The complete explicit migration path to v2.
    #[must_use]
    pub fn mapping() -> [Mapping; 12] {
        [
            Mapping::Carry {
                from: "refs",
                to: "project.source_ref",
            },
            Mapping::Carry {
                from: "builds",
                to: "builds",
            },
            Mapping::Carry {
                from: "feature_sets",
                to: "feature_groups",
            },
            Mapping::Carry {
                from: "fixtures",
                to: "fixtures",
            },
            Mapping::Carry {
                from: "workloads",
                to: "workloads",
            },
            Mapping::Retain {
                from: "invariants",
            },
            Mapping::Extend {
                to: "heuristics",
            },
            Mapping::Extend {
                to: "hardware",
            },
            Mapping::Extend {
                to: "agents",
            },
            Mapping::Extend {
                to: "safety",
            },
            Mapping::Extend {
                to: "benchmark",
            },
            Mapping::Extend {
                to: "storage",
            },
        ]
    }

    /// Warn about every v2 dimension that has no v1 source, so no reader can
    /// mistake an extension for a reinterpretation of v1 config.
    pub fn audit(&self, warnings: &mut Vec<Warning>) {
        Self::mapping()
            .into_iter()
            .filter(|step| matches!(step, Mapping::Extend { .. }))
            .for_each(|step| match step {
                Mapping::Extend { to } => warnings.push(Warning::Unmapped {
                    from: "v1 plan",
                    to,
                }),
                _ => {}
            });
        [("refs", self.refs.is_empty()), ("builds", self.builds.is_empty())]
            .into_iter()
            .filter(|(_, empty)| *empty)
            .for_each(|(domain, _)| warnings.push(Warning::Empty { domain }));
    }
}
