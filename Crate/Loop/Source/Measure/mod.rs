//! Trace-off authoritative measurement: exclusive queue, disjoint CPU leases,
//! paired sequential comparisons. Coupled to `benchmark::runner` and the
//! `runner` SUT CLI surface. Grounded in `ARCHITECTURE.md:56-57,82-86`,
//! `AGENT_CONTRACT.md:47-61` and `00-system-constitution.md:35-36`.

#[path = "Queue.rs"]
pub mod queue;
#[path = "Lease.rs"]
pub mod lease;
#[path = "Cpuset.rs"]
pub mod cpuset;
#[path = "Runner.rs"]
pub mod runner;
#[path = "Sample.rs"]
pub mod sample;
#[path = "Schema.rs"]
pub mod schema;
