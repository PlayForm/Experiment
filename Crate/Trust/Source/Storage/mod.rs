//! File-permission and secret-handling contract.
//!
//! Grounded in `security/src/storage.rs:42-64` (`secret` uses 0600, create_new,
//! write and sync_all; `public` uses create_new and sync_all; `mark_once` is a
//! secret write), `storage.rs:83-99,320-344` (hash-chained signed journals),
//! `storage.rs:124-135,248-294` (0700 directories, staged view exclusions) and
//! `security/README.md:28-34`. This facade declares leaves only.

#[path = "State.rs"]
pub mod state;
#[path = "Permission.rs"]
pub mod permission;
#[path = "Secret.rs"]
pub mod secret;
