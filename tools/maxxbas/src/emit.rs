use crate::error::CompileError;
use crate::parse::{program_bytes, Instruction};

pub const CART_SIZE: usize = 4096;
pub const ENTRY_ADDR: u16 = 0xA013;
pub const ENTRY_OFF: usize = 0x0013;
pub const PROG_OFF: usize = 0x35;
pub const PHRASE_OFF: usize = 0x81;
pub const MUSIC_OFF: usize = 0xBB;

pub const COPYRIGHT_CBS: &[u8; 17] = b"(c) 1985 CBS Toys";
pub const COPYRIGHT_ULTRAMAXX: &[u8; 17] = b"(c) UltraMaxx    ";

const BOOTSTRAP_STUB: [u8; 34] = [
    0xA9, 0x02, 0x85, 0x02, 0xA9, 0x82, 0x85, 0x03, 0xA2, 0x00, 0xBD, 0x35, 0xA0, 0x9D, 0x00,
    0x02, 0xBD, 0x81, 0xA0, 0x9D, 0x00, 0x05, 0xBD, 0xBB, 0xA0, 0x9D, 0x00, 0x04, 0xE8, 0xD0, 0xEB,
    0x4C, 0xB6, 0xE0,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Copyright {
    Cbs,
    UltraMaxx,
}

impl Copyright {
    pub fn as_bytes(self) -> &'static [u8; 17] {
        match self {
            Copyright::Cbs => COPYRIGHT_CBS,
            Copyright::UltraMaxx => COPYRIGHT_ULTRAMAXX,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "cbs" => Some(Copyright::Cbs),
            "ultramaxx" => Some(Copyright::UltraMaxx),
            _ => None,
        }
    }
}

pub struct EmitOptions<'a> {
    pub copyright: Copyright,
    pub phrase_table: Option<&'a [u8]>,
    pub music_table: Option<&'a [u8]>,
}

impl Default for EmitOptions<'_> {
    fn default() -> Self {
        Self {
            copyright: Copyright::UltraMaxx,
            phrase_table: None,
            music_table: None,
        }
    }
}

pub fn emit_cart(program: &[Instruction], options: EmitOptions<'_>) -> Result<Vec<u8>, CompileError> {
    let copyright = options.copyright.as_bytes();
    if copyright.len() != 17 {
        return Err(CompileError::CopyrightLength {
            len: copyright.len(),
        });
    }

    let mut img = vec![0xFF; CART_SIZE];
    img[0] = (ENTRY_ADDR & 0xFF) as u8;
    img[1] = (ENTRY_ADDR >> 8) as u8;
    img[2..19].copy_from_slice(copyright);
    img[ENTRY_OFF..ENTRY_OFF + BOOTSTRAP_STUB.len()].copy_from_slice(&BOOTSTRAP_STUB);

    let prog = program_bytes(program)?;
    img[PROG_OFF..PROG_OFF + prog.len()].copy_from_slice(&prog);

    let phrase_default_len = MUSIC_OFF - PHRASE_OFF;
    let music_default_len = CART_SIZE - MUSIC_OFF;

    let phrases = options.phrase_table.unwrap_or(&[]);
    let music = options.music_table.unwrap_or(&[]);

    for i in 0..phrase_default_len {
        img[PHRASE_OFF + i] = phrases.get(i).copied().unwrap_or(0xFF);
    }
    for i in 0..music_default_len {
        img[MUSIC_OFF + i] = music.get(i).copied().unwrap_or(0x00);
    }

    Ok(img)
}

pub fn compile_source(text: &str, copyright: Copyright) -> Result<Vec<u8>, CompileError> {
    compile_source_with_tables(text, copyright, None, None)
}

pub fn compile_source_with_tables(
    text: &str,
    copyright: Copyright,
    phrase_table: Option<&[u8]>,
    music_table: Option<&[u8]>,
) -> Result<Vec<u8>, CompileError> {
    let program = crate::parse::parse_source(text)?;
    emit_cart(
        &program,
        EmitOptions {
            copyright,
            phrase_table,
            music_table,
            ..Default::default()
        },
    )
}

/// Bytes the robot copies from `$A035` before the phrase table at `$A081`.
/// The keypad recorder also refuses a program that would pass `$037F`.
pub fn program_slot_bytes(pairs: &[(u8, u8)]) -> Result<Vec<u8>, CompileError> {
    let mut insns = Vec::new();
    for (i, &(opcode, operand)) in pairs.iter().enumerate() {
        if opcode == 0xFF && operand == 0xFF {
            continue;
        }
        insns.push(Instruction {
            opcode,
            operand,
            source_line: i + 1,
            text: String::new(),
        });
    }
    insns.push(Instruction {
        opcode: 0xFF,
        operand: 0xFF,
        source_line: 0,
        text: "END".into(),
    });
    let data = program_bytes(&insns)?;
    let last = 0x0200 + data.len() - 1;
    if last > 0x037F {
        return Err(CompileError::ProgramTooLarge {
            pairs: data.len() / 2,
            bytes: data.len(),
            max_pairs: (0x037F - 0x0200 + 1) / 2,
        });
    }
    Ok(data)
}

/// Rebuild a cartridge from edited pairs. Copyright, copy stub, speech table,
/// and music table stay. The program is the bytes `emit_cart` writes at `$A035`.
pub fn emit_edited_cart(original: &[u8], pairs: &[(u8, u8)]) -> Result<Vec<u8>, CompileError> {
    if original.len() != CART_SIZE {
        return Err(CompileError::CopyrightLength {
            len: original.len(),
        });
    }
    let header = &original[2..19];
    let copyright = if header == COPYRIGHT_CBS.as_slice() {
        Copyright::Cbs
    } else if header == COPYRIGHT_ULTRAMAXX.as_slice() {
        Copyright::UltraMaxx
    } else {
        return Err(CompileError::CopyrightLength { len: header.len() });
    };
    let phrases = &original[PHRASE_OFF..MUSIC_OFF];
    let music = &original[MUSIC_OFF..];
    let slot = program_slot_bytes(pairs)?;
    let mut insns = Vec::new();
    for (i, chunk) in slot.chunks(2).enumerate() {
        insns.push(Instruction {
            opcode: chunk[0],
            operand: chunk[1],
            source_line: i + 1,
            text: String::new(),
        });
    }
    let img = emit_cart(
        &insns,
        EmitOptions {
            copyright,
            phrase_table: Some(phrases),
            music_table: Some(music),
        },
    )?;
    Ok(img)
}

pub fn format_listing(program: &[Instruction]) -> String {
    let mut lines = vec!["; MaxxBAS compiled program".to_string()];
    for insn in program {
        lines.push(format!("; line {}: {}", insn.source_line, insn.text));
        lines.push(format!("{:02X} {:02X}", insn.opcode, insn.operand));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cart::CartImage;
    use crate::decode::decode_cart;

    #[test]
    fn cbsdemo_noop_edit_keeps_program_bytes() {
        let raw = include_bytes!(
            "../../../Cartridge/Examples/CBSDemo/Firmware/Binary/CBSDemo.532"
        );
        let cart = CartImage::from_bytes(raw.to_vec()).unwrap();
        let trace = decode_cart(&cart).unwrap();
        let pairs: Vec<(u8, u8)> = trace.steps.iter().map(|s| (s.opcode, s.operand)).collect();
        let edited = emit_edited_cart(raw, &pairs).unwrap();
        let start = PROG_OFF;
        let end = start
            + pairs.len() * 2;
        assert_eq!(&edited[start..end], &raw[start..end]);
        assert_eq!(&edited[2..19], &raw[2..19]);
        assert_eq!(&edited[PHRASE_OFF..MUSIC_OFF], &raw[PHRASE_OFF..MUSIC_OFF]);
        assert_eq!(&edited[MUSIC_OFF..], &raw[MUSIC_OFF..]);
        assert_eq!(
            &edited[ENTRY_OFF..ENTRY_OFF + 34],
            &raw[ENTRY_OFF..ENTRY_OFF + 34]
        );
    }

    #[test]
    fn program_past_phrase_table_is_refused() {
        let pairs = vec![(0x0C, 0x01); (PHRASE_OFF - PROG_OFF) / 2];
        let err = program_slot_bytes(&pairs).unwrap_err();
        assert!(matches!(err, CompileError::ProgramTooLarge { .. }));
    }
}