//! Two-timescale feedback: fast private capsules, slow sanctioned knowledge.
//! Grounded in `17-v61-constitution.md:45-57` (feedback is split in time),
//! `AGENTS.md` feedback-bias control and `PUBLISHING.md:105-113` (cross-repo
//! transfer carries only an abstract, hypothesis-shaped card).

#[path = "Capsule.rs"]
pub mod capsule;
#[path = "Redaction.rs"]
pub mod redaction;
#[path = "Visibility.rs"]
pub mod visibility;
#[path = "TwoTimescale.rs"]
pub mod timescale;
