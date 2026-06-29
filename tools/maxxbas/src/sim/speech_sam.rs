//! SAM (Software Automatic Mouth) phrase synthesis via [`rustsam`].

use rustsam::{parser, reciter, renderer};

pub const SAM_SAMPLE_RATE: u32 = 22_050;
/// SAM "Little Robot" preset (speed, pitch, mouth, throat).
const SAM_SPEED: u8 = 92;
const SAM_PITCH: u8 = 60;
const SAM_MOUTH: u8 = 190;
const SAM_THROAT: u8 = 190;
const SAM_GAIN: f32 = 1.15;

#[derive(Debug)]
pub enum SamError {
    Recite(reciter::ReciterError),
    Parse(parser::ParseError),
    Empty,
}

pub fn synthesize(text: &str) -> Result<Vec<f32>, SamError> {
    let phoneme_str = reciter::text_to_phonemes(text).map_err(SamError::Recite)?;
    let phonemes = parser::parse_phonemes(&phoneme_str).map_err(SamError::Parse)?;
    if phonemes.is_empty() {
        return Err(SamError::Empty);
    }
    let raw = renderer::render(
        &phonemes,
        SAM_PITCH,
        SAM_MOUTH,
        SAM_THROAT,
        SAM_SPEED,
        false,
    );
    if raw.is_empty() {
        return Err(SamError::Empty);
    }
    Ok(raw
        .iter()
        .map(|&b| ((f32::from(b) - 128.0) / 128.0) * SAM_GAIN)
        .collect())
}

pub fn duration_secs(sample_count: usize) -> f64 {
    sample_count as f64 / f64::from(SAM_SAMPLE_RATE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthesize_maxxos_phrases() {
        for text in [
            "Hello. I am Maxx Steele.",
            "Good play.",
            "I'm ready.",
        ] {
            let samples = synthesize(text).unwrap_or_else(|e| panic!("{text:?}: {e:?}"));
            assert!(samples.len() > 1_000, "{text:?} too short");
        }
    }
}
