use serde::Serialize;

use crate::{decode::opcode_kind, ProgramStep, ProgramTrace, StepKind};

const PROG_RAM_START: usize = 0x0200;
const PROG_RAM_END: usize = 0x0400;

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

/// Pose replayed from live firmware program RAM (`$0200`) during execute mode.
#[derive(Debug, Clone)]
pub struct LiveRobotPose {
    pub state: RobotState,
    pub active_kind: StepKind,
}

impl Default for LiveRobotPose {
    fn default() -> Self {
        Self {
            state: RobotState::default(),
            active_kind: StepKind::End,
        }
    }
}

/// Replay bytecode up to the current program pointer so the playfield animates with the cart.
pub fn sync_live_robot_pose(mem: &[u8; 65536], pose: &mut LiveRobotPose) {
    pose.state = RobotState::default();
    pose.active_kind = StepKind::End;

    if mem[0x0D] != 3 {
        return;
    }

    let ptr = u16::from_le_bytes([mem[0x0F], mem[0x10]]) as usize;
    if !(PROG_RAM_START..PROG_RAM_END).contains(&ptr) {
        return;
    }

    let mut i = PROG_RAM_START;
    while i + 1 < ptr.min(PROG_RAM_END) {
        let opcode = mem[i];
        let operand = mem[i + 1];
        if opcode == 0xFF && operand == 0xFF {
            break;
        }
        let kind = opcode_kind(opcode, operand);
        let step = ProgramStep {
            index: 0,
            rom_addr: 0,
            opcode,
            operand,
            kind: kind.clone(),
            comment: String::new(),
        };
        apply_step(&mut pose.state, &step);
        pose.active_kind = kind;
        i += 2;
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StepKind;

    #[test]
    fn sync_live_robot_pose_replays_execute_bytecode() {
        let mut mem = [0u8; 65536];
        mem[0x0D] = 3;
        mem[0x0200] = 0x01; // forward
        mem[0x0201] = 20;
        mem[0x0202] = 0x06; // arms up
        mem[0x0203] = 40;
        mem[0x0204] = 0xFF;
        mem[0x0205] = 0xFF;
        mem[0x0F] = 0x04;
        mem[0x10] = 0x02;

        let mut pose = LiveRobotPose::default();
        sync_live_robot_pose(&mem, &mut pose);

        assert_eq!(pose.state.arms, 40);
        assert!((pose.state.y - 2.0).abs() < 0.01);
        assert!(matches!(pose.active_kind, StepKind::ArmsUp { value: 40 }));
    }

    #[test]
    fn sync_live_robot_pose_idle_outside_execute() {
        let mut mem = [0u8; 65536];
        mem[0x0D] = 0;
        mem[0x0200] = 0x06;
        mem[0x0201] = 40;

        let mut pose = LiveRobotPose::default();
        sync_live_robot_pose(&mem, &mut pose);

        assert_eq!(pose.state.arms, 0);
        assert!(matches!(pose.active_kind, StepKind::End));
    }
}