//! Durable work-order lease, heartbeat and reclamation.
//!
//! Grounded in `STORAGE.md:3,6` (lease/heartbeat, idempotent ACK, outbox),
//! `REQUIREMENTS_V3.md` AGT3-008/EXP-011 and `RUNBOOK.md:20-22,52-58`
//! (stale lease reclamation, reconstructable status after crash). This is a
//! *task* lease; the exclusive *hardware measurement* lease lives in the
//! `queue` domain.

/// Lifecycle of a work-order lease at an observed logical sequence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    /// The lease is valid and the worker is active.
    Active,
    /// The lease deadline passed; work may be reclaimed.
    Expired,
    /// The lease was reclaimed and the work order is reconstructable.
    Reclaimed,
}

/// A task lease owned by one worker at a time.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lease {
    /// The work order this lease governs.
    pub work_order: String,
    /// The worker that currently owns the lease.
    pub owner: String,
    /// Logical sequence at which the lease expires.
    pub deadline_seq: u64,
    /// Number of heartbeats received.
    pub heartbeats: u64,
    /// Attempts already made, bounded by quota.
    pub attempts: u32,
}

impl Lease {
    /// Project the lease state at an observed sequence.
    #[must_use]
    pub const fn state(&self, observed_seq: u64) -> State {
        match observed_seq <= self.deadline_seq {
            true => State::Active,
            false => State::Expired,
        }
    }

    /// Renew the lease by a heartbeat span, without panicking on overflow.
    #[must_use]
    pub fn renew(mut self, observed_seq: u64, span: u64) -> Self {
        self.deadline_seq = observed_seq.saturating_add(span);
        self.heartbeats = self.heartbeats.saturating_add(1);
        self
    }

    /// Reclaim an expired lease for a new worker, preserving the attempt count
    /// so a crashed worker cannot multiply experiments (`AGT3-011`).
    #[must_use]
    pub fn reclaim(mut self, new_owner: String, observed_seq: u64, span: u64) -> Self {
        self.owner = new_owner;
        self.attempts = self.attempts.saturating_add(1);
        self.renew(observed_seq, span)
    }
}
