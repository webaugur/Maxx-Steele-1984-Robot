use crate::emit::{CART_SIZE, COPYRIGHT_CBS, COPYRIGHT_ULTRAMAXX, PROG_OFF};

pub fn validate_cart(data: &[u8], base_addr: u16) -> Vec<String> {
    let mut issues = Vec::new();

    if data.len() != CART_SIZE {
        issues.push(format!("expected {CART_SIZE} bytes, got {}", data.len()));
        return issues;
    }

    let copyright = &data[2..19];
    if copyright != COPYRIGHT_CBS.as_slice() && copyright != COPYRIGHT_ULTRAMAXX.as_slice() {
        issues.push(format!("copyright mismatch: {:?}", String::from_utf8_lossy(copyright)));
    }

    let entry_vector = u16::from_le_bytes([data[0], data[1]]);
    let entry_off = entry_vector as i32 - base_addr as i32;
    if entry_off < 0 || entry_off as usize >= CART_SIZE {
        issues.push(format!(
            "entry vector ${entry_vector:04X} out of range"
        ));
    } else {
        let chunk = &data[entry_off as usize..entry_off as usize + 4];
        if chunk != [0xA9, 0x02, 0x85, 0x02] {
            issues.push(format!(
                "unexpected entry code at ${entry_vector:04X}: {}",
                hex_encode(chunk)
            ));
        }
    }

    if let Some(prog_start) = find_program_table(data) {
        let prog = decode_program(data, prog_start);
        if prog.is_empty() || prog.last().map(|(_, op, operand, _)| (*op, *operand)) != Some((0xFF, 0xFF)) {
            issues.push("program table missing FF FF terminator".into());
        }
    } else {
        issues.push("could not locate program table".into());
    }

    issues
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn find_program_table(data: &[u8]) -> Option<usize> {
    if data.get(PROG_OFF)? != &0xFF || data.get(PROG_OFF + 1)? != &0xFF {
        return Some(PROG_OFF);
    }

    for marker in [[0x83, 0x10], [0x0C, 0x02], [0x81, 0x00], [0xFF, 0xFF]] {
        if let Some(idx) = data.windows(2).position(|w| w == marker) {
            if (0x20..0x200).contains(&idx) {
                return Some(idx);
            }
        }
    }

    None
}

fn decode_program(data: &[u8], start: usize) -> Vec<(usize, u8, u8, String)> {
    let mut lines = Vec::new();
    let mut i = start;

    while i + 1 < data.len() {
        let op = data[i];
        let operand = data[i + 1];
        let comment = opcode_comment(op, operand);
        lines.push((i, op, operand, comment));
        if op == 0xFF && operand == 0xFF {
            break;
        }
        i += 2;
    }

    lines
}

fn opcode_comment(op: u8, operand: u8) -> String {
    match op {
        0x0C => format!(
            "delay {operand} second{}",
            if operand == 1 { "" } else { "s" }
        ),
        0x0A => {
            if operand == 0 {
                "turn light off".into()
            } else {
                "turn light on".into()
            }
        }
        0x81 => format!("play tune #{operand}"),
        0x82 | 0x83 | 0x0E => format!("speak phrase #{operand}"),
        0xFF if operand == 0xFF => "end".into(),
        0x01 => "drive forward".into(),
        0x02 => "drive reverse".into(),
        0x00 => "turn left".into(),
        0x03 => "turn right".into(),
        0x0B => "home".into(),
        _ => format!("opcode {op:02X}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emit::{compile_source, Copyright};

    #[test]
    fn hello_bas_validates() {
        let src = include_str!(
            "../../../Cartridge/Examples/UltraMaxx/Firmware/Basic/hello.bas"
        );
        let image = compile_source(src, Copyright::UltraMaxx).unwrap();
        assert_eq!(validate_cart(&image, 0xA000), Vec::<String>::new());
    }
}