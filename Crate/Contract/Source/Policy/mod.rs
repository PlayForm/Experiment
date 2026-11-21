//! Versioned policy data facade.
//!
//! Declares the authorization matrix, role, classification, rule, state-machine
//! and telemetry-contract leaves from `policy/access-matrix.json`,
//! `policy/v61-roles.json`, `policy/record-classification.yaml`,
//! `policy/state-machines.yaml`, `policy/v61-state-machines.yaml` and
//! `policy/telemetry.json`. Policy is pinned and additive; nothing here
//! authorizes a live request or executes a Cedar decision.

#[path = "Role.rs"]
pub mod role;
#[path = "Domain.rs"]
pub mod domain;
#[path = "Service.rs"]
pub mod service;
#[path = "Rule.rs"]
pub mod rule;
#[path = "AccessMatrix.rs"]
pub mod access_matrix;
#[path = "Classification.rs"]
pub mod classification;
#[path = "Transition.rs"]
pub mod transition;
#[path = "StateMachine.rs"]
pub mod state_machine;
#[path = "TelemetryContract.rs"]
pub mod telemetry_contract;

pub use access_matrix::AccessMatrix;
pub use classification::Classification;
pub use domain::Domain;
pub use role::Role;
pub use rule::Rule;
pub use service::Service;
pub use state_machine::StateMachine;
pub use telemetry_contract::TelemetryContract;
pub use transition::Transition;
