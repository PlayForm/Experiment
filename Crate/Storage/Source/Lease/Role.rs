//! The three kinds of exclusive lease holder.

/// Which trusted workload holds a lease.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Role {
    /// Exclusive benchmark host.
    Benchmark,
    /// Deterministic build worker.
    Builder,
    /// Trusted local supervisor of an offline worker.
    Supervisor,
}

impl Role {
    /// The declared role name.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Benchmark => "benchmark",
            Self::Builder => "builder",
            Self::Supervisor => "supervisor",
        }
    }
}
