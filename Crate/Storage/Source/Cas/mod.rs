//! Content-addressed artifact facade.
//!
//! `cas://sha256/<digest>` identifies bytes, not a grant; a recipient, purpose,
//! storage class and cryptographic commitment are separate data
//! (`docs/08-data-and-storage.md:26-38`). No hash is computed and no byte
//! stream is opened in this crate.

#[path = "Digest.rs"]
pub mod digest;
#[path = "StorageClass.rs"]
pub mod storage_class;
#[path = "Representation.rs"]
pub mod representation;
#[path = "Blob.rs"]
pub mod blob;
#[path = "Artifact.rs"]
pub mod artifact;
#[path = "Ref.rs"]
pub mod r#ref;
#[path = "Grant.rs"]
pub mod grant;
