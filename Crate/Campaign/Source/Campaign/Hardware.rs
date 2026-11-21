//! Hardware and execution-environment dimensions.
//!
//! Grounded in `campaign.v2.json:55-66`, the schema at
//! `schemas/campaign.v2.schema.json:272-318` and `REQUIREMENTS.md:673-716`
//! (REQ-HW-01..09). Three independent profiles - physical host, effective
//! environment and compiler target - stay distinct fields; `native=true` is
//! never a distribution policy (LAW-012).
//!
//! Exception declared: `Smt`, `Numa` and `Cohort` are declared beside `Hardware`
//! because each is the typed form of one field and has no independent
//! lifecycle.

use serde::{Deserialize, Serialize};

use crate::agent::Permission;

/// Sibling-thread policy (`schemas/campaign.v2.schema.json:286-289`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Smt {
    /// Isolate siblings so a competing thread cannot contaminate timing.
    #[serde(rename = "isolate_siblings")]
    Isolate,
    /// Allow only when recorded as a separate cohort.
    #[serde(rename = "allow_recorded")]
    Recorded,
    /// Unknown values are not treated as equal (REQ-ID-09).
    #[serde(rename = "unknown")]
    Unknown,
}

/// NUMA policy (`schemas/campaign.v2.schema.json:290-293`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Numa {
    Local,
    Interleaved,
    Auto,
    Unknown,
}

/// How the host is exposed to the compiler target (`REQ-HW-09`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Cohort {
    /// Explicit three-axis host/effective/target specification.
    Explicit,
    /// Build-host detection silently substituted for the target - forbidden.
    BuilderDetected,
}

/// A hardware cohort descriptor.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Hardware {
    #[serde(rename = "host_id")]
    pub host_id: String,
    #[serde(rename = "class_id")]
    pub class_id: String,
    #[serde(rename = "cpu_set")]
    pub cpu_set: String,
    #[serde(rename = "smt_policy", default)]
    pub smt: Smt,
    #[serde(rename = "numa_policy", default)]
    pub numa: Numa,
    #[serde(rename = "measurement_exclusive", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub measurement_exclusive: Permission,
    #[serde(rename = "physical_isolation_required_for_promotion", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub physical_isolation: Permission,
    #[serde(rename = "effective_cpu_quota", default)]
    pub effective_cpu_quota: String,
    #[serde(rename = "storage_cohort")]
    pub storage_cohort: String,
    #[serde(rename = "hardware_profile_artifact", default)]
    pub hardware_profile_artifact: Option<String>,
    /// Which profile supplied the target capability (a typed v2 addition).
    #[serde(rename = "cohort_kind", default)]
    pub cohort: Cohort,
}

impl Hardware {
    /// Recognised JSON keys (`schemas/campaign.v2.schema.json:311-317`).
    pub const KEYS: [&'static str; 10] = [
        "host_id",
        "class_id",
        "cpu_set",
        "smt_policy",
        "numa_policy",
        "measurement_exclusive",
        "physical_isolation_required_for_promotion",
        "effective_cpu_quota",
        "storage_cohort",
        "hardware_profile_artifact",
    ];

    /// Whether a candidate may be promoted on this cohort. Disjoint cpusets
    /// are necessary but not sufficient (LAW-005, REQ-HW-03).
    #[must_use]
    pub fn promotion_is_isolated(&self) -> Permission {
        self.measurement_exclusive
            .and(self.physical_isolation)
            .and(match self.smt {
                Smt::Isolate => Permission::Allow,
                Smt::Recorded | Smt::Unknown => Permission::Deny,
            })
    }

    /// Warn when an unknown environment field is treated as a known cohort
    /// (REQ-ID-09, REQ-HW-01).
    #[must_use]
    pub fn unknown_is_explicit(&self) -> bool {
        matches!(self.smt, Smt::Unknown) || matches!(self.numa, Numa::Unknown)
    }
}
