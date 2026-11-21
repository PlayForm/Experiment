//! The thirty register entries from `docs/25-failure-analysis.md:3-34`.
//!
//! Each entry is a typed category with a severity and a cause; the negative
//! acceptance test named in the register is recorded as its summary. No case
//! is silently treated as a pass and no success notice is synthesized.

use super::cause::Cause;
use super::severity::Severity;

/// One failure case with its register identity and control intent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Case {
    /// Register ID such as `F01`.
    pub id: &'static str,
    /// Severity assigned in the register.
    pub severity: Severity,
    /// Typed cause of the failure.
    pub cause: Cause,
    /// Short statement of the negative acceptance control.
    pub summary: &'static str,
}

impl Case {
    /// F01 - environment or tool stdout leaks the internal host.
    pub const F01: Self = Self {
        id: "F01",
        severity: Severity::Critical,
        cause: Cause::Leak,
        summary: "offline namespace, allowlisted env and no socket route",
    };
    /// F02 - a model modifies the trusted oracle or benchmark.
    pub const F02: Self = Self {
        id: "F02",
        severity: Severity::Critical,
        cause: Cause::Oracle,
        summary: "trusted image and fixtures are not writable by a worker",
    };
    /// F03 - a build script escapes into the trusted host.
    pub const F03: Self = Self {
        id: "F03",
        severity: Severity::Critical,
        cause: Cause::Sandbox,
        summary: "build.rs, proc macros and plugins are untrusted execution",
    };
    /// F04 - embargoed source or paper is transferred without authority.
    pub const F04: Self = Self {
        id: "F04",
        severity: Severity::Critical,
        cause: Cause::Rights,
        summary: "signed preflight denies before any byte is staged",
    };
    /// F05 - a license or EULA forbids cross-repository reuse.
    pub const F05: Self = Self {
        id: "F05",
        severity: Severity::Critical,
        cause: Cause::Rights,
        summary: "SPDX, exact terms and review record; unknowns fail closed",
    };
    /// F06 - a denial status leaks that a sibling exists.
    pub const F06: Self = Self {
        id: "F06",
        severity: Severity::Critical,
        cause: Cause::Disclosure,
        summary: "fixed generic worker view with no lateral channel",
    };
    /// F07 - a worker claims its own tests prove behavior.
    pub const F07: Self = Self {
        id: "F07",
        severity: Severity::High,
        cause: Cause::Evidence,
        summary: "forged pass cannot promote a candidate; oracle is separate",
    };
    /// F08 - a false speedup from shared CPU, cache or trace overhead.
    pub const F08: Self = Self {
        id: "F08",
        severity: Severity::High,
        cause: Cause::Evidence,
        summary: "exclusive lease, paired trials and trace-off baseline",
    };
    /// F09 - two researchers share a model context and appear independent.
    pub const F09: Self = Self {
        id: "F09",
        severity: Severity::High,
        cause: Cause::Isolation,
        summary: "view manifests, lineage and overlap; independent replication",
    };
    /// F10 - conflicting policy epochs during transfer.
    pub const F10: Self = Self {
        id: "F10",
        severity: Severity::High,
        cause: Cause::Epoch,
        summary: "policy digest bound to permit and commit; stale is fenced",
    };
    /// F11 - a holder claims capacity then fills before receive.
    pub const F11: Self = Self {
        id: "F11",
        severity: Severity::High,
        cause: Cause::Capacity,
        summary: "reservation and fence; no partial committed receipt",
    };
    /// F12 - a CAS hash reveals existence of a private document.
    pub const F12: Self = Self {
        id: "F12",
        severity: Severity::High,
        cause: Cause::Disclosure,
        summary: "per-tenant index, no unauthenticated global lookup",
    };
    /// F13 - deduplication conflates distinct rights.
    pub const F13: Self = Self {
        id: "F13",
        severity: Severity::High,
        cause: Cause::Rights,
        summary: "bytes identity is distinct from custody and policy state",
    };
    /// F14 - revocation treated as wiping model memory.
    pub const F14: Self = Self {
        id: "F14",
        severity: Severity::High,
        cause: Cause::Rights,
        summary: "future keys only; no claim of perfect erasure",
    };
    /// F15 - a policy service error is treated as permission.
    pub const F15: Self = Self {
        id: "F15",
        severity: Severity::High,
        cause: Cause::Unreachable,
        summary: "fail closed on timeout, unknown enum or unavailable policy",
    };
    /// F16 - a neutral error lies about test outcomes.
    pub const F16: Self = Self {
        id: "F16",
        severity: Severity::High,
        cause: Cause::Disclosure,
        summary: "acknowledge only; real status goes to the owner",
    };
    /// F17 - a paper contains a secret or license-prohibited code.
    pub const F17: Self = Self {
        id: "F17",
        severity: Severity::High,
        cause: Cause::Rights,
        summary: "sanitizer plus evidence and human review; deny publication",
    };
    /// F18 - knowledge crossing a repo is treated as proven optimization.
    pub const F18: Self = Self {
        id: "F18",
        severity: Severity::High,
        cause: Cause::Evidence,
        summary: "new baseline, oracle and benchmark; separate attestation",
    };
    /// F19 - source paths, hardlinks or symlinks escape the mount.
    pub const F19: Self = Self {
        id: "F19",
        severity: Severity::High,
        cause: Cause::Escape,
        summary: "canonical path policy rejects traversal and symlink escape",
    };
    /// F20 - tool features read global caches or credentials.
    pub const F20: Self = Self {
        id: "F20",
        severity: Severity::High,
        cause: Cause::Leak,
        summary: "offline dependency archive, sandboxed home, env allowlist",
    };
    /// F21 - the evidence journal and OTLP disagree.
    pub const F21: Self = Self {
        id: "F21",
        severity: Severity::High,
        cause: Cause::Evidence,
        summary: "ledger canonical; telemetry droppable and diagnostic",
    };
    /// F22 - event ordering confused with wall clock.
    pub const F22: Self = Self {
        id: "F22",
        severity: Severity::High,
        cause: Cause::Causality,
        summary: "aggregate sequence, causation ID and clock uncertainty",
    };
    /// F23 - the population converges on one unhelpful family.
    pub const F23: Self = Self {
        id: "F23",
        severity: Severity::Medium,
        cause: Cause::Convergence,
        summary: "fixed assignment diversity and retained specialties",
    };
    /// F24 - covert communication in timings or artifact names.
    pub const F24: Self = Self {
        id: "F24",
        severity: Severity::Medium,
        cause: Cause::Channel,
        summary: "controlled views, fixed schema and quota monitoring",
    };
    /// F25 - resource spending seen as benchmark quality.
    pub const F25: Self = Self {
        id: "F25",
        severity: Severity::Medium,
        cause: Cause::Cost,
        summary: "build tokens and inference cost excluded from verdict",
    };
    /// F26 - license terms changed without a versioned review.
    pub const F26: Self = Self {
        id: "F26",
        severity: Severity::Medium,
        cause: Cause::Rights,
        summary: "versioned review; no retroactive grant",
    };
    /// F27 - experimental tests miss a behavioral mismatch.
    pub const F27: Self = Self {
        id: "F27",
        severity: Severity::Medium,
        cause: Cause::Evidence,
        summary: "differential, property and fuzz tests retain uncertainty",
    };
    /// F28 - an infinite recursive loop without useful gain.
    pub const F28: Self = Self {
        id: "F28",
        severity: Severity::Medium,
        cause: Cause::Recursion,
        summary: "signed campaign limits and owner cancellation",
    };
    /// F29 - a duplicate transfer executes after restart.
    pub const F29: Self = Self {
        id: "F29",
        severity: Severity::Medium,
        cause: Cause::Idempotency,
        summary: "one-use nonce, fence and reconciliation; one valid receipt",
    };
    /// F30 - an API becomes a covert worker tool.
    pub const F30: Self = Self {
        id: "F30",
        severity: Severity::Medium,
        cause: Cause::Endpoint,
        summary: "no route or credential mounted in the worker namespace",
    };

    /// All thirty cases in register order.
    #[must_use]
    pub fn all() -> [Self; 30] {
        [
            Self::F01, Self::F02, Self::F03, Self::F04, Self::F05, Self::F06, Self::F07,
            Self::F08, Self::F09, Self::F10, Self::F11, Self::F12, Self::F13, Self::F14,
            Self::F15, Self::F16, Self::F17, Self::F18, Self::F19, Self::F20, Self::F21,
            Self::F22, Self::F23, Self::F24, Self::F25, Self::F26, Self::F27, Self::F28,
            Self::F29, Self::F30,
        ]
    }

    /// Whether this case is allowed to reveal a passing outcome.
    #[must_use]
    pub fn may_report_pass(&self) -> bool {
        match self.severity {
            super::severity::Severity::Critical
            | super::severity::Severity::High
            | super::severity::Severity::Medium => false,
        }
    }
}
