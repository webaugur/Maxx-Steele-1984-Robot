//! Re-export SAM synthesis from shared [`sam_say`] crate.

pub use sam_say::{
    all_voices, duration_secs, find_voice, format_voice_list, synthesize, synthesize_voice,
    synthesize_with, SamError, SamPreset, SamVoice, SAM_SAMPLE_RATE,
};
