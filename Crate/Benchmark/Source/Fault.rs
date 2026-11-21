//! Typed, non-panicking failures for the benchmark model.
//!
//! Grounded in the validation performed by `prun_lab.py` (`load_config`,
//! `matrix`, `bench`, `report`) and in this card's graceful-degradation rule:
//! a bad input yields a typed value the caller may print, never a panic.

use std::fmt;

/// A fatal, typed failure. The model returns this to its caller; it never
/// panics and never aborts a surrounding process.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Fault {
    /// A path could not be read (prun_lab.py:190, :522).
    Io { path: String, detail: String },
    /// JSON was malformed or did not fit the typed model (:190, :606).
    Json { path: String, detail: String },
    /// A schema version was outside the supported one (:191, :608).
    Version { found: String, expected: String },
    /// A required field was absent (schema.json `required`, :194).
    Field { domain: &'static str, field: String },
    /// An identifier was duplicated within one axis (:196).
    Duplicate { domain: &'static str, id: String },
    /// A workload referenced an undefined fixture (:201).
    Reference { from: String, to: String },
    /// A value failed a domain rule: profile, env key, memory, runs (:203).
    Malformed {
        domain: &'static str,
        field: String,
        detail: String,
    },
    /// A Cartesian expansion yielded no variants (:239).
    Empty,
    /// Runner CPU sets were missing or overlapping (:542, :549).
    Affinity { detail: String },
    /// The built variant matrix no longer matches the config (:526).
    Stale,
    /// A preflight count differed from the fixture contract (:134).
    Count { found: usize, expected: usize },
}

impl Fault {
    /// Render one human-readable diagnostic line.
    #[must_use]
    pub fn line(&self) -> String {
        match self {
            Self::Io { path, detail } => format!("fault: cannot read {path}: {detail}"),
            Self::Json { path, detail } => format!("fault: malformed JSON in {path}: {detail}"),
            Self::Version { found, expected } => {
                format!("fault: version {found} unsupported, expected {expected}")
            }
            Self::Field { domain, field } => {
                format!("fault: {domain} is missing required field {field}")
            }
            Self::Duplicate { domain, id } => {
                format!("fault: duplicate identifier {id} in {domain}")
            }
            Self::Reference { from, to } => {
                format!("fault: {from} references undefined {to}")
            }
            Self::Malformed {
                domain,
                field,
                detail,
            } => format!("fault: {domain}.{field} is invalid: {detail}"),
            Self::Empty => "fault: all variants were excluded".to_owned(),
            Self::Affinity { detail } => format!("fault: runner affinity rejected: {detail}"),
            Self::Stale => "fault: matrix changed since build; rebuild first".to_owned(),
            Self::Count { found, expected } => {
                format!("fault: preflight touched {found} directories, expected {expected}")
            }
        }
    }
}

impl fmt::Display for Fault {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.line())
    }
}

impl std::error::Error for Fault {}
