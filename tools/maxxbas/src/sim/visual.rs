use crate::StepKind;

use super::robot::RobotStep;

/// Two-character LED segment label from internal ROM table `$F878`.
pub fn opcode_display(opcode: u8) -> &'static str {
    match opcode {
        0x00 => "L",
        0x01 => "F",
        0x02 => "b",
        0x03 => "r",
        0x04 => "Uu",
        0x05 => "Ud",
        0x06 => "Au",
        0x07 => "Ad",
        0x08 => "Cr",
        0x09 => "Cc",
        0x0A => "HL",
        0x0B => "init",
        0x0C => "d",
        0x0D => "Sn",
        0x0E => "S",
        0x0F => "SS",
        0x10 => "PS",
        0x81 => "PLAY",
        0x82 => "SPEE",
        0x83 => "SS",
        0xFF => "End",
        _ => "??",
    }
}

pub fn action_glyph(kind: &StepKind) -> &'static str {
    match kind {
        StepKind::Delay { .. } => "ZZZ",
        StepKind::Forward { .. } => ">>>",
        StepKind::Back { .. } => "<<<",
        StepKind::Left { .. } => "<<-",
        StepKind::Right { .. } => "->>",
        StepKind::WristUp { .. } => "wrist^",
        StepKind::WristDown { .. } => "wristv",
        StepKind::ArmsUp { .. } => "arms^",
        StepKind::ArmsDown { .. } => "armsv",
        StepKind::ClawRotate { .. } => "claw~",
        StepKind::ClawOpenClose { close } => {
            if *close {
                "claw]"
            } else {
                "claw["
            }
        }
        StepKind::Lamp { on } => {
            if *on {
                "LAMP*"
            } else {
                "lamp "
            }
        }
        StepKind::Home => "HOME",
        StepKind::Play { .. } => "PLAY",
        StepKind::SpeakRom { .. } | StepKind::SpeakRam { .. } => "TALK",
        StepKind::Unknown { .. } => "????",
        StepKind::End => " END",
    }
}

fn bar(level: u8, max: u8, width: usize) -> String {
    let filled = (usize::from(level.min(max)) * width) / usize::from(max.max(1));
    let mut s = String::with_capacity(width);
    for i in 0..width {
        s.push(if i < filled { '#' } else { '-' });
    }
    s
}

fn heading_arrow(deg: f32) -> &'static str {
    let d = deg.rem_euclid(360.0);
    if d < 22.5 || d >= 337.5 {
        "^"
    } else if d < 67.5 {
        "/"
    } else if d < 112.5 {
        "<"
    } else if d < 157.5 {
        "\\"
    } else if d < 202.5 {
        "v"
    } else if d < 247.5 {
        "/"
    } else if d < 292.5 {
        ">"
    } else {
        "\\"
    }
}

/// ASCII frame showing robot pose and the opcode-driven action for one step.
pub fn render_step_frame(step: &RobotStep) -> String {
    let s = &step.state;
    let arms = bar(s.arms.min(64), 64, 8);
    let wrist = bar(s.wrist.min(64), 64, 6);
    let claw_rot = (s.claw_rotate % 4) as usize;
    let claw_chars = ['|', '/', '-', '\\'];
    let claw_ang = claw_chars[claw_rot];
    let claw_gap = if s.claw_open { "  " } else { "||" };
    let lamp = if s.lamp { "(*)" } else { "( )" };
    let glyph = action_glyph(&step.kind);
    let display = opcode_display(step.opcode);
    let arrow = heading_arrow(s.heading_deg);

    let mut out = String::new();
    out.push_str(&format!(
        "  opcode ${:02X} {:02X}  display [{display}]  action [{glyph}]\n",
        step.opcode, step.operand
    ));
    out.push_str(&format!("  {lamp}\n"));
    out.push_str("      ┌────────┐\n");
    out.push_str(&format!(
        "   {claw_ang}──│ {claw_gap} │──{claw_ang}   arms [{arms}]\n",
    ));
    out.push_str(&format!("      │ wrist  │   wrist [{wrist}]\n"));
    out.push_str("      └───┬────┘\n");
    out.push_str("          │\n");
    out.push_str(&format!(
        "      ────┴────   plan ({:.1},{:.1}) {arrow} {:.0}°\n",
        s.x, s.y, s.heading_deg
    ));
    if !s.events.is_empty() {
        if let Some(last) = s.events.last() {
            out.push_str(&format!("      event: {last}\n"));
        }
    }
    out
}

/// Full program storyboard for terminal output.
pub fn render_storyboard(steps: &[RobotStep]) -> String {
    let mut out = String::new();
    out.push_str("Visual program trace (opcode inspection):\n");
    for step in steps {
        if matches!(step.kind, StepKind::End) {
            continue;
        }
        out.push_str(&format!(
            "\n── Step {:>2}: {} ──\n",
            step.index, step.comment
        ));
        out.push_str(&render_step_frame(step));
    }
    if let Some(last) = steps.last() {
        out.push_str("\n── Final pose ──\n");
        out.push_str(&render_step_frame(last));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StepKind;

    use super::super::robot::RobotState;

    #[test]
    fn forward_frame_shows_drive_glyph() {
        let step = RobotStep {
            index: 1,
            opcode: 0x01,
            operand: 20,
            display: "F".into(),
            comment: "forward 20".into(),
            kind: StepKind::Forward { distance: 20 },
            state: RobotState {
                y: 2.0,
                ..Default::default()
            },
            visual: String::new(),
        };
        let frame = render_step_frame(&step);
        assert!(frame.contains("[>>>]"));
        assert!(frame.contains("display [F]"));
        assert!(frame.contains("plan (0.0,2.0)"));
    }
}