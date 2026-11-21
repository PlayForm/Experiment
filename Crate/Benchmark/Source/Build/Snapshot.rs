//! A clean, committed git snapshot of one ref.
//!
//! Grounded in `README.md:34-37` (only committed snapshots are benchmarked)
//! and `prun_lab.py:244-268`: resolve `git rev-parse --verify <ref>^{commit}`,
//! then `git archive --format=tar <sha>`, rejecting paths outside the
//! destination and symlinks that escape it.

use crate::config::Ref;
use crate::Fault;

/// A resolved commit ready to be archived into a staging directory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Snapshot {
    /// The selector from the config.
    pub reference: Ref,
    /// The resolved commit SHA.
    pub commit: String,
}

impl Snapshot {
    /// Bind a resolved commit to its selector (prun_lab.py:244-247).
    #[must_use]
    pub fn resolve(reference: &Ref, commit: impl Into<String>) -> Self {
        Self {
            reference: reference.clone(),
            commit: commit.into(),
        }
    }

    /// `git rev-parse` argv used to resolve the ref (prun_lab.py:244-247).
    #[must_use]
    pub fn verify_argv(reference: &Ref, repo: &str) -> Vec<String> {
        vec![
            "git".to_owned(),
            "-C".to_owned(),
            repo.to_owned(),
            "rev-parse".to_owned(),
            "--verify".to_owned(),
            format!("{}^{{commit}}", reference.git_ref),
        ]
    }

    /// `git archive` argv used to stage the snapshot (prun_lab.py:252-256).
    #[must_use]
    pub fn archive_argv(&self) -> Vec<String> {
        vec![
            "git".to_owned(),
            "archive".to_owned(),
            "--format=tar".to_owned(),
            self.commit.clone(),
        ]
    }

    /// Reject an archive member that would escape the staging directory
    /// (prun_lab.py:259-261).
    pub fn safe_path(name: &str) -> Result<&str, Fault> {
        let escaping = name.starts_with('/') || name.split('/').any(|part| part == "..");
        match escaping.then(|| Fault::Malformed {
            domain: "archive",
            field: "path".to_owned(),
            detail: name.to_owned(),
        }) {
            Some(reason) => Err(reason),
            None => Ok(name),
        }
    }

    /// Keep only ordinary symlinks; reject absolute or traversing targets
    /// (prun_lab.py:262-267).
    pub fn safe_link(link: &str) -> Result<&str, Fault> {
        let escaping = link.starts_with('/') || link.split('/').any(|part| part == "..");
        match escaping.then(|| Fault::Malformed {
            domain: "archive",
            field: "symlink".to_owned(),
            detail: link.to_owned(),
        }) {
            Some(reason) => Err(reason),
            None => Ok(link),
        }
    }
}
