use serde::Serialize;

use crate::cart::CartImage;
use crate::emit::PROG_OFF;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum StepKind {
    Delay {
        seconds: u8,
    },
    Forward {
        distance: u8,
    },
    Back {
        distance: u8,
    },
    Left {
        distance: u8,
    },
    Right {
        angle: u8,
    },
    WristUp {
        value: u8,
    },
    WristDown {
        value: u8,
    },
    ArmsUp {
        value: u8,
    },
    ArmsDown {
        value: u8,
    },
    ClawRotate {
        value: u8,
    },
    ClawOpenClose {
        close: bool,
    },
    Lamp {
        on: bool,
    },
    Home,
    Play {
        tune: u8,
    },
    SpeakRom {
        phrase: u8,
    },
    SpeakRam {
        phrase: u8,
    },
    Unknown {
        opcode: u8,
        operand: u8,
    },
    End,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ProgramStep {
    pub index: usize,
    pub rom_addr: u16,
    pub opcode: u8,
    pub operand: u8,
    pub kind: StepKind,
    pub comment: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProgramTrace {
    pub base_addr: u16,
    pub program_addr: u16,
    pub copyright: String,
    pub steps: Vec<ProgramStep>,
}

pub fn find_program_table(data: &[u8]) -> Option<usize> {
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

pub fn decode_cart(cart: &CartImage) -> Result<ProgramTrace, String> {
    let start = find_program_table(&cart.data).ok_or("could not locate program table")?;
    let steps = decode_program(&cart.data, cart.base_addr, start);
    if steps.is_empty() {
        return Err("program table is empty".into());
    }
    Ok(ProgramTrace {
        base_addr: cart.base_addr,
        program_addr: cart.base_addr + start as u16,
        copyright: cart.copyright_str(),
        steps,
    })
}

pub fn decode_program(data: &[u8], base_addr: u16, start: usize) -> Vec<ProgramStep> {
    let mut steps = Vec::new();
    let mut i = start;
    let mut index = 0;

    while i + 1 < data.len() {
        let opcode = data[i];
        let operand = data[i + 1];
        let kind = classify_opcode(opcode, operand);
        let comment = step_comment(&kind);
        steps.push(ProgramStep {
            index,
            rom_addr: base_addr + i as u16,
            opcode,
            operand,
            kind,
            comment,
        });
        if opcode == 0xFF && operand == 0xFF {
            break;
        }
        i += 2;
        index += 1;
    }

    steps
}

fn classify_opcode(opcode: u8, operand: u8) -> StepKind {
    match opcode {
        0x00 => StepKind::Left { distance: operand },
        0x01 => StepKind::Forward { distance: operand },
        0x02 => StepKind::Back { distance: operand },
        0x03 => StepKind::Right { angle: operand },
        0x04 => StepKind::WristUp { value: operand },
        0x05 => StepKind::WristDown { value: operand },
        0x06 => StepKind::ArmsUp { value: operand },
        0x07 => StepKind::ArmsDown { value: operand },
        0x08 => StepKind::ClawRotate { value: operand },
        0x09 => StepKind::ClawOpenClose { close: operand != 0 },
        0x0A => StepKind::Lamp { on: operand != 0 },
        0x0B => StepKind::Home,
        0x0C => StepKind::Delay { seconds: operand },
        0x81 => StepKind::Play { tune: operand },
        0x82 | 0x0E => StepKind::SpeakRom { phrase: operand },
        0x83 | 0x0F | 0x10 => StepKind::SpeakRam { phrase: operand },
        0xFF if operand == 0xFF => StepKind::End,
        _ => StepKind::Unknown { opcode, operand },
    }
}

fn step_comment(kind: &StepKind) -> String {
    match kind {
        StepKind::Delay { seconds } => format!(
            "delay {seconds} second{}",
            if *seconds == 1 { "" } else { "s" }
        ),
        StepKind::Forward { distance } => format!("forward {distance}"),
        StepKind::Back { distance } => format!("back {distance}"),
        StepKind::Left { distance } => format!("left {distance}"),
        StepKind::Right { angle } => format!("right {angle}"),
        StepKind::WristUp { value } => format!("wrist up {value}"),
        StepKind::WristDown { value } => format!("wrist down {value}"),
        StepKind::ArmsUp { value } => format!("arms up {value}"),
        StepKind::ArmsDown { value } => format!("arms down {value}"),
        StepKind::ClawRotate { value } => format!("claw rotate {value}"),
        StepKind::ClawOpenClose { close } => {
            if *close {
                "claw close".into()
            } else {
                "claw open".into()
            }
        }
        StepKind::Lamp { on } => {
            if *on {
                "lamp on".into()
            } else {
                "lamp off".into()
            }
        }
        StepKind::Home => "home".into(),
        StepKind::Play { tune } => format!("play tune #{tune}"),
        StepKind::SpeakRom { phrase } => format!("speak ROM phrase #{phrase}"),
        StepKind::SpeakRam { phrase } => format!("speak RAM phrase #{phrase}"),
        StepKind::Unknown { opcode, operand } => format!("unknown {opcode:02X} {operand:02X}"),
        StepKind::End => "end".into(),
    }
}

pub fn format_rom_listing(trace: &ProgramTrace) -> String {
    let mut out = vec![
        format!("// Cartridge image @ ${:04X}", trace.base_addr),
        format!("// Copyright: {}", trace.copyright),
        format!(
            "// Program table @ ${:04X} -> RAM $0200",
            trace.program_addr
        ),
        String::new(),
    ];

    for step in &trace.steps {
        out.push(format!(
            "{:06X}: {:02X} {:02X}  {}",
            step.rom_addr, step.opcode, step.operand, step.comment
        ));
    }

    out.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emit::{compile_source, Copyright};

    #[test]
    fn hello_trace_has_motion_steps() {
        let src = include_str!("../../../Cartridge/Examples/UltraMaxx/Firmware/Basic/hello.bas");
        let image = compile_source(src, Copyright::UltraMaxx).unwrap();
        let cart = CartImage::from_bytes(image).unwrap();
        let trace = decode_cart(&cart).unwrap();
        assert!(trace.steps.iter().any(|s| matches!(s.kind, StepKind::Forward { .. })));
        assert!(trace.steps.last().unwrap().kind == StepKind::End);
    }
}