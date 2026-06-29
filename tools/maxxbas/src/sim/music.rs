//! Factory tune playback for `PLAY` (`$81`) in the live sim.
//!
//! ROM tunes 1–8 use embedded recordings from `Chassis/Sounds/maxx-song-*.wma`.
//! Tune 0 is synthesized from the cart music table at `$0400`.

use std::io::Cursor;
use std::sync::OnceLock;
use std::time::Duration;

use rodio::buffer::SamplesBuffer;
use rodio::{Decoder, Sink, Source};

use super::audio::AudioOutput;
use super::speech::SpeechPlayer;

use super::speech::SPEECH_CYCLES_PER_SEC;

const SAMPLE_RATE: u32 = 44_100;
const MIN_MUSIC_BUSY_CYCLES: u64 = 30_000;
const PLAY_OPCODE: u8 = 0x81;

/// `JSR $EF01` in the cart `PLAY` (`$81`) handler.
pub const PLAY_JSR_GET_TUNE: u16 = 0xE4EA;
/// `JSR $EF01` — tune index in X (`$13` operand).
pub const ROM_GET_TUNE_POINTERS: u16 = 0xEF01;

/// Duration bytes from ROM `$F15B` (whole … sixteenth).
const DURATION_CODES: [u8; 8] = [0xE0, 0xA8, 0x70, 0x54, 0x38, 0x2A, 0x1C, 0x0E];

/// Quarter-note length for synthesized tune 0.
const QUARTER_NOTE_SECS: f64 = 0.34;

static TUNE_OGG: [&[u8]; 9] = [
    &[],
    include_bytes!("../../assets/music/1.ogg"),
    include_bytes!("../../assets/music/2.ogg"),
    include_bytes!("../../assets/music/3.ogg"),
    include_bytes!("../../assets/music/4.ogg"),
    include_bytes!("../../assets/music/5.ogg"),
    include_bytes!("../../assets/music/6.ogg"),
    include_bytes!("../../assets/music/7.ogg"),
    include_bytes!("../../assets/music/8.ogg"),
];

static DECODED_ROM_TUNES: OnceLock<[Option<(Vec<f32>, u32)>; 9]> = OnceLock::new();

pub fn rom_tune_label(tune: u8) -> Option<&'static str> {
    match tune {
        1 => Some("Immediate mode"),
        2 => Some("Learn mode"),
        3 => Some("Program mode"),
        4 => Some("Execute mode"),
        5 => Some("Game mode"),
        6 => Some("Reveille"),
        7 => Some("Power down"),
        8 => Some("Taps"),
        _ => None,
    }
}

fn duration_secs(code: u8) -> f64 {
    let idx = DURATION_CODES
        .iter()
        .position(|&d| d == code)
        .unwrap_or(4);
    let scale = [4.0, 3.0, 2.0, 1.5, 1.0, 0.75, 0.5, 0.25][idx];
    QUARTER_NOTE_SECS * scale
}

fn note_frequency(code: u8) -> f32 {
    if code == 0 {
        return 0.0;
    }
    let semitone = i16::from(code).saturating_sub(0x0C);
    let hz = 440.0_f32 * 2.0_f32.powf((f32::from(semitone) - 9.0) / 12.0);
    hz.clamp(110.0, 2_000.0)
}

fn synthesize_cart_tune(mem: &[u8; 65536]) -> Vec<f32> {
    let table = &mem[0x0400..0x0500];
    let mut samples = Vec::new();
    let mut i = 0usize;
    while i + 1 < table.len() {
        let dur_code = table[i];
        let freq_code = table[i + 1];
        i += 2;
        if dur_code == 0 && freq_code == 0 {
            break;
        }
        let hz = note_frequency(freq_code);
        if hz <= 0.0 {
            continue;
        }
        let secs = duration_secs(dur_code);
        let n = (secs * f64::from(SAMPLE_RATE)).round().max(1.0) as usize;
        let phase_inc = hz / SAMPLE_RATE as f32;
        let mut phase = 0.0_f32;
        for s in 0..n {
            let v = if phase < 0.5 { 0.22 } else { -0.22 };
            let attack = (s.min(200) as f32) / 200.0;
            let release = ((n.saturating_sub(s)).min(400) as f32) / 400.0;
            samples.push(v * attack * release);
            phase += phase_inc;
            if phase >= 1.0 {
                phase -= 1.0;
            }
        }
    }
    if samples.len() < SAMPLE_RATE as usize / 10 {
        samples.extend(std::iter::repeat_n(0.0, SAMPLE_RATE as usize / 5));
    }
    samples
}

fn decode_ogg_mono(bytes: &[u8]) -> Result<(Vec<f32>, u32), String> {
    let decoder = Decoder::new(Cursor::new(bytes.to_vec())).map_err(|e| e.to_string())?;
    let channels = decoder.channels();
    let sample_rate = decoder.sample_rate();
    let raw: Vec<f32> = decoder.collect();
    if raw.is_empty() {
        return Err("no samples".into());
    }
    let samples = if channels <= 1 {
        raw
    } else {
        let ch = usize::from(channels);
        raw.chunks(ch)
            .map(|frame| frame.iter().sum::<f32>() / ch as f32)
            .collect()
    };
    Ok((samples, sample_rate))
}

fn cached_rom_tune(tune: u8) -> Option<(Vec<f32>, u32)> {
    let table = DECODED_ROM_TUNES.get_or_init(|| {
        let mut out: [Option<(Vec<f32>, u32)>; 9] = std::array::from_fn(|_| None);
        for tune in 1..=8 {
            let bytes = TUNE_OGG[tune];
            match decode_ogg_mono(bytes) {
                Ok(decoded) => out[tune] = Some(decoded),
                Err(e) => eprintln!("music: preload tune #{tune}: {e}"),
            }
        }
        out
    });
    table.get(usize::from(tune)).and_then(|entry| entry.clone())
}

fn busy_cycles_for_duration(duration: Duration) -> u64 {
    (duration.as_secs_f64() * SPEECH_CYCLES_PER_SEC as f64)
        .round()
        .max(MIN_MUSIC_BUSY_CYCLES as f64) as u64
}

fn busy_cycles_for_samples(sample_count: usize, sample_rate: u32) -> u64 {
    let secs = sample_count as f64 / f64::from(sample_rate.max(1));
    busy_cycles_for_duration(Duration::from_secs_f64(secs))
}

pub struct MusicPlayer {
    sink: Option<Sink>,
    last_tune: Option<u8>,
    busy_until_cycles: Option<u64>,
    enabled: bool,
}

impl MusicPlayer {
    pub fn new(enabled: bool) -> Self {
        Self {
            sink: None,
            last_tune: None,
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

    pub fn last_tune(&self) -> Option<u8> {
        self.last_tune
    }

    pub fn has_active_sink(&self) -> bool {
        self.sink.is_some()
    }

    pub fn music_busy(&self, cpu_cycles: u64) -> bool {
        self.busy_until_cycles
            .is_some_and(|until| cpu_cycles < until)
            || self
                .sink
                .as_ref()
                .is_some_and(|sink| !sink.empty())
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

    pub fn play_tune(
        &mut self,
        tune: u8,
        mem: &[u8; 65536],
        audio: &mut AudioOutput,
        _speech: &mut SpeechPlayer,
        cpu_cycles: u64,
    ) -> bool {
        self.last_tune = Some(tune);
        if !self.enabled {
            self.busy_until_cycles = Some(cpu_cycles.saturating_add(MIN_MUSIC_BUSY_CYCLES));
            return false;
        }

        self.stop();

        let (samples, sample_rate) = if tune == 0 {
            (synthesize_cart_tune(mem), SAMPLE_RATE)
        } else {
            let Some(cached) = cached_rom_tune(tune) else {
                eprintln!("music: unknown tune #{tune}");
                self.busy_until_cycles = Some(cpu_cycles.saturating_add(MIN_MUSIC_BUSY_CYCLES));
                return false;
            };
            cached
        };

        let busy = busy_cycles_for_samples(samples.len(), sample_rate);
        self.start_samples(samples, sample_rate, audio, cpu_cycles, busy)
    }

    fn start_samples(
        &mut self,
        samples: Vec<f32>,
        sample_rate: u32,
        audio: &mut AudioOutput,
        cpu_cycles: u64,
        busy: u64,
    ) -> bool {
        let Some(sink) = audio.open_sink() else {
            eprintln!("music: no audio output device");
            self.busy_until_cycles = Some(cpu_cycles.saturating_add(busy));
            return true;
        };
        sink.set_volume(1.0);
        let source = SamplesBuffer::new(1, sample_rate, samples);
        sink.append(source);
        self.busy_until_cycles = Some(cpu_cycles.saturating_add(busy));
        self.sink = Some(sink);
        true
    }
}

/// Keep `$2B`/`$2C` set while audio is still playing so execute mode waits like hardware.
pub fn sync_music_voice_busy(mem: &mut [u8; 65536], player: &MusicPlayer, cpu_cycles: u64) {
    if player.music_busy(cpu_cycles) {
        mem[0x2B] = 0x80;
        mem[0x2C] = 0xFF;
    } else if mem[0x2B] != 0 {
        mem[0x2B] = 0;
        mem[0x2C] = 0;
    }
}

fn tune_for_play_hook(cpu_pc: u16, mem: &[u8; 65536]) -> Option<u8> {
    match cpu_pc {
        PLAY_JSR_GET_TUNE | ROM_GET_TUNE_POINTERS if mem[0x11] == PLAY_OPCODE => Some(mem[0x13]),
        _ => None,
    }
}

pub fn enter_play_tune(
    cpu_pc: u16,
    mem: &[u8; 65536],
    player: &mut MusicPlayer,
    audio: &mut AudioOutput,
    speech: &mut SpeechPlayer,
    cpu_cycles: u64,
) -> bool {
    let Some(tune) = tune_for_play_hook(cpu_pc, mem) else {
        return false;
    };
    if speech.speech_busy(cpu_cycles) {
        return false;
    }
    if player.last_tune == Some(tune)
        && player.has_active_sink()
        && player.music_busy(cpu_cycles)
    {
        return false;
    }
    player.play_tune(tune, mem, audio, speech, cpu_cycles);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rom_tune_labels_cover_demo() {
        assert_eq!(rom_tune_label(6), Some("Reveille"));
        assert!(rom_tune_label(0).is_none());
    }

    #[test]
    fn cbs_cart_tune_zero_synthesizes() {
        let mut mem = [0u8; 65536];
        mem[0x0400..0x0408].copy_from_slice(&[0x70, 0x12, 0x70, 0x11, 0x38, 0x0F, 0x00, 0x00]);
        let samples = synthesize_cart_tune(&mem);
        assert!(samples.len() > SAMPLE_RATE as usize);
    }

    #[test]
    fn reveille_asset_decodes() {
        assert!(TUNE_OGG[6].len() > 8_000);
        let (samples, rate) = cached_rom_tune(6).expect("decode");
        assert!(rate > 0);
        assert!(samples.len() > rate as usize);
    }

    #[test]
    fn play_tune_attaches_sink_when_audio_available() {
        let mut mem = [0u8; 65536];
        mem[0x11] = PLAY_OPCODE;
        mem[0x13] = 6;
        let mut audio = AudioOutput::new();
        let mut music = MusicPlayer::new(true);
        let mut speech = SpeechPlayer::new(true);
        audio.warm();
        assert!(enter_play_tune(
            PLAY_JSR_GET_TUNE,
            &mem,
            &mut music,
            &mut audio,
            &mut speech,
            0,
        ));
        assert_eq!(music.last_tune(), Some(6));
        if audio.available() {
            assert!(
                music.has_active_sink(),
                "expected music sink when audio device is available"
            );
        }
    }
}