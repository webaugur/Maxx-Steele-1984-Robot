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
    let program = crate::parse::parse_source(text)?;
    emit_cart(
        &program,
        EmitOptions {
            copyright,
            ..Default::default()
        },
    )
}

pub fn format_listing(program: &[Instruction]) -> String {
    let mut lines = vec!["; MaxxBAS compiled program".to_string()];
    for insn in program {
        lines.push(format!("; line {}: {}", insn.source_line, insn.text));
        lines.push(format!("{:02X} {:02X}", insn.opcode, insn.operand));
    }
    lines.join("\n")
}