//! Signed event taxonomy facade.
//!
//! Declares the event kind, catalog, envelope, correspondence and signature
//! leaves from `events/catalog.json`, `events/correspondence-catalog.json`,
//! `events/EVENT_INDEX.md:1-146` and `api/events.asyncapi.yaml:1-199`. Every
//! kind is trusted-software-only: models never subscribe
//! (`EVENT_INDEX.md:3-4`). Nothing here publishes, signs or verifies a message.

#[path = "Privacy.rs"]
pub mod privacy;
#[path = "Channel.rs"]
pub mod channel;
#[path = "Kind.rs"]
pub mod kind;
#[path = "Entry.rs"]
pub mod entry;
#[path = "Catalog.rs"]
pub mod catalog;
#[path = "Envelope.rs"]
pub mod envelope;
#[path = "Correspondence.rs"]
pub mod correspondence;
#[path = "Signature.rs"]
pub mod signature;

pub use catalog::Catalog;
pub use channel::Channel;
pub use correspondence::Correspondence;
pub use entry::Entry;
pub use envelope::Envelope;
pub use kind::Kind;
pub use privacy::Privacy;
pub use signature::Signature;
