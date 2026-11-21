//! Hash-chained journal state.
//!
//! Grounded in `security/src/storage.rs:20-34` (`AuditBody` carries `seq`,
//! `previous_sha256`, `when_unix`, `event`, `subject`, `reason`,
//! `evidence_sha256`), `storage.rs:83-99` (read verifies sequence and previous
//! hash, then chains to the record digest), and `storage.rs:299-344`
//! (`RelayEventBody` with an eight-field body and the same chaining rule). This
//! type validates chain shape; it reads no journal and verifies no signature.

use crate::warning::Warning;

/// A typed descriptor of the prototype's signed hash-chained journals.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    /// Issuance journal path (`storage.rs:84`).
    pub issuance_journal: &'static str,
    /// Relay observation journal path (`storage.rs:321`).
    pub relay_journal: &'static str,
    /// First sequence number (`storage.rs:89,329`: index + 1).
    pub first_sequence: u64,
    /// Number of fields in an issuance body (`storage.rs:21-29`).
    pub issuance_fields: usize,
    /// Number of fields in a relay event body (`storage.rs:300-309`).
    pub relay_fields: usize,
}

impl State {
    /// Construct the prototype's journal descriptor.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            issuance_journal: "audit/issuance.jsonl",
            relay_journal: "audit/relay-events.jsonl",
            first_sequence: 1,
            issuance_fields: 7,
            relay_fields: 8,
        }
    }

    /// The all-zero previous hash at the chain head (`storage.rs:85,327`).
    #[must_use]
    pub fn genesis() -> String {
        "0".repeat(64)
    }

    /// Check that a record's sequence continues the chain without a gap
    /// (`storage.rs:90,330`).
    pub fn check_sequence(expected: u64, found: u64) -> Result<(), Warning> {
        match expected == found {
            true => Ok(()),
            false => Err(Warning::Drift {
                domain: "journal-sequence",
                expected: expected as u32,
                found: found as u32,
            }),
        }
    }

    /// Check that a record points at the preceding digest (`storage.rs:90,330`).
    pub fn check_link(previous: &str, expected: &str) -> Result<(), Warning> {
        match previous == expected {
            true => Ok(()),
            false => Err(Warning::Malformed {
                domain: "journal-previous-hash",
                detail: previous.to_owned(),
            }),
        }
    }
}
