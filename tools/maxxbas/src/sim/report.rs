use std::fs;
use serde::Serialize;

use super::firmware::{build_memory_image, run_firmware, FirmwareOptions, FirmwareResult};
use super::patches::PatchSet;
use super::robot::{simulate_robot, RobotSimulation};
use super::visual::render_storyboard;
use crate::{decode_cart, CartImage, ProgramTrace};

#[derive(Debug, Clone)]
pub struct SimulationOptions {
    pub max_cycles: u64,
    pub inject_key: Option<u8>,
    pub run_firmware: bool,
    pub cart_bootstrap: bool,
    pub image_out: Option<std::path::PathBuf>,
    pub plain: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct SimulationReport {
    pub input: String,
    pub program: ProgramTrace,
    pub robot: RobotSimulation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware: Option<FirmwareResult>,
}

pub fn run_simulation(
    cart: &CartImage,
    input_label: &str,
    options: &SimulationOptions,
) -> Result<SimulationReport, String> {
    let trace = decode_cart(cart)?;
    let robot = simulate_robot(&trace);

    let patches = PatchSet::embedded();
    let mut mem = build_memory_image(Some(cart), &patches)?;

    if let Some(path) = &options.image_out {
        fs::write(path, &mem).map_err(|e| format!("{}: {e}", path.display()))?;
    }

    let firmware = if options.run_firmware {
        Some(run_firmware(
            &mut mem,
            &patches,
            &FirmwareOptions {
                max_cycles: options.max_cycles,
                inject_key: options.inject_key,
                run_cart_bootstrap: options.cart_bootstrap,
                cart: Some(cart.clone()),
            },
        ))
    } else {
        None
    };

    Ok(SimulationReport {
        input: input_label.to_string(),
        program: trace,
        robot,
        firmware,
    })
}

pub fn format_human(report: &SimulationReport, plain: bool) -> String {
    let mut out = String::new();
    out.push_str("Maxx Steele Simulator\n");
    out.push_str(&format!("Input: {}\n", report.input));
    out.push_str(&format!("Copyright: {}\n", report.program.copyright));
    out.push_str(&format!("Program steps: {}\n\n", report.program.steps.len()));

    if !plain && !report.robot.steps.is_empty() {
        out.push_str(&render_storyboard(&report.robot.steps));
    }

    out.push_str("\nRobot model (numeric):\n");
    for step in &report.robot.steps {
        let s = &step.state;
        out.push_str(&format!(
            "  {:3}  {:<28} t={:.1}s pos=({:.1},{:.1}) hdg={:.0} lamp={}\n",
            step.index, step.comment, s.time_s, s.x, s.y, s.heading_deg, s.lamp
        ));
    }
    let fin = &report.robot.final_state;
    out.push_str(&format!(
        "\nFinal: t={:.1}s pos=({:.1},{:.1}) events={}\n",
        fin.time_s,
        fin.x,
        fin.y,
        fin.events.len()
    ));

    if let Some(fw) = &report.firmware {
        out.push_str("\nFirmware (patched 65C02):\n");
        out.push_str(&format!(
            "  cycles={} pc=${:04X} reason={}\n",
            fw.cycles, fw.final_pc, fw.stopped_reason
        ));
        out.push_str(&format!(
            "  status $02=${:02X} $03=${:02X} mode $0D=${:02X} ram_steps={}\n",
            fw.status_02, fw.status_03, fw.mode_0d, fw.program_steps_in_ram
        ));
        if !fw.traps_hit.is_empty() {
            out.push_str("  traps:\n");
            for t in &fw.traps_hit {
                out.push_str(&format!(
                    "    {} @ ${:04X} (cycle {})\n",
                    t.name, t.addr, t.cycle
                ));
            }
        }
    }

    out
}

