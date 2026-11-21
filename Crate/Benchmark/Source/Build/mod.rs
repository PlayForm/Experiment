//! Git snapshots, build recipes, images, lock capture and artifact records.
//!
//! Grounded in `README.md:34-37,148-151` and `prun_lab.py:177-180,244-461`.
//! Commands are argv data and digests are typed values; nothing is spawned or
//! written. `Cargo.lock` is never tracked in a repository - the lab captures
//! generated locks as artifacts under `.prun-bench/locks/`.

#[path = "Digest.rs"]
pub mod digest;
#[path = "Snapshot.rs"]
pub mod snapshot;
#[path = "Recipe.rs"]
pub mod recipe;
#[path = "Image.rs"]
pub mod image;
#[path = "Lock.rs"]
pub mod lock;
#[path = "Artifact.rs"]
pub mod artifact;

pub use artifact::Artifact;
pub use digest::Digest;
pub use image::Image;
pub use lock::Lock;
pub use recipe::Recipe;
pub use snapshot::Snapshot;
