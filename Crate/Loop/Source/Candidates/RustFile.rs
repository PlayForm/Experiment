//! The validated identity of a materialised candidate `.rs` file.
//! Grounded in the hard boundary that generated Rust is staged into the loop's
//! own crate and never into `Module/Run`. This value only validates and
//! describes a path; it performs no filesystem write.

/// A candidate Rust source path under the loop crate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RustFile {
    /// Path relative to the workspace root.
    pub relative_path: String,
}

impl RustFile {
    /// Whether a path is an admissible candidate destination.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.relative_path.starts_with(crate::CANDIDATE_ROOT)
            && self.relative_path.ends_with(".rs")
            && !self.relative_path.contains("..")
    }

    /// The projected write path, as data only; no file is created here.
    #[must_use]
    pub fn projected_path(&self) -> &str {
        &self.relative_path
    }

    /// Whether the path would escape into the SUT submodule.
    #[must_use]
    pub fn touches_submodule(&self) -> bool {
        self.relative_path.contains("Module/")
    }
}
