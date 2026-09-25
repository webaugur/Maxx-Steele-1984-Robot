//! Mono 16-bit PCM WAV writer (no external deps).

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::voice::SAM_SAMPLE_RATE;

/// Write mono f32 samples as 16-bit PCM WAV @ [`SAM_SAMPLE_RATE`].
pub fn write_wav(path: &Path, samples: &[f32]) -> Result<(), String> {
    if samples.is_empty() {
        return Err("no samples to write".into());
    }
    let mut pcm = Vec::with_capacity(samples.len() * 2);
    for &s in samples {
        let v = (s.clamp(-1.0, 1.0) * 32767.0).round() as i16;
        pcm.extend_from_slice(&v.to_le_bytes());
    }
    let data_len = pcm.len() as u32;
    let sample_rate = SAM_SAMPLE_RATE;
    let byte_rate = sample_rate * 2; // mono 16-bit
    let mut out = File::create(path).map_err(|e| format!("{}: {e}", path.display()))?;
    out.write_all(b"RIFF").map_err(|e| e.to_string())?;
    out.write_all(&(36 + data_len).to_le_bytes())
        .map_err(|e| e.to_string())?;
    out.write_all(b"WAVEfmt ").map_err(|e| e.to_string())?;
    out.write_all(&16u32.to_le_bytes()).map_err(|e| e.to_string())?;
    out.write_all(&1u16.to_le_bytes()).map_err(|e| e.to_string())?;
    out.write_all(&1u16.to_le_bytes()).map_err(|e| e.to_string())?;
    out.write_all(&sample_rate.to_le_bytes())
        .map_err(|e| e.to_string())?;
    out.write_all(&byte_rate.to_le_bytes())
        .map_err(|e| e.to_string())?;
    out.write_all(&2u16.to_le_bytes()).map_err(|e| e.to_string())?;
    out.write_all(&16u16.to_le_bytes()).map_err(|e| e.to_string())?;
    out.write_all(b"data").map_err(|e| e.to_string())?;
    out.write_all(&data_len.to_le_bytes())
        .map_err(|e| e.to_string())?;
    out.write_all(&pcm).map_err(|e| e.to_string())?;
    Ok(())
}
