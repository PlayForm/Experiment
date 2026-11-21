//! The twelve unresolved research questions as typed values.
//!
//! Grounded in `REQUIREMENTS.md:1001-1032` (section 20). Each entry is a
//! question to be answered by evidence, not a claim; the wording is kept close
//! to the source and shortened only where the source lists several examples.
//! The physical file keeps the card's `ResearchQuestion.rs` name; the Rust
//! type is one word by house style.

/// One open research question.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Question {
    pub number: u8,
    pub text: &'static str,
}

impl Question {
    /// All twelve questions in document order.
    pub const ALL: [Self; 12] = [
        Self { number: 1, text: "What does upstream guarantee about command order and stdout/stderr interleaving? characterize with source and tests before changing scheduler." },
        Self { number: 2, text: "Does Run execute command strings through a shell, tokenization, or direct Command argv? no rewriting until characterized." },
        Self { number: 3, text: "What does upstream guarantee for errors, symlinks, exclusions, filters and concurrent directory changes? the oracle needs precise contracts." },
        Self { number: 4, text: "How much of observed elapsed time is child execution versus Run overhead on representative real workloads? without this there is no justified planner budget." },
        Self { number: 5, text: "Does the local target platform expose reliable CPU/NUMA and perf counters, or only container-virtualized views? log capabilities and uncertainty." },
        Self { number: 6, text: "Is a high-throughput filesystem benchmark dominated by page cache, overlayfs, disk seek, per-directory syscalls or application logic? separate cohorts." },
        Self { number: 7, text: "Can one build recipe support per-machine specialization without locking the installer to the builder's CPU? prefer explicit target specification and runtime dispatch." },
        Self { number: 8, text: "Do small policy models outperform a few transparent thresholds after accounting for inference cost and generalization? verify empirically." },
        Self { number: 9, text: "Which benchmark result summaries must be human-readable versus machine-queryable? both are required; choose a canonical machine schema." },
        Self { number: 10, text: "What is the minimum performance gain worth complexity, maintainability and support risk? set campaign-specific promotion gates." },
        Self { number: 11, text: "How should cross-platform support be maintained when the initial measurement lab is Linux Docker? preserve contract and separate test infrastructure." },
        Self { number: 12, text: "When does an optimization affecting output ordering constitute a new explicit mode versus a transparent improvement? require contract-specific opt-in." },
    ];

    /// Recover a question by number.
    #[must_use]
    pub fn find(number: u8) -> Option<Self> {
        Self::ALL.into_iter().find(|question| question.number == number)
    }

    /// Render one line for a research register.
    #[must_use]
    pub fn line(&self) -> String {
        format!("RQ-{:02}: {}", self.number, self.text)
    }
}
