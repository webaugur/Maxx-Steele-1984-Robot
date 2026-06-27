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
}

/// MaxxOS answer digit + `?` prompt glyphs (see cart `$A1F0`).
const MAXXOS_PROMPT_SEG: u8 = 0xEE;

impl LedDisplay {
    /// Record a completed digit from `$ED4F` (A = segment pattern on entry).
    pub fn push_segment(&mut self, seg: u8) {
        if ED48_PREFIX.contains(&seg) {
            return;
        }
        self.digits.push(seg);
        if self.digits.len() > 9 {
            self.digits.remove(0);
        }
    }

    /// MaxxOS `disp_answer` — digit from ROM `$F8BE` table plus `?`.
    pub fn mirror_answer_digit(&mut self, mem: &[u8; 65536], digit: u8) {
        if digit >= 10 {
            return;
        }
        let seg = mem[0xF8BE + digit as usize];
        let prompt = mem[0xA1F2];
        self.push_segment(seg);
        self.push_segment(if prompt == 0 { MAXXOS_PROMPT_SEG } else { prompt });
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
    fn mirror_answer_shows_digit() {
        let mut mem = [0u8; 65536];
        mem[0xF8BE + 7] = 0xE0;
        mem[0xA1F2] = 0xEE;
        let mut d = LedDisplay::default();
        d.mirror_answer_digit(&mem, 7);
        assert_eq!(d.pair(), "7?");
    }

    #[test]
    fn maxxos_prompt_pair() {
        let mut d = LedDisplay::default();
        d.push_segment(0xDA); // 3
        d.push_segment(0x2A); // +
        d.push_segment(0xE2); // space (skipped)
        d.push_segment(0xB6); // 5
        d.push_segment(0xEE); // ?
        d.push_segment(0x10); // blank (skipped)
        d.push_segment(0xE2); // space (skipped)
        assert_eq!(d.pair(), "5?");
    }
}