//! The seven-dimensional native resource vector.
//!
//! Grounded in `MARKET.md:9` (input/output tokens, CPU core seconds, exclusive
//! benchmark seconds, GPU seconds, storage byte-days, outbound reference
//! requests and the scientific holdout quota), `MARKET.md:15-19` (book in
//! native units, fixed-point integers, versioned price schedule), the identical
//! seven-key ceiling in `schemas/research-case.v5.schema.json:104-146`, and
//! `rust/market-prototype/src/main.rs:14-59`. The holdout quota is a policy
//! dimension, not a purchasable balance (`MARKET.md:11-13`), so it is carried
//! by `crate::case::Holdout` rather than bought here.

use crate::{Warning, FIXED_SCALE};

/// An owner-funded native resource vector, in integer units.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Resource {
    /// Inference input tokens (`MARKET.md:9`).
    pub input_tokens: u64,
    /// Inference output tokens (`MARKET.md:9`).
    pub output_tokens: u64,
    /// CPU core seconds (`MARKET.md:9`).
    pub cpu_core_seconds: u64,
    /// Exclusive benchmark seconds (`MARKET.md:9`).
    pub benchmark_seconds: u64,
    /// GPU seconds (`MARKET.md:9`).
    pub gpu_seconds: u64,
    /// Storage byte-days (`MARKET.md:9`).
    pub storage_byte_days: u64,
    /// Outbound reference requests (`MARKET.md:9`).
    pub reference_requests: u64,
}

impl Resource {
    /// Number of native dimensions (`MARKET.md:9`, `crate::RESOURCE_DIMENSIONS`).
    pub const DIMENSIONS: usize = 7;

    /// Whether every dimension of `need` fits inside `self` (`main.rs:26-34`).
    #[must_use]
    pub const fn fits(&self, need: &Self) -> bool {
        self.input_tokens >= need.input_tokens
            && self.output_tokens >= need.output_tokens
            && self.cpu_core_seconds >= need.cpu_core_seconds
            && self.benchmark_seconds >= need.benchmark_seconds
            && self.gpu_seconds >= need.gpu_seconds
            && self.storage_byte_days >= need.storage_byte_days
            && self.reference_requests >= need.reference_requests
    }

    /// Subtract an upper bound atomically, warning instead of asserting when it
    /// does not fit. The prototype used `assert!` (`main.rs:37`); this model
    /// degrades as the card's defensive rule requires.
    pub fn checked_sub(&self, need: &Self) -> Result<Self, Warning> {
        match self.fits(need) {
            true => Ok(Self {
                input_tokens: self.input_tokens - need.input_tokens,
                output_tokens: self.output_tokens - need.output_tokens,
                cpu_core_seconds: self.cpu_core_seconds - need.cpu_core_seconds,
                benchmark_seconds: self.benchmark_seconds - need.benchmark_seconds,
                gpu_seconds: self.gpu_seconds - need.gpu_seconds,
                storage_byte_days: self.storage_byte_days - need.storage_byte_days,
                reference_requests: self.reference_requests - need.reference_requests,
            }),
            false => Err(Warning::Insufficient {
                resource: "native_resource_vector",
            }),
        }
    }

    /// Normalized scarce-resource cost in millionths (`MARKET.md:18-19`,
    /// `main.rs:47-58`). Integer arithmetic replaces binary floating point: a
    /// zero-capacity dimension with nonzero demand is priced at one whole unit.
    #[must_use]
    pub fn norm_cost_micros(&self, cap: &Self) -> u128 {
        let scale = u128::from(FIXED_SCALE);
        let values = [
            self.input_tokens,
            self.output_tokens,
            self.cpu_core_seconds,
            self.benchmark_seconds,
            self.gpu_seconds,
            self.storage_byte_days,
            self.reference_requests,
        ];
        let caps = [
            cap.input_tokens,
            cap.output_tokens,
            cap.cpu_core_seconds,
            cap.benchmark_seconds,
            cap.gpu_seconds,
            cap.storage_byte_days,
            cap.reference_requests,
        ];
        values
            .iter()
            .zip(caps.iter())
            .map(|(value, ceiling)| match (*ceiling, *value) {
                (0, 0) => 0,
                (0, _) => scale,
                (_, _) => u128::from(*value) * scale / u128::from(*ceiling),
            })
            .sum()
    }
}
