//! Deterministic control state facade.
//!
//! Declares the five required leaves and the supporting `Phase` vocabulary.
//! The transition function is deterministic given pinned policy and ordered
//! events; wall-clock measurements are observations, never control
//! (`docs/17-v61-constitution.md:65-79`).

#[path = "Machine.rs"]
pub mod machine;
#[path = "Transition.rs"]
pub mod transition;
#[path = "Event.rs"]
pub mod event;
#[path = "Policy.rs"]
pub mod policy;
#[path = "Epoch.rs"]
pub mod epoch;
#[path = "Phase/mod.rs"]
pub mod phase;

pub use epoch::Epoch;
pub use event::Event;
pub use machine::Machine;
pub use policy::Policy;
pub use transition::Transition;
