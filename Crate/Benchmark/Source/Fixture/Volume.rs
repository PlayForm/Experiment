//! The deterministic Docker named volume that holds generated fixtures.
//!
//! Grounded in `README.md:17-18` (named volume, not host-shared files) and
//! `prun_lab.py::ensure_fixtures` (464-483): the name is
//! `prunlab-fixtures-` plus the first 16 hex digits of the digest of the
//! sorted fixture specs. The digest value is supplied as data by the caller;
//! computing it is an executor concern, not this model's.

/// A named-volume specification, never created by this crate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Volume {
    /// Stable name prefix.
    pub prefix: &'static str,
    /// Truncated digest of the fixture specification.
    pub fingerprint: String,
}

impl Volume {
    /// Name prefix from prun_lab.py:466.
    pub const PREFIX: &'static str = "prunlab-fixtures-";

    /// Compose the deterministic volume name (prun_lab.py:466).
    #[must_use]
    pub fn name(&self) -> String {
        format!("{}{}", self.prefix, self.fingerprint)
    }

    /// The `--mount` argument used in every runner invocation
    /// (prun_lab.py:473-475, :573).
    #[must_use]
    pub fn mount(&self) -> String {
        format!("type=volume,src={},dst=/data", self.name())
    }
}
