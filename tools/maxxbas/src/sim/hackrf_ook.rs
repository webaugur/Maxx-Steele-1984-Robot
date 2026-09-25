//! 27 MHz OOK for the live simulator's on-screen remote.
//!
//! The firmware path still injects `RemoteKey::keycode` into `$75`. This module
//! sends the on-air envelope from the reverse-engineering notes: about 645 baud,
//! 1.55 ms cells, 29 ms frames (21 ms for Power/Stop). Center frequency is the
//! `tools/rfcap` capture tuning, 27.095 MHz.

use std::io::Write;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use super::keypad::RemoteKey;

/// HackRF minimum sample rate.
pub const SAMPLE_RATE_HZ: u32 = 2_000_000;
/// Measured OOK cell. 1 / 1550 µs ≈ 645 baud.
pub const BIT_US: u32 = 1_550;
pub const FRAME_US: u32 = 29_000;
pub const FRAME_Y_US: u32 = 21_000;
/// `tools/rfcap` capture center. Nudge here if the robot's crystal is off this.
pub const CENTER_HZ: u64 = 27_095_000;
/// TX VGA in dB. Amp stays off. Raise only if the robot does not hear the carrier.
const TX_VGA_DB: u8 = 0;
const CARRIER_I: i8 = 80;

struct OokWord {
    repeat: u16,
    /// Zero means this key has no separate release word (Power/Stop).
    release: u16,
    nbits: u8,
}

fn ook_word(key: RemoteKey) -> OokWord {
    let (repeat, release, nbits) = match key {
        RemoteKey::DriveU => (0b10101000101, 0b11010000101, 11),
        RemoteKey::Drive1 => (0b10101000110, 0b11010000110, 11),
        RemoteKey::Drive2 => (0b11001000111, 0b11110000111, 11),
        RemoteKey::Drive3 => (0b10101001000, 0b11010001000, 11),
        RemoteKey::Wrist4 => (0b11001001001, 0b11110001001, 11),
        RemoteKey::Wrist5 => (0b11001001010, 0b11110001010, 11),
        RemoteKey::Arms6 => (0b11101001011, 0b10010001011, 11),
        RemoteKey::Arms7 => (0b10101010100, 0b11010010100, 11),
        RemoteKey::Claw8 => (0b11001010101, 0b11110010101, 11),
        RemoteKey::Claw9 => (0b11001010110, 0b11110010110, 11),
        RemoteKey::LampA => (0b11101010111, 0b10010010111, 11),
        RemoteKey::HomeB => (0b11001011000, 0b11110011000, 11),
        RemoteKey::Wait => (0b11101011001, 0b10010011001, 11),
        RemoteKey::ShiftOctave => (0b11101011010, 0b10010011010, 11),
        RemoteKey::Clear => (0b10001011011, 0b10110011011, 11),
        RemoteKey::Enter => (0b10101100100, 0b11010100100, 11),
        RemoteKey::SongNotes => (0b11001100101, 0b11110100101, 11),
        RemoteKey::ClockStatus => (0b11001100110, 0b11110100110, 11),
        RemoteKey::Speech => (0b11101100111, 0b10010100111, 11),
        RemoteKey::Motion => (0b11001101000, 0b11110101000, 11),
        RemoteKey::Game => (0b11101101001, 0b10010101001, 11),
        RemoteKey::Program => (0b11101101010, 0b10010101010, 11),
        RemoteKey::Learn => (0b10001101011, 0b10110101011, 11),
        RemoteKey::Execute => (0b11001110100, 0b11110110100, 11),
        RemoteKey::PowerStop => (0b1110111010100, 0, 13),
    };
    OokWord {
        repeat,
        release,
        nbits,
    }
}

pub fn samples_per_bit() -> usize {
    (SAMPLE_RATE_HZ as u64 * BIT_US as u64 / 1_000_000) as usize
}

pub fn frame_samples(nbits: u8) -> usize {
    let us = if nbits > 11 { FRAME_Y_US } else { FRAME_US };
    (SAMPLE_RATE_HZ as u64 * us as u64 / 1_000_000) as usize
}

/// Interleaved signed-8 I/Q. Carrier on is a DC level at `CENTER_HZ`. Off is zero.
pub fn render_frame(key: RemoteKey, release: bool) -> Vec<i8> {
    let word = ook_word(key);
    let nbits = word.nbits;
    let total = frame_samples(nbits);
    let mut iq = vec![0i8; total.saturating_mul(2)];
    let bits = if release { word.release } else { word.repeat };
    if release && bits == 0 {
        return iq;
    }
    let cell = samples_per_bit();
    for index in 0..nbits {
        let on = (bits >> (nbits - 1 - index)) & 1 == 1;
        if !on {
            continue;
        }
        let start = index as usize * cell;
        let end = (start + cell).min(total);
        for sample in start..end {
            iq[sample * 2] = CARRIER_I;
        }
    }
    iq
}

struct TxState {
    transmitting: bool,
    error: Option<String>,
}

enum Msg {
    Held(Option<RemoteKey>),
    Shutdown,
}

pub struct HackRfTx {
    enabled: Arc<AtomicBool>,
    tx: Mutex<Sender<Msg>>,
    state: Arc<Mutex<TxState>>,
    child: Arc<Mutex<Option<Child>>>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
    last_held: Option<RemoteKey>,
}

impl HackRfTx {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let state = Arc::new(Mutex::new(TxState {
            transmitting: false,
            error: None,
        }));
        let child = Arc::new(Mutex::new(None));
        let stop = Arc::new(AtomicBool::new(false));
        let enabled = Arc::new(AtomicBool::new(false));
        let thread_state = Arc::clone(&state);
        let thread_child = Arc::clone(&child);
        let thread_stop = Arc::clone(&stop);
        let thread = thread::Builder::new()
            .name("hackrf-ook".into())
            .spawn(move || worker(rx, thread_state, thread_child, thread_stop))
            .expect("hackrf worker thread");
        Self {
            enabled,
            tx: Mutex::new(tx),
            state,
            child,
            stop,
            thread: Some(thread),
            last_held: None,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    pub fn toggle(&mut self) {
        let next = !self.is_enabled();
        self.enabled.store(next, Ordering::Relaxed);
        if !next {
            self.note_held(None);
            return;
        }
        // Pressing TX with nothing plugged in should say so immediately.
        // The on-screen robot does not depend on this.
        if let Err(err) = probe_hackrf() {
            eprintln!("{err}");
            set_error(&self.state, err);
        }
    }

    /// Latest key the pointer or keyboard is holding. `None` is release.
    pub fn note_held(&mut self, key: Option<RemoteKey>) {
        if key == self.last_held {
            return;
        }
        self.last_held = key;
        if let Ok(tx) = self.tx.lock() {
            let _ = tx.send(Msg::Held(key));
        }
    }

    /// `Some("on")` while samples are flowing, or a short error from the tool.
    pub fn status_label(&self) -> Option<String> {
        let state = self.state.lock().ok()?;
        if let Some(err) = &state.error {
            Some(err.clone())
        } else if state.transmitting {
            Some("on".into())
        } else if self.is_enabled() {
            Some("idle".into())
        } else {
            None
        }
    }
}

impl Drop for HackRfTx {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        self.enabled.store(false, Ordering::Relaxed);
        if let Ok(tx) = self.tx.lock() {
            let _ = tx.send(Msg::Shutdown);
        }
        if let Ok(mut slot) = self.child.lock() {
            if let Some(child) = slot.as_mut() {
                let _ = child.kill();
            }
        }
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn worker(
    rx: Receiver<Msg>,
    state: Arc<Mutex<TxState>>,
    child_slot: Arc<Mutex<Option<Child>>>,
    stop: Arc<AtomicBool>,
) {
    let mut current: Option<RemoteKey> = None;
    let mut on_air: Option<RemoteKey> = None;
    let mut stdin: Option<ChildStdin> = None;
    loop {
        while let Ok(msg) = rx.try_recv() {
            match msg {
                Msg::Shutdown => stop.store(true, Ordering::Relaxed),
                Msg::Held(key) => current = key,
            }
        }
        if stop.load(Ordering::Relaxed) {
            break;
        }
        if current != on_air {
            if let Some(old) = on_air {
                if ook_word(old).release != 0 {
                    if !write_frame(&mut stdin, old, true, &state) {
                        stop_radio(&mut stdin, &child_slot, &state);
                        on_air = None;
                        current = None;
                        continue;
                    }
                }
            }
            if current.is_none() {
                stop_radio(&mut stdin, &child_slot, &state);
                on_air = None;
            } else if !ensure_radio(&mut stdin, &child_slot, &state) {
                current = None;
                on_air = None;
                continue;
            } else {
                on_air = current;
            }
        }
        if let Some(key) = current {
            if !write_frame(&mut stdin, key, false, &state) {
                stop_radio(&mut stdin, &child_slot, &state);
                on_air = None;
                current = None;
            }
        } else {
            match rx.recv() {
                Ok(Msg::Shutdown) => break,
                Ok(Msg::Held(key)) => current = key,
                Err(_) => break,
            }
        }
    }
    stop_radio(&mut stdin, &child_slot, &state);
}

fn write_frame(
    stdin: &mut Option<ChildStdin>,
    key: RemoteKey,
    release: bool,
    state: &Mutex<TxState>,
) -> bool {
    let Some(stdin) = stdin.as_mut() else {
        return false;
    };
    let iq = render_frame(key, release);
    match stdin.write_all(&iq.iter().map(|s| *s as u8).collect::<Vec<_>>()) {
        Ok(()) => true,
        Err(err) => {
            set_error(state, format!("hackrf write: {err}"));
            false
        }
    }
}

fn ensure_radio(
    stdin: &mut Option<ChildStdin>,
    child_slot: &Mutex<Option<Child>>,
    state: &Mutex<TxState>,
) -> bool {
    if stdin.is_some() {
        return true;
    }
    if let Err(err) = probe_hackrf() {
        eprintln!("{err}");
        set_error(state, err);
        return false;
    }
    match spawn_hackrf() {
        Ok((child, pipe)) => {
            if let Ok(mut slot) = child_slot.lock() {
                *slot = Some(child);
            }
            *stdin = Some(pipe);
            if let Ok(mut st) = state.lock() {
                st.transmitting = true;
                st.error = None;
            }
            true
        }
        Err(err) => {
            set_error(state, err);
            false
        }
    }
}

/// `MaxxSim/maxx/<os>/maxx` looks in `MaxxSim/hackrf/<os>/`. `PATH` is the fallback.
fn bundled_os_dir(exe: &std::path::Path) -> Option<std::path::PathBuf> {
    let os = if cfg!(windows) {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "linux"
    };
    Some(exe.parent()?.parent()?.parent()?.join("hackrf").join(os))
}

fn tool_path(name: &str) -> Result<std::path::PathBuf, String> {
    let file = if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    };
    if let Some(dir) = std::env::current_exe()
        .ok()
        .as_deref()
        .and_then(bundled_os_dir)
    {
        let candidate = dir.join(&file);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    which::which(name).map_err(|_| format!("{name} not on PATH"))
}

fn hackrf_command(name: &str) -> Result<Command, String> {
    let path = tool_path(name)?;
    let mut cmd = Command::new(&path);
    #[cfg(unix)]
    if let (Some(dir), Some(bundle)) = (
        path.parent(),
        std::env::current_exe()
            .ok()
            .as_deref()
            .and_then(bundled_os_dir),
    ) {
        if path.starts_with(&bundle) {
            let key = if cfg!(target_os = "macos") {
                "DYLD_LIBRARY_PATH"
            } else {
                "LD_LIBRARY_PATH"
            };
            let mut value = dir.as_os_str().to_owned();
            if let Some(old) = std::env::var_os(key) {
                value.push(":");
                value.push(old);
            }
            cmd.env(key, value);
        }
    }
    Ok(cmd)
}

/// `hackrf_info` prints "No HackRF boards found." on stdout and exits 1.
/// One stderr line is enough; do not launch `hackrf_transfer`, which then
/// dumps its whole usage text on stdout.
fn probe_hackrf() -> Result<(), String> {
    let out = hackrf_command("hackrf_info")?
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|err| format!("hackrf_info: {err}"))?;
    if out.status.success() {
        return Ok(());
    }
    Err("HackRF not found".to_string())
}

fn spawn_hackrf() -> Result<(Child, ChildStdin), String> {
    let freq = CENTER_HZ.to_string();
    let rate = SAMPLE_RATE_HZ.to_string();
    let gain = TX_VGA_DB.to_string();
    let mut child = hackrf_command("hackrf_transfer")?
        .args([
            "-t", "-", "-f", &freq, "-s", &rate, "-a", "0", "-x", &gain,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|err| format!("hackrf_transfer: {err}"))?;
    let pipe = child
        .stdin
        .take()
        .ok_or_else(|| "hackrf_transfer stdin missing".to_string())?;
    Ok((child, pipe))
}

fn stop_radio(
    stdin: &mut Option<ChildStdin>,
    child_slot: &Mutex<Option<Child>>,
    state: &Mutex<TxState>,
) {
    drop(stdin.take());
    if let Ok(mut slot) = child_slot.lock() {
        if let Some(mut child) = slot.take() {
            let started = std::time::Instant::now();
            loop {
                match child.try_wait() {
                    Ok(Some(code)) => {
                        if !code.success() {
                            set_error(state, format!("hackrf_transfer exited {code}"));
                        }
                        break;
                    }
                    Ok(None) if started.elapsed() < Duration::from_millis(400) => {
                        thread::sleep(Duration::from_millis(20));
                    }
                    _ => {
                        let _ = child.kill();
                        let _ = child.wait();
                        break;
                    }
                }
            }
        }
    }
    if let Ok(mut st) = state.lock() {
        st.transmitting = false;
    }
}

fn set_error(state: &Mutex<TxState>, err: String) {
    if let Ok(mut st) = state.lock() {
        st.transmitting = false;
        st.error = Some(trim_err(&err));
    }
}

fn trim_err(err: &str) -> String {
    let one_line: String = err.split_whitespace().collect::<Vec<_>>().join(" ");
    const MAX: usize = 80;
    if one_line.chars().count() <= MAX {
        one_line
    } else {
        one_line.chars().take(MAX).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_tools_sit_beside_the_start_scripts() {
        let exe = std::path::Path::new("/stick/MaxxSim/maxx/linux/maxx");
        let dir = bundled_os_dir(exe).unwrap();
        assert_eq!(
            dir.parent().unwrap(),
            std::path::Path::new("/stick/MaxxSim/hackrf")
        );
    }

    #[test]
    fn frame_is_29_ms_at_2_msps() {
        assert_eq!(frame_samples(11), 2_000_000 * 29 / 1000);
        assert_eq!(frame_samples(13), 2_000_000 * 21 / 1000);
        assert_eq!(samples_per_bit(), 2_000_000 * 1550 / 1_000_000);
        assert!(11 * samples_per_bit() < frame_samples(11));
        assert!(13 * samples_per_bit() < frame_samples(13));
    }

    #[test]
    fn patterns_match_reverse_engineering_notes() {
        let expected = [
            (RemoteKey::DriveU, 0b10101000101u16, 0b11010000101u16, 11u8),
            (RemoteKey::Drive1, 0b10101000110, 0b11010000110, 11),
            (RemoteKey::Drive2, 0b11001000111, 0b11110000111, 11),
            (RemoteKey::Drive3, 0b10101001000, 0b11010001000, 11),
            (RemoteKey::Wrist4, 0b11001001001, 0b11110001001, 11),
            (RemoteKey::Wrist5, 0b11001001010, 0b11110001010, 11),
            (RemoteKey::Arms6, 0b11101001011, 0b10010001011, 11),
            (RemoteKey::Arms7, 0b10101010100, 0b11010010100, 11),
            (RemoteKey::Claw8, 0b11001010101, 0b11110010101, 11),
            (RemoteKey::Claw9, 0b11001010110, 0b11110010110, 11),
            (RemoteKey::LampA, 0b11101010111, 0b10010010111, 11),
            (RemoteKey::HomeB, 0b11001011000, 0b11110011000, 11),
            (RemoteKey::Wait, 0b11101011001, 0b10010011001, 11),
            (RemoteKey::ShiftOctave, 0b11101011010, 0b10010011010, 11),
            (RemoteKey::Clear, 0b10001011011, 0b10110011011, 11),
            (RemoteKey::Enter, 0b10101100100, 0b11010100100, 11),
            (RemoteKey::SongNotes, 0b11001100101, 0b11110100101, 11),
            (RemoteKey::ClockStatus, 0b11001100110, 0b11110100110, 11),
            (RemoteKey::Speech, 0b11101100111, 0b10010100111, 11),
            (RemoteKey::Motion, 0b11001101000, 0b11110101000, 11),
            (RemoteKey::Game, 0b11101101001, 0b10010101001, 11),
            (RemoteKey::Program, 0b11101101010, 0b10010101010, 11),
            (RemoteKey::Learn, 0b10001101011, 0b10110101011, 11),
            (RemoteKey::Execute, 0b11001110100, 0b11110110100, 11),
            (RemoteKey::PowerStop, 0b1110111010100, 0, 13),
        ];
        for (key, repeat, release, nbits) in expected {
            let word = ook_word(key);
            assert_eq!(word.repeat, repeat, "{key:?} repeat");
            assert_eq!(word.release, release, "{key:?} release");
            assert_eq!(word.nbits, nbits, "{key:?} width");
            let frame = render_frame(key, false);
            assert_eq!(frame.len(), frame_samples(nbits) * 2);
            let on = (repeat >> (nbits - 1)) & 1 == 1;
            assert_eq!(frame[0] == CARRIER_I, on);
        }
    }
}
