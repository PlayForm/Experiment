//! Untrusted staged proposals from LLM workers. Grounded in
//! `AGENT_CONTRACT.md:25-45` (one conceptual change, structured final
//! artifact, never self-promote) and `17-v61-constitution.md:19-34` (models
//! produce only staged proposals; generated code is never executed directly).

#[path = "Worker.rs"]
pub mod worker;
#[path = "Scope.rs"]
pub mod scope;
#[path = "Artifact.rs"]
pub mod artifact;
#[path = "Patch.rs"]
pub mod patch;
#[path = "Staging.rs"]
pub mod staging;
