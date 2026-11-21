//! Generated `Cargo.lock` capture and reuse.
//!
//! Grounded in `README.md:148-151` (a missing lock is generated in Docker,
//! retained under `.prun-bench/locks/`, and reused for identical manifests)
//! and `prun_lab.py::prepare_lock` (271-306). The key is the first 20 hex
//! digits of `sha256(Cargo.toml)`; the stored filename is `<key>.lock`.

use super::Digest;

/// How the staged lock was obtained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Origin {
    Generated,
    Reused,
    Preexisting,
}

/// A retained lock artifact for one manifest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lock {
    /// Key derived from the manifest digest.
    pub key: String,
    /// Filename under the lock store.
    pub filename: String,
    /// Digest of the lock bytes.
    pub digest: Digest,
    /// Acquisition path.
    pub origin: Origin,
}

impl Lock {
    /// Decide the acquisition path from what is staged and what is stored
    /// (prun_lab.py:281-305), using typed `Option` presence rather than flags.
    #[must_use]
    pub fn decide(
        manifest: &Digest,
        result: &Digest,
        staged: Option<&Digest>,
        stored: Option<&Digest>,
    ) -> Self {
        let origin = match (staged, stored) {
            (Some(_), _) => Origin::Preexisting,
            (None, Some(_)) => Origin::Reused,
            (None, None) => Origin::Generated,
        };
        let key = manifest.prefix(20).to_owned();
        let filename = format!("{key}.lock");
        Self {
            key,
            filename,
            digest: result.clone(),
            origin,
        }
    }

    /// Path under the lock store (prun_lab.py:280).
    #[must_use]
    pub fn store_path(&self, store: &str) -> String {
        format!("{}/{}", store.trim_end_matches('/'), self.filename)
    }

    /// The warning line printed when a lock must be generated
    /// (prun_lab.py:285-286).
    #[must_use]
    pub fn notice(&self) -> Option<&'static str> {
        match self.origin {
            Origin::Generated => {
                Some("Generating and retaining missing Cargo.lock for reproducible follow-up runs")
            }
            Origin::Reused | Origin::Preexisting => None,
        }
    }
}
