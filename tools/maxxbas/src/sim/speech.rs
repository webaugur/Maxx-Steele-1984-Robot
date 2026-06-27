//! ROM phrase playback for interactive sim (`JSR $F475` / `$F47E`).

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::Duration;

use rodio::source::Source;
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};

/// Match interactive sim frame rate (`455_000 / 60` cycles per frame at 60 Hz).
const SPEECH_CYCLES_PER_SEC: u64 = 455_000;
const MIN_SPEECH_BUSY_CYCLES: u64 = 20_000;

pub const ROM_SAY_PHRASE: u16 = 0xF475;
pub const ROM_WAIT_SPEECH: u16 = 0xF47E;

/// Factory ROM phrase index → English (User Manual phrases 16–32).
pub fn phrase_label(phrase: u8) -> Option<&'static str> {
    match phrase {
        0x10 => Some("Hello. I am Maxx Steele."),
        0x12 => Some("Please choose game."),
        0x11 => Some("Please choose how tough."),
        0x13 => Some("Good play."),
        0x18 => Some("Maxx Steele wins."),
        0x19 => Some("Congratulations, you win."),
        0x1B => Some("Game over."),
        0x1C => Some("Choose enter to play again."),
        0x20 => Some("I'm ready."),
        _ => None,
    }
}

fn asset_path(phrase: u8) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("assets/speech/rom/{phrase:02X}.ogg"))
}

pub struct SpeechPlayer {
    stream: Option<OutputStream>,
    sink: Option<Sink>,
    last_phrase: Option<u8>,
    busy_until_cycles: Option<u64>,
    enabled: bool,
}

impl SpeechPlayer {
    pub fn new(enabled: bool) -> Self {
        Self {
            stream: OutputStreamBuilder::open_default_stream().ok(),
            sink: None,
            last_phrase: None,
            busy_until_cycles: None,
            enabled,
        }
    }

    pub fn audio_available(&self) -> bool {
        self.stream.is_some()
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.stop();
        }
    }

    pub fn last_phrase(&self) -> Option<u8> {
        self.last_phrase
    }

    /// ROM `$F47E` busy-wait — cycle budget only (not wall-clock audio).
    pub fn speech_busy(&self, cpu_cycles: u64) -> bool {
        self.busy_until_cycles
            .is_some_and(|until| cpu_cycles < until)
    }

    /// GUI status: cycle-modeled speech or live sink still draining.
    pub fn is_playing(&self, cpu_cycles: u64) -> bool {
        self.speech_busy(cpu_cycles)
            || self
                .sink
                .as_ref()
                .is_some_and(|sink| !sink.empty())
    }

    pub fn stop(&mut self) {
        self.busy_until_cycles = None;
        if let Some(sink) = self.sink.take() {
            sink.stop();
        }
    }

    fn phrase_busy_cycles(decoder: &Decoder<BufReader<File>>) -> u64 {
        let secs = decoder
            .total_duration()
            .unwrap_or(Duration::from_millis(800))
            .as_secs_f64();
        (secs * SPEECH_CYCLES_PER_SEC as f64)
            .round()
            .max(MIN_SPEECH_BUSY_CYCLES as f64) as u64
    }

    pub fn play_rom_phrase(&mut self, phrase: u8, cpu_cycles: u64) -> bool {
        self.last_phrase = Some(phrase);
        if !self.enabled {
            return false;
        }
        let path = asset_path(phrase);
        if !path.is_file() {
            eprintln!(
                "speech: missing asset {} ({})",
                path.display(),
                phrase_label(phrase).unwrap_or("unknown phrase")
            );
            return false;
        }
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("speech: open {}: {e}", path.display());
                return false;
            }
        };
        let decoder = match Decoder::new(BufReader::new(file)) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("speech: decode {}: {e}", path.display());
                return false;
            }
        };
        let busy_cycles = Self::phrase_busy_cycles(&decoder);
        self.stop();
        let Some(stream) = &self.stream else {
            self.busy_until_cycles = Some(cpu_cycles.saturating_add(busy_cycles));
            return false;
        };
        let sink = Sink::connect_new(stream.mixer());
        sink.set_volume(1.0);
        sink.append(decoder);
        self.busy_until_cycles = Some(cpu_cycles.saturating_add(busy_cycles));
        self.sink = Some(sink);
        true
    }
}

/// Begin phrase indexed in X at `$F475`; fall through to `$F47E` busy wait.
pub fn enter_say_phrase(
    cpu_pc: u16,
    phrase: u8,
    player: &mut SpeechPlayer,
    cpu_cycles: u64,
) -> Option<u16> {
    if cpu_pc != ROM_SAY_PHRASE {
        return None;
    }
    player.play_rom_phrase(phrase, cpu_cycles);
    Some(ROM_WAIT_SPEECH)
}

/// Hold at `$F47E` while audio plays; return `Some(done)` when RTS should run.
pub fn spin_wait_speech(cpu_pc: u16, player: &SpeechPlayer, cpu_cycles: u64) -> Option<bool> {
    if cpu_pc != ROM_WAIT_SPEECH {
        return None;
    }
    Some(!player.speech_busy(cpu_cycles))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phrase_labels_cover_maxxos() {
        assert_eq!(phrase_label(0x13), Some("Good play."));
        assert_eq!(phrase_label(0x20), Some("I'm ready."));
    }

    #[test]
    fn maxxos_assets_exist() {
        for id in [0x10, 0x13, 0x20] {
            assert!(
                asset_path(id).is_file(),
                "missing speech asset for ${id:02X}"
            );
        }
    }
}