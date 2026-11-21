//! Typed model of the additive v3 SQL migration.
//!
//! Grounded in `sql/v3_migration.sql:1-229` (eleven new tables, five indexes,
//! six immutability triggers on attestations, agent turns and feedback) and
//! `STORAGE.md:39-49` (the full identifier chain). This crate emits no SQL:
//! the table names and immutable sets are typed data a future migration runner
//! can consume. The migration supplements v2 and never redefines its rows.

/// Whether a table may be updated or deleted after commit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mutability {
    /// Append-only; triggers abort any update or delete.
    Immutable,
    /// A projection row that may change with state.
    Mutable,
}

/// A typed v3 migration descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Migration {
    /// Schema version this migration extends.
    pub from_schema: u32,
    /// Schema version this migration establishes.
    pub to_schema: u32,
    /// Tables created by the migration, in declaration order.
    pub tables: Vec<&'static str>,
    /// Tables protected by deny-update and deny-delete triggers.
    pub immutable: Vec<&'static str>,
    /// The identifier chain recorded in `STORAGE.md:41`.
    pub id_chain: Vec<&'static str>,
}

impl Migration {
    /// The eleven tables created by `v3_migration.sql`.
    pub const TABLES: [&'static str; 11] = [
        "generations",
        "work_orders",
        "agent_sessions",
        "context_snapshots",
        "agent_turns",
        "proposals",
        "novelty_verdicts",
        "feedback_capsules",
        "attestations",
        "telemetry_references",
        "dispatch_outbox",
    ];

    /// The three tables with immutability triggers (`:192-229`).
    pub const IMMUTABLE: [&'static str; 3] =
        ["agent_turns", "feedback_capsules", "attestations"];

    /// The identifier chain from `STORAGE.md:41`.
    pub const CHAIN: [&'static str; 19] = [
        "campaign_id",
        "generation_id",
        "work_order_id",
        "agent_session_id",
        "context_snapshot_id",
        "proposal_id",
        "mutation_id",
        "source_snapshot_sha",
        "build_key",
        "binary_sha",
        "experiment_id",
        "trial_id",
        "analysis_id",
        "attestation_id",
        "decision_id",
        "feedback_capsule_id",
        "next_mutation_id",
        "causation_event_id",
        "correlation_id",
    ];

    /// Construct the migration descriptor.
    #[must_use]
    pub fn v3() -> Self {
        Self {
            from_schema: 2,
            to_schema: 3,
            tables: Self::TABLES.to_vec(),
            immutable: Self::IMMUTABLE.to_vec(),
            id_chain: Self::CHAIN.to_vec(),
        }
    }

    /// Whether a named table is protected from update and delete.
    #[must_use]
    pub fn mutability(&self, table: &str) -> Mutability {
        match self.immutable.iter().any(|held| *held == table) {
            true => Mutability::Immutable,
            false => Mutability::Mutable,
        }
    }

    /// The migration is additive: it never rewrites v2 rows.
    #[must_use]
    pub const fn is_additive() -> bool {
        true
    }
}
