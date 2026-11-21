//! The sealed private symbol table that conceals canonical naming.
//!
//! `docs/05:20-24` produces an encrypted symbol table mapping canonical
//! semantic IDs to internal code identifiers and wire tokens; the key broker
//! need not reveal raw keys. `docs/05:52-62` warns that compiled binaries do
//! not automatically conceal names, so private names must not be compiled as
//! plaintext constants. No store is opened and no symbol is decrypted here.

use crate::verdict::Verdict;

/// A private registry whose canonical names never enter a public binary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Private {
    /// Digest of the encrypted symbol table.
    pub table_digest: String,
    /// Key epoch under which the table is sealed.
    pub epoch: u64,
    /// Number of concealed canonical names, supplied by the trusted compiler.
    pub concealed: usize,
}

impl Private {
    /// Record a sealed registry view from trusted fields.
    #[must_use]
    pub fn of(table_digest: &str, epoch: u64, concealed: usize) -> Self {
        Self {
            table_digest: table_digest.to_owned(),
            epoch,
            concealed,
        }
    }

    /// Whether the private view is complete enough for trusted resolution.
    #[must_use]
    pub fn sealed(&self) -> Verdict {
        Verdict::digest(&self.table_digest)
            .and(Verdict::of(self.epoch > 0))
            .and(Verdict::of(self.concealed > 0))
    }

    /// Canonical names are loaded from the encrypted store at runtime, never
    /// compiled into a worker-inspectable binary (`docs/05:58-61`).
    #[must_use]
    pub const fn in_binary() -> Verdict {
        Verdict::Deny
    }
}
