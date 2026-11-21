//! EULA obligations distinct from an SPDX identifier.
//!
//! A private EULA may constrain modification, model-training use,
//! redistribution, publication, transfer or jurisdiction
//! (`docs/20:18-24,54-55`).

use crate::permission::Permission;
use crate::switch::Switch;

/// The governing end-user terms for one artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Eula {
    /// `eula_artifact_digest` in the protected vault.
    pub artifact_digest: String,
    /// Digest of the verbatim governing text.
    pub verbatim_digest: String,
    /// Whether modification is constrained.
    pub modification_restricted: Switch,
    /// Whether model-training use is constrained.
    pub training_restricted: Switch,
    /// Whether redistribution is constrained.
    pub redistribution_restricted: Switch,
    /// Whether publication is constrained.
    pub publication_restricted: Switch,
    /// Jurisdiction named by the terms.
    pub jurisdiction: String,
}

impl Eula {
    /// Whether the verbatim terms were preserved for review.
    #[must_use]
    pub fn preserved(&self) -> Permission {
        Permission::digest(&self.artifact_digest).and(Permission::digest(&self.verbatim_digest))
    }

    /// Whether a named use is constrained by the terms.
    #[must_use]
    pub fn restricts(&self, use_kind: &str) -> bool {
        match use_kind {
            "modify" => self.modification_restricted == Switch::On,
            "train" => self.training_restricted == Switch::On,
            "redistribute" => self.redistribution_restricted == Switch::On,
            "publish" => self.publication_restricted == Switch::On,
            _ => true,
        }
    }

    /// Whether a use is clear of the recorded restrictions.
    #[must_use]
    pub fn permits(&self, use_kind: &str) -> Permission {
        Permission::of(self.preserved() == Permission::Allow && !self.restricts(use_kind))
    }
}
