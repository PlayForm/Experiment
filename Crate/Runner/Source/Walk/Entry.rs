//! A typed candidate path produced by the walk.
//!
//! Grounded in `Fn/Binary/Command/Entry.rs:45-52`: `DirEntry::path()`,
//! `DirEntry::file_type().is_file()` and `DirEntry::into_path()`. The
//! execution stages then compare `file_name()` to the pattern
//! (`Parallel.rs:46`, `Sequential.rs:30`) and take `parent()`.

use std::path::Path;

/// One non-excluded walk result, with only the flags the SUT inspects.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entry {
    path: String,
    is_file: bool,
    is_symlink: bool,
}

impl Entry {
    /// Build an entry from a path and its observed file-type flags.
    #[must_use]
    pub const fn build(path: String, is_file: bool, is_symlink: bool) -> Self {
        Self {
            path,
            is_file,
            is_symlink,
        }
    }

    /// The path handed to `GlobSet::is_match` and later to `parent()`.
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    /// `DirEntry::file_type().is_file()` (`Entry.rs:50`).
    #[must_use]
    pub const fn is_file(&self) -> bool {
        self.is_file
    }

    /// Whether this entry is a link; always false while `follow_links(false)`.
    #[must_use]
    pub const fn is_symlink(&self) -> bool {
        self.is_symlink
    }

    /// `Path::file_name()` as compared to `Option.Pattern`.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        Path::new(&self.path)
            .file_name()
            .and_then(|component| component.to_str())
    }

    /// `Path::parent()`: the directory a command actually runs in.
    #[must_use]
    pub fn parent(&self) -> Option<String> {
        Path::new(&self.path)
            .parent()
            .map(|parent| parent.to_string_lossy().into_owned())
    }
}
