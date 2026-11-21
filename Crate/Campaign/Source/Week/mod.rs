//! The bounded 168-hour autonomous-week contract, modelled as typed phases.
//!
//! Grounded in the 102-line `playbooks/autonomous-week.yaml` (all sections:
//! resource policy, ten phases, recovery, reporting), `campaign.v2.json:10-44`
//! and `REQUIREMENTS.md:556-625`. This is a future-facing coordinator
//! playbook; no scheduler, lease, timer or campaign loop is implemented here.

#[path = "Week.rs"]
pub mod root;
#[path = "Phase.rs"]
pub mod phase;
#[path = "Kind.rs"]
pub mod kind;
#[path = "Role.rs"]
pub mod role;
#[path = "Approval.rs"]
pub mod approval;
#[path = "Budget.rs"]
pub mod budget;
#[path = "Cap.rs"]
pub mod cap;
#[path = "Playbook.rs"]
pub mod playbook;
#[path = "Coordinator.rs"]
pub mod coordinator;
#[path = "Recovery.rs"]
pub mod recovery;
#[path = "Reporting.rs"]
pub mod reporting;

pub use approval::Approval;
pub use budget::Budget;
pub use cap::Cap;
pub use coordinator::Coordinator;
pub use kind::Kind;
pub use phase::Phase;
pub use playbook::Playbook;
pub use recovery::Recovery;
pub use reporting::Reporting;
pub use role::Role;
pub use root::Week;
