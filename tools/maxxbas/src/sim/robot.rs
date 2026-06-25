use serde::Serialize;

use crate::{ProgramStep, ProgramTrace, StepKind};

use super::visual::{opcode_display, render_step_frame};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RobotState {
    pub time_s: f32,
    pub x: f32,
    pub y: f32,
    pub heading_deg: f32,
    pub lamp: bool,
    pub arms: u8,
    pub wrist: u8,
    pub claw_open: bool,
    pub claw_rotate: u8,
    pub events: Vec<String>,
}

impl Default for RobotState {
    fn default() -> Self {
        Self {
            time_s: 0.0,
            x: 0.0,
            y: 0.0,
            heading_deg: 0.0,
            lamp: false,
            arms: 0,
            wrist: 0,
            claw_open: false,
            claw_rotate: 0,
            events: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RobotStep {
    pub index: usize,
    pub opcode: u8,
    pub operand: u8,
    /// LED segment label from internal ROM display table.
    pub display: String,
    pub comment: String,
    pub kind: StepKind,
    pub state: RobotState,
    /// ASCII pose frame after this opcode (terminal visual).
    pub visual: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RobotSimulation {
    pub steps: Vec<RobotStep>,
    pub final_state: RobotState,
}

pub fn simulate_robot(trace: &ProgramTrace) -> RobotSimulation {
    let mut state = RobotState::default();
    let mut steps = Vec::with_capacity(trace.steps.len());

    for step in &trace.steps {
        apply_step(&mut state, step);
        let mut robot_step = RobotStep {
            index: step.index,
            opcode: step.opcode,
            operand: step.operand,
            display: opcode_display(step.opcode).to_string(),
            comment: step.comment.clone(),
            kind: step.kind.clone(),
            state: state.clone(),
            visual: String::new(),
        };
        robot_step.visual = render_step_frame(&robot_step);
        steps.push(robot_step);
    }

    RobotSimulation {
        final_state: state,
        steps,
    }
}

fn apply_step(state: &mut RobotState, step: &ProgramStep) {
    match &step.kind {
        StepKind::Delay { seconds } => {
            state.time_s += f32::from(*seconds);
        }
        StepKind::Forward { distance } => {
            drive(state, f32::from(*distance));
        }
        StepKind::Back { distance } => {
            drive(state, -f32::from(*distance));
        }
        StepKind::Left { distance } => {
            state.heading_deg -= f32::from(*distance);
        }
        StepKind::Right { angle } => {
            state.heading_deg += f32::from(*angle);
        }
        StepKind::WristUp { value } => {
            state.wrist = state.wrist.saturating_add(*value);
        }
        StepKind::WristDown { value } => {
            state.wrist = state.wrist.saturating_sub(*value);
        }
        StepKind::ArmsUp { value } => {
            state.arms = state.arms.saturating_add(*value);
        }
        StepKind::ArmsDown { value } => {
            state.arms = state.arms.saturating_sub(*value);
        }
        StepKind::ClawRotate { value } => {
            state.claw_rotate = state.claw_rotate.wrapping_add(*value);
        }
        StepKind::ClawOpenClose { close } => {
            state.claw_open = !close;
        }
        StepKind::Lamp { on } => {
            state.lamp = *on;
        }
        StepKind::Home => {
            state.arms = 0;
            state.wrist = 0;
            state.claw_open = false;
            state.claw_rotate = 0;
            state.heading_deg = 0.0;
            state.events.push("home".into());
        }
        StepKind::Play { tune } => {
            state.events.push(format!("play tune {tune}"));
        }
        StepKind::SpeakRom { phrase } | StepKind::SpeakRam { phrase } => {
            state.events.push(format!("speak {phrase}"));
        }
        StepKind::Unknown { opcode, operand } => {
            state
                .events
                .push(format!("unknown {opcode:02X} {operand:02X}"));
        }
        StepKind::End => {}
    }
}

fn drive(state: &mut RobotState, distance: f32) {
    let scale = 0.1;
    let rad = state.heading_deg.to_radians();
    let delta = distance * scale;
    state.x += delta * rad.sin();
    state.y += delta * rad.cos();
}