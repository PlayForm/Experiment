//! API contract facade.
//!
//! Declares the operation, path, method, plane, index and document leaves
//! derived from `api/HTTP_INDEX.md:7-128` and the OpenAPI/AsyncAPI maps. Each
//! leaf is a descriptor: no HTTP route is mounted and no request is executed.

#[path = "Method.rs"]
pub mod method;
#[path = "Path.rs"]
pub mod path;
#[path = "Plane.rs"]
pub mod plane;
#[path = "Document.rs"]
pub mod document;
#[path = "Operation.rs"]
pub mod operation;
#[path = "Index.rs"]
pub mod index;

pub use document::Document;
pub use index::Index;
pub use method::Method;
pub use operation::Operation;
pub use path::Path;
pub use plane::Plane;
