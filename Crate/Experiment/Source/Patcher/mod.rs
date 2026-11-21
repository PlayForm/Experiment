//! Patcher: five disjoint scoped mutation workers.
//!
//! Grounded in `ARCHITECTURE.md:41-42,64-96`, `AGENT_CONTRACTS.md:10`,
//! `REQUIREMENTS_V3.md` AGT3-004..009 and `playbooks/fanout-5.yaml:22-47`.
//! No rival patch, no whole-project history, no holdout outcome.

#[path = "Worker.rs"]
pub mod worker;
#[path = "Scope.rs"]
pub mod scope;
#[path = "Worktree.rs"]
pub mod worktree;
#[path = "Summary.rs"]
pub mod summary;
#[path = "Feedback.rs"]
pub mod feedback;
