//! One signed, immutable semantic definition in the sealed registry.
//!
//! Mirrors `docs/05:3-7`: authored and signed IR with an immutable semantic
//! identifier and version, plus the `definition_hash` and `revision` fields of
//! `schemas/identifier-definition.schema.json`. LLM submissions cannot alter
//! trusted IR (`docs/05:11`).

use crate::verdict::Verdict;
use super::ir::Ir;

/// One authored definition awaiting deterministic compilation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Definition {
    /// Immutable private semantic identifier.
    pub semantic_id: String,
    /// Semantic IR kind of this definition.
    pub ir: Ir,
    /// Authored revision, never rewritten in place.
    pub revision: u64,
    /// Digest of the canonical signed IR bytes.
    pub definition_hash: String,
}

impl Definition {
    /// Record a signed definition from trusted fields.
    #[must_use]
    pub fn of(semantic_id: &str, ir: Ir, revision: u64, hash: &str) -> Self {
        Self {
            semantic_id: semantic_id.to_owned(),
            ir,
            revision,
            definition_hash: hash.to_owned(),
        }
    }

    /// Whether the definition is structurally complete and signed-shaped.
    #[must_use]
    pub fn authentic_shape(&self) -> Verdict {
        Verdict::filled(&self.semantic_id)
            .and(Verdict::of(self.revision >= 1))
            .and(Verdict::digest(&self.definition_hash))
    }

    /// Two definitions are ambiguous when the same semantic id appears at
    /// different revisions inside one immutable version.
    #[must_use]
    pub fn ambiguous_with(&self, other: &Self) -> Verdict {
        Verdict::of(self.semantic_id == other.semantic_id && self.revision != other.revision)
    }
}
