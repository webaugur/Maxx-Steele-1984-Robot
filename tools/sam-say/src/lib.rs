//! SAM speech, robot boops, and mouth helpers.
//!
//! # Features
//! - **default / `playback`**: rodio playback, [`AudioOutput`], mouth TTY animation
//! - disable default features for synth-only (no audio device)

#![forbid(unsafe_code)]

pub mod boop;
pub mod mouth;
pub mod voice;
pub mod wav;

#[cfg(feature = "playback")]
pub mod audio;
#[cfg(feature = "playback")]
pub mod play;

pub use boop::{
    find_boop, random_boop, seed_boop_rng, select_boop_for_statement, split_statements,
    synthesize_boop, synthesize_boop_for_statement, synthesize_random_boop, trailing_punctuation,
    BoopPattern, BOOP_SAMPLE_RATE, PATTERNS as BOOP_PATTERNS,
};
pub use mouth::{ascii_mouth_frame, ascii_mouth_line, mouth_open};
pub use voice::{
    all_voices, duration_secs, find_voice, format_voice_list, synthesize, synthesize_voice,
    synthesize_with, SamError, SamPreset, SamVoice, SAM_SAMPLE_RATE,
};
pub use wav::write_wav;

#[cfg(feature = "playback")]
pub use audio::AudioOutput;
#[cfg(feature = "playback")]
pub use play::{play_samples, play_samples_while, play_samples_with_mouth};
