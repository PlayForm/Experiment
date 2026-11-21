//! The observed file type of one walk candidate.
//!
//! Mirrors `DirEntry::file_type()` in `Fn/Binary/Command/Entry.rs:45-52`;
//! `follow_links(false)` keeps symbolic links visible as links.

/// The file-type class of a non-excluded walk result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    /// A regular file.
    File,
    /// A symbolic link.
    Symlink,
    /// Any other entry type, including a directory.
    Other,
}
