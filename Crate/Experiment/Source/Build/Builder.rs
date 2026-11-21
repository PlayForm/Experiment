//! Non-LLM builder: validate a recipe, then publish an observed artifact.
//!
//! Grounded in `ARCHITECTURE.md:52-53` (reproducibly compiles immutable source
//! using a pinned recipe, then hashes and publishes artifacts),
//! `playbooks/fanout-5.yaml:61-65` (concurrency 3, requires novelty_checked and
//! scope_checked) and `CONTAINER_MATRIX.md:17`. This model performs no compile:
//! it validates that the recipe is pinned and records the observed digest as a
//! published artifact. A missing or malformed hash degrades to a warning.

use crate::build::artifact::{Artifact, Custody};
use crate::build::digest::Digest;
use crate::build::recipe::Recipe;
use crate::permission::Permission;
use crate::warning::Warning;

/// A purpose-bound builder, never an LLM.
#[derive(Clone, Debug)]
pub struct Builder {
    /// Stable builder identity, recorded on the build attestation.
    pub name: String,
    /// The pinned recipe this builder is bound to.
    pub recipe: Recipe,
    /// The maximum number of parallel compiles in the pool.
    pub concurrency: usize,
}

impl Builder {
    /// Validate the recipe before consuming build time.
    pub fn prepare(&self) -> Result<&Recipe, Warning> {
        match self.recipe.is_pinned() {
            true => Ok(&self.recipe),
            false => Err(Warning::Unreachable {
                service: "builder-unpinned-recipe",
            }),
        }
    }

    /// Publish the observed digest as an immutable artifact.
    pub fn publish(&self, observed: Digest, binary_relpath: String) -> Artifact {
        Artifact {
            digest: observed.value,
            build_key: self.recipe.key().to_string(),
            instrumentation_mode: self.recipe.instrumentation_mode.clone(),
            binary_relpath,
            custody: Custody::Published,
        }
    }

    /// Whether this builder may run an artifact through a measurement. It never
    /// may; building and measuring are separate purposes
    /// (`ARCHITECTURE.md:60-61`).
    #[must_use]
    pub const fn may_measure() -> Permission {
        Permission::Deny
    }
}
