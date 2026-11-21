//! The staged, untrusted output bundle of a worker.
//! Grounded in `22-hidden-worker-and-correspondence.md:23-28` (submitted files
//! are `change.patch`, `claims.json`, `proposed-tests/` and an optional
//! `paper-draft.md`; model command text is never coordinator control input).

/// The fixed list of files a worker is allowed to submit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Artifact {
    /// Submitted file names, relative to the worker's scratch root.
    pub files: Vec<String>,
}

impl Artifact {
    /// The canonical submission vocabulary.
    pub const ACCEPTED: [&'static str; 4] = [
        "change.patch",
        "claims.json",
        "proposed-tests/",
        "paper-draft.md",
    ];

    /// Empty artifact, before a worker submits anything.
    #[must_use]
    pub fn empty() -> Self {
        Self { files: Vec::new() }
    }

    /// Whether every submitted file is in the accepted vocabulary.
    #[must_use]
    pub fn is_admissible(&self) -> bool {
        self.files
            .iter()
            .all(|name| Self::ACCEPTED.contains(&name.as_str()))
    }

    /// Whether a worker-supplied test is present (it never becomes trusted).
    #[must_use]
    pub fn has_proposed_tests(&self) -> bool {
        self.files.iter().any(|name| name == "proposed-tests/")
    }
}
