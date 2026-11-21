//! Signed claims bound to independently produced evidence.
//!
//! Grounded in `schemas/claim.v5.schema.json:6-107` (15 required keys, six
//! claim types, supports/contradicts edges, optional evidence and source
//! references, 64-hex context commitment and signature),
//! `examples/claim.v5.json:2-16`, `sql/v5_migration.sql:60-97` (immutable claims
//! and five link relations), `README.md:39-49` (opinions may conflict, facts
//! come from signed independent evidence) and `PUBLISHING.md:72-84` (evidence
//! quality). Nothing is signed, verified or promoted here.

#[path = "Claim.rs"]
pub mod claim;
#[path = "Signature.rs"]
pub mod signature;
#[path = "Status.rs"]
pub mod status;
#[path = "Evidence.rs"]
pub mod evidence;
#[path = "Kind.rs"]
pub mod kind;
#[path = "Relation.rs"]
pub mod relation;

pub use claim::Claim;
pub use evidence::Evidence;
pub use kind::Kind;
pub use relation::Relation;
pub use signature::Signature;
pub use status::Status;
