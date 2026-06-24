use crate::error::CompileError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub opcode: u8,
    pub operand: u8,
    pub source_line: usize,
    pub text: String,
}

impl Instruction {
    pub fn as_bytes(&self) -> [u8; 2] {
        [self.opcode, self.operand]
    }
}

pub fn parse_source(text: &str) -> Result<Vec<Instruction>, CompileError> {
    let mut program = Vec::new();

    for (line_no, raw) in text.lines().enumerate() {
        if let Some(insn) = parse_line(raw, line_no + 1)? {
            program.push(insn);
        }
    }

    if program.is_empty() {
        return Err(CompileError::EmptyProgram);
    }

    if program.last().map(|i| i.opcode) != Some(0xFF) {
        program.push(Instruction {
            opcode: 0xFF,
            operand: 0xFF,
            source_line: 0,
            text: "END (implicit)".into(),
        });
    } else if program.last().map(|i| i.operand) != Some(0xFF) {
        let line = program.last().unwrap().source_line;
        return Err(CompileError::Line {
            line,
            message: "END must be sole statement or last line".into(),
        });
    }

    Ok(program)
}

pub fn program_bytes(program: &[Instruction]) -> Result<Vec<u8>, CompileError> {
    const MAX_PROG_BYTES: usize = crate::emit::PHRASE_OFF - crate::emit::PROG_OFF;

    let mut data = Vec::with_capacity(program.len() * 2);
    for insn in program {
        data.extend_from_slice(&insn.as_bytes());
    }

    if data.len() > MAX_PROG_BYTES {
        return Err(CompileError::ProgramTooLarge {
            pairs: data.len() / 2,
            bytes: data.len(),
            max_pairs: MAX_PROG_BYTES / 2,
        });
    }

    Ok(data)
}

fn strip_comments(line: &str) -> String {
    let mut line = line;
    if let Some(idx) = line.find('#') {
        line = &line[..idx];
    }

    let trimmed = line.trim_start();
    if trimmed.len() >= 3 && trimmed[..3].eq_ignore_ascii_case("REM") {
        let rest = trimmed.get(3..).unwrap_or("");
        if rest.is_empty() || rest.starts_with(' ') || rest.starts_with('\t') {
            return String::new();
        }
    }

    let upper = line.to_ascii_uppercase();
    if let Some(idx) = upper.find(" REM") {
        let after_rem = &upper[idx + 4..];
        if after_rem.is_empty() || after_rem.starts_with(' ') || after_rem.starts_with('\t') {
            line = &line[..idx];
        }
    }

    line.trim().to_string()
}

fn parse_operand(token: &str, line_no: usize, stmt: &'static str) -> Result<u8, CompileError> {
    let value = parse_integer(token).map_err(|_| CompileError::BadOperand {
        line: line_no,
        stmt,
        token: token.to_string(),
    })?;

    if !(0..=255).contains(&value) {
        return Err(CompileError::OperandRange {
            line: line_no,
            stmt,
            value,
        });
    }

    Ok(value as u8)
}

fn parse_integer(token: &str) -> Result<i64, ()> {
    let token = token.trim();
    if token.is_empty() {
        return Err(());
    }

    let (radix, digits) = if let Some(rest) = token.strip_prefix("0x").or_else(|| token.strip_prefix("0X"))
    {
        (16, rest)
    } else if token.starts_with('0') && token.len() > 1 && token.chars().all(|c| c.is_ascii_digit()) {
        (8, &token[1..])
    } else {
        (10, token)
    };

    i64::from_str_radix(digits, radix).map_err(|_| ())
}

fn parse_line(raw: &str, line_no: usize) -> Result<Option<Instruction>, CompileError> {
    let mut line = strip_comments(raw);
    if line.is_empty() {
        return Ok(None);
    }

    // Optional line number prefix (10 DELAY 2)
    if let Some((first, rest)) = line.split_once(' ') {
        if !first.is_empty() && first.chars().all(|c| c.is_ascii_digit()) {
            line = rest.trim().to_string();
        }
    }

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(None);
    }

    let keyword = parts[0].to_ascii_uppercase();
    let rest = &parts[1..];
    let text = line.clone();

    let insn = match keyword.as_str() {
        "DELAY" => {
            require_args(rest, 1, line_no, "DELAY requires one operand (seconds)")?;
            Instruction {
                opcode: 0x0C,
                operand: parse_operand(rest[0], line_no, "DELAY")?,
                source_line: line_no,
                text,
            }
        }
        "FORWARD" => {
            require_args(rest, 1, line_no, "FORWARD requires one operand")?;
            Instruction {
                opcode: 0x01,
                operand: parse_operand(rest[0], line_no, "FORWARD")?,
                source_line: line_no,
                text,
            }
        }
        "BACK" => {
            require_args(rest, 1, line_no, "BACK requires one operand")?;
            Instruction {
                opcode: 0x02,
                operand: parse_operand(rest[0], line_no, "BACK")?,
                source_line: line_no,
                text,
            }
        }
        "LEFT" => {
            require_args(rest, 1, line_no, "LEFT requires one operand")?;
            Instruction {
                opcode: 0x00,
                operand: parse_operand(rest[0], line_no, "LEFT")?,
                source_line: line_no,
                text,
            }
        }
        "RIGHT" => {
            require_args(rest, 1, line_no, "RIGHT requires one operand")?;
            Instruction {
                opcode: 0x03,
                operand: parse_operand(rest[0], line_no, "RIGHT")?,
                source_line: line_no,
                text,
            }
        }
        "LAMP" => {
            require_args(rest, 1, line_no, "LAMP requires ON or OFF")?;
            let operand = match rest[0].to_ascii_uppercase().as_str() {
                "ON" => 1,
                "OFF" => 0,
                _ => {
                    return Err(CompileError::Line {
                        line: line_no,
                        message: "LAMP requires ON or OFF".into(),
                    });
                }
            };
            Instruction {
                opcode: 0x0A,
                operand,
                source_line: line_no,
                text,
            }
        }
        "HOME" => {
            if !rest.is_empty() {
                return Err(CompileError::Line {
                    line: line_no,
                    message: "HOME takes no operands".into(),
                });
            }
            Instruction {
                opcode: 0x0B,
                operand: 0,
                source_line: line_no,
                text,
            }
        }
        "PLAY" => {
            require_args(rest, 1, line_no, "PLAY requires tune number")?;
            Instruction {
                opcode: 0x81,
                operand: parse_operand(rest[0], line_no, "PLAY")?,
                source_line: line_no,
                text,
            }
        }
        "SAY" => {
            require_args(rest, 1, line_no, "SAY requires RAM phrase index")?;
            Instruction {
                opcode: 0x83,
                operand: parse_operand(rest[0], line_no, "SAY")?,
                source_line: line_no,
                text,
            }
        }
        "SPEAK" => {
            require_args(rest, 1, line_no, "SPEAK requires ROM phrase index")?;
            Instruction {
                opcode: 0x82,
                operand: parse_operand(rest[0], line_no, "SPEAK")?,
                source_line: line_no,
                text,
            }
        }
        "END" => {
            if !rest.is_empty() {
                return Err(CompileError::Line {
                    line: line_no,
                    message: "END takes no operands".into(),
                });
            }
            Instruction {
                opcode: 0xFF,
                operand: 0xFF,
                source_line: line_no,
                text,
            }
        }
        other => {
            return Err(CompileError::Line {
                line: line_no,
                message: format!("unknown statement {other:?}"),
            });
        }
    };

    Ok(Some(insn))
}

fn require_args(
    rest: &[&str],
    expected: usize,
    line_no: usize,
    message: &str,
) -> Result<(), CompileError> {
    if rest.len() != expected {
        return Err(CompileError::Line {
            line: line_no,
            message: message.into(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_program() {
        let prog = parse_source("DELAY 1\nEND\n").unwrap();
        assert_eq!(prog.len(), 2);
        assert_eq!(prog[0].opcode, 0x0C);
        assert_eq!(prog[0].operand, 1);
        assert_eq!(prog[1].as_bytes(), [0xFF, 0xFF]);
    }

    #[test]
    fn line_numbers_and_comments() {
        let src = "10 DELAY 2  REM wait\n# full-line comment\n20 END\n";
        let prog = parse_source(src).unwrap();
        assert_eq!(prog.len(), 2);
        assert_eq!(prog[0].operand, 2);
    }

    #[test]
    fn lamp_on_off() {
        let prog = parse_source("LAMP ON\nLAMP OFF\nEND\n").unwrap();
        assert_eq!(prog[0].as_bytes(), [0x0A, 0x01]);
        assert_eq!(prog[1].as_bytes(), [0x0A, 0x00]);
    }

    #[test]
    fn say_and_speak() {
        let prog = parse_source("SAY 0\nSPEAK 63\nEND\n").unwrap();
        assert_eq!(prog[0].as_bytes(), [0x83, 0x00]);
        assert_eq!(prog[1].as_bytes(), [0x82, 0x3F]);
    }

    #[test]
    fn implicit_end() {
        let prog = parse_source("HOME\n").unwrap();
        assert_eq!(prog.last().unwrap().opcode, 0xFF);
    }

    #[test]
    fn unknown_statement() {
        assert!(parse_source("GOTO 10\nEND\n").is_err());
    }

    #[test]
    fn operand_range() {
        assert!(parse_source("DELAY 300\nEND\n").is_err());
    }

    #[test]
    fn program_size_limit() {
        let lines: String = (0..(crate::emit::PHRASE_OFF - crate::emit::PROG_OFF) / 2)
            .map(|i| format!("DELAY {}\n", i % 10))
            .chain(std::iter::once("END\n".to_string()))
            .collect();
        assert!(program_bytes(&parse_source(&lines).unwrap()).is_err());
    }
}