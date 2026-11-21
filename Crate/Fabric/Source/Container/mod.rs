//! Container provisioning, boundary isolation and additive internal APIs.
//!
//! Mirrors `docs/08:1-56`, `docs/10:39-51`, `docs/09:10` and `README.md:19-21,34-36`.
//! The LLM worker receives no key, policy registry, internal API or egress;
//! generated Rust build and test code is untrusted and isolated. No container
//! is launched and no build script is executed here.

#[path = "Provision.rs"]
pub mod provision;
#[path = "Boundary.rs"]
pub mod boundary;
#[path = "Egress.rs"]
pub mod egress;
#[path = "Api.rs"]
pub mod api;
#[path = "Compatibility.rs"]
pub mod compatibility;
#[path = "Build.rs"]
pub mod build;
