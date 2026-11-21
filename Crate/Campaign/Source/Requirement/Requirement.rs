//! Numbered requirements paired with their verification kind.
//!
//! Grounded in `REQUIREMENTS.md`: the 17 non-negotiable laws (`:43-103`) and
//! every REQ-* family (`:182-192`, `:216-242`, `:321-334`, `:359-374`,
//! `:405-424`, `:438-451`, `:488-506`, `:580-603`, `:657-671`, `:705-716`,
 `:736-750`, `:822-830`, `:885-892`, `:915-927`, `:974-999`, `:1099-1108`).
//! Laws are individualized; REQ families are recorded by code range with their
//! count and verification kind so no numbered family is silently dropped.
//!
//! Exception declared: `Verification` is declared beside `Requirement` because
//! it is the verification vocabulary of this one type and has no independent
//! lifecycle.

use crate::requirement::{Category, ID, Section};

/// How a requirement is verified.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Verification {
    /// Executable characterization of current behaviour.
    Characterization,
    /// Automated test or unit test.
    Test,
    /// Paired measurement with declared thresholds.
    Measurement,
    /// Semantic equivalence oracle.
    Oracle,
    /// Explicit human approval.
    Human,
    /// Fault-injection and recovery scenario.
    FaultInjection,
    /// Statistical analysis of raw samples.
    Analysis,
    /// One-command reproduction from lineage.
    Reproduction,
}

impl Verification {
    /// Token for status output.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Characterization => "characterization",
            Self::Test => "test",
            Self::Measurement => "measurement",
            Self::Oracle => "oracle",
            Self::Human => "human",
            Self::FaultInjection => "fault_injection",
            Self::Analysis => "analysis",
            Self::Reproduction => "reproduction",
        }
    }
}

/// One numbered requirement or requirement family.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Requirement {
    pub id: ID,
    pub section: Section,
    pub category: Category,
    pub text: &'static str,
    pub verification: Verification,
}

impl Requirement {
    /// (code, section, category, summary, verification) tuples.
    const CATALOG: [(&'static str, Section, Category, &'static str, Verification); 33] = [
        ("LAW-001", Section::Law, Category::Invariant, "Semantic fidelity: preserve exact externally observable behavior under the documented contract", Verification::Characterization),
        ("LAW-002", Section::Law, Category::Invariant, "Total-cost accounting: measure full process end-to-end elapsed time, not a single stage", Verification::Measurement),
        ("LAW-003", Section::Law, Category::Invariant, "Zero hidden execution: no probe of the user command, no file changes, no network without explicit opt-in", Verification::Test),
        ("LAW-004", Section::Law, Category::Invariant, "Controlled measurement: fixtures, fingerprint, baseline, raw samples, statistics and oracle all required", Verification::Oracle),
        ("LAW-005", Section::Law, Category::Invariant, "Independent measurement: candidates sharing physical benchmark resources must not compete during timing", Verification::Measurement),
        ("LAW-006", Section::Law, Category::Invariant, "Append-only provenance: never overwrite raw results, mutation records, manifests or source hashes", Verification::Reproduction),
        ("LAW-007", Section::Law, Category::Invariant, "Single-change causal studies: one parent and one conceptual variable per causal lane", Verification::Analysis),
        ("LAW-008", Section::Law, Category::Invariant, "Safe fallback: unsupported architectures, low confidence and errors use the verified reference strategy", Verification::Test),
        ("LAW-009", Section::Law, Category::Invariant, "Local-first, offline-resumable: cached toolchains and images suffice; every external model is optional", Verification::Test),
        ("LAW-010", Section::Law, Category::Invariant, "Resource governance: explicit duration, attempts, CPU, memory, disk, network and process limits", Verification::FaultInjection),
        ("LAW-011", Section::Law, Category::Invariant, "No benchmark overfitting: training/development/holdout split and a final sealed validation set", Verification::Analysis),
        ("LAW-012", Section::Law, Category::Invariant, "Explicit portability: host, execution-environment and compiler-target capabilities are separate fields", Verification::Characterization),
        ("LAW-013", Section::Law, Category::Invariant, "No unexplained magic: opt-in explain-plan records version, inputs, confidence, policy, benefit and fallback", Verification::Test),
        ("LAW-014", Section::Law, Category::Invariant, "Security by architecture: worktree-only edits, no Docker socket, no host mounts, no direct promotion", Verification::Test),
        ("LAW-015", Section::Law, Category::Invariant, "Negative evidence matters: regressions, crashes, flaky tests and failed combinations are first-class", Verification::Analysis),
        ("LAW-016", Section::Law, Category::Invariant, "Energy and reliability are objectives: CPU-seconds, memory, I/O, process counts, failure rates and optional power", Verification::Measurement),
        ("LAW-017", Section::Law, Category::Invariant, "Configs are versioned APIs: schema bumps, migrations, backward-compatible reading and explicit unsupported errors", Verification::Test),
        ("REQ-ARC-01..06", Section::Arc, Category::Capability, "Module contracts, ReferenceStrategy availability, per-stage interfaces, instrumentation split, size/cold-start tracking and equivalence tiers", Verification::Characterization),
        ("REQ-HEU-01..16", Section::Heuristic, Category::Capability, "Analysis and ExecutionPlan shapes, classification labels, no speculation, bounded sampling, safe fallback, explain-plan, TTL cache, redaction and overhead accounting", Verification::Test),
        ("REQ-MUT-01..08", Section::Mutation, Category::Constraint, "Hypothesis, smallest diff, compiler/source distinction, multi-parent baselines, failed-branch retention, committed patches, interaction tests and declarative incompatibilities", Verification::Analysis),
        ("REQ-ID-01..10", Section::Identity, Category::Constraint, "Immutable logical entities, versioned IDs and foreign keys, patch provenance, artifact digests, environment capture, immutable results, migrations, deterministic IDs, unknown inequality and redaction", Verification::Reproduction),
        ("REQ-WLD-01..13", Section::Workload, Category::Constraint, "Disposable sandboxes, exact oracle expectations, retained seeds and hashes, immutable splits, declared weights, one-driver scaling, null experiments, separate filesystem cohorts and equivalent fixtures", Verification::Oracle),
        ("REQ-COR-01..08", Section::Oracle, Category::Constraint, "Characterized reference, independent instrumentation, pluggable differential/metamorphic/property/fuzz testing, preflight and extended oracles, no promotion on failure and platform variants", Verification::Oracle),
        ("REQ-BEN-01..13", Section::Bench, Category::Objective, "Wall-clock primary metric with planning time, counters, repeats, predeclared victory thresholds, geometric mean, no debug/release mixing, inconclusive labels, anchors and cohort labels", Verification::Measurement),
        ("REQ-AGT-01..18", Section::Agent, Category::Constraint, "Durable state, idempotency, leases, isolated worktrees, no main-branch edits, retry taxonomy, event-first persistence, one conceptual patch, independent baseline and central promotion", Verification::FaultInjection),
        ("REQ-DOC-01..10", Section::Docker, Category::Constraint, "OCI digests, fixed cgroup and mounts, no shared writable fixtures, explicit privileged diagnostics, contention detection, offline classes, content-addressed artifacts and matched mount visibility", Verification::Characterization),
        ("REQ-HW-01..09", Section::Hardware, Category::Capability, "ISA rejection and fallback, tagged specialization, per-class winners, Run-versus-child attribution, multiversion cost, local opt-in, PGO on training and explicit target detection", Verification::Measurement),
        ("REQ-LIN-01..10", Section::Lineage, Category::Constraint, "Single causal parent, source and binary hashes, all attempted settings, origin metadata, linked decisions, one-command reconstruction, retained diffs, DAG views, explicit cohorts and immutable raw samples", Verification::Reproduction),
        ("REQ-CLI-01..06", Section::Interface, Category::Capability, "Stable machine-readable output, reports with failed experiments, shared coordinator target, durable service beyond chat, dry-run projection and designed/implemented/verified status", Verification::Test),
        ("REQ-FEA-01..06", Section::Feature, Category::Constraint, "Exclusivity outside Cargo, per-combo oracle, versioned IDs, exact ablation records, optimized-away path testing and rejection of unknown features", Verification::Test),
        ("REQ-SEC-01..10", Section::Security, Category::Constraint, "Three operation profiles, deny-by-default, workload allowlist, disposable fixtures, redacted logs, artifact integrity, manual release, fail-closed sandbox, recorded approvals and emergency stop", Verification::FaultInjection),
        ("REQ-DEC-01..06", Section::Decomposition, Category::Capability, "Mutation-to-release mapping, inline/eliminated function records, diagnostic toolchain capture, compatibility labeling of deletions and separate code-size attribution", Verification::Analysis),
        ("REQ-ACC-01..12", Section::Gate, Category::Objective, "Twelve automated scenarios: reproducibility, skip-invocation rejection, inconclusive noise, crash recovery, SMT contamination, ancestry, unknown planner fallback, ISA rejection, analyzer cost, overfitting, cross-architecture and budget pause", Verification::FaultInjection),
    ];

    /// Build the full typed catalog.
    #[must_use]
    pub fn all() -> Vec<Self> {
        Self::CATALOG
            .into_iter()
            .filter_map(|(code, section, category, text, verification)| {
                ID::parse(code)
                    .map(|id| Self { id, section, category, text, verification })
                    .ok()
            })
            .collect()
    }

    /// Find a requirement by code.
    #[must_use]
    pub fn find(code: &str) -> Option<Self> {
        Self::all().into_iter().find(|requirement| requirement.id.as_str() == code)
    }
}
