//! Unified Maxx Steele simulator — program trace, robot model, firmware CPU.

mod firmware;
mod gui;
mod patches;
mod report;
mod robot;
mod visual;

pub use gui::run_gui;
pub use report::{format_human, run_simulation, SimulationOptions, SimulationReport};