//! Competitive allocation of scarce research capacity.
//!
//! Grounded in `MARKET.md:22-47` (signed bids with upper bounds and frozen
//! forecasts, two-step assignment under global/per-project/per-agent/per-model/
//! per-topic caps), `:48-68` (calibrated expected value, Brier score, no
//! self-assessed novelty), `:108-123` (a deterministic priority allocator,
//! explicitly not called a truthful auction, with epoch expiry),
//! `schemas/research-bid.v5.schema.json:6-148`, `examples/research-bid.v5.json:2-28`,
//! `examples/market-scenario.v5.json:2-66`, and the uncompiled
//! `rust/market-prototype/src/main.rs:60-159` read as a specification. No bid is
//! ranked by an LLM and no resource is actually reserved by this crate.

#[path = "Bid.rs"]
pub mod bid;
#[path = "Allocation.rs"]
pub mod allocation;
#[path = "Blind.rs"]
pub mod blind;
#[path = "Score.rs"]
pub mod score;
#[path = "Round.rs"]
pub mod round;
#[path = "Reason.rs"]
pub mod reason;

pub use allocation::Allocation;
pub use bid::Bid;
pub use blind::Blind;
pub use reason::Reason;
pub use round::Round;
pub use score::Score;
