//! Talking-mouth helpers shared by live GUI and `say --mouth`.
//!
//! v1 is a simple chatter open/close — not phoneme visemes.

/// Mouth openness in `0.0..=1.0` (closed → fully open).
///
/// When `talking` is false, always closed. When true, a sine chatter so the
/// face looks animated without analyzing audio.
pub fn mouth_open(talking: bool, now_secs: f64) -> f32 {
    if !talking {
        return 0.0;
    }
    // ~1.9 Hz chatter
    let phase = now_secs * std::f64::consts::TAU * 1.9;
    let s = phase.sin() as f32;
    ((s * 0.5) + 0.5).clamp(0.0, 1.0)
}

/// Single-line ASCII mouth for terminal `\r` redraw (`say --mouth`).
///
/// Frames are fixed width so carriage-return updates stay clean.
pub fn ascii_mouth_line(open: f32) -> &'static str {
    // Fixed width for `\r` redraw; "SAM" brand-neutral (Maxx still uses this crate).
    if open < 0.25 {
        "SAM  [───]"
    } else if open < 0.55 {
        "SAM  [─o─]"
    } else if open < 0.8 {
        "SAM  [ o ]"
    } else {
        "SAM  [ O ]"
    }
}

/// Compact multi-line face (optional; prefer [`ascii_mouth_line`] for live TTY).
pub fn ascii_mouth_frame(open: f32) -> &'static str {
    if open < 0.33 {
        "  (•_•)\n  ─────"
    } else if open < 0.66 {
        "  (•_•)\n  ─ o ─"
    } else {
        "  (•_•)\n  ( O )"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_when_not_talking() {
        assert_eq!(mouth_open(false, 0.0), 0.0);
        assert_eq!(mouth_open(false, 1.5), 0.0);
    }

    #[test]
    fn open_in_range_when_talking() {
        for t in [0.0, 0.1, 0.25, 0.5, 1.0, 2.7] {
            let o = mouth_open(true, t);
            assert!((0.0..=1.0).contains(&o), "t={t} open={o}");
        }
    }

    #[test]
    fn ascii_frames_nonempty() {
        for o in [0.0, 0.3, 0.6, 1.0] {
            assert!(!ascii_mouth_line(o).is_empty());
            assert!(!ascii_mouth_frame(o).is_empty());
        }
    }
}
