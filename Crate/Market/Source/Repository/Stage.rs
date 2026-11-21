//! Cross-repository stage ontology for transportable knowledge.
//!
//! Grounded in `REPOSITORIES.md:43-53`: reusable ideas are represented in a
//! cross-repository ontology, not hard-coded Rust feature names; each
//! applicability predicate names a stage and each knowledge card records
//! negative-transfer evidence. A source speedup is never portable evidence.

use crate::Warning;

/// One semantic stage in the repository ontology (`REPOSITORIES.md:47`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Stage {
    /// Directory and tree traversal.
    Traversal,
    /// Pattern or predicate matching.
    Matching,
    /// Text or binary parsing.
    Parsing,
    /// Task scheduling.
    Scheduling,
    /// Process or thread spawning.
    Spawning,
    /// Input/output operations.
    IO,
    /// Memory allocation.
    Allocation,
    /// Data compression.
    Compression,
    /// Event dispatch.
    Dispatch,
    /// Memory access patterns.
    Memory,
    /// SIMD or vector execution.
    Vectorization,
    /// Request or work batching.
    Batching,
    /// Accelerator kernel execution.
    Kernel,
    /// Compilation and code generation.
    Compilation,
}

impl Stage {
    /// All ontology stages (`REPOSITORIES.md:47`).
    pub const ALL: [Self; 14] = [
        Self::Traversal,
        Self::Matching,
        Self::Parsing,
        Self::Scheduling,
        Self::Spawning,
        Self::IO,
        Self::Allocation,
        Self::Compression,
        Self::Dispatch,
        Self::Memory,
        Self::Vectorization,
        Self::Batching,
        Self::Kernel,
        Self::Compilation,
    ];

    /// The wire label from the ontology list.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Traversal => "traversal",
            Self::Matching => "matching",
            Self::Parsing => "parsing",
            Self::Scheduling => "scheduling",
            Self::Spawning => "spawning",
            Self::IO => "I/O",
            Self::Allocation => "allocation",
            Self::Compression => "compression",
            Self::Dispatch => "dispatch",
            Self::Memory => "memory",
            Self::Vectorization => "vectorization",
            Self::Batching => "batching",
            Self::Kernel => "GPU kernel",
            Self::Compilation => "compilation",
        }
    }

    /// Decode an ontology label, warning on an unknown one.
    pub fn from_label(label: &str) -> Result<Self, Warning> {
        match label {
            "traversal" => Ok(Self::Traversal),
            "matching" => Ok(Self::Matching),
            "parsing" => Ok(Self::Parsing),
            "scheduling" => Ok(Self::Scheduling),
            "spawning" => Ok(Self::Spawning),
            "I/O" => Ok(Self::IO),
            "allocation" => Ok(Self::Allocation),
            "compression" => Ok(Self::Compression),
            "dispatch" => Ok(Self::Dispatch),
            "memory" => Ok(Self::Memory),
            "vectorization" => Ok(Self::Vectorization),
            "batching" => Ok(Self::Batching),
            "GPU kernel" => Ok(Self::Kernel),
            "compilation" => Ok(Self::Compilation),
            other => Err(Warning::InvalidValue {
                field: "stage",
                detail: format!("unknown stage {other}"),
            }),
        }
    }

    /// A stage name alone never proves transfer (`REPOSITORIES.md:51-53`).
    #[must_use]
    pub const fn is_transfer_evidence() -> crate::Permission {
        crate::Permission::Deny
    }
}
