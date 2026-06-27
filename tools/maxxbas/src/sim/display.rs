//! Robot LED digits — capture 7-segment patterns at ROM `$ED4F` (not raw `$1200` wire bytes).

const SEG_HEX: &[u8; 16] = b"0123456789ABCDEF";
const SEG_TABLE: [u8; 16] = [
    0xFC, 0x60, 0xDA, 0xF2, 0x66, 0xB6, 0xBE, 0xE0, 0xFE, 0xE6, 0xEE, 0x3E, 0x9C, 0x7A, 0x9E, 0x8E,
];

/// Serial-link framing bytes sent before each digit (see ROM `$ED48`).
const ED48_PREFIX: [u8; 2] = [0xE3, 0xF3];

#[derive(Debug, Clone, Default)]
pub struct LedDisplay {
    /// Completed segment patterns (newest at end).
    digits: Vec<u8>,
    /// Last pair shown in the GUI after the shift-register stream settles.
    settled: String,
    last_push_cycle: u64,
}

/// Hold the last decoded pair this many CPU cycles after the final `$ED4F` of a burst.
const SETTLE_CYCLES: u64 = 1_200;

/// MaxxOS answer digit + `?` prompt glyphs (see cart `$A1F0`).
const MAXXOS_PROMPT_SEG: u8 = 0xEE;

impl LedDisplay {
    /// Record a completed digit from `$ED4F` (A = segment pattern on entry).
    pub fn push_segment(&mut self, seg: u8, cycles: u64) {
        if ED48_PREFIX.contains(&seg) {
            return;
        }
        self.digits.push(seg);
        if self.digits.len() > 9 {
            self.digits.remove(0);
        }
        self.last_push_cycle = cycles;
    }

    /// Reset for a new quiz prompt (`show_problem`).
    pub fn begin_problem(&mut self) {
        self.digits.clear();
        self.settled.clear();
        self.last_push_cycle = 0;
    }

    /// MaxxOS answer entry — two-digit face shows [digit][?], not stacked history.
    pub fn show_answer(&mut self, mem: &[u8; 65536], digit: u8, cycles: u64) {
        if digit >= 10 {
            return;
        }
        self.digits.clear();
        let seg = mem[0xF8BE + digit as usize];
        let prompt = mem[0xA1F3];
        self.push_segment(seg, cycles);
        self.push_segment(
            if prompt == 0 { MAXXOS_PROMPT_SEG } else { prompt },
            cycles,
        );
        self.settled = self.pair();
    }

    /// Pair held on the robot face after the COP411 shift register finishes a burst.
    pub fn settled_pair(&mut self, cycles: u64) -> String {
        if cycles.saturating_sub(self.last_push_cycle) >= SETTLE_CYCLES || self.settled.is_empty() {
            self.settled = self.pair();
        }
        self.settled.clone()
    }

    pub fn pair(&self) -> String {
        let visible: Vec<u8> = self
            .digits
            .iter()
            .copied()
            .filter(|seg| !is_filler(*seg))
            .collect();
        let (a, b) = match visible.len() {
            0 => (0x10_u8, 0x10),
            1 => (visible[0], 0x10),
            _ => (
                visible[visible.len() - 2],
                visible[visible.len() - 1],
            ),
        };
        format!("{}{}", seg_to_char(a), seg_to_char(b))
    }
}

fn is_filler(seg: u8) -> bool {
    matches!(seg, 0x10 | 0xE2) || ED48_PREFIX.contains(&seg)
}

fn seg_to_char(seg: u8) -> char {
    match seg {
        0x10 => '_',
        0xE2 => ' ',
        0x2A => '+',
        0xEE => '?', // MaxxOS prompt glyph
        _ => {
            for (i, &pat) in SEG_TABLE.iter().enumerate() {
                if pat == seg {
                    return SEG_HEX[i] as char;
                }
            }
            '?'
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_answer_replaces_history() {
        let mut mem = [0u8; 65536];
        mem[0xF8BE + 7] = 0xE0;
        mem[0xA1F3] = 0xEE;
        let mut d = LedDisplay::default();
        d.push_segment(0xB6, 0); // prior quiz operand "5"
        d.push_segment(0xB6, 0); // duplicate would have shown [55]
        d.show_answer(&mem, 7, 100);
        assert_eq!(d.pair(), "7?");
    }

    #[test]
    fn maxxos_prompt_pair() {
        let mut d = LedDisplay::default();
        d.push_segment(0xDA, 0); // 3
        d.push_segment(0x2A, 0); // +
        d.push_segment(0xE2, 0); // space (skipped)
        d.push_segment(0xB6, 0); // 5
        d.push_segment(0xEE, 0); // ?
        d.push_segment(0x10, 0); // blank (skipped)
        d.push_segment(0xE2, 0); // space (skipped)
        assert_eq!(d.pair(), "5?");
    }

    #[test]
    fn settled_pair_holds_through_burst() {
        let mut d = LedDisplay::default();
        d.push_segment(0xB6, 100);
        assert_eq!(d.settled_pair(500), "5_");
        d.push_segment(0xEE, 900);
        assert_eq!(d.settled_pair(1_000), "5_");
        assert_eq!(d.settled_pair(2_200), "5?");
    }
}