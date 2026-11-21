//! A pure model of one globset pattern.
//!
//! Grounded in `Fn/Binary/Command/Entry.rs:30,39-43`: one `Glob::new` per
//! exclusion string, collected into a `GlobSet`, then `is_match(e.path())`
//! at `:48`. The matcher supports the three constructs the SUT relies on:
//! brace alternatives `{a,b}`, whole-segment `**`, and in-segment `*`.
//! It never reads a directory and never panics on a malformed pattern.

use crate::fault::Fault;

/// A compiled exclusion glob.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Glob {
    pattern: String,
}

impl Glob {
    /// Compile a pattern. Empty input is a typed fault, not a panic.
    pub fn build(pattern: &str) -> Result<Self, Fault> {
        match pattern.is_empty() {
            true => Err(Fault::Glob {
                pattern: pattern.to_owned(),
                detail: "empty pattern".to_owned(),
            }),
            false => Ok(Self {
                pattern: pattern.to_owned(),
            }),
        }
    }

    /// The original pattern string.
    #[must_use]
    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    /// `GlobSet::is_match` for a candidate path. A path is normalized by
    /// dropping empty and `.` segments, matching `WalkDir`'s relative output.
    #[must_use]
    pub fn is_match(&self, path: &str) -> bool {
        let segments = normalize(path);
        expansions(&self.pattern)
            .into_iter()
            .any(|pattern| segments_match(&segments_of(&pattern), &segments))
    }
}

fn normalize(path: &str) -> Vec<String> {
    path.split('/')
        .filter(|segment| !segment.is_empty() && *segment != ".")
        .map(str::to_owned)
        .collect()
}

fn segments_of(pattern: &str) -> Vec<&str> {
    pattern.split('/').filter(|segment| !segment.is_empty()).collect()
}

fn first_brace(pattern: &str) -> Option<(usize, usize)> {
    let open = pattern.find('{')?;
    let close = pattern[open..].find('}')? + open;
    match close > open {
        true => Some((open, close)),
        false => None,
    }
}

fn expansions(pattern: &str) -> Vec<String> {
    match first_brace(pattern) {
        None => vec![pattern.to_owned()],
        Some((open, close)) => {
            let prefix = &pattern[..open];
            let inner = &pattern[open + 1..close];
            let suffix = &pattern[close + 1..];

            inner
                .split(',')
                .flat_map(|choice| expansions(&format!("{prefix}{choice}{suffix}")))
                .collect()
        }
    }
}

fn segments_match(pattern: &[&str], path: &[String]) -> bool {
    match pattern.first() {
        None => path.is_empty(),
        // Exception: a match guard is the structural test for the `**` arm;
        // it is not a call-site branch.
        Some(head) if *head == "**" => {
            segments_match(&pattern[1..], path)
                || (!path.is_empty() && segments_match(pattern, &path[1..]))
        }
        Some(_) if path.is_empty() => false,
        Some(head) => {
            wildcard((*head).as_bytes(), path[0].as_bytes())
                && segments_match(&pattern[1..], &path[1..])
        }
    }
}

fn wildcard(pattern: &[u8], text: &[u8]) -> bool {
    match pattern {
        [] => text.is_empty(),
        [b'*', rest @ ..] => {
            wildcard(rest, text) || (!text.is_empty() && wildcard(pattern, &text[1..]))
        }
        [byte, rest @ ..] => {
            !text.is_empty() && *byte == text[0] && wildcard(rest, &text[1..])
        }
    }
}
