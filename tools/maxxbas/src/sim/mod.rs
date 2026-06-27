//! Unified Maxx Steele simulator — program trace, robot model, firmware CPU.

mod display;
mod firmware;
mod gui;
mod interactive;
mod keypad;
mod live_gui;
mod patches;
mod remote_panel;
mod report;
mod robot;
mod trace;
mod visual;

pub use gui::run_gui;
pub use live_gui::run_live_gui;
pub use report::{format_human, run_simulation, SimulationOptions, SimulationReport};