//! Maxx Steele toolchain library — compile MaxxBAS, decode ROM programs, validate carts.

mod cart;
mod decode;
mod emit;
mod error;
mod input;
mod parse;
mod sim;
mod upload;
mod validate;

pub use cart::CartImage;
pub use decode::{
    decode_cart, decode_program, format_rom_listing, ProgramStep, ProgramTrace, StepKind,
};
pub use emit::{
    compile_source, compile_source_with_tables, emit_cart, format_listing, Copyright, EmitOptions,
    CART_SIZE, MUSIC_OFF, PHRASE_OFF, PROG_OFF,
};
pub use error::CompileError;
pub use input::{
    compile_to_output, default_output, input_kind, load_tables_from_reference, resolve_input,
    InputKind, ResolvedRom,
};
pub use parse::{parse_source, program_bytes, Instruction};
pub use upload::{picorom_size_token, run_upload, upload_command, PICOROM_SIZES};
pub use sim::{
    format_human as format_simulation, run_gui, run_simulation, SimulationOptions,
    SimulationReport,
};
pub use validate::{validate_cart, validate_cart_image};

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

    #[test]
    fn ultramaxx_bas_matches_community_rom() {
        let src = include_str!("../../../Cartridge/Examples/UltraMaxx/Firmware/Basic/ultramaxx.bas");
        let factory =
            include_bytes!("../../../Cartridge/Examples/UltraMaxx/Firmware/Binary/UltraMaxx.532");
        let (phrase, music) = (
            factory[PHRASE_OFF..MUSIC_OFF].to_vec(),
            factory[MUSIC_OFF..].to_vec(),
        );
        let image = compile_source_with_tables(
            src,
            Copyright::UltraMaxx,
            Some(&phrase),
            Some(&music),
        )
        .unwrap();
        assert_eq!(image.as_slice(), factory);
    }

    #[test]
    fn cbsdemo_bas_matches_factory_rom() {
        let src = include_str!("../../../Cartridge/Examples/CBSDemo/Firmware/Basic/cbsdemo.bas");
        let factory = include_bytes!("../../../Cartridge/Examples/CBSDemo/Firmware/Binary/CBSDemo.532");
        let (phrase, music) = (
            factory[crate::emit::PHRASE_OFF..crate::emit::MUSIC_OFF].to_vec(),
            factory[crate::emit::MUSIC_OFF..].to_vec(),
        );
        let image = compile_source_with_tables(
            src,
            Copyright::Cbs,
            Some(&phrase),
            Some(&music),
        )
        .unwrap();
        assert_eq!(image.as_slice(), factory);
    }

    #[test]
    fn hello_trace_json_round_trip() {
        let cart = CartImage::from_bytes(HELLO_532.to_vec()).unwrap();
        let trace = decode_cart(&cart).unwrap();
        let json = serde_json::to_string(&trace).unwrap();
        assert!(json.contains("\"op\":\"delay\""));
        assert!(json.contains("\"op\":\"forward\""));
    }

    #[test]
    fn unified_simulator_hello() {
        use crate::sim::{run_simulation, SimulationOptions};

        let cart = CartImage::from_bytes(HELLO_532.to_vec()).unwrap();
        let report = run_simulation(
            &cart,
            "hello.532",
            &SimulationOptions {
                max_cycles: 18_000,
                inject_key: None,
                run_firmware: true,
                cart_bootstrap: true,
                image_out: None,
                plain: false,
            },
        )
        .unwrap();
        assert_eq!(report.robot.steps.len(), report.program.steps.len());
        assert!(report.robot.final_state.time_s >= 4.0);
        let fw = report.firmware.expect("firmware sim");
        assert!(fw.cycles > 1_000);
    }
}