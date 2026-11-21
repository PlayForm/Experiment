//! Exclusive benchmark queue: parallel construction, serial measurement.
//!
//! File name is the card-mandated `BenchmarkQueue.rs`; the Rust type is the
//! single-word `Queue` in the `queue` module (declared house-style exception:
//! the file name is fixed by the task, the identifier is not).
//!
//! Grounded in `ARCHITECTURE.md:82-86` (multiple ready candidates queue; do not
//! run five competing measurements on shared cores/disks), `REQUIREMENTS_V3.md`
//! EXP-001 and `playbooks/fanout-5.yaml:69-83` (max_parallel_per_host 1,
//! exclusive lease, oracle_passed prerequisite).

use crate::queue::lease::Lease;
use crate::warning::Warning;

/// A serial measurement queue for one benchmark host.
#[derive(Clone, Debug)]
pub struct Queue {
    /// Host identifier whose reserved resources this queue governs.
    pub host: String,
    /// Maximum simultaneous measurements; the design fixes this at one.
    pub max_parallel: usize,
    /// Ready candidates in submission order.
    pub ready: Vec<String>,
    /// The candidate currently holding the measurement slot.
    pub active: Option<String>,
}

impl Queue {
    /// The design's exclusive measurement limit (`fanout-5.yaml:70-72`).
    pub const MAX_PARALLEL_PER_HOST: usize = 1;

    /// Enqueue an oracle-passed candidate; duplicates are rejected so retries
    /// cannot double-measure (`AGT3-011`).
    pub fn enqueue(&mut self, candidate: String) -> Result<(), Warning> {
        let duplicate = self.ready.contains(&candidate);
        match duplicate {
            true => Err(Warning::Collision {
                domain: "benchmark-queue",
                id: candidate,
            }),
            false => {
                self.ready.push(candidate);
                Ok(())
            }
        }
    }

    /// Begin the next measurement only when a valid lease is held and no
    /// candidate is active.
    pub fn begin(&mut self, lease: &Lease) -> Result<String, Warning> {
        match (self.active.is_none(), lease.is_held(), self.ready.first()) {
            (true, true, Some(candidate)) => {
                self.active = Some(candidate.clone());
                Ok(candidate.clone())
            }
            _ => Err(Warning::Unreachable {
                service: "benchmark-queue-exclusive-slot",
            }),
        }
    }

    /// Release the active slot after a signed result manifest is recorded.
    pub fn finish(&mut self) -> Result<(), Warning> {
        match self.active.take() {
            Some(_) => Ok(()),
            None => Err(Warning::Unreachable {
                service: "benchmark-queue-idle",
            }),
        }
    }
}
