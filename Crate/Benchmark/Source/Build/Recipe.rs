//! Builder recipe represented as argv and environment, not a Dockerfile.
//!
//! Grounded in `prun_lab.py::builder_dockerfile` (309-332): the build is
//! `cargo build --locked --profile <profile> --bin Run`, optionally with
//! `--no-default-features` and `--features <list>`, with `RUSTFLAGS` and the
//! sorted `CARGO_PROFILE_*` overrides injected as environment data.

use crate::config::{BuildConfig, FeatureSet, Invariant};

/// The typed build command for one variant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Recipe {
    /// Builder base image.
    pub base: String,
    /// Cargo profile argument.
    pub profile: String,
    /// Cargo output subdirectory for the binary.
    pub outdir: String,
    /// Complete cargo argv.
    pub argv: Vec<String>,
    /// Environment assignments in deterministic order.
    pub env: Vec<(String, String)>,
}

impl Recipe {
    /// Compose the recipe from config data (prun_lab.py:309-332).
    #[must_use]
    pub fn compose(invariant: &Invariant, build: &BuildConfig, features: &FeatureSet) -> Self {
        let profile = build.profile.cargo_name().to_owned();
        let outdir = build.profile.outdir().to_owned();
        let mut argv = vec![
            "cargo".to_owned(),
            "build".to_owned(),
            "--locked".to_owned(),
            "--profile".to_owned(),
            profile.clone(),
            "--bin".to_owned(),
            "Run".to_owned(),
        ];
        features
            .defaults
            .flag()
            .into_iter()
            .for_each(|flag| argv.push(flag.to_owned()));
        features.selection().into_iter().for_each(|selection| {
            argv.push("--features".to_owned());
            argv.push(selection);
        });
        let mut env = vec![("RUSTFLAGS".to_owned(), build.rustflag_value())];
        env.extend(
            build
                .ordered_env()
                .into_iter()
                .map(|(key, value)| (key.to_owned(), value.to_owned())),
        );
        Self {
            base: invariant.builder_base.clone(),
            profile,
            outdir,
            argv,
            env,
        }
    }

    /// Source path of the copied binary (prun_lab.py:331).
    #[must_use]
    pub fn binary_source(&self) -> String {
        format!("/output/{}/Run", self.outdir)
    }
}
