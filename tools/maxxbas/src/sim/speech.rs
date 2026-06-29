//! SAM phrase playback for interactive sim.
//!
//! Firmware routes speech three ways (all must be hooked):
//! - `$F44B` — cart execute `SPEAK` (`$82`) / `SAY` (`$83`), operand in `$13`
//! - `$F40F` — game / status `JSR` with phrase index in X (ROM if `X >= $10`, else RAM `$0500`)
//! - `$F475` / `$F47E` — power-on / mode entry ROM phrases (X = `$10`–`$20`)

use rodio::buffer::SamplesBuffer;
use rodio::Sink;

use super::audio::AudioOutput;
use super::speech_sam::{self, SamError, SAM_SAMPLE_RATE};

/// Match interactive sim frame rate (`455_000 / 60` cycles per frame at 60 Hz).
pub(crate) const SPEECH_CYCLES_PER_SEC: u64 = 455_000;
const MIN_SPEECH_BUSY_CYCLES: u64 = 20_000;

/// Cart `SPEAK` / `SAY` entry — operand in `$13`, opcode in `$11`.
pub const ROM_CART_SPEAK: u16 = 0xF44B;
/// `JSR $F40F` — phrase index in X (ROM table if `X >= $10`, else RAM `$0500` slot).
pub const ROM_SAY_F40F: u16 = 0xF40F;
/// `RTS` after the `$F40F` phoneme loop sets `$5B`.
pub const ROM_SPEECH_RTS: u16 = 0xF43B;
pub const ROM_SAY_PHRASE: u16 = 0xF475;
pub const ROM_WAIT_SPEECH: u16 = 0xF47E;
/// `RTS` at end of `$F475` speech subroutine (after `$5B` wait loop).
pub const ROM_SPEECH_DONE: u16 = 0xF482;

/// RAM phrase slots `0`–`15` (`$83` operand `<= $0F`, table at `$0500` / cart `$A081`).
pub fn ram_phrase_label(slot: u8) -> Option<&'static str> {
    match slot {
        0x00 => Some("I am great, and you."),
        0x01 => Some("I am ready when you are."),
        0x02 => Some("I am a great match for humans."),
        0x03 => Some("Goodbye for now, have a good day."),
        _ => None,
    }
}

/// Built-in ROM phrase indices `$10`–`$20` (User Manual phrases 16–32).
pub fn rom_phrase_label(id: u8) -> Option<&'static str> {
    match id {
        0x10 => Some("Hello. I am Maxx Steele."),
        0x11 => Some("Please choose how tough."),
        0x12 => Some("Please choose game."),
        0x13 => Some("Good play."),
        0x14 => Some("Thank you."),
        0x15 => Some("Is there anything I can do for you?"),
        0x16 => Some("Good morning."),
        0x17 => Some("It is time to get up."),
        0x18 => Some("Maxx Steele wins."),
        0x19 => Some("Congratulations, you win."),
        0x1A => Some("I need energy, please recharge me."),
        0x1B => Some("Game over."),
        0x1C => Some("Choose enter to play again."),
        0x1D => Some("Sorry, my circuits are full."),
        0x1E => Some("Please teach me."),
        0x1F => Some("Please program me."),
        0x20 => Some("I'm ready."),
        _ => None,
    }
}

/// `SPEAK` (`$82`) — single phoneme index clocked via `$F3D5` (not the `$F5F3` phrase table).
pub fn speak_phoneme_label(id: u8) -> Option<&'static str> {
    match id {
        0x3F => Some("Ha ha ha ha ha."),
        _ => None,
    }
}

/// Resolve index the way `$F40F` does: low slots are RAM, `$10+` are ROM.
pub fn phrase_for_index(index: u8) -> Option<&'static str> {
    if index <= 0x0F {
        ram_phrase_label(index)
    } else {
        rom_phrase_label(index)
    }
}

/// Cart execute: opcode selects RAM/ROM (`$83`) vs phoneme (`$82`) lookup.
pub fn phrase_for_cart_opcode(opcode: u8, operand: u8) -> Option<&'static str> {
    match opcode {
        0x82 => speak_phoneme_label(operand),
        0x83 => phrase_for_index(operand),
        _ => None,
    }
}

/// Bubble / status display — same tables as playback.
pub fn phrase_label(index: u8) -> Option<&'static str> {
    phrase_for_index(index).or_else(|| speak_phoneme_label(index))
}

/// Wall-clock length of a SAM phrase (for speech-bubble timing).
pub fn phrase_duration_secs(phrase: u8) -> f64 {
    let Some(text) = phrase_label(phrase) else {
        return 0.8;
    };
    match speech_sam::synthesize(text) {
        Ok(samples) => speech_sam::duration_secs(samples.len()),
        Err(_) => 0.8,
    }
}

fn phrase_busy_cycles(sample_count: usize) -> u64 {
    let secs = speech_sam::duration_secs(sample_count);
    (secs * SPEECH_CYCLES_PER_SEC as f64)
        .round()
        .max(MIN_SPEECH_BUSY_CYCLES as f64) as u64
}

pub struct SpeechPlayer {
    sink: Option<Sink>,
    last_phrase: Option<u8>,
    busy_until_cycles: Option<u64>,
    enabled: bool,
}

impl SpeechPlayer {
    pub fn new(enabled: bool) -> Self {
        Self {
            sink: None,
            last_phrase: None,
            busy_until_cycles: None,
            enabled,
        }
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

    pub fn speech_busy(&self, cpu_cycles: u64) -> bool {
        if self.sink.as_ref().is_some_and(|sink| !sink.empty()) {
            return true;
        }
        self.busy_until_cycles
            .is_some_and(|until| cpu_cycles < until)
    }

    /// True once audio has finished (sink drained or cycle budget elapsed).
    pub fn speech_wait_done(&self, cpu_cycles: u64) -> bool {
        !self.speech_busy(cpu_cycles)
    }

    pub fn is_playing(&self, cpu_cycles: u64) -> bool {
        !self.speech_wait_done(cpu_cycles)
    }

    pub fn clear_busy(&mut self) {
        self.busy_until_cycles = None;
    }

    pub fn stop(&mut self) {
        self.busy_until_cycles = None;
        if let Some(sink) = self.sink.take() {
            sink.stop();
        }
    }

    pub fn play_text(
        &mut self,
        audio: &mut AudioOutput,
        index: u8,
        text: &str,
        cpu_cycles: u64,
    ) -> bool {
        self.last_phrase = Some(index);
        if !self.enabled {
            return false;
        }
        let samples = match speech_sam::synthesize(text) {
            Ok(s) => s,
            Err(SamError::Recite(e)) => {
                eprintln!("speech: SAM reciter ({text}): {e:?}");
                return false;
            }
            Err(SamError::Parse(e)) => {
                eprintln!("speech: SAM parser ({text}): {e:?}");
                return false;
            }
            Err(SamError::Empty) => {
                eprintln!("speech: SAM produced no audio ({text})");
                return false;
            }
        };
        let busy_cycles = phrase_busy_cycles(samples.len());
        self.stop();
        let Some(sink) = audio.open_sink() else {
            self.busy_until_cycles = Some(cpu_cycles.saturating_add(busy_cycles));
            return true;
        };
        sink.set_volume(1.0);
        let source = SamplesBuffer::new(1, SAM_SAMPLE_RATE, samples);
        sink.append(source);
        self.busy_until_cycles = Some(cpu_cycles.saturating_add(busy_cycles));
        self.sink = Some(sink);
        true
    }

    pub fn play_phrase_index(
        &mut self,
        audio: &mut AudioOutput,
        index: u8,
        cpu_cycles: u64,
    ) -> bool {
        let Some(text) = phrase_for_index(index) else {
            eprintln!("speech: unknown phrase index ${index:02X}");
            return false;
        };
        self.play_text(audio, index, text, cpu_cycles)
    }

    pub fn play_cart_phrase(
        &mut self,
        audio: &mut AudioOutput,
        opcode: u8,
        operand: u8,
        cpu_cycles: u64,
    ) -> bool {
        let Some(text) = phrase_for_cart_opcode(opcode, operand) else {
            eprintln!("speech: unknown cart phrase op=${opcode:02X} idx=${operand:02X}");
            return false;
        };
        self.play_text(audio, operand, text, cpu_cycles)
    }
}

/// `$F44B`: cartridge `SPEAK` (`$82`) / `SAY` (`$83`) — operand in `$13`.
pub fn enter_cart_speak(
    mem: &[u8; 65536],
    cpu_pc: u16,
    player: &mut SpeechPlayer,
    audio: &mut AudioOutput,
    cpu_cycles: u64,
) -> Option<u16> {
    if cpu_pc != ROM_CART_SPEAK {
        return None;
    }
    let opcode = mem[0x11];
    if opcode != 0x82 && opcode != 0x83 {
        return None;
    }
    player.play_cart_phrase(audio, opcode, mem[0x13], cpu_cycles);
    Some(ROM_SPEECH_RTS)
}

/// `$F40F`: phrase index in X — ROM if `X >= $10`, else RAM slot (game / status speech).
pub fn enter_f40f_speak(
    cpu_pc: u16,
    phrase: u8,
    player: &mut SpeechPlayer,
    audio: &mut AudioOutput,
    cpu_cycles: u64,
) -> Option<u16> {
    if cpu_pc != ROM_SAY_F40F {
        return None;
    }
    player.play_phrase_index(audio, phrase, cpu_cycles);
    Some(ROM_SPEECH_RTS)
}

/// `$F475` entry: ROM phrase index in X, then busy-wait at `$F47E`.
pub fn enter_say_phrase(
    cpu_pc: u16,
    phrase: u8,
    player: &mut SpeechPlayer,
    audio: &mut AudioOutput,
    cpu_cycles: u64,
) -> Option<u16> {
    if cpu_pc != ROM_SAY_PHRASE {
        return None;
    }
    if player.play_phrase_index(audio, phrase, cpu_cycles) {
        Some(ROM_WAIT_SPEECH)
    } else {
        Some(ROM_SPEECH_DONE)
    }
}

/// `$F47E` / `JSR $F47E`: wait until speech finishes.
pub fn spin_wait_speech(cpu_pc: u16, player: &SpeechPlayer, cpu_cycles: u64) -> Option<bool> {
    if cpu_pc != ROM_WAIT_SPEECH {
        return None;
    }
    Some(player.speech_wait_done(cpu_cycles))
}

/// Keep `$5B` set while audio is still playing so `$E504` / `$F47E` wait like hardware.
pub fn sync_speech_voice_busy(mem: &mut [u8; 65536], player: &SpeechPlayer, cpu_cycles: u64) {
    if player.speech_busy(cpu_cycles) {
        mem[0x5B] = 0x80;
    } else if mem[0x5B] != 0 {
        mem[0x5B] = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ram_and_rom_tables_differ() {
        assert_eq!(ram_phrase_label(0x00), Some("I am great, and you."));
        assert_eq!(rom_phrase_label(0x10), Some("Hello. I am Maxx Steele."));
        assert!(ram_phrase_label(0x10).is_none());
        assert!(rom_phrase_label(0x00).is_none());
    }

    #[test]
    fn phrase_for_index_routes_ram_vs_rom() {
        assert_eq!(phrase_for_index(0x03), ram_phrase_label(0x03));
        assert_eq!(phrase_for_index(0x16), rom_phrase_label(0x16));
    }

    #[test]
    fn cart_opcode_routes_say_vs_speak() {
        assert_eq!(
            phrase_for_cart_opcode(0x83, 0x00),
            ram_phrase_label(0x00)
        );
        assert_eq!(
            phrase_for_cart_opcode(0x83, 0x10),
            rom_phrase_label(0x10)
        );
        assert_eq!(
            phrase_for_cart_opcode(0x82, 0x3F),
            speak_phoneme_label(0x3F)
        );
        assert!(phrase_for_cart_opcode(0x82, 0x10).is_none());
    }

    #[test]
    fn phrase_labels_cover_cbs_demo() {
        for (op, idx) in [(0x83, 0x00), (0x83, 0x10), (0x83, 0x16), (0x82, 0x3F)] {
            assert!(
                phrase_for_cart_opcode(op, idx).is_some(),
                "missing op=${op:02X} idx=${idx:02X}"
            );
        }
    }

    #[test]
    fn maxxos_phrases_synthesize() {
        for idx in [0x10, 0x13, 0x20, 0x00, 0x16] {
            let text = phrase_for_index(idx).expect("label");
            let samples = speech_sam::synthesize(text).expect("synthesize");
            assert!(samples.len() > 1_000, "phrase ${idx:02X} too short");
        }
    }
}