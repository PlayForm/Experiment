//! Typed campaign configuration: the v2 example/schema field coverage.
//!
//! Grounded in `examples/campaign.v2.json` (all 242 lines), the 724-line
//! `schemas/campaign.v2.schema.json`, and `REQUIREMENTS.md` sections 3, 6, 7,
//! 10, 12, 13, 17 and 22. The root document carries an explicit schema
//! version and an explicit, non-reinterpreting migration from the v1 plan;
//! every field that is only a proposed contract is marked by `Overlay`.

#[path = "Campaign.rs"]
pub mod root;
#[path = "Charter.rs"]
pub mod charter;
#[path = "Source.rs"]
pub mod source;
#[path = "Objective.rs"]
pub mod objective;
#[path = "Container.rs"]
pub mod container;
#[path = "Hardware.rs"]
pub mod hardware;
#[path = "Build.rs"]
pub mod build;
#[path = "Feature.rs"]
pub mod feature;
#[path = "Fixture.rs"]
pub mod fixture;
#[path = "Workload.rs"]
pub mod workload;
#[path = "Bench.rs"]
pub mod bench;
#[path = "Agent.rs"]
pub mod agent;
#[path = "Safety.rs"]
pub mod safety;
#[path = "Storage.rs"]
pub mod storage;
#[path = "Dimension.rs"]
pub mod dimension;
#[path = "Overlay.rs"]
pub mod overlay;
#[path = "Plan.rs"]
pub mod plan;

pub use agent::Agent;
pub use bench::Bench;
pub use build::Build;
pub use charter::Charter;
pub use container::Container;
pub use dimension::Dimension;
pub use feature::Feature;
pub use fixture::Fixture;
pub use hardware::Hardware;
pub use objective::Objective;
pub use overlay::Overlay;
pub use plan::Plan;
pub use root::Campaign;
pub use safety::Safety;
pub use source::Source;
pub use storage::Storage;
pub use workload::Workload;
