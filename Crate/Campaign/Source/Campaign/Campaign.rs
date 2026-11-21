//! The v2 campaign root document.
//!
//! Grounded in every field of `examples/campaign.v2.json` (242 lines) and
//! `schemas/campaign.v2.schema.json` (724 lines, `additionalProperties: false`).
//! This is the typed decode target for the DESIGN bundle: schema version 2 is
//! explicit, v1 axes are migrated only through `Plan::mapping`, and unknown
//! keys or out-of-range values become warnings rather than aborts. No JSON is
//! produced by this crate.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    agent::Permission, campaign::{Agent, Bench, Build, Charter, Container, Feature, Fixture,
    Hardware, Source, Storage, Workload}, heuristic::Heuristic, Fault, Warning, SCHEMA_VERSION,
};

/// The complete v2 campaign document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Campaign {
    #[serde(rename = "schema_version")]
    pub schema_version: u32,
    pub project: Source,
    pub campaign: Charter,
    pub containers: Container,
    pub hardware: Hardware,
    pub builds: Vec<Build>,
    #[serde(rename = "feature_groups")]
    pub feature_groups: Vec<Feature>,
    pub heuristics: Heuristic,
    pub fixtures: Vec<Fixture>,
    pub workloads: Vec<Workload>,
    pub benchmark: Bench,
    pub agents: Agent,
    pub storage: Storage,
}

impl Campaign {
    /// The 13 required root keys (`schemas/campaign.v2.schema.json:8-22`).
    pub const KEYS: [&'static str; 13] = [
        "schema_version",
        "project",
        "campaign",
        "containers",
        "hardware",
        "builds",
        "feature_groups",
        "heuristics",
        "fixtures",
        "workloads",
        "benchmark",
        "agents",
        "storage",
    ];

    /// Decode JSON text into a typed campaign plus recoverable warnings.
    /// Malformed JSON is the only fatal case.
    pub fn decode(text: &str) -> Result<(Self, Vec<Warning>), Fault> {
        let value: Value = serde_json::from_str(text).map_err(|error| Fault::json("campaign", &error))?;
        let mut warnings = Warning::unknown(&value, &Self::KEYS, "campaign");
        warnings.extend(Self::version_warning(&value));
        let mut document: Self = serde_json::from_value(value)
            .map_err(|error| Fault::json("campaign", &error))?;
        document.validate(&mut warnings);
        Ok((document, warnings))
    }

    /// Warn on a version other than 2 rather than guessing a mapping
    /// (`schema.json:25`, `campaign.v2.json:2`, LAW-017).
    fn version_warning(value: &Value) -> Option<Warning> {
        let found = value
            .get("schema_version")
            .and_then(Value::as_u64)
            .unwrap_or(u64::from(SCHEMA_VERSION));
        (found != u64::from(SCHEMA_VERSION)).then(|| Warning::Version {
            found: found.to_string(),
            expected: SCHEMA_VERSION.to_string(),
        })
    }

    /// Check axes, references and declared thresholds; degrade with warnings.
    pub fn validate(&mut self, warnings: &mut Vec<Warning>) {
        [
            ("builds", self.builds.is_empty()),
            ("feature_groups", self.feature_groups.is_empty()),
            ("fixtures", self.fixtures.is_empty()),
            ("workloads", self.workloads.is_empty()),
        ]
        .into_iter()
        .filter(|(_, empty)| *empty)
        .for_each(|(domain, _)| warnings.push(Warning::Empty { domain }));

        Warning::collisions(&self.builds, "builds", |item| item.id.as_str(), warnings);
        Warning::collisions(&self.feature_groups, "feature_groups", |item| item.id.as_str(), warnings);
        Warning::collisions(&self.fixtures, "fixtures", |item| item.id.as_str(), warnings);
        Warning::collisions(&self.workloads, "workloads", |item| item.id.as_str(), warnings);

        let fixtures: std::collections::BTreeSet<&str> =
            self.fixtures.iter().map(|item| item.id.as_str()).collect();
        self.workloads
            .iter()
            .filter(|item| !fixtures.contains(item.fixture.as_str()))
            .for_each(|item| warnings.push(Warning::Reference {
                from: item.id.clone(),
                to: item.fixture.clone(),
            }));

        self.objectives().validate(warnings);
        self.benchmark.validate(warnings);
        self.campaign.validate(warnings);
        self.fixtures.iter().for_each(|item| item.validate(warnings));
        self.feature_groups.iter().for_each(|item| item.validate(warnings));

        match self.hardware.unknown_is_explicit() {
            true => warnings.push(Warning::Unavailable {
                source: "hardware policy".to_owned(),
                substitute: "conservative reference plan".to_owned(),
            }),
            false => {}
        }
        match self.agents.merge_is_manual() {
            Permission::Allow => warnings.push(Warning::Bound {
                domain: "agents",
                key: "auto_merge must be false".to_owned(),
            }),
            Permission::Deny => {}
        }
    }

    /// The objective block, reached through the charter.
    #[must_use]
    pub fn objectives(&self) -> &crate::campaign::Objective {
        &self.campaign.objectives
    }

    /// Whether this document is a v2 design document, never a v1 plan.
    #[must_use]
    pub fn is_v2(&self) -> bool {
        self.schema_version == SCHEMA_VERSION
    }

    /// The future-facing fields that remain design contracts, for status
    /// output (REQ-CLI-06).
    #[must_use]
    pub fn design_only_fields(&self) -> Vec<&'static str> {
        crate::campaign::Overlay::DESIGN_ONLY
            .into_iter()
            .map(|(field, _)| field)
            .collect()
    }
}
