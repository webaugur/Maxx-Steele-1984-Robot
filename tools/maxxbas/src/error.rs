use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum CompileError {
    #[error("empty program")]
    EmptyProgram,

    #[error("line {line}: {message}")]
    Line { line: usize, message: String },

    #[error("line {line}: {stmt}: expected integer operand, got {token:?}")]
    BadOperand {
        line: usize,
        stmt: &'static str,
        token: String,
    },

    #[error("line {line}: {stmt}: operand {value} out of range 0..255")]
    OperandRange {
        line: usize,
        stmt: &'static str,
        value: i64,
    },

    #[error("program too large: {pairs} pairs ({bytes} bytes), max {max_pairs}")]
    ProgramTooLarge {
        pairs: usize,
        bytes: usize,
        max_pairs: usize,
    },

    #[error("copyright must be 17 bytes, got {len}")]
    CopyrightLength { len: usize },
}