//! In-place editor for the CBS opcode list on the loaded cartridge.

use std::path::PathBuf;

use eframe::egui;

use super::interactive::InteractiveFirmware;

const OPCODES: &[(u8, &str)] = &[
    (0x00, "L left"),
    (0x01, "F forward"),
    (0x02, "b reverse"),
    (0x03, "r right"),
    (0x04, "Uu wrist up"),
    (0x05, "Ud wrist down"),
    (0x06, "Au arms up"),
    (0x07, "Ad arms down"),
    (0x08, "Cr claw rotate"),
    (0x09, "Cc claw"),
    (0x0A, "HL lamp"),
    (0x0B, "init home"),
    (0x0C, "d delay"),
    (0x0D, "Sn song"),
    (0x0E, "S speech"),
    (0x0F, "SS shift"),
    (0x10, "PS capture"),
    (0x81, "PLAY tune"),
    (0x82, "SPEE phrase"),
    (0x83, "SS RAM phrase"),
    (0x84, "CLr clear"),
];

/// 38 pairs fit before the phrase table at `$A081`. The robot prints FULL past `$037F`.
const MAX_PAIRS: usize = (crate::emit::PHRASE_OFF - crate::emit::PROG_OFF) / 2;

pub struct ProgramEditor {
    pub open: bool,
    steps: Vec<(u8, u8)>,
    has_cart: bool,
    status: String,
    save_path: String,
}

impl ProgramEditor {
    pub fn new() -> Self {
        Self {
            open: false,
            steps: Vec::new(),
            has_cart: false,
            status: String::new(),
            save_path: String::new(),
        }
    }

    pub fn load_from(&mut self, firmware: &InteractiveFirmware, default_save: String) {
        match firmware.program_pairs() {
            None => {
                self.has_cart = false;
                self.steps.clear();
                self.status =
                    "No cartridge program is loaded. The internal ROM is running.".into();
            }
            Some(Err(err)) => {
                self.has_cart = true;
                self.steps.clear();
                self.status = err;
            }
            Some(Ok(steps)) => {
                self.has_cart = true;
                self.steps = steps;
                self.status.clear();
                if self.save_path.is_empty() {
                    self.save_path = default_save;
                }
            }
        }
    }
}

enum Edit {
    Opcode(usize, u8),
    Operand(usize, u8),
    Insert(usize),
    Delete(usize),
    Save,
}

pub fn paint_program_editor(
    ctx: &egui::Context,
    editor: &mut ProgramEditor,
    firmware: &mut InteractiveFirmware,
) {
    if !editor.open {
        return;
    }
    let mut edit = None;
    egui::Window::new("Program")
        .open(&mut editor.open)
        .default_width(420.0)
        .show(ctx, |ui| {
            if !editor.has_cart {
                ui.label("No cartridge program is loaded. The internal ROM is running.");
                return;
            }
            ui.label(format!(
                "{} pairs. Max {MAX_PAIRS} before the phrase table ($A081). The robot prints FULL past $037F.",
                editor.steps.len()
            ));
            let last = editor.steps.len().saturating_sub(1);
            egui::ScrollArea::vertical().max_height(360.0).show(ui, |ui| {
                for i in 0..editor.steps.len() {
                    let end = editor.steps[i].0 == 0xFF && editor.steps[i].1 == 0xFF && i == last;
                    ui.horizontal(|ui| {
                        ui.label(format!("{i:02}"));
                        if end {
                            ui.label("End  FF FF");
                        } else {
                            let mut opcode = editor.steps[i].0;
                            let label = opcode_label(opcode);
                            egui::ComboBox::from_id_salt(("op", i))
                                .selected_text(label)
                                .show_ui(ui, |ui| {
                                    for &(code, name) in OPCODES {
                                        ui.selectable_value(&mut opcode, code, name);
                                    }
                                });
                            if opcode != editor.steps[i].0 {
                                edit = Some(Edit::Opcode(i, opcode));
                            }
                            let mut operand = editor.steps[i].1;
                            let resp = ui.add(
                                egui::DragValue::new(&mut operand)
                                    .range(0..=255)
                                    .hexadecimal(1, false, true),
                            );
                            if (resp.lost_focus() || resp.drag_stopped()) && operand != editor.steps[i].1
                            {
                                edit = Some(Edit::Operand(i, operand));
                            }
                            if ui.small_button("insert").clicked() {
                                edit = Some(Edit::Insert(i));
                            }
                            if ui.small_button("delete").clicked() {
                                edit = Some(Edit::Delete(i));
                            }
                        }
                    });
                }
            });
            ui.separator();
            ui.label("Save .532");
            ui.text_edit_singleline(&mut editor.save_path);
            if ui.button("Save").clicked() {
                edit = Some(Edit::Save);
            }
            if !editor.status.is_empty() {
                ui.label(editor.status.as_str());
            }
        });

    match edit {
        Some(Edit::Opcode(i, opcode)) => {
            let mut steps = editor.steps.clone();
            if let Some(step) = steps.get_mut(i) {
                step.0 = opcode;
            }
            apply(editor, firmware, steps);
        }
        Some(Edit::Operand(i, operand)) => {
            let mut steps = editor.steps.clone();
            if let Some(step) = steps.get_mut(i) {
                step.1 = operand;
            }
            apply(editor, firmware, steps);
        }
        Some(Edit::Insert(i)) => {
            if editor.steps.len() >= MAX_PAIRS {
                editor.status = format!("FULL: {MAX_PAIRS} pairs is the cartridge limit");
                return;
            }
            let mut steps = editor.steps.clone();
            steps.insert(i, (0x0C, 0x01));
            apply(editor, firmware, steps);
        }
        Some(Edit::Delete(i)) => {
            if editor.steps.len() <= 1 {
                editor.status = "The program must keep FF FF".into();
                return;
            }
            let mut steps = editor.steps.clone();
            steps.remove(i);
            apply(editor, firmware, steps);
        }
        Some(Edit::Save) => save(editor, firmware),
        None => {}
    }
}

fn apply(editor: &mut ProgramEditor, firmware: &mut InteractiveFirmware, steps: Vec<(u8, u8)>) {
    match firmware.set_program_pairs(&steps) {
        Ok(()) => {
            editor.steps = steps;
            editor.status = "Reset. The copy stub will run this list.".into();
        }
        Err(err) => editor.status = err,
    }
}

fn save(editor: &mut ProgramEditor, firmware: &InteractiveFirmware) {
    let path = editor.save_path.trim();
    if path.is_empty() {
        editor.status = "Enter a path for the .532 file.".into();
        return;
    }
    match firmware.write_cart_file(PathBuf::from(path).as_path()) {
        Ok(()) => editor.status = format!("Saved {path}"),
        Err(err) => editor.status = err,
    }
}

fn opcode_label(opcode: u8) -> String {
    OPCODES
        .iter()
        .find(|(code, _)| *code == opcode)
        .map(|(_, name)| format!("{name} ({opcode:02X})"))
        .unwrap_or_else(|| format!("unknown {opcode:02X}"))
}

pub fn default_save_path(portable_root: Option<&std::path::Path>) -> String {
    match portable_root {
        Some(root) => root.join("carts").join("edited.532").display().to_string(),
        None => String::new(),
    }
}
