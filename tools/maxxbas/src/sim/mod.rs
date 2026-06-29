//! Unified Maxx Steele simulator — program trace, robot model, firmware CPU.
//!
//! **Audio checkpoint (v0.2.78):** Live sim verified — startup tune 1, cart/ROM PLAY tunes,
//! SAM speech phrases, and instruction waits (`$E504` / `$F47E`) all play correctly.

mod audio;
mod display;
mod firmware;
mod gui;
mod interactive;
mod keypad;
mod live_gui;
mod patches;
mod plastic_skin;
mod remote_branding;
mod remote_font;
mod ui_font;
mod remote_panel;
mod report;
mod robot;
mod robot_view;
mod emoji_font;
mod music;
mod speech;
mod speech_font;
mod speech_sam;
mod trace;
mod trace_breakpoint;
mod visual;

pub use gui::run_gui;
pub use live_gui::run_live_gui;
pub use report::{format_human, run_simulation, SimulationOptions, SimulationReport};