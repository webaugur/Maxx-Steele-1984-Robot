//! Re-export robot boops from shared [`sam_say`] crate.

pub use sam_say::{
    find_boop, random_boop, seed_boop_rng, select_boop_for_statement, split_statements,
    synthesize_boop, synthesize_boop_for_statement, synthesize_random_boop, trailing_punctuation,
    BoopPattern, BOOP_PATTERNS, BOOP_SAMPLE_RATE,
};

/// Alias for historical `PATTERNS` name in this repo.
pub use sam_say::BOOP_PATTERNS as PATTERNS;
