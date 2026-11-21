//! Use and derivation restrictions attached to a rights record.
//!
//! Mirrors `prohibited_derivations` and `permitted_purposes` of
//! `examples/v61-artifact-policy.example.json` and the conservative GPL
//! rule of `docs/20:21-24`.

use crate::permission::Permission;

/// The narrow list of things a grant expressly withholds.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Restriction {
    /// `grant_id` this restriction belongs to.
    pub grant_id: String,
    /// Uses the artifact may not serve.
    pub prohibited_uses: Vec<String>,
    /// Derivative classes that may not be produced.
    pub prohibited_derivations: Vec<String>,
    /// Attribution or notice obligations that must survive.
    pub obligations: Vec<String>,
    /// Jurisdiction whose law makes the restriction necessary.
    pub jurisdiction: String,
}

impl Restriction {
    /// Whether a proposed use is withheld.
    #[must_use]
    pub fn forbids(&self, use_kind: &str) -> bool {
        self.prohibited_uses.iter().any(|item| item == use_kind)
    }

    /// Whether a proposed derivation is withheld.
    #[must_use]
    pub fn forbids_derivation(&self, derivation: &str) -> bool {
        self.prohibited_derivations
            .iter()
            .any(|item| item == derivation)
    }

    /// A use passes only when nothing forbids it.
    #[must_use]
    pub fn permits(&self, use_kind: &str, derivation: &str) -> Permission {
        Permission::of(
            !self.forbids(use_kind)
                && !self.forbids_derivation(derivation)
                && Permission::filled(&self.grant_id) == Permission::Allow,
        )
    }
}
