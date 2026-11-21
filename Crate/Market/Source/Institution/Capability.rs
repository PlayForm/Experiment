//! Capability descriptor of a repository adapter.
//!
//! Grounded in `REPOSITORIES.md:33-40`: a `capability_manifest` states exactly
//! which features an adapter supports; a public Python project may support only
//! CLI latency and correctness, a Rust CLI may support CPU flags, `perf`, LTO
//! and PGO, and a GPU project may need exclusive accelerator leases. No
//! universal metric is assumed.

/// One declared adapter capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Capability {
    /// Plain CLI latency and exit-status correctness (`REPOSITORIES.md:39`).
    CliLatency,
    /// Correctness fixture and oracle contract (`REPOSITORIES.md:22`).
    Correctness,
    /// Compiler-flag dimension (`REPOSITORIES.md:20`).
    CpuFlags,
    /// Hardware counter profiling (`REPOSITORIES.md:39`).
    Perf,
    /// Link-time optimization candidate (`REPOSITORIES.md:39`).
    Lto,
    /// Profile-guided optimization candidate (`REPOSITORIES.md:39`).
    Pgo,
    /// CUDA-specific builder and lease (`REPOSITORIES.md:40`).
    Cuda,
    /// ROCm-specific builder and lease (`REPOSITORIES.md:40`).
    Rocm,
    /// Exclusive accelerator lease (`REPOSITORIES.md:40`).
    AcceleratorLease,
}

impl Capability {
    /// All declared capabilities.
    pub const ALL: [Self; 9] = [
        Self::CliLatency,
        Self::Correctness,
        Self::CpuFlags,
        Self::Perf,
        Self::Lto,
        Self::Pgo,
        Self::Cuda,
        Self::Rocm,
        Self::AcceleratorLease,
    ];

    /// Whether this capability needs an exclusive hardware lease
    /// (`REPOSITORIES.md:40`, `SYSTEM.md:146-148`).
    #[must_use]
    pub const fn needs_exclusive_lease(self) -> bool {
        matches!(self, Self::Cuda | Self::Rocm | Self::AcceleratorLease)
    }

    /// Whether this capability is an optimization candidate rather than a
    /// baseline requirement (`REPOSITORIES.md:31`).
    #[must_use]
    pub const fn is_acceleration_candidate(self) -> bool {
        matches!(self, Self::Lto | Self::Pgo | Self::Cuda | Self::Rocm)
    }
}
