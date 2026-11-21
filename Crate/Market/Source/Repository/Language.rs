//! Language and toolchain family of an adapter.
//!
//! Grounded in `REPOSITORIES.md:3-11` (the same control plane serves many
//! languages) and `:33-40` (a public Python project may support only CLI
//! latency and correctness; a Rust CLI may support CPU flags, `perf`, LTO and
//! PGO; a GPU project needs accelerator builders). The schema carries the
//! concrete toolchain name at `repository-adapter.v5.schema.json:61`.

/// A source-language family with its capability ceiling.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Language {
    /// Rust toolchain family (`REPOSITORIES.md:39`).
    Rust,
    /// Python toolchain family (`REPOSITORIES.md:39`).
    Python,
    /// C toolchain family.
    C,
    /// C++ toolchain family.
    CPP,
    /// Go toolchain family.
    Go,
    /// JavaScript toolchain family.
    JavaScript,
    /// TypeScript toolchain family.
    TypeScript,
    /// An unclassified toolchain with no assumed capabilities.
    Other,
}

impl Language {
    /// All supported families.
    pub const ALL: [Self; 8] = [
        Self::Rust,
        Self::Python,
        Self::C,
        Self::CPP,
        Self::Go,
        Self::JavaScript,
        Self::TypeScript,
        Self::Other,
    ];

    /// Whether compiler-flag sweeps are assumed for this family
    /// (`REPOSITORIES.md:39`).
    #[must_use]
    pub const fn supports_compiler_flags(self) -> bool {
        matches!(self, Self::Rust | Self::C | Self::CPP)
    }

    /// Whether only CLI latency and correctness may be claimed
    /// (`REPOSITORIES.md:39`).
    #[must_use]
    pub const fn is_cli_only(self) -> bool {
        matches!(self, Self::Python | Self::JavaScript | Self::TypeScript)
    }

    /// No language implies universal metrics (`REPOSITORIES.md:40`).
    #[must_use]
    pub const fn has_universal_metrics() -> crate::Permission {
        crate::Permission::Deny
    }
}
