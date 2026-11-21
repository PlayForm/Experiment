//! Typed `prun-bench.json` root with versioned defaults and warnings.
//!
//! Grounded in `prun-bench.schema.json` (Draft 2020-12 contract),
//! `prun-bench.json`, and `prun_lab.py:189-217` / `:30-91`. The controller
//! aborts on drift; this model records a `Warning`, clamps where safe, and
//! keeps the document inspectable. Only unreadable or unparseable JSON is a
//! `Fault`.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::fixture::Fixture;
use crate::{Fault, Warning, CONFIG_VERSION};

use super::{BuildConfig, DefaultFeatures, Exclude, FeatureSet, Invariant, Parallel, Profile, Ref, Workload};

/// The decoded benchmark configuration document.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(default = "Manifest::version")]
    pub schema_version: u32,
    #[serde(default)]
    pub invariants: Invariant,
    #[serde(default = "Manifest::references")]
    pub refs: Vec<Ref>,
    #[serde(default = "Manifest::builds")]
    pub builds: Vec<BuildConfig>,
    #[serde(default = "Manifest::feature_sets")]
    pub feature_sets: Vec<FeatureSet>,
    #[serde(default = "Manifest::fixtures")]
    pub fixtures: Vec<Fixture>,
    #[serde(default = "Manifest::workloads")]
    pub workloads: Vec<Workload>,
    #[serde(default)]
    pub exclude: Vec<Exclude>,
}

impl Manifest {
    /// Recognised root keys (schema.json:8-16).
    const KEYS: [&'static str; 8] = [
        "schema_version",
        "invariants",
        "refs",
        "builds",
        "feature_sets",
        "fixtures",
        "workloads",
        "exclude",
    ];

    /// Recognised invariant keys (schema.json:24-85).
    const INVARIANT_KEYS: [&'static str; 10] = [
        "builder_base",
        "runner_base",
        "hyperfine_version",
        "cpuset",
        "memory",
        "warmups",
        "runs",
        "environment",
        "metrics",
        "runner_cpusets",
    ];

    /// Decode JSON text into a typed manifest plus its warnings.
    pub fn decode(text: &str) -> Result<(Self, Vec<Warning>), Fault> {
        let value: Value = serde_json::from_str(text).map_err(|error| Fault::Json {
            path: "config".to_owned(),
            detail: error.to_string(),
        })?;
        let mut warnings = Warning::unknown(&value, &Self::KEYS, "Manifest");
        warnings.extend(Self::version_warning(&value));
        warnings.extend(
            value
                .get("invariants")
                .map(|part| Warning::unknown(part, &Self::INVARIANT_KEYS, "Invariant"))
                .into_iter()
                .flatten(),
        );
        let mut manifest: Self = serde_json::from_value(value).map_err(|error| Fault::Json {
            path: "config".to_owned(),
            detail: error.to_string(),
        })?;
        manifest.validate(&mut warnings);
        Ok((manifest, warnings))
    }

    /// Warn on a schema version other than the supported one (prun_lab.py:191).
    fn version_warning(value: &Value) -> Option<Warning> {
        let found = value
            .get("schema_version")
            .and_then(Value::as_u64)
            .unwrap_or(u64::from(CONFIG_VERSION));
        (found != u64::from(CONFIG_VERSION)).then(|| Warning::Version {
            found: found.to_string(),
            expected: CONFIG_VERSION.to_string(),
        })
    }

    /// Detect drift and degrade: clamp `runs`, warn on empty axes, duplicate
    /// identifiers, dangling fixture references and bad profile env keys.
    pub fn validate(&mut self, warnings: &mut Vec<Warning>) {
        let clamped = (self.invariants.runs < 2).then(|| {
            self.invariants.runs = 2;
            Warning::Range {
                domain: "invariants",
                key: "runs".to_owned(),
            }
        });
        warnings.extend(clamped);

        [
            ("refs", self.refs.is_empty()),
            ("builds", self.builds.is_empty()),
            ("feature_sets", self.feature_sets.is_empty()),
            ("fixtures", self.fixtures.is_empty()),
            ("workloads", self.workloads.is_empty()),
        ]
        .into_iter()
        .filter(|(_, empty)| *empty)
        .for_each(|(domain, _)| warnings.push(Warning::Empty { domain }));

        Self::collisions(&self.refs, "refs", |item| item.id.as_str(), warnings);
        Self::collisions(&self.builds, "builds", |item| item.id.as_str(), warnings);
        Self::collisions(
            &self.feature_sets,
            "feature_sets",
            |item| item.id.as_str(),
            warnings,
        );
        Self::collisions(&self.fixtures, "fixtures", |item| item.id.as_str(), warnings);
        Self::collisions(&self.workloads, "workloads", |item| item.id.as_str(), warnings);

        let defined: BTreeSet<&str> = self.fixtures.iter().map(|item| item.id.as_str()).collect();
        self.workloads
            .iter()
            .filter(|item| !defined.contains(item.fixture.as_str()))
            .for_each(|item| {
                warnings.push(Warning::Reference {
                    from: item.id.clone(),
                    to: item.fixture.clone(),
                })
            });

        self.builds
            .iter()
            .flat_map(|build| {
                build
                    .profile_env
                    .keys()
                    .map(move |key| (build.id.as_str(), key.as_str()))
            })
            .filter(|(_, key)| !Self::env_key(key))
            .for_each(|(build, key)| {
                warnings.push(Warning::Range {
                    domain: "profile_env",
                    key: format!("{build}.{key}"),
                })
            });
    }

    /// Fold repeated identifiers, keeping the first (prun_lab.py:196-199).
    fn collisions<T>(
        items: &[T],
        domain: &'static str,
        field: impl Fn(&T) -> &str,
        warnings: &mut Vec<Warning>,
    ) {
        let counts = items.iter().fold(BTreeMap::<&str, usize>::new(), |mut tally, item| {
            *tally.entry(field(item)).or_default() += 1;
            tally
        });
        counts
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .for_each(|(key, _)| warnings.push(Warning::Collision {
                domain,
                id: key.to_owned(),
            }));
    }

    /// Match `CARGO_PROFILE_(RELEASE|DEV)_[A-Z0-9_]+` without a regex crate
    /// (prun_lab.py:208-214).
    fn env_key(key: &str) -> bool {
        key.strip_prefix("CARGO_PROFILE_RELEASE_")
            .or_else(|| key.strip_prefix("CARGO_PROFILE_DEV_"))
            .map(|tail| {
                !tail.is_empty()
                    && tail.chars().all(|character| {
                        character.is_ascii_uppercase()
                            || character.is_ascii_digit()
                            || character == '_'
                    })
            })
            .unwrap_or(false)
    }

    fn version() -> u32 {
        CONFIG_VERSION
    }

    fn references() -> Vec<Ref> {
        vec![Ref {
            id: "base".to_owned(),
            git_ref: "HEAD".to_owned(),
        }]
    }

    fn builds() -> Vec<BuildConfig> {
        vec![
            BuildConfig {
                id: "release".to_owned(),
                profile: Profile::Release,
                rustflags: Vec::new(),
                profile_env: BTreeMap::new(),
            },
            BuildConfig {
                id: "native".to_owned(),
                profile: Profile::Release,
                rustflags: vec!["-Ctarget-cpu=native".to_owned()],
                profile_env: BTreeMap::new(),
            },
            BuildConfig {
                id: "debug".to_owned(),
                profile: Profile::Dev,
                rustflags: Vec::new(),
                profile_env: BTreeMap::new(),
            },
        ]
    }

    fn feature_sets() -> Vec<FeatureSet> {
        vec![FeatureSet {
            id: "default".to_owned(),
            features: Vec::new(),
            defaults: DefaultFeatures::Enabled,
        }]
    }

    fn fixtures() -> Vec<Fixture> {
        vec![
            Fixture {
                id: "small".to_owned(),
                directories: 32,
                depth: 2,
                noise_files: 3,
            },
            Fixture {
                id: "wide".to_owned(),
                directories: 256,
                depth: 3,
                noise_files: 3,
            },
        ]
    }

    fn workloads() -> Vec<Workload> {
        vec![
            Workload {
                id: "small-serial".to_owned(),
                fixture: "small".to_owned(),
                pattern: "bench.marker".to_owned(),
                parallel: Parallel::Serial,
                command: "/bin/true".to_owned(),
            },
            Workload {
                id: "small-parallel".to_owned(),
                fixture: "small".to_owned(),
                pattern: "bench.marker".to_owned(),
                parallel: Parallel::Concurrent,
                command: "/bin/true".to_owned(),
            },
            Workload {
                id: "wide-serial".to_owned(),
                fixture: "wide".to_owned(),
                pattern: "bench.marker".to_owned(),
                parallel: Parallel::Serial,
                command: "/bin/true".to_owned(),
            },
            Workload {
                id: "wide-parallel".to_owned(),
                fixture: "wide".to_owned(),
                pattern: "bench.marker".to_owned(),
                parallel: Parallel::Concurrent,
                command: "/bin/true".to_owned(),
            },
        ]
    }
}
