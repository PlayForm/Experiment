//! Queue: exclusive hardware lease, paired workloads, signed manifests.
//!
//! Grounded in `ARCHITECTURE.md:56-57,82-86`, `REQUIREMENTS_V3.md` EXP-001..007
//! and `playbooks/fanout-5.yaml:69-83`. Construction is parallel; measurement
//! is exclusive on a given benchmark host.

#[path = "BenchmarkQueue.rs"]
pub mod queue;
#[path = "Exclusive.rs"]
pub mod exclusive;
#[path = "Lease.rs"]
pub mod lease;
#[path = "Workload.rs"]
pub mod workload;
#[path = "Release.rs"]
pub mod release;
#[path = "Diagnostic.rs"]
pub mod diagnostic;
