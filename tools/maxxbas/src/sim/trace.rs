//! Rolling 65C02 instruction trace for the live simulator GUI.

use mos6502::instruction::{AddressingMode, Cmos6502, Instruction};
use mos6502::Variant;

const TRACE_CAP: usize = 400;

#[derive(Debug, Clone, Copy)]
struct TraceEntry {
    pc: u16,
    len: u8,
    bytes: [u8; 3],
    a: u8,
    x: u8,
    y: u8,
}

#[derive(Debug, Clone)]
pub struct TraceBuffer {
    entries: [TraceEntry; TRACE_CAP],
    head: usize,
    len: usize,
}

impl Default for TraceBuffer {
    fn default() -> Self {
        Self {
            entries: [TraceEntry {
                pc: 0,
                len: 0,
                bytes: [0; 3],
                a: 0,
                x: 0,
                y: 0,
            }; TRACE_CAP],
            head: 0,
            len: 0,
        }
    }
}

impl TraceBuffer {
    pub fn clear(&mut self) {
        self.head = 0;
        self.len = 0;
    }

    pub fn record(&mut self, mem: &[u8; 65536], pc: u16, a: u8, x: u8, y: u8) {
        let opcode = mem[pc as usize];
        let ilen = instruction_len(opcode).min(3) as u8;
        let mut bytes = [0_u8; 3];
        for i in 0..ilen as usize {
            bytes[i] = mem[pc.wrapping_add(i as u16) as usize];
        }
        let entry = TraceEntry {
            pc,
            len: ilen,
            bytes,
            a,
            x,
            y,
        };
        self.entries[self.head] = entry;
        self.head = (self.head + 1) % TRACE_CAP;
        if self.len < TRACE_CAP {
            self.len += 1;
        }
    }

    pub fn format_lines(&self, mem: &[u8; 65536]) -> String {
        if self.len == 0 {
            return String::new();
        }
        let ordered = self.ordered_entries();
        let mut out = String::new();
        let mut i = 0;
        while i < ordered.len() {
            let entry = ordered[i];
            let line = format_entry(mem, entry);
            let mut repeat = 1_usize;
            while i + repeat < ordered.len() {
                let next = ordered[i + repeat];
                if next.pc == entry.pc && next.bytes == entry.bytes {
                    repeat += 1;
                } else {
                    break;
                }
            }
            if repeat > 1 {
                out.push_str(&format!("{line}  (x{repeat})\n"));
            } else {
                out.push_str(&line);
                out.push('\n');
            }
            i += repeat;
        }
        out
    }
}

fn instruction_len(opcode: u8) -> usize {
    Cmos6502::decode(opcode)
        .map(|(_, mode)| 1 + mode.extra_bytes() as usize)
        .unwrap_or(1)
}

fn format_entry(mem: &[u8; 65536], entry: TraceEntry) -> String {
    let region = region_tag(entry.pc);
    let hex = format_hex_bytes(&entry.bytes, entry.len);
    let disasm = disasm_at(mem, entry.pc, entry.x, entry.y);
    format!(
        "{region}${:04X}  {hex:<8}  {disasm}  ; A=${:02X} X=${:02X} Y=${:02X}",
        entry.pc, entry.a, entry.x, entry.y
    )
}

fn region_tag(pc: u16) -> &'static str {
    if pc >= 0xE000 {
        "[ROM] "
    } else if pc >= 0xA000 {
        "[CART] "
    } else if pc >= 0x0200 {
        "[RAM] "
    } else {
        "[ZP] "
    }
}

fn format_hex_bytes(bytes: &[u8; 3], len: u8) -> String {
    let mut s = String::new();
    for i in 0..len as usize {
        if i > 0 {
            s.push(' ');
        }
        s.push_str(&format!("{:02X}", bytes[i]));
    }
    s
}

fn read_u16(mem: &[u8; 65536], addr: u16) -> u16 {
    let lo = mem[addr as usize];
    let hi = mem[addr.wrapping_add(1) as usize];
    u16::from_le_bytes([lo, hi])
}

fn disasm_at(mem: &[u8; 65536], pc: u16, x: u8, y: u8) -> String {
    let opcode = mem[pc as usize];
    let Some((instr, mode)) = Cmos6502::decode(opcode) else {
        return format!(".byte ${opcode:02X}");
    };
    let mnemonic = instr_mnemonic(instr);
    let operand = format_operand(mode, mem, pc, x, y);
    if operand.is_empty()
        || matches!(
            mode,
            AddressingMode::Implied | AddressingMode::Accumulator
        )
    {
        mnemonic
    } else {
        format!("{mnemonic} {operand}")
    }
}

fn format_operand(mode: AddressingMode, mem: &[u8; 65536], pc: u16, x: u8, y: u8) -> String {
    let b1 = mem[pc.wrapping_add(1) as usize];
    match mode {
        AddressingMode::Accumulator | AddressingMode::Implied => String::new(),
        AddressingMode::Immediate => format!("#${b1:02X}"),
        AddressingMode::ZeroPage => format!("${b1:02X}"),
        AddressingMode::ZeroPageX => format!("${:02X},X", b1.wrapping_add(x)),
        AddressingMode::ZeroPageY => format!("${:02X},Y", b1.wrapping_add(y)),
        AddressingMode::Relative => {
            let target = pc.wrapping_add(2).wrapping_add(b1 as u16);
            format!("${target:04X}")
        }
        AddressingMode::Absolute => format!("${:04X}", read_u16(mem, pc.wrapping_add(1))),
        AddressingMode::AbsoluteX => {
            format!("${:04X},X", read_u16(mem, pc.wrapping_add(1)).wrapping_add(x as u16))
        }
        AddressingMode::AbsoluteY => {
            format!("${:04X},Y", read_u16(mem, pc.wrapping_add(1)).wrapping_add(y as u16))
        }
        AddressingMode::Indirect | AddressingMode::BuggyIndirect => {
            format!("(${:04X})", read_u16(mem, pc.wrapping_add(1)))
        }
        AddressingMode::IndexedIndirectX => format!("(${:02X},X)", b1.wrapping_add(x)),
        AddressingMode::IndirectIndexedY => {
            let zp = b1;
            let base = read_u16(mem, u16::from(zp));
            format!(
                "(${zp:02X}),Y  ;->${:04X}",
                base.wrapping_add(y as u16)
            )
        }
        AddressingMode::ZeroPageIndirect => format!("(${b1:02X})"),
        AddressingMode::AbsoluteIndexedIndirect => {
            format!("(${:04X},X)", read_u16(mem, pc.wrapping_add(1)).wrapping_add(x as u16))
        }
        AddressingMode::ZeroPageRelative => {
            let b2 = mem[pc.wrapping_add(2) as usize];
            let target = pc.wrapping_add(3).wrapping_add(b2 as u16);
            format!("${b1:02X},${target:04X}")
        }
    }
}

fn instr_mnemonic(instr: Instruction) -> String {
    let raw = format!("{instr:?}");
    raw.split('(').next().unwrap_or("?").to_string()
}

impl TraceBuffer {
    fn ordered_entries(&self) -> Vec<TraceEntry> {
        let mut out = Vec::with_capacity(self.len);
        let start = if self.len < TRACE_CAP {
            0
        } else {
            self.head
        };
        for i in 0..self.len {
            out.push(self.entries[(start + i) % TRACE_CAP]);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disasm_ldx_abs() {
        let mut mem = [0_u8; 65536];
        mem[0xE6AC] = 0xA6; // LDX zp
        mem[0xE6AD] = 0x75;
        assert_eq!(disasm_at(&mem, 0xE6AC, 0, 0), "LDX $75");
    }

    #[test]
    fn trace_collapses_tight_loops() {
        let mut mem = [0_u8; 65536];
        mem[0xE617] = 0x20; // JSR abs
        mem[0xE618] = 0xA4;
        mem[0xE619] = 0xE6;
        let mut trace = TraceBuffer::default();
        for _ in 0..5 {
            trace.record(&mem, 0xE617, 0x80, 0x80, 0);
        }
        let text = trace.format_lines(&mem);
        assert!(text.contains("(x5)"), "expected collapsed loop, got: {text}");
    }
}