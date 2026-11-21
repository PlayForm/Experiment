//! Multi-resource, non-transferable research credit.
//!
//! Grounded in `MARKET.md:1-19` (seven native resource dimensions, fixed-point
//! accounting, no single exchange rate, optional shadow price only),
//! `:70-84` (immutable ledger, atomic reservation, metered consumption, no
//! transfer or minting) and `:117-123` (epoch expiry and return to treasury),
//! `sql/v5_migration.sql:110-169,267-278`, and
//! `rust/market-prototype/src/main.rs:16-59` (upper-bound `fits` logic read as
//! a specification). Nothing is reserved, consumed, settled or refunded here.

#[path = "Resource.rs"]
pub mod resource;
#[path = "Token.rs"]
pub mod token;
#[path = "Compute.rs"]
pub mod compute;
#[path = "Storage.rs"]
pub mod storage;
#[path = "Lease.rs"]
pub mod lease;
#[path = "LeaseState.rs"]
pub mod lease_state;
#[path = "Budget.rs"]
pub mod budget;
#[path = "Ledger.rs"]
pub mod ledger;
#[path = "Transaction.rs"]
pub mod transaction;

pub use budget::Budget;
pub use compute::Compute;
pub use lease::Lease;
pub use lease_state::LeaseState;
pub use ledger::Ledger;
pub use resource::Resource;
pub use storage::Storage;
pub use token::Token;
pub use transaction::Transaction;
