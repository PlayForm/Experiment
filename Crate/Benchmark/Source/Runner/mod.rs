//! Runner groups, CPU affinity leases and benchmark invocation data.
//!
//! Grounded in `README.md:13-14,64-82,119-143` and `prun_lab.py:341-367,
//! 486-598`. Comparisons inside a group stay sequential; release and
//! diagnostic groups may overlap only on disjoint `runner_cpusets`.

#[path = "Group.rs"]
pub mod group;
#[path = "Lease.rs"]
pub mod lease;
#[path = "CpuSet.rs"]
pub mod cpu_set;
#[path = "Invocation.rs"]
pub mod invocation;
#[path = "Hyperfine.rs"]
pub mod hyperfine;

pub use cpu_set::CpuSet;
pub use group::Group;
pub use hyperfine::Hyperfine;
pub use invocation::Invocation;
pub use lease::Lease;
