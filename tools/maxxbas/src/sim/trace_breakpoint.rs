//! Lightweight trace breakpoints — pause when a selected address, opcode, or value appears.

use mos6502::instruction::{AddressingMode, Cmos6502, Instruction};
use mos6502::Variant;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BreakpointKind {
    /// Break when the program counter matches (e.g. `$E617`).
    Pc(u16),
    /// Break when instruction bytes at PC match (e.g. `A9 00` or `20`).
    OpcodeBytes(Vec<u8>),
    /// Break when the accumulator matches before the instruction runs.
    RegisterA(u8),
    RegisterX(u8),
    RegisterY(u8),
    /// Break when an immediate operand byte matches (e.g. `#$0F`).
    Immediate(u8),
    /// Break when a zero-page or absolute operand byte matches (e.g. `$75`).
    OperandByte(u8),
    /// Break when the decoded mnemonic matches (e.g. `JSR`).
    Mnemonic(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceBreakpoint {
    pub label: String,
    pub kind: BreakpointKind,
}

impl TraceBreakpoint {
    pub fn matches(
        &self,
        mem: &[u8; 65536],
        pc: u16,
        a: u8,
        x: u8,
        y: u8,
    ) -> bool {
        match &self.kind {
            BreakpointKind::Pc(addr) => pc == *addr,
            BreakpointKind::OpcodeBytes(bytes) => matches_opcode_bytes(mem, pc, bytes),
            BreakpointKind::RegisterA(v) => a == *v,
            BreakpointKind::RegisterX(v) => x == *v,
            BreakpointKind::RegisterY(v) => y == *v,
            BreakpointKind::Immediate(v) => matches_immediate(mem, pc, *v),
            BreakpointKind::OperandByte(v) => matches_operand_byte(mem, pc, *v),
            BreakpointKind::Mnemonic(m) => mnemonic_at(mem, pc)
                .is_some_and(|mn| mn.eq_ignore_ascii_case(m)),
        }
    }
}

/// Expand a trace click to the full breakpoint-friendly token (opcode bytes, address, etc.).
///
/// Trace text is ASCII, so byte offsets match egui character indices.
pub fn expand_trace_selection(text: &str, char_idx: usize) -> Option<(usize, usize)> {
    let (line_start, line_rel, line) = find_trace_line(text, char_idx)?;
    let (rel_start, rel_end) = expand_trace_line(line, line_rel)?;
    Some((line_start + rel_start, line_start + rel_end))
}

fn find_trace_line(text: &str, char_idx: usize) -> Option<(usize, usize, &str)> {
    let mut start = 0usize;
    for line in text.split('\n') {
        let end = start + line.len();
        if char_idx <= end {
            return Some((start, char_idx.saturating_sub(start), line));
        }
        start = end + 1;
    }
    None
}

fn expand_trace_line(line: &str, rel: usize) -> Option<(usize, usize)> {
    let parse_line = strip_repeat_suffix(line);
    if rel > parse_line.len() {
        return None;
    }
    if let Some(span) = expand_instruction_line(parse_line, rel) {
        return Some(span);
    }
    if parse_line.starts_with(';') {
        return expand_header_line(parse_line, rel);
    }
    None
}

fn strip_repeat_suffix(line: &str) -> &str {
    let Some((body, count)) = line.rsplit_once("  (x") else {
        return line;
    };
    if body.is_empty() || !count.ends_with(')') {
        return line;
    }
    let digits = &count[..count.len() - 1];
    if digits.is_empty() || !digits.chars().all(|c| c.is_ascii_digit()) {
        return line;
    }
    body
}

fn expand_instruction_line(line: &str, rel: usize) -> Option<(usize, usize)> {
    let (body, regs) = line.rsplit_once("  ; ")?;
    let regs_start = body.len() + 4;
    if rel >= regs_start {
        return expand_regs(regs, rel - regs_start).map(|(s, e)| (regs_start + s, regs_start + e));
    }

    let pc_dollar = body.find('$')?;
    let pc_end = pc_dollar + 5;
    if rel >= pc_dollar && rel < pc_end && is_hex_digits(&body[pc_dollar + 1..pc_dollar + 5]) {
        return Some((pc_dollar, pc_end));
    }

    let hex_start = pc_end + 2;
    if hex_start + 8 > body.len() {
        return None;
    }
    let hex_end = hex_start + 8;
    if rel >= hex_start && rel < hex_end {
        let rel_in_field = rel - hex_start;
        return expand_hex_field(&body[hex_start..hex_end], rel_in_field)
            .map(|(s, e)| (hex_start + s, hex_start + e));
    }

    let disasm_start = hex_end + 2;
    if rel >= disasm_start && rel < body.len() {
        return expand_disasm(&body[disasm_start..], rel - disasm_start)
            .map(|(s, e)| (disasm_start + s, disasm_start + e));
    }
    None
}

/// Per-byte selection in the fixed-width opcode hex column.
///
/// Click byte *i* selects bytes `0..=i`, except the last byte of a 3+ byte run
/// selects only that byte (e.g. `20 A4 E6` → `20`, `20 A4`, or `E6`).
fn expand_hex_field(hex_field: &str, rel_in_field: usize) -> Option<(usize, usize)> {
    let trimmed_len = hex_field.trim_end().len();
    if rel_in_field >= trimmed_len {
        return None;
    }
    let mut bytes: Vec<(usize, usize)> = Vec::new();
    let mut i = 0usize;
    while i < trimmed_len {
        if hex_field.as_bytes()[i] == b' ' {
            i += 1;
            continue;
        }
        if i + 2 > trimmed_len {
            return None;
        }
        if !hex_field[i..i + 2]
            .chars()
            .all(|c| c.is_ascii_hexdigit())
        {
            return None;
        }
        bytes.push((i, i + 2));
        i += 2;
        if i < trimmed_len && hex_field.as_bytes()[i] == b' ' {
            i += 1;
        }
    }
    if bytes.is_empty() {
        return None;
    }
    let byte_idx = bytes
        .iter()
        .position(|(start, end)| rel_in_field >= *start && rel_in_field < *end)?;
    let last_idx = bytes.len() - 1;
    if byte_idx == last_idx && bytes.len() >= 3 {
        Some(bytes[byte_idx])
    } else {
        Some((bytes[0].0, bytes[byte_idx].1))
    }
}

fn expand_regs(regs: &str, rel: usize) -> Option<(usize, usize)> {
    let mut off = 0usize;
    for part in regs.split(' ') {
        if part.is_empty() {
            continue;
        }
        let len = part.len();
        if rel >= off && rel < off + len {
            return Some((off, off + len));
        }
        off += len + 1;
    }
    None
}

fn expand_disasm(disasm: &str, rel: usize) -> Option<(usize, usize)> {
    let mut i = 0usize;
    while i < disasm.len() {
        if let Some(span) = scan_disasm_token(disasm, i) {
            if rel >= span.0 && rel < span.1 {
                return Some(span);
            }
            i = span.1;
        } else {
            i += 1;
        }
    }
    let mnemonic_end = disasm.find(' ').unwrap_or(disasm.len());
    if !disasm.is_empty() && rel < mnemonic_end {
        return Some((0, mnemonic_end));
    }
    None
}

fn scan_disasm_token(disasm: &str, i: usize) -> Option<(usize, usize)> {
    let bytes = disasm.as_bytes();
    if disasm[i..].starts_with("#$") && i + 4 <= disasm.len() {
        if bytes[i + 2].is_ascii_hexdigit() && bytes[i + 3].is_ascii_hexdigit() {
            return Some((i, i + 4));
        }
    }
    if bytes[i] == b'$' {
        let hex_len = disasm[i + 1..]
            .chars()
            .take(4)
            .take_while(|c| c.is_ascii_hexdigit())
            .count();
        if hex_len == 2 || hex_len == 4 {
            return Some((i, i + 1 + hex_len));
        }
    }
    None
}

fn expand_header_line(line: &str, rel: usize) -> Option<(usize, usize)> {
    let mut i = 0usize;
    while i < line.len() {
        if line.as_bytes()[i] == b'$' {
            let hex_len = line[i + 1..]
                .chars()
                .take(4)
                .take_while(|c| c.is_ascii_hexdigit())
                .count();
            if hex_len == 2 || hex_len == 4 {
                let end = i + 1 + hex_len;
                if rel >= i && rel < end {
                    return Some((i, end));
                }
                i = end;
                continue;
            }
        }
        i += 1;
    }
    None
}

fn is_hex_digits(s: &str) -> bool {
    s.len() == 4 && s.chars().all(|c| c.is_ascii_hexdigit())
}

/// Parse a trace text selection into a breakpoint, if recognizable.
pub fn parse_trace_selection(sel: &str) -> Option<TraceBreakpoint> {
    let mut s = sel.trim();
    if s.is_empty() {
        return None;
    }
    if let Some(rest) = s.strip_prefix(';') {
        s = rest.trim();
    }
    if let Some(rest) = s.strip_prefix("->") {
        s = rest.trim();
    }

    if let Some(bp) = parse_register(s) {
        return Some(bp);
    }
    if let Some(bp) = parse_immediate(s) {
        return Some(bp);
    }
    if let Some(bp) = parse_hex_address(s) {
        return Some(bp);
    }
    if let Some(bp) = parse_opcode_bytes(s) {
        return Some(bp);
    }
    parse_mnemonic(s)
}

fn parse_register(s: &str) -> Option<TraceBreakpoint> {
    let upper = s.to_ascii_uppercase();
    for (tag, reg) in [("A=", 'A'), ("X=", 'X'), ("Y=", 'Y')] {
        let Some(rest) = upper.strip_prefix(tag) else {
            continue;
        };
        let rest = rest.strip_prefix('$').unwrap_or(rest);
        let value = parse_hex_byte(rest)?;
        let kind = match reg {
            'A' => BreakpointKind::RegisterA(value),
            'X' => BreakpointKind::RegisterX(value),
            'Y' => BreakpointKind::RegisterY(value),
            _ => unreachable!(),
        };
        return Some(TraceBreakpoint {
            label: format!("{reg}=${value:02X}"),
            kind,
        });
    }
    None
}

fn parse_immediate(s: &str) -> Option<TraceBreakpoint> {
    let upper = s.to_ascii_uppercase();
    let rest = upper.strip_prefix("#$").or_else(|| upper.strip_prefix('#'))?;
    let value = parse_hex_byte(rest)?;
    Some(TraceBreakpoint {
        label: format!("#${value:02X}"),
        kind: BreakpointKind::Immediate(value),
    })
}

fn parse_hex_address(s: &str) -> Option<TraceBreakpoint> {
    let upper = s.to_ascii_uppercase();
    let hex = upper.strip_prefix('$').unwrap_or(&upper);
    if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    match hex.len() {
        4 => {
            let addr = parse_hex_word(hex)?;
            Some(TraceBreakpoint {
                label: format!("${addr:04X}"),
                kind: BreakpointKind::Pc(addr),
            })
        }
        2 => {
            let byte = parse_hex_byte(hex)?;
            Some(TraceBreakpoint {
                label: format!("${byte:02X}"),
                kind: BreakpointKind::OperandByte(byte),
            })
        }
        _ => None,
    }
}

fn parse_opcode_bytes(s: &str) -> Option<TraceBreakpoint> {
    let upper = s.to_ascii_uppercase();
    if upper.contains('$') || upper.contains('=') || upper.contains('#') {
        return None;
    }
    let parts: Vec<&str> = upper.split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }
    let mut bytes = Vec::with_capacity(parts.len());
    for part in parts {
        if part.len() != 2 || !part.chars().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }
        bytes.push(parse_hex_byte(part)?);
    }
    let label = bytes
        .iter()
        .map(|b| format!("{b:02X}"))
        .collect::<Vec<_>>()
        .join(" ");
    Some(TraceBreakpoint {
        label,
        kind: BreakpointKind::OpcodeBytes(bytes),
    })
}

fn parse_mnemonic(s: &str) -> Option<TraceBreakpoint> {
    let upper = s.to_ascii_uppercase();
    if upper.len() < 2 || upper.len() > 5 {
        return None;
    }
    if !upper.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    Some(TraceBreakpoint {
        label: upper.clone(),
        kind: BreakpointKind::Mnemonic(upper),
    })
}

fn parse_hex_byte(s: &str) -> Option<u8> {
    if s.len() != 2 || !s.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    u8::from_str_radix(s, 16).ok()
}

fn parse_hex_word(s: &str) -> Option<u16> {
    if s.len() != 4 || !s.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    u16::from_str_radix(s, 16).ok()
}

fn instruction_len(opcode: u8) -> usize {
    Cmos6502::decode(opcode)
        .map(|(_, mode)| 1 + mode.extra_bytes() as usize)
        .unwrap_or(1)
}

fn matches_opcode_bytes(mem: &[u8; 65536], pc: u16, bytes: &[u8]) -> bool {
    bytes.iter().enumerate().all(|(i, &want)| {
        mem[pc.wrapping_add(i as u16) as usize] == want
    })
}

fn matches_immediate(mem: &[u8; 65536], pc: u16, value: u8) -> bool {
    let opcode = mem[pc as usize];
    let Some((_, mode)) = Cmos6502::decode(opcode) else {
        return false;
    };
    if !matches!(mode, AddressingMode::Immediate) {
        return false;
    }
    mem[pc.wrapping_add(1) as usize] == value
}

fn matches_operand_byte(mem: &[u8; 65536], pc: u16, value: u8) -> bool {
    let opcode = mem[pc as usize];
    let len = instruction_len(opcode).min(3);
    (1..len).any(|i| mem[pc.wrapping_add(i as u16) as usize] == value)
}

fn mnemonic_at(mem: &[u8; 65536], pc: u16) -> Option<String> {
    let opcode = mem[pc as usize];
    Cmos6502::decode(opcode).map(|(instr, _)| instr_mnemonic(instr))
}

fn instr_mnemonic(instr: Instruction) -> String {
    let raw = format!("{instr:?}");
    raw.split('(').next().unwrap_or("?").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pc_address() {
        let bp = parse_trace_selection("$E617").unwrap();
        assert_eq!(bp.kind, BreakpointKind::Pc(0xE617));
    }

    #[test]
    fn parse_register_a() {
        let bp = parse_trace_selection("A=$02").unwrap();
        assert_eq!(bp.kind, BreakpointKind::RegisterA(0x02));
    }

    #[test]
    fn parse_opcode_run() {
        let bp = parse_trace_selection("20 A4 E6").unwrap();
        assert_eq!(
            bp.kind,
            BreakpointKind::OpcodeBytes(vec![0x20, 0xA4, 0xE6])
        );
    }

    #[test]
    fn parse_immediate_hash() {
        let bp = parse_trace_selection("#$0F").unwrap();
        assert_eq!(bp.kind, BreakpointKind::Immediate(0x0F));
    }

    #[test]
    fn parse_operand_zp() {
        let bp = parse_trace_selection("$75").unwrap();
        assert_eq!(bp.kind, BreakpointKind::OperandByte(0x75));
    }

    #[test]
    fn parse_mnemonic_jsr() {
        let bp = parse_trace_selection("JSR").unwrap();
        assert_eq!(bp.kind, BreakpointKind::Mnemonic("JSR".into()));
    }

    #[test]
    fn expand_opcode_hex_byte_click() {
        let line = "[ROM] $E617  A9 00     LDA #$00  ; A=$02 X=$80 Y=$00";
        let rel = line.find("00").unwrap();
        let (s, e) = expand_trace_selection(line, rel).unwrap();
        assert_eq!(&line[s..e], "A9 00");
    }

    #[test]
    fn expand_pc_address_click() {
        let line = "[ROM] $E617  A9 00     LDA #$00  ; A=$02 X=$80 Y=$00";
        let rel = line.find("E6").unwrap();
        let (s, e) = expand_trace_selection(line, rel).unwrap();
        assert_eq!(&line[s..e], "$E617");
    }

    #[test]
    fn expand_immediate_in_disasm() {
        let line = "[ROM] $E617  A9 00     LDA #$00  ; A=$02 X=$80 Y=$00";
        let rel = line.find("#$00").unwrap() + 3;
        let (s, e) = expand_trace_selection(line, rel).unwrap();
        assert_eq!(&line[s..e], "#$00");
    }

    #[test]
    fn expand_register_a_click() {
        let line = "[ROM] $E617  A9 00     LDA #$00  ; A=$02 X=$80 Y=$00";
        let rel = line.find("A=$02").unwrap() + 4;
        let (s, e) = expand_trace_selection(line, rel).unwrap();
        assert_eq!(&line[s..e], "A=$02");
    }

    #[test]
    fn expand_three_byte_opcode_first_byte() {
        let line = "[ROM] $E622  20 A4 E6  JSR $E6A4  ; A=$00 X=$00 Y=$00";
        let rel = line.find("20").unwrap();
        let (s, e) = expand_trace_selection(line, rel).unwrap();
        assert_eq!(&line[s..e], "20");
    }

    #[test]
    fn expand_three_byte_opcode_second_byte() {
        let line = "[ROM] $E622  20 A4 E6  JSR $E6A4  ; A=$00 X=$00 Y=$00";
        let rel = line.find("A4").unwrap();
        let (s, e) = expand_trace_selection(line, rel).unwrap();
        assert_eq!(&line[s..e], "20 A4");
    }

    #[test]
    fn expand_three_byte_opcode_last_byte() {
        let line = "[ROM] $E622  20 A4 E6  JSR $E6A4  ; A=$00 X=$00 Y=$00";
        let rel = line.find("20 A4 E6").unwrap() + "20 A4 ".len();
        let (s, e) = expand_trace_selection(line, rel).unwrap();
        assert_eq!(&line[s..e], "E6");
    }

    #[test]
    fn expand_two_byte_opcode_last_byte() {
        let line = "[ROM] $E617  A9 00     LDA #$00  ; A=$02 X=$80 Y=$00";
        let rel = line.find("00").unwrap();
        let (s, e) = expand_trace_selection(line, rel).unwrap();
        assert_eq!(&line[s..e], "A9 00");
    }

    #[test]
    fn matches_pc() {
        let bp = TraceBreakpoint {
            label: "$E000".into(),
            kind: BreakpointKind::Pc(0xE000),
        };
        let mem = [0_u8; 65536];
        assert!(bp.matches(&mem, 0xE000, 0, 0, 0));
        assert!(!bp.matches(&mem, 0xE001, 0, 0, 0));
    }
}