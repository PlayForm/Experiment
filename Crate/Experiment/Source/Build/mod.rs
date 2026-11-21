//! Builder: reproducible, immutable compile and hash/publish.
//!
//! Grounded in `ARCHITECTURE.md:52-53,127-129`, `REQUIREMENTS_V3.md` INV-006,
//! `playbooks/fanout-5.yaml:61-65` and `CONTAINER_MATRIX.md:17`. Non-LLM.

#[path = "Builder.rs"]
pub mod builder;
#[path = "Recipe.rs"]
pub mod recipe;
#[path = "Artifact.rs"]
pub mod artifact;
#[path = "Digest.rs"]
pub mod digest;
