//! Inference-token resource dimensions.
//!
//! Grounded in `MARKET.md:9` (input and output tokens as separately booked
//! native units), `schemas/research-bid.v5.schema.json:63-71`, and
//! `playbooks/research-market-week.yaml:25-34` (per-role initial token limits).

/// One inference-token direction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Token {
    /// Prompt and context tokens consumed (`MARKET.md:9`).
    Input,
    /// Generated tokens produced (`MARKET.md:9`).
    Output,
}

impl Token {
    /// Both directions.
    pub const ALL: [Self; 2] = [Self::Input, Self::Output];

    /// Native-unit key used by the schemas and ledger.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Input => "input_tokens",
            Self::Output => "output_tokens",
        }
    }
}
