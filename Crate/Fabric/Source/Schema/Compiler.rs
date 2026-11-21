//! Deterministic compilation of private IR into opaque handles.
//!
//! Mirrors steps 2-5 of `docs/05:9-25`: validate uniqueness, types, reference
//! closure and backwards compatibility; reject ambiguous cycles; compile
//! matching tables; produce a private symbol table; sign a build manifest
//! binding code, IR, compiler, resolver, policy, key epoch and migration state.
//! `docs/05:29-32` forbids randomizing internal executable layout. No code is
//! emitted and no manifest is signed here.

use crate::verdict::Verdict;
use super::definition::Definition;

/// A compiler identified by its deterministic version.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Compiler {
    /// Stable compiler revision recorded in the signed build manifest.
    pub version: String,
    /// Digest of the resolver closure the compiler validated.
    pub closure_digest: String,
}

impl Compiler {
    /// Record a compiler identity from trusted fields.
    #[must_use]
    pub fn of(version: &str, closure: &str) -> Self {
        Self {
            version: version.to_owned(),
            closure_digest: closure.to_owned(),
        }
    }

    /// Validate a supplied definition set for uniqueness and ambiguity using
    /// a named iterator chain, never a mutable loop.
    #[must_use]
    pub fn accepts<'a>(
        &self,
        definitions: impl IntoIterator<Item = &'a Definition>,
    ) -> Verdict {
        let items: Vec<&Definition> = definitions.into_iter().collect();
        let unique = items.iter().all(|left| {
            items
                .iter()
                .all(|right| left.ambiguous_with(right) != Verdict::Allow)
        });
        let shaped = items.iter().all(|item| item.authentic_shape() == Verdict::Allow);
        Verdict::filled(&self.version)
            .and(Verdict::digest(&self.closure_digest))
            .and(Verdict::of(unique && shaped))
    }

    /// Compilation is deterministic: internal layout never changes at restart
    /// (`docs/05:29-32`). Randomness belongs only in external handles, nonces
    /// and purpose-scoped pseudonyms.
    #[must_use]
    pub const fn deterministic() -> Verdict {
        Verdict::Allow
    }

    /// A build manifest binds these digests but is not produced by a worker.
    #[must_use]
    pub fn manifest_fields() -> [&'static str; 7] {
        [
            "code_digest",
            "source_ir_digest",
            "compiler_version",
            "resolver_closure",
            "policy_version",
            "key_epoch",
            "migration_state",
        ]
    }
}
