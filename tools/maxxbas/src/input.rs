use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use tempfile::NamedTempFile;

use crate::emit::{CART_SIZE, MUSIC_OFF, PHRASE_OFF};
use crate::validate::validate_cart;
use crate::Copyright;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputKind {
    MaxxBas,
    Rom,
}

pub fn input_kind(path: &Path) -> InputKind {
    match path
        .extension()
        .and_then(|s| s.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("bas") | Some("maxx") => InputKind::MaxxBas,
        _ => InputKind::Rom,
    }
}

pub struct ResolvedRom {
    pub path: PathBuf,
    _temp: Option<NamedTempFile>,
}

pub fn resolve_input(
    path: &Path,
    copyright: Copyright,
    output: Option<&Path>,
    tables_from: Option<&Path>,
) -> Result<ResolvedRom, String> {
    match input_kind(path) {
        InputKind::Rom => {
            if !path.is_file() {
                return Err(format!("{}: not found", path.display()));
            }
            Ok(ResolvedRom {
                path: path.to_path_buf(),
                _temp: None,
            })
        }
        InputKind::MaxxBas => compile_source_to_rom(path, copyright, output, tables_from),
    }
}

fn compile_source_to_rom(
    source: &Path,
    copyright: Copyright,
    output: Option<&Path>,
    tables_from: Option<&Path>,
) -> Result<ResolvedRom, String> {
    let image = read_and_compile(source, copyright, tables_from)?;

    if let Some(out) = output {
        fs::write(out, &image).map_err(|e| format!("{}: {e}", out.display()))?;
        return Ok(ResolvedRom {
            path: out.to_path_buf(),
            _temp: None,
        });
    }

    let mut temp = NamedTempFile::new().map_err(|e| format!("temp file: {e}"))?;
    temp.write_all(&image)
        .map_err(|e| format!("temp write: {e}"))?;
    let path = temp.path().to_path_buf();
    Ok(ResolvedRom {
        path,
        _temp: Some(temp),
    })
}

pub fn default_output(path: &Path) -> PathBuf {
    path.with_extension("532")
}

pub fn load_tables_from_reference(path: &Path) -> Result<(Vec<u8>, Vec<u8>), String> {
    let data = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if data.len() != CART_SIZE {
        return Err(format!("reference ROM expected {CART_SIZE} bytes, got {}", data.len()));
    }
    Ok((data[PHRASE_OFF..MUSIC_OFF].to_vec(), data[MUSIC_OFF..].to_vec()))
}

pub fn compile_to_output(
    source: &Path,
    output: &Path,
    copyright: Copyright,
    tables_from: Option<&Path>,
) -> Result<(), String> {
    let image = read_and_compile(source, copyright, tables_from)?;
    fs::write(output, &image).map_err(|e| format!("{}: {e}", output.display()))?;
    Ok(())
}

fn read_and_compile(
    source: &Path,
    copyright: Copyright,
    tables_from: Option<&Path>,
) -> Result<Vec<u8>, String> {
    let text = fs::read_to_string(source)
        .map_err(|e| format!("{}: {e}", source.display()))?;
    let (phrase_table, music_table) = if let Some(reference) = tables_from {
        let (phrase, music) = load_tables_from_reference(reference)?;
        (Some(phrase), Some(music))
    } else {
        (None, None)
    };
    let image = crate::emit::compile_source_with_tables(
        &text,
        copyright,
        phrase_table.as_deref(),
        music_table.as_deref(),
    )
    .map_err(|e| e.to_string())?;
    let issues = validate_cart(&image, 0xA000);
    if !issues.is_empty() {
        return Err(format!("validation failed: {}", issues.join("; ")));
    }
    if image.len() != CART_SIZE {
        return Err(format!("expected {CART_SIZE} bytes"));
    }
    Ok(image)
}