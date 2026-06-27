//! Maxx Steele remote keypad — matrix keycodes wired to internal ROM `$E6B5` / `$E617`.

/// Faceplate key identity (matrix label in parentheses).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteKey {
    DriveU,
    Drive1,
    Drive2,
    Drive3,
    Wrist4,
    Wrist5,
    Arms6,
    Arms7,
    Claw8,
    Claw9,
    LampA,
    HomeB,
    Wait,
    ShiftOctave,
    Clear,
    Enter,
    SongNotes,
    ClockStatus,
    Speech,
    Motion,
    Game,
    Program,
    Learn,
    Execute,
    PowerStop,
}

impl RemoteKey {
    /// Keycode presented on the RadioIn / `$75` wire (before `$E6A4` latches into `$15`).
    pub fn keycode(self) -> u8 {
        match self {
            RemoteKey::DriveU => 0x00,
            RemoteKey::Drive1 => 0x01,
            RemoteKey::Drive2 => 0x02,
            RemoteKey::Drive3 => 0x03,
            RemoteKey::Wrist4 => 0x04,
            RemoteKey::Wrist5 => 0x05,
            RemoteKey::Arms6 => 0x06,
            RemoteKey::Arms7 => 0x07,
            RemoteKey::Claw8 => 0x08,
            RemoteKey::Claw9 => 0x09,
            RemoteKey::LampA => 0x0A,
            RemoteKey::HomeB => 0x0B,
            RemoteKey::Wait => 0x0C,
            RemoteKey::ShiftOctave => 0x0D,
            RemoteKey::Clear => 0x0E,
            RemoteKey::Enter => 0x0F,
            // Extended table @ ROM `$E6B5` + 16 (`$E6C5`…)
            RemoteKey::SongNotes => 0x41,
            RemoteKey::ClockStatus => 0x46,
            RemoteKey::Speech => 0x43,
            RemoteKey::Motion => 0x13,
            RemoteKey::Game => 0x84,
            RemoteKey::Program => 0x82,
            RemoteKey::Learn => 0x81,
            RemoteKey::Execute => 0x83,
            RemoteKey::PowerStop => 0x80,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            RemoteKey::DriveU => "U",
            RemoteKey::Drive1 => "1",
            RemoteKey::Drive2 => "2",
            RemoteKey::Drive3 => "3",
            RemoteKey::Wrist4 => "4",
            RemoteKey::Wrist5 => "5",
            RemoteKey::Arms6 => "6",
            RemoteKey::Arms7 => "7",
            RemoteKey::Claw8 => "8",
            RemoteKey::Claw9 => "9",
            RemoteKey::LampA => "A",
            RemoteKey::HomeB => "B",
            RemoteKey::Wait => "WAIT",
            RemoteKey::ShiftOctave => "SHIFT",
            RemoteKey::Clear => "CLEAR",
            RemoteKey::Enter => "ENTER",
            RemoteKey::SongNotes => "SONG",
            RemoteKey::ClockStatus => "CLOCK",
            RemoteKey::Speech => "SPEECH",
            RemoteKey::Motion => "MOTION",
            RemoteKey::Game => "GAME",
            RemoteKey::Program => "PROGRAM",
            RemoteKey::Learn => "LEARN",
            RemoteKey::Execute => "EXECUTE",
            RemoteKey::PowerStop => "POWER/STOP",
        }
    }

    pub fn faceplate(self) -> &'static str {
        match self {
            RemoteKey::DriveU | RemoteKey::Drive1 | RemoteKey::Drive2 | RemoteKey::Drive3 => {
                "DRIVE"
            }
            RemoteKey::Wrist4 | RemoteKey::Wrist5 => "WRIST",
            RemoteKey::Arms6 | RemoteKey::Arms7 => "ARMS",
            RemoteKey::Claw8 | RemoteKey::Claw9 => "CLAW",
            RemoteKey::LampA => "LAMP",
            RemoteKey::HomeB => "HOME",
            RemoteKey::Wait => "NOTE REST",
            RemoteKey::ShiftOctave => "OCTAVE",
            RemoteKey::Clear => "CLEAR",
            RemoteKey::Enter => "ENTER",
            RemoteKey::SongNotes => "NOTES",
            RemoteKey::ClockStatus => "STATUS",
            RemoteKey::Speech => "SPEECH",
            RemoteKey::Motion => "MOTION",
            RemoteKey::Game => "GAME",
            RemoteKey::Program => "PROGRAM",
            RemoteKey::Learn => "LEARN",
            RemoteKey::Execute => "EXECUTE",
            RemoteKey::PowerStop => "POWER/STOP",
        }
    }

    pub fn matrix(self) -> char {
        match self {
            RemoteKey::DriveU => 'A',
            RemoteKey::Drive1 => 'B',
            RemoteKey::Drive2 => 'C',
            RemoteKey::Drive3 => 'D',
            RemoteKey::Wrist4 => 'E',
            RemoteKey::Wrist5 => 'F',
            RemoteKey::Arms6 => 'G',
            RemoteKey::Arms7 => 'H',
            RemoteKey::Claw8 => 'I',
            RemoteKey::Claw9 => 'J',
            RemoteKey::LampA => 'K',
            RemoteKey::HomeB => 'L',
            RemoteKey::Wait => 'M',
            RemoteKey::ShiftOctave => 'N',
            RemoteKey::Clear => 'O',
            RemoteKey::Enter => 'P',
            RemoteKey::SongNotes => 'Q',
            RemoteKey::ClockStatus => 'R',
            RemoteKey::Speech => 'S',
            RemoteKey::Motion => 'T',
            RemoteKey::Game => 'U',
            RemoteKey::Program => 'V',
            RemoteKey::Learn => 'W',
            RemoteKey::Execute => 'X',
            RemoteKey::PowerStop => 'Y',
        }
    }
}