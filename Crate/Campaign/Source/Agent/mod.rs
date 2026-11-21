//! Bounded local-agent contract: constrained mutation and separate measurement.
//!
//! Modelled from `playbooks/AGENT_CONTRACT.md` (all 89 lines) and
//! `REQUIREMENTS.md` sections 11, 14 and 18. The orchestration plane is
//! trusted; agents and generated code are untrusted inputs (`REQ-AGT-18`,
//! `REQ-SEC-01`). This facade models permissions, worktrees, mutations and
//! bounds as data - no agent is created and no source is edited.

#[path = "Contract.rs"]
pub mod contract;
#[path = "Permission.rs"]
pub mod permission;
#[path = "Worktree.rs"]
pub mod worktree;
#[path = "Mutation.rs"]
pub mod mutation;
#[path = "Bound.rs"]
pub mod bound;

pub use bound::Bound;
pub use contract::Contract;
pub use mutation::Mutation;
pub use permission::Permission;
pub use worktree::Worktree;
