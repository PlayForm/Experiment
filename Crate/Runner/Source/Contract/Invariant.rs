//! Named invariants, each tied to the SUT file that establishes it.
//!
//! Every rule is a typed value with a `source` citation so a benchmark run
//! cannot silently drift from the behaviour it claims to characterize.

use crate::contract::behavior::Behavior;

/// A rule that a faithful re-implementation must preserve.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Invariant {
    behavior: Behavior,
    rule: &'static str,
    source: &'static str,
}

impl Invariant {
    /// The twelve grounded rules, in contract order.
    pub const ALL: [Invariant; 12] = [
        Invariant {
            behavior: Behavior::Exclusion,
            rule: "follow_links(false): links are never traversed",
            source: "Fn/Binary/Command/Entry.rs:46",
        },
        Invariant {
            behavior: Behavior::Exclusion,
            rule: "default exclusion **/{node_modules,.git,target,dist,vendor}/**/*",
            source: "README.md:118; Fn/Binary/Command.rs:54",
        },
        Invariant {
            behavior: Behavior::Exclusion,
            rule: "filter_entry rejects a glob match before any other work",
            source: "Fn/Binary/Command/Entry.rs:48",
        },
        Invariant {
            behavior: Behavior::FileMode,
            rule: "-F keeps only file_type().is_file() entries",
            source: "Fn/Binary/Command/Entry.rs:50",
        },
        Invariant {
            behavior: Behavior::MatchSet,
            rule: "target is parent() of an entry whose file_name == Pattern",
            source: "Fn/Binary/Command/Parallel.rs:46-47; Sequential.rs:30-31",
        },
        Invariant {
            behavior: Behavior::OutputOrdering,
            rule: "sequential targets and commands keep input order",
            source: "Fn/Binary/Command/Sequential.rs:38,47",
        },
        Invariant {
            behavior: Behavior::OutputOrdering,
            rule: "parallel classifies with par_iter and drains a pre-sized ArrayQueue",
            source: "Fn/Binary/Command/Parallel.rs:30-40,59-73",
        },
        Invariant {
            behavior: Behavior::OutputOrdering,
            rule: "one Tokio worker per rayon::current_num_threads()",
            source: "Fn/Binary/Command/Parallel.rs:64-72",
        },
        Invariant {
            behavior: Behavior::OutputOrdering,
            rule: "AllDone is emitted last, including when no targets exist",
            source: "Parallel.rs:54-57,145; Sequential.rs:134",
        },
        Invariant {
            behavior: Behavior::ErrorPropagation,
            rule: "spawn via sh -c; non-zero status becomes an error with stderr",
            source: "Fn/Binary/Command/Parallel/Process.rs:27-64",
        },
        Invariant {
            behavior: Behavior::ErrorPropagation,
            rule: "index lock: 30s timeout, 50->500ms backoff, 30s stale removal",
            source: "Fn/Binary/Command/Index/Lock.rs:5-15,44-69",
        },
        Invariant {
            behavior: Behavior::ErrorPropagation,
            rule: "a global GPG mutex serializes signing-related git commands",
            source: "Parallel.rs:19,90-95; Parallel/GPG.rs:20-24; GPG/Git.rs:23-32",
        },
    ];

    /// Attach an axis, rule text and citation.
    #[must_use]
    pub const fn build(behavior: Behavior, rule: &'static str, source: &'static str) -> Self {
        Self {
            behavior,
            rule,
            source,
        }
    }

    /// The axis this rule constrains.
    #[must_use]
    pub const fn behavior(&self) -> Behavior {
        self.behavior
    }

    /// The rule text.
    #[must_use]
    pub const fn rule(&self) -> &'static str {
        self.rule
    }

    /// The SUT file and lines that establish the rule.
    #[must_use]
    pub const fn source(&self) -> &'static str {
        self.source
    }

    /// Rules for one axis, with their citations intact.
    pub fn for_behavior(behavior: Behavior) -> impl Iterator<Item = Self> {
        Self::ALL.into_iter().filter(move |invariant| invariant.behavior == behavior)
    }
}
