//! Builder and runner image identities.
//!
//! Grounded in `prun_lab.py:335-338` (`docker image inspect`), `:341-367`
//! (two runner images tagged `prunlab-runner:<target>`) and `:380`
//! (builder tag `prunlab-builder:` plus a 20-hex fingerprint).

use super::Digest;

/// A tagged image with an optional resolved image ID.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Image {
    /// Docker tag.
    pub tag: String,
    /// Resolved image ID, when inspected.
    pub id: Option<Digest>,
}

impl Image {
    /// A release or diagnostic runner image (prun_lab.py:348).
    #[must_use]
    pub fn runner(group: &str) -> Self {
        Self {
            tag: format!("prunlab-runner:{group}"),
            id: None,
        }
    }

    /// A builder image keyed by variant fingerprint (prun_lab.py:380).
    #[must_use]
    pub fn builder(fingerprint: &Digest) -> Self {
        Self {
            tag: format!("prunlab-builder:{}", fingerprint.prefix(20)),
            id: None,
        }
    }

    /// `docker image inspect` argv used to capture the ID (prun_lab.py:335-338).
    #[must_use]
    pub fn inspect_argv(&self) -> Vec<String> {
        vec![
            "docker".to_owned(),
            "image".to_owned(),
            "inspect".to_owned(),
            "--format".to_owned(),
            "{{.Id}}".to_owned(),
            self.tag.clone(),
        ]
    }

    /// Attach the inspected ID.
    #[must_use]
    pub fn with_id(mut self, digest: Digest) -> Self {
        self.id = Some(digest);
        self
    }
}
