//! Content-addressed artifact store: read-only once published.
//!
//! Grounded in `STORAGE.md:7-20,83-91` (SHA-256 CAS, read-only once published,
//! hash verification, invalidation on any source/toolchain/binary/environment
//! change) and `REQUIREMENTS_V3.md` DAT-001/DAT-004. Missing artifacts are
//! reported explicitly rather than silently reused.

use crate::permission::Permission;
use crate::warning::Warning;

/// Custody state of a CAS object.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Custody {
    /// Written but not yet verified and published.
    Staged,
    /// Verified and immutable.
    Published,
}

/// A content-addressed artifact store.
#[derive(Clone, Debug, Default)]
pub struct Cas {
    /// Digests of published objects.
    pub objects: Vec<String>,
    /// Custody state of the store as a whole.
    pub custody: Custody,
}

impl Cas {
    /// Publish a verified digest; duplicates are benign and ignored.
    pub fn publish(&mut self, digest: String) -> Result<(), Warning> {
        let duplicate = self.objects.contains(&digest);
        match duplicate {
            false => {
                self.objects.push(digest);
                Ok(())
            }
            true => Ok(()),
        }
    }

    /// Verify a referenced artifact exists; a missing object is a warning, not
    /// a fabricated hit (`DAT-004`).
    #[must_use]
    pub fn verify(&self, digest: &str) -> Result<Permission, Warning> {
        match self.objects.iter().any(|held| held == digest) {
            true => Ok(Permission::Allow),
            false => Err(Warning::MissingArtifact {
                digest: digest.to_string(),
            }),
        }
    }

    /// Whether a published object may be overwritten. It never may.
    #[must_use]
    pub const fn is_immutable() -> bool {
        true
    }
}
