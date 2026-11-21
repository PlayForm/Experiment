//! Offline-root custody and the restricted relay-only view.
//!
//! Grounded in `security/README.md:31-34`, `security/src/storage.rs:124-162`
//! (init creates `offline/` at 0700 and keeps the root keys there),
//! `storage.rs:248-294` (`stage_runtime` copies only a relay-only bundle),
//! `crypto.rs:104`, `ssh_ca.rs:24`, and `deploy/compose.dev.yaml:1-2` (the
//! offline directory must never be mounted into a relay or agent container).
//!
//! This facade declares leaves only; it holds no behaviour.

#[path = "Root.rs"]
pub mod root;
#[path = "Offline.rs"]
pub mod offline;
#[path = "Issuer.rs"]
pub mod issuer;
#[path = "State.rs"]
pub mod state;
#[path = "Stage.rs"]
pub mod stage;
