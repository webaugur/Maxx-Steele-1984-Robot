//! Rodio playback (feature `playback`).

use rodio::buffer::SamplesBuffer;

use crate::audio::AudioOutput;
use crate::mouth::{ascii_mouth_line, mouth_open};
use crate::voice::SAM_SAMPLE_RATE;

/// Play mono f32 samples on the default device (blocks until finished).
pub fn play_samples(samples: &[f32]) -> Result<(), String> {
    play_samples_while(samples, |_| {})
}

/// Play samples, calling `on_tick(elapsed_secs)` about every 40ms until done.
pub fn play_samples_while(
    samples: &[f32],
    mut on_tick: impl FnMut(f64),
) -> Result<(), String> {
    if samples.is_empty() {
        return Err("no samples to play".into());
    }
    let mut audio = AudioOutput::new();
    audio.warm();
    let sink = audio
        .open_sink()
        .ok_or_else(|| "could not open audio output device".to_string())?;
    sink.set_volume(1.0);
    sink.append(SamplesBuffer::new(1, SAM_SAMPLE_RATE, samples.to_vec()));
    let start = std::time::Instant::now();
    on_tick(0.0);
    while !sink.empty() {
        std::thread::sleep(std::time::Duration::from_millis(40));
        on_tick(start.elapsed().as_secs_f64());
    }
    on_tick(start.elapsed().as_secs_f64());
    Ok(())
}

/// Play samples while animating an ASCII mouth on stderr.
pub fn play_samples_with_mouth(samples: &[f32]) -> Result<(), String> {
    use std::io::{self, IsTerminal, Write};

    let use_tty = io::stderr().is_terminal();
    if !use_tty {
        return play_samples(samples);
    }
    let mut err = io::stderr().lock();
    let result = play_samples_while(samples, |elapsed| {
        let open = mouth_open(true, elapsed);
        let _ = write!(err, "\r{} ", ascii_mouth_line(open));
        let _ = err.flush();
    });
    let _ = write!(err, "\r{} \n", ascii_mouth_line(0.0));
    let _ = err.flush();
    result
}
