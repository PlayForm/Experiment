# Rust deterministic allocation-plan prototype

This is a small executable for verifying the shape of the _planning_ decision.
It accepts JSON and emits JSON without modifying external state.

```bash
cargo test
cargo run -- ../../examples/market-scenario.v5.json
```

It is **not** a production exchange, budget ledger, cryptographically
authenticated service, or trained value-of-information estimator. It does not
verify signatures, apply tenant policy, persist reservations, prevent
distributed double spending, or call an LLM. The
`externally_estimated_information_gain` input MUST come from a policy-controlled
scorer, not from a bidding agent. Allocation is deterministic within the same
binary and JSON input. No compilation was possible in the authorship environment
because `cargo`/`rustc` were unavailable.
