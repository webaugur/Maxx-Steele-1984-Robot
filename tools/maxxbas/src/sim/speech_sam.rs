//! SAM (Software Automatic Mouth) phrase synthesis via [`rustsam`].
//!
//! **Voices are not Apple TTS.** macOS `say -v` names are accepted for CLI
//! compatibility and mapped to distinct SAM formant presets (speed/pitch/throat/mouth).
//! Audio always comes from rustsam.

use rustsam::{parser, reciter, renderer};

pub const SAM_SAMPLE_RATE: u32 = 22_050;
const SAM_GAIN: f32 = 1.15;

/// One SAM voice profile (formant parameters).
///
/// Values are classic SAM: higher **speed** / **pitch** numbers make speech
/// *slower* / *lower*.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SamVoice {
    /// CLI name (`robot`, `Alex`, …) — matching is case-insensitive.
    pub name: &'static str,
    /// Short note (locale / character), shown in `say -v ?`.
    pub note: &'static str,
    pub speed: u8,
    pub pitch: u8,
    pub throat: u8,
    pub mouth: u8,
}

impl SamVoice {
    pub const fn params(self) -> (u8, u8, u8, u8) {
        (self.speed, self.pitch, self.throat, self.mouth)
    }
}

/// Classic SamJs presets + macOS `say -v` name catalog (SAM approximations).
///
/// macOS voices cannot be reproduced; names exist so scripts using
/// `say -v Alex` etc. keep working under this SAM backend.
const VOICES: &[SamVoice] = &[
    // ── Classic SAM presets (SamJs) ──────────────────────────────────────
    v("sam", "SAM default (C64)", 72, 64, 128, 128),
    v("elf", "SAM Elf", 72, 64, 110, 160),
    v("robot", "SAM Little Robot (Maxx default)", 92, 60, 190, 190),
    v("stuffy", "SAM Stuffy Guy", 82, 72, 110, 105),
    v("lady", "SAM Little Old Lady", 82, 32, 145, 145),
    v("alien", "SAM Extra-Terrestrial", 100, 64, 150, 200),
    // aliases as separate list entries for -v ? visibility
    v("default", "alias → sam", 72, 64, 128, 128),
    v("little-robot", "alias → robot", 92, 60, 190, 190),
    v("maxx", "alias → robot", 92, 60, 190, 190),
    v("stuffy-guy", "alias → stuffy", 82, 72, 110, 105),
    v("little-old-lady", "alias → lady", 82, 32, 145, 145),
    v("et", "alias → alien", 100, 64, 150, 200),
    v("extra-terrestrial", "alias → alien", 100, 64, 150, 200),
    // ── macOS classic English / novelty (approximate with SAM) ───────────
    // Neutral / male
    v("Alex", "en_US — male (SAM approx)", 72, 58, 128, 128),
    v("Bruce", "en_US — male (SAM approx)", 74, 62, 120, 125),
    v("Fred", "en_US — male classic (SAM approx)", 70, 66, 125, 130),
    v("Junior", "en_US — young male (SAM approx)", 68, 48, 135, 145),
    v("Ralph", "en_US — male (SAM approx)", 76, 70, 118, 122),
    v("Tom", "en_US — male (SAM approx)", 72, 60, 126, 124),
    v("Daniel", "en_GB — male (SAM approx)", 74, 56, 122, 130),
    v("Oliver", "en_GB — male (SAM approx)", 73, 58, 124, 128),
    v("Arthur", "en_GB — male (SAM approx)", 75, 61, 120, 126),
    v("Aaron", "en_US — male (SAM approx)", 71, 57, 127, 129),
    v("Albert", "en_US — male older (SAM approx)", 84, 78, 115, 110),
    // Female
    v("Samantha", "en_US — female (SAM approx)", 70, 40, 140, 150),
    v("Victoria", "en_US — female (SAM approx)", 72, 38, 142, 148),
    v("Vicki", "en_US — female (SAM approx)", 68, 42, 138, 152),
    v("Kathy", "en_US — female (SAM approx)", 71, 44, 136, 146),
    v("Agnes", "en_US — older female (SAM approx)", 80, 36, 145, 140),
    v("Princess", "en_US — young female (SAM approx)", 66, 34, 150, 160),
    v("Susan", "en_US — female (SAM approx)", 72, 41, 139, 147),
    v("Allison", "en_US — female (SAM approx)", 69, 39, 141, 149),
    v("Ava", "en_US — female (SAM approx)", 68, 37, 143, 151),
    v("Zoe", "en_US — female (SAM approx)", 67, 35, 144, 153),
    v("Karen", "en_AU — female (SAM approx)", 73, 40, 138, 148),
    v("Lee", "en_AU — male (SAM approx)", 74, 59, 125, 127),
    v("Fiona", "en_GB-scotland — female (SAM approx)", 72, 39, 140, 150),
    v("Moira", "en_IE — female (SAM approx)", 73, 41, 137, 149),
    v("Tessa", "en_ZA — female (SAM approx)", 72, 42, 136, 147),
    v("Veena", "en_IN — female (SAM approx)", 74, 43, 135, 145),
    v("Rishi", "en_IN — male (SAM approx)", 75, 58, 123, 128),
    v("Catherine", "en_AU — female (SAM approx)", 71, 38, 141, 150),
    v("Gordon", "en_AU — male (SAM approx)", 76, 63, 121, 124),
    v("Serena", "en_GB — female (SAM approx)", 70, 37, 142, 151),
    v("Stephanie", "en_US — female (SAM approx)", 69, 38, 140, 149),
    v("Nicky", "en_US — female (SAM approx)", 68, 40, 139, 148),
    v("Noelle", "en_US — female (SAM approx)", 70, 39, 141, 150),
    v("Evan", "en_US — male (SAM approx)", 71, 57, 126, 128),
    v("Nathan", "en_US — male (SAM approx)", 72, 59, 125, 127),
    v("Paul", "en_US — male (SAM approx)", 73, 61, 124, 126),
    // Novelty / effects (classic Mac)
    v("Bad News", "en_US — novelty (SAM approx)", 95, 80, 100, 100),
    v("Bahh", "en_US — novelty (SAM approx)", 88, 90, 95, 90),
    v("Bells", "en_US — novelty (SAM approx)", 85, 50, 160, 170),
    v("Boing", "en_US — novelty (SAM approx)", 60, 45, 180, 100),
    v("Bubbles", "en_US — novelty (SAM approx)", 65, 35, 170, 180),
    v("Cellos", "en_US — novelty (SAM approx)", 100, 85, 110, 115),
    v("Deranged", "en_US — novelty (SAM approx)", 55, 55, 200, 80),
    v("Good News", "en_US — novelty (SAM approx)", 78, 48, 130, 140),
    v("Hysterical", "en_US — novelty (SAM approx)", 50, 28, 155, 175),
    v("Pipe Organ", "en_US — novelty (SAM approx)", 105, 70, 100, 105),
    v("Trinoids", "en_US — robot novelty (SAM approx)", 90, 55, 185, 185),
    v("Whisper", "en_US — novelty (SAM approx)", 88, 52, 90, 95),
    v("Zarvox", "en_US — robot novelty (SAM approx)", 94, 58, 195, 200),
    v("Superstar", "en_US — novelty (SAM approx)", 70, 46, 145, 155),
    v("Sandy", "en_US — novelty (SAM approx)", 74, 44, 135, 145),
    v("Shelley", "en_US — novelty (SAM approx)", 72, 42, 138, 148),
    // ── Common non-English macOS voice *names* (still English SAM reciter) ─
    // The reciter is English-only; these only change timbre for -v name compat.
    v("Anna", "de_DE — name only (SAM approx)", 72, 40, 140, 148),
    v("Helena", "de_DE — name only (SAM approx)", 73, 39, 141, 149),
    v("Markus", "de_DE — name only (SAM approx)", 74, 60, 125, 128),
    v("Yannick", "de_DE — name only (SAM approx)", 73, 58, 126, 129),
    v("Thomas", "fr_FR — name only (SAM approx)", 74, 59, 124, 127),
    v("Audrey", "fr_FR — name only (SAM approx)", 71, 38, 142, 150),
    v("Aurelie", "fr_FR — name only (SAM approx)", 70, 37, 143, 151),
    v("Marie", "fr_FR — name only (SAM approx)", 72, 40, 140, 149),
    v("Amelie", "fr_CA — name only (SAM approx)", 71, 39, 141, 150),
    v("Chantal", "fr_CA — name only (SAM approx)", 72, 41, 139, 148),
    v("Nicolas", "fr_CA — name only (SAM approx)", 74, 57, 125, 128),
    v("Alice", "it_IT — name only (SAM approx)", 70, 38, 142, 150),
    v("Luca", "it_IT — name only (SAM approx)", 73, 58, 126, 128),
    v("Paola", "it_IT — name only (SAM approx)", 71, 40, 140, 149),
    v("Silvia", "it_IT — name only (SAM approx)", 72, 39, 141, 150),
    v("Jorge", "es_ES — name only (SAM approx)", 74, 60, 124, 127),
    v("Monica", "es_ES — name only (SAM approx)", 71, 40, 140, 149),
    v("Juan", "es_MX — name only (SAM approx)", 73, 59, 125, 128),
    v("Paulina", "es_MX — name only (SAM approx)", 70, 39, 141, 150),
    v("Diego", "es_AR — name only (SAM approx)", 74, 61, 123, 126),
    v("Carlos", "es_CO? — name only (SAM approx)", 73, 60, 124, 127),
    v("Kyoko", "ja_JP — name only (SAM approx)", 68, 36, 145, 155),
    v("Otoya", "ja_JP — name only (SAM approx)", 72, 55, 130, 135),
    v("Sin-ji", "zh_HK — name only (SAM approx)", 74, 50, 135, 140),
    v("Mei-Jia", "zh_TW — name only (SAM approx)", 72, 42, 140, 148),
    v("Ting-Ting", "zh_CN — name only (SAM approx)", 70, 40, 142, 150),
    v("Yu-shu", "zh_CN — name only (SAM approx)", 73, 56, 128, 132),
    v("Yuna", "ko_KR — name only (SAM approx)", 69, 38, 143, 151),
    v("Sora", "ko_KR — name only (SAM approx)", 71, 42, 140, 148),
    v("Ellen", "nl_BE — name only (SAM approx)", 72, 40, 140, 149),
    v("Xander", "nl_NL — name only (SAM approx)", 74, 58, 125, 128),
    v("Claire", "nl_NL — name only (SAM approx)", 71, 39, 141, 150),
    v("Satu", "fi_FI — name only (SAM approx)", 72, 41, 139, 148),
    v("Nora", "nb_NO — name only (SAM approx)", 71, 40, 140, 149),
    v("Henrik", "nb_NO — name only (SAM approx)", 74, 59, 125, 128),
    v("Alva", "sv_SE — name only (SAM approx)", 70, 38, 142, 150),
    v("Oskar", "sv_SE — name only (SAM approx)", 73, 58, 126, 129),
    v("Zosia", "pl_PL — name only (SAM approx)", 71, 40, 140, 149),
    v("Ioana", "ro_RO — name only (SAM approx)", 72, 41, 139, 148),
    v("Joana", "pt_PT — name only (SAM approx)", 71, 39, 141, 150),
    v("Luciana", "pt_BR — name only (SAM approx)", 70, 38, 142, 151),
    v("Felipe", "pt_BR — name only (SAM approx)", 74, 60, 124, 127),
    v("Zuzana", "cs_CZ — name only (SAM approx)", 72, 40, 140, 149),
    v("Lekha", "hi_IN — name only (SAM approx)", 73, 42, 138, 147),
    v("Kanya", "th_TH — name only (SAM approx)", 71, 40, 140, 149),
    v("Carmit", "he_IL — name only (SAM approx)", 72, 41, 139, 148),
    v("Maged", "ar_SA? — name only (SAM approx)", 74, 58, 126, 128),
    v("Tarik", "ar — name only (SAM approx)", 75, 60, 124, 127),
    v("Milena", "ru_RU — name only (SAM approx)", 72, 39, 141, 150),
    v("Yuri", "ru_RU — name only (SAM approx)", 74, 59, 125, 128),
    v("Melina", "el_GR — name only (SAM approx)", 71, 40, 140, 149),
    v("Nikola", "bg? — name only (SAM approx)", 73, 58, 126, 128),
    v("Damayanti", "id_ID — name only (SAM approx)", 72, 41, 139, 148),
    v("Mariska", "hu_HU — name only (SAM approx)", 71, 40, 140, 149),
    v("Sinji", "alias → Sin-ji", 74, 50, 135, 140),
    v("Meijia", "alias → Mei-Jia", 72, 42, 140, 148),
    v("Tingting", "alias → Ting-Ting", 70, 40, 142, 150),
];

const fn v(
    name: &'static str,
    note: &'static str,
    speed: u8,
    pitch: u8,
    throat: u8,
    mouth: u8,
) -> SamVoice {
    SamVoice {
        name,
        note,
        speed,
        pitch,
        throat,
        mouth,
    }
}

/// Classic named presets (subset of [`all_voices`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SamPreset {
    Sam,
    Elf,
    #[default]
    Robot,
    Stuffy,
    Lady,
    Alien,
}

impl SamPreset {
    pub const ALL: &'static [SamPreset] = &[
        Self::Sam,
        Self::Elf,
        Self::Robot,
        Self::Stuffy,
        Self::Lady,
        Self::Alien,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Sam => "sam",
            Self::Elf => "elf",
            Self::Robot => "robot",
            Self::Stuffy => "stuffy",
            Self::Lady => "lady",
            Self::Alien => "alien",
        }
    }

    pub fn label(self) -> &'static str {
        self.voice().note
    }

    pub fn params(self) -> (u8, u8, u8, u8) {
        self.voice().params()
    }

    pub fn voice(self) -> &'static SamVoice {
        // Infallible: names exist in VOICES.
        find_voice(self.name()).expect("built-in preset missing from VOICES")
    }

    pub fn parse_name(s: &str) -> Result<Self, String> {
        let key = normalize_voice_key(s);
        match key.as_str() {
            "sam" | "default" => Ok(Self::Sam),
            "elf" => Ok(Self::Elf),
            "robot" | "little-robot" | "littlerobot" | "maxx" => Ok(Self::Robot),
            "stuffy" | "stuffy-guy" | "stuffyguy" => Ok(Self::Stuffy),
            "lady" | "little-old-lady" | "old-lady" | "littleoldlady" => Ok(Self::Lady),
            "alien" | "et" | "extra-terrestrial" | "extraterrestrial" | "xterrestrial" => {
                Ok(Self::Alien)
            }
            _ => Err(format!(
                "unknown SAM preset {s:?}; choose: {}",
                Self::ALL
                    .iter()
                    .map(|p| p.name())
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
        }
    }
}

fn normalize_voice_key(s: &str) -> String {
    s.trim()
        .to_ascii_lowercase()
        .chars()
        .map(|c| if c == '_' { '-' } else { c })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// All registered voices (classic SAM + macOS-name approximations).
pub fn all_voices() -> &'static [SamVoice] {
    VOICES
}

/// Resolve a voice by name (case-insensitive; spaces/`_`/`-` flexible).
pub fn find_voice(name: &str) -> Result<&'static SamVoice, String> {
    let key = normalize_voice_key(name);
    if key.is_empty() || key == "?" {
        return Err("__list_voices__".into());
    }
    // Exact / normalized match
    if let Some(v) = VOICES.iter().find(|v| normalize_voice_key(v.name) == key) {
        return Ok(v);
    }
    // Compact match: strip spaces and hyphens ("Bad News" ↔ "badnews")
    let compact = |s: &str| {
        normalize_voice_key(s)
            .chars()
            .filter(|c| *c != ' ' && *c != '-')
            .collect::<String>()
    };
    let key_c = compact(name);
    if let Some(v) = VOICES.iter().find(|v| compact(v.name) == key_c) {
        return Ok(v);
    }

    let mut names: Vec<&str> = VOICES.iter().map(|v| v.name).collect();
    names.sort_unstable();
    names.dedup();
    Err(format!(
        "unknown voice {name:?}; try `say -v ?` for the list ({} voices). Classic SAM: sam, elf, robot, stuffy, lady, alien",
        names.len()
    ))
}

/// Format voice list like macOS `say -v ?`.
pub fn format_voice_list() -> String {
    let mut lines = Vec::with_capacity(VOICES.len() + 4);
    lines.push(
        "Voices (SAM formant synthesis — macOS names are approximations, not Apple TTS):\n"
            .to_string(),
    );
    let mut rows: Vec<_> = VOICES.iter().collect();
    rows.sort_by(|a, b| {
        a.name
            .to_ascii_lowercase()
            .cmp(&b.name.to_ascii_lowercase())
    });
    for v in rows {
        lines.push(format!(
            "  {:<18} {}\n",
            v.name, v.note
        ));
    }
    lines.push(format!(
        "\n{} voices. Default: robot. Engine: rustsam @ {} Hz.\n",
        VOICES.len(),
        SAM_SAMPLE_RATE
    ));
    lines.concat()
}

#[derive(Debug)]
pub enum SamError {
    Recite(reciter::ReciterError),
    Parse(parser::ParseError),
    Empty,
}

/// Synthesize with the Maxx default voice (Little Robot, no sing mode).
pub fn synthesize(text: &str) -> Result<Vec<f32>, SamError> {
    synthesize_with(text, SamPreset::Robot, false)
}

/// Synthesize with a classic preset.
pub fn synthesize_with(
    text: &str,
    preset: SamPreset,
    sing: bool,
) -> Result<Vec<f32>, SamError> {
    synthesize_voice(text, preset.voice(), sing)
}

/// Synthesize with an arbitrary registered voice.
pub fn synthesize_voice(
    text: &str,
    voice: &SamVoice,
    sing: bool,
) -> Result<Vec<f32>, SamError> {
    let phoneme_str = reciter::text_to_phonemes(text).map_err(SamError::Recite)?;
    let phonemes = parser::parse_phonemes(&phoneme_str).map_err(SamError::Parse)?;
    if phonemes.is_empty() {
        return Err(SamError::Empty);
    }
    let (speed, pitch, throat, mouth) = voice.params();
    // rustsam: render(phonemes, pitch, mouth, throat, speed, sing_mode)
    let raw = renderer::render(&phonemes, pitch, mouth, throat, speed, sing);
    if raw.is_empty() {
        return Err(SamError::Empty);
    }
    Ok(raw
        .iter()
        .map(|&b| ((f32::from(b) - 128.0) / 128.0) * SAM_GAIN)
        .collect())
}

pub fn duration_secs(sample_count: usize) -> f64 {
    sample_count as f64 / f64::from(SAM_SAMPLE_RATE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthesize_maxxos_phrases() {
        for text in [
            "Hello. I am Maxx Steele.",
            "Good play.",
            "I'm ready.",
        ] {
            let samples = synthesize(text).unwrap_or_else(|e| panic!("{text:?}: {e:?}"));
            assert!(samples.len() > 1_000, "{text:?} too short");
        }
    }

    #[test]
    fn all_presets_synthesize() {
        for preset in SamPreset::ALL {
            let samples = synthesize_with("Hello.", *preset, false)
                .unwrap_or_else(|e| panic!("{:?}: {e:?}", preset));
            assert!(samples.len() > 500, "{:?} too short", preset);
        }
    }

    #[test]
    fn macos_voice_names_resolve() {
        for name in ["Alex", "Samantha", "Zarvox", "Bad News", "bad-news", "Fiona"] {
            find_voice(name).unwrap_or_else(|e| panic!("{name}: {e}"));
        }
    }

    #[test]
    fn default_robot_is_little_robot_params() {
        let v = find_voice("robot").unwrap();
        assert_eq!(v.params(), (92, 60, 190, 190));
    }

    #[test]
    fn parse_preset_names_and_aliases() {
        assert_eq!(SamPreset::parse_name("robot").unwrap(), SamPreset::Robot);
        assert_eq!(SamPreset::parse_name("et").unwrap(), SamPreset::Alien);
        assert!(SamPreset::parse_name("Alex").is_err()); // macOS name is find_voice, not preset enum
    }

    #[test]
    fn voice_list_nonempty() {
        let list = format_voice_list();
        assert!(list.contains("Alex"));
        assert!(list.contains("robot"));
        assert!(VOICES.len() > 50);
    }
}
