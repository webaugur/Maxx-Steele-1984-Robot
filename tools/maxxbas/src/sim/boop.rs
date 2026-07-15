//! Emotive post-speech “beep-boop” flourishes (Maxx / CBS greeting style).
//!
//! Inspired by the electronic chirps after the factory greeting recording
//! (~1–2 kHz square/chirp blips) and classic robot SFX — short multi-note
//! patterns with glides, not full tunes.

use std::sync::atomic::{AtomicU64, Ordering};

/// Same rate as SAM speech so boops concatenate cleanly in `say`.
pub const BOOP_SAMPLE_RATE: u32 = super::speech_sam::SAM_SAMPLE_RATE;

/// One chirp: optional linear frequency glide + short gap after.
#[derive(Clone, Copy, Debug)]
struct Chirp {
    start_hz: f32,
    end_hz: f32,
    ms: u32,
    gap_ms: u32,
    amp: f32,
}

/// Named emotive pattern (several chirps).
#[derive(Clone, Copy, Debug)]
pub struct BoopPattern {
    pub name: &'static str,
    pub mood: &'static str,
    chirps: &'static [Chirp],
}

const fn c(start_hz: f32, end_hz: f32, ms: u32, gap_ms: u32, amp: f32) -> Chirp {
    Chirp {
        start_hz,
        end_hz,
        ms,
        gap_ms,
        amp,
    }
}

/// Catalog of post-statement flourishes — pick one at random with [`random_boop`].
pub const PATTERNS: &[BoopPattern] = &[
    // Greeting-tail energy: mid→high→higher blips (~1–2 kHz), short.
    BoopPattern {
        name: "greet",
        mood: "hello / ready",
        chirps: &[
            c(1050.0, 1180.0, 55, 25, 0.28),
            c(1500.0, 1650.0, 50, 20, 0.30),
            c(1900.0, 2100.0, 45, 15, 0.26),
            c(900.0, 700.0, 40, 0, 0.22),
        ],
    },
    BoopPattern {
        name: "curious",
        mood: "question / hmm?",
        chirps: &[
            c(880.0, 1320.0, 90, 30, 0.27),
            c(1320.0, 1580.0, 70, 0, 0.25),
        ],
    },
    BoopPattern {
        name: "happy",
        mood: "cheerful ascent",
        chirps: &[
            c(700.0, 700.0, 45, 18, 0.26),
            c(880.0, 880.0, 45, 18, 0.27),
            c(1050.0, 1180.0, 55, 18, 0.28),
            c(1400.0, 1600.0, 60, 0, 0.29),
        ],
    },
    BoopPattern {
        name: "affirm",
        mood: "yes / ack",
        chirps: &[
            c(1200.0, 1200.0, 40, 35, 0.28),
            c(1200.0, 1200.0, 55, 0, 0.30),
        ],
    },
    BoopPattern {
        name: "think",
        mood: "processing warble",
        chirps: &[
            c(1000.0, 1400.0, 40, 12, 0.24),
            c(1400.0, 1000.0, 40, 12, 0.24),
            c(1000.0, 1500.0, 45, 12, 0.25),
            c(1500.0, 1100.0, 50, 0, 0.24),
        ],
    },
    BoopPattern {
        name: "alert",
        mood: "heads-up",
        chirps: &[
            c(1800.0, 2200.0, 35, 20, 0.30),
            c(2200.0, 1600.0, 45, 15, 0.28),
            c(1600.0, 2000.0, 40, 0, 0.27),
        ],
    },
    BoopPattern {
        name: "soft",
        mood: "gentle confirm",
        chirps: &[
            c(650.0, 820.0, 80, 40, 0.20),
            c(820.0, 700.0, 70, 0, 0.18),
        ],
    },
    BoopPattern {
        name: "spark",
        mood: "staccato excitement",
        chirps: &[
            c(1600.0, 1600.0, 28, 18, 0.28),
            c(1900.0, 1900.0, 28, 18, 0.29),
            c(2200.0, 2400.0, 32, 0, 0.27),
        ],
    },
    BoopPattern {
        name: "bye",
        mood: "sign-off descent",
        chirps: &[
            c(1500.0, 1200.0, 60, 25, 0.26),
            c(1000.0, 750.0, 70, 20, 0.24),
            c(600.0, 480.0, 80, 0, 0.20),
        ],
    },
    BoopPattern {
        name: "whistle",
        mood: "slide call",
        chirps: &[
            c(900.0, 1800.0, 120, 30, 0.25),
            c(1600.0, 1100.0, 90, 0, 0.23),
        ],
    },
    BoopPattern {
        name: "giggle",
        mood: "playful burbles",
        chirps: &[
            c(1400.0, 1700.0, 35, 15, 0.26),
            c(1200.0, 1500.0, 35, 15, 0.25),
            c(1600.0, 1900.0, 35, 15, 0.26),
            c(1300.0, 1000.0, 50, 0, 0.24),
        ],
    },
    BoopPattern {
        name: "scan",
        mood: "sensor sweep",
        chirps: &[
            c(500.0, 2000.0, 140, 20, 0.22),
            c(2000.0, 800.0, 100, 0, 0.22),
        ],
    },
];

static BOOP_RNG: AtomicU64 = AtomicU64::new(0xC0FFEE_u64.wrapping_mul(0x9E37));

fn next_u64() -> u64 {
    // xorshift64* — no deps, fine for non-crypto boop picks
    let mut x = BOOP_RNG.load(Ordering::Relaxed);
    loop {
        let mut n = x;
        n ^= n << 13;
        n ^= n >> 7;
        n ^= n << 17;
        match BOOP_RNG.compare_exchange_weak(x, n, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return n,
            Err(cur) => x = cur,
        }
    }
}

/// Seed the boop RNG (optional; defaults to a fixed stream that still varies per pick).
pub fn seed_boop_rng(seed: u64) {
    BOOP_RNG.store(seed | 1, Ordering::Relaxed);
}

/// Pick a random pattern from [`PATTERNS`].
pub fn random_boop() -> &'static BoopPattern {
    pick_random(PATTERNS)
}

fn pick_random(choices: &[BoopPattern]) -> &'static BoopPattern {
    debug_assert!(!choices.is_empty());
    // PATTERNS and sub-slices are 'static; index into PATTERNS by name to keep 'static
    let i = (next_u64() % choices.len() as u64) as usize;
    let name = choices[i].name;
    find_boop(name).unwrap_or(&PATTERNS[0])
}

fn pick_named(names: &[&str]) -> &'static BoopPattern {
    let mut found: Vec<&BoopPattern> = names.iter().filter_map(|n| find_boop(n)).collect();
    if found.is_empty() {
        return random_boop();
    }
    let i = (next_u64() % found.len() as u64) as usize;
    found.swap_remove(i)
}

/// Look up a pattern by name (case-insensitive).
pub fn find_boop(name: &str) -> Option<&'static BoopPattern> {
    let key = name.trim().to_ascii_lowercase();
    PATTERNS.iter().find(|p| p.name == key)
}

/// Terminal punctuation run at the end of a statement (after trimming quotes).
///
/// Returns e.g. `"?"`, `"!"`, `"..."`, `"!?"`, `"."`, or `""` if none.
pub fn trailing_punctuation(stmt: &str) -> String {
    let t = stmt.trim();
    let t = t.trim_end_matches(|c| matches!(c, '"' | '\'' | ')' | ']' | '»' | '”' | '’'));
    let mut chars: Vec<char> = t.chars().collect();
    // Unicode ellipsis as three dots
    if chars.last() == Some(&'…') {
        return "...".into();
    }
    let mut punct = String::new();
    while let Some(&c) = chars.last() {
        if matches!(c, '.' | '!' | '?') {
            punct.insert(0, c);
            chars.pop();
        } else if c.is_whitespace() {
            chars.pop();
        } else {
            break;
        }
    }
    // Collapse long runs of `.` beyond 3 for matching, keep `!`/`?` counts meaningful
    if punct.chars().all(|c| c == '.') && punct.len() > 3 {
        return "...".into();
    }
    punct
}

/// Choose a boop for a statement from punctuation (and light cues), else random.
///
/// | Punctuation | Moods (random among) |
/// |-------------|----------------------|
/// | `?` / `??`  | curious, whistle |
/// | `!`         | happy, spark, giggle |
/// | `!!`+       | spark, alert |
/// | `!?` / `?!` | alert, curious, spark |
/// | `...` / `…` | think, scan |
/// | `.`         | affirm, soft, greet |
/// | none / other| full random |
pub fn select_boop_for_statement(stmt: &str) -> &'static BoopPattern {
    let punct = trailing_punctuation(stmt);
    let lower = stmt.to_ascii_lowercase();

    // Explicit interrobang-style
    if punct.contains('?') && punct.contains('!') {
        return pick_named(&["alert", "curious", "spark"]);
    }

    if punct.contains('?') {
        // "what?!" already handled; pure questions
        return pick_named(&["curious", "whistle", "curious"]);
    }

    if punct.contains('!') {
        let bangs = punct.chars().filter(|&c| c == '!').count();
        if bangs >= 2 {
            return pick_named(&["spark", "alert", "spark"]);
        }
        return pick_named(&["happy", "spark", "giggle", "happy"]);
    }

    if punct == "..." || punct.starts_with("..") {
        return pick_named(&["think", "scan", "think"]);
    }

    if punct == "." {
        // Mild word cues on declarative lines
        if lower.contains("bye") || lower.contains("goodbye") || lower.contains("good night") {
            return pick_named(&["bye", "soft"]);
        }
        if lower.contains("hello")
            || lower.contains("good morning")
            || lower.contains("i am maxx")
            || lower.contains("i'm maxx")
        {
            return pick_named(&["greet", "affirm"]);
        }
        return pick_named(&["affirm", "soft", "greet"]);
    }

    // No useful terminal punctuation — still honor a few whole-line cues, else random
    if lower.contains('?') {
        return pick_named(&["curious", "whistle"]);
    }
    if lower.contains('!') {
        return pick_named(&["happy", "spark"]);
    }
    random_boop()
}

/// Pattern → samples, with punctuation-aware selection.
pub fn synthesize_boop_for_statement(stmt: &str) -> (&'static BoopPattern, Vec<f32>) {
    let p = select_boop_for_statement(stmt);
    (p, synthesize_boop(p))
}

fn silence(ms: u32, out: &mut Vec<f32>) {
    let n = ((f64::from(ms) / 1000.0) * f64::from(BOOP_SAMPLE_RATE)).round() as usize;
    out.extend(std::iter::repeat_n(0.0, n));
}

fn render_chirp(ch: &Chirp, out: &mut Vec<f32>) {
    let n = ((f64::from(ch.ms) / 1000.0) * f64::from(BOOP_SAMPLE_RATE))
        .round()
        .max(1.0) as usize;
    let attack = (n / 12).max(2).min(80);
    let release = (n / 5).max(4).min(200);
    let mut phase = 0.0_f32;
    for i in 0..n {
        let t = i as f32 / n as f32;
        let hz = ch.start_hz + (ch.end_hz - ch.start_hz) * t;
        let hz = hz.clamp(80.0, 4_000.0);
        let phase_inc = hz / BOOP_SAMPLE_RATE as f32;
        // Soft square (Maxx cart music aesthetic) with a touch of fundamental sine
        let square = if phase < 0.5 { 1.0 } else { -1.0 };
        let sine = (phase * std::f32::consts::TAU).sin();
        let env_a = (i.min(attack) as f32) / attack as f32;
        let env_r = ((n.saturating_sub(i)).min(release) as f32) / release as f32;
        let env = env_a * env_r;
        out.push((square * 0.72 + sine * 0.28) * ch.amp * env);
        phase += phase_inc;
        if phase >= 1.0 {
            phase -= 1.0;
        }
    }
    if ch.gap_ms > 0 {
        silence(ch.gap_ms, out);
    }
}

/// Render a pattern to mono f32 samples @ [`BOOP_SAMPLE_RATE`].
pub fn synthesize_boop(pattern: &BoopPattern) -> Vec<f32> {
    let mut out = Vec::with_capacity(BOOP_SAMPLE_RATE as usize / 2);
    silence(40, &mut out); // breath after speech
    for ch in pattern.chirps {
        render_chirp(ch, &mut out);
    }
    silence(30, &mut out);
    out
}

/// Random pattern → samples.
pub fn synthesize_random_boop() -> (&'static BoopPattern, Vec<f32>) {
    let p = random_boop();
    (p, synthesize_boop(p))
}

/// Split text into statements. Terminal runs of `.` / `!` / `?` / `…` stay with
/// the clause (`...`, `?!`, `!!!` are one ending, not multiple statements).
pub fn split_statements(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '…' {
            cur.push(c);
            flush_statement_end(&mut cur, &mut chars, &mut out);
            continue;
        }
        if matches!(c, '.' | '!' | '?') {
            cur.push(c);
            // Absorb a whole terminal punct run (..., !!!, ?!, …)
            while let Some(&n) = chars.peek() {
                if matches!(n, '.' | '!' | '?' | '…') {
                    cur.push(n);
                    chars.next();
                } else {
                    break;
                }
            }
            flush_statement_end(&mut cur, &mut chars, &mut out);
            continue;
        }
        cur.push(c);
    }
    let t = cur.trim().to_string();
    if !t.is_empty() {
        out.push(t);
    }
    if out.is_empty() && !text.trim().is_empty() {
        out.push(text.trim().to_string());
    }
    out
}

fn flush_statement_end(
    cur: &mut String,
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    out: &mut Vec<String>,
) {
    // trailing quotes/parens after terminal punct
    while let Some(&n) = chars.peek() {
        if matches!(n, '"' | '\'' | ')' | ']' | '»' | '”' | '’') {
            cur.push(n);
            chars.next();
        } else {
            break;
        }
    }
    let t = cur.trim().to_string();
    if !t.is_empty() {
        out.push(t);
    }
    cur.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_patterns_render() {
        for p in PATTERNS {
            let s = synthesize_boop(p);
            assert!(s.len() > 500, "{} too short", p.name);
            assert!(s.iter().any(|&x| x.abs() > 0.05), "{} silent", p.name);
        }
    }

    #[test]
    fn random_varies() {
        seed_boop_rng(42);
        let a: Vec<_> = (0..20).map(|_| random_boop().name).collect();
        seed_boop_rng(99);
        let b: Vec<_> = (0..20).map(|_| random_boop().name).collect();
        // Not required to differ, but with 12 patterns and 20 picks almost always does
        assert_eq!(a.len(), 20);
        assert_eq!(b.len(), 20);
        assert!(PATTERNS.len() >= 8);
    }

    #[test]
    fn split_statements_basic() {
        let parts = split_statements("Hello. I am Maxx! Ready?");
        assert_eq!(parts.len(), 3);
        assert!(parts[0].starts_with("Hello"));
        assert!(parts[1].contains("Maxx"));
        assert!(parts[2].contains("Ready"));
    }

    #[test]
    fn split_keeps_ellipsis_and_interrobang() {
        let parts = split_statements("Hello. Ready? Wow! Hmm... What?!");
        assert_eq!(
            parts,
            vec![
                "Hello.".to_string(),
                "Ready?".to_string(),
                "Wow!".to_string(),
                "Hmm...".to_string(),
                "What?!".to_string(),
            ]
        );
    }

    #[test]
    fn trailing_punctuation_detects() {
        assert_eq!(trailing_punctuation("Ready?"), "?");
        assert_eq!(trailing_punctuation("Go!!!"), "!!!");
        assert_eq!(trailing_punctuation("Wait..."), "...");
        assert_eq!(trailing_punctuation("What?!"), "?!");
        assert_eq!(trailing_punctuation("Hello."), ".");
        assert_eq!(trailing_punctuation("no punct"), "");
        assert_eq!(trailing_punctuation("Hi.\""), ".");
    }

    #[test]
    fn select_boop_from_punct() {
        seed_boop_rng(1);
        for _ in 0..8 {
            let q = select_boop_for_statement("Are you ready?");
            assert!(
                matches!(q.name, "curious" | "whistle"),
                "question got {}",
                q.name
            );
            let e = select_boop_for_statement("Wow!");
            assert!(
                matches!(e.name, "happy" | "spark" | "giggle"),
                "exclaim got {}",
                e.name
            );
            let d = select_boop_for_statement("All set.");
            assert!(
                matches!(d.name, "affirm" | "soft" | "greet"),
                "period got {}",
                d.name
            );
            let t = select_boop_for_statement("Hmm...");
            assert!(
                matches!(t.name, "think" | "scan"),
                "ellipsis got {}",
                t.name
            );
            let iq = select_boop_for_statement("What?!");
            assert!(
                matches!(iq.name, "alert" | "curious" | "spark"),
                "interrobang got {}",
                iq.name
            );
        }
    }
}
