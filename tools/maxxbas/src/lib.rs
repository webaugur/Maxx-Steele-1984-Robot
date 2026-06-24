//! MaxxBAS compiler — compile line-oriented Maxx programs into 4 KB cartridge images.

mod emit;
mod error;
mod parse;
mod validate;

pub use emit::{
    compile_source, emit_cart, format_listing, Copyright, EmitOptions, CART_SIZE,
};
pub use error::CompileError;
pub use parse::{parse_source, program_bytes, Instruction};
pub use validate::validate_cart;

/// Compile MaxxBAS source with the given copyright string.
pub fn compile(text: &str, copyright: Copyright) -> Result<Vec<u8>, CompileError> {
    compile_source(text, copyright)
}

#[cfg(test)]
mod integration {
    use super::*;

    const HELLO_BAS: &str =
        include_str!("../../../Cartridge/Examples/UltraMaxx/Firmware/Basic/hello.bas");
    const HELLO_532: &[u8] =
        include_bytes!("../../../Cartridge/Examples/UltraMaxx/Firmware/Binary/hello.532");

    #[test]
    fn hello_matches_python_reference_binary() {
        let image = compile(HELLO_BAS, Copyright::UltraMaxx).unwrap();
        assert_eq!(image, HELLO_532);
    }

    #[test]
    fn hello_validates_clean() {
        let image = compile(HELLO_BAS, Copyright::UltraMaxx).unwrap();
        assert!(validate_cart(&image, 0xA000).is_empty());
    }
}