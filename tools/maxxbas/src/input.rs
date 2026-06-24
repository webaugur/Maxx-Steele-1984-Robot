use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use tempfile::NamedTempFile;

use crate::compile;
use crate::emit::CART_SIZE;
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
        InputKind::MaxxBas => compile_source_to_rom(path, copyright, output),
    }
}

fn compile_source_to_rom(
    source: &Path,
    copyright: Copyright,
    output: Option<&Path>,
) -> Result<ResolvedRom, String> {
    let image = read_and_compile(source, copyright)?;

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

pub fn compile_to_output(
    source: &Path,
    output: &Path,
    copyright: Copyright,
) -> Result<(), String> {
    let image = read_and_compile(source, copyright)?;
    fs::write(output, &image).map_err(|e| format!("{}: {e}", output.display()))?;
    Ok(())
}

fn read_and_compile(source: &Path, copyright: Copyright) -> Result<Vec<u8>, String> {
    let text = fs::read_to_string(source)
        .map_err(|e| format!("{}: {e}", source.display()))?;
    let image = compile(&text, copyright).map_err(|e| e.to_string())?;
    let issues = validate_cart(&image, 0xA000);
    if !issues.is_empty() {
        return Err(format!("validation failed: {}", issues.join("; ")));
    }
    if image.len() != CART_SIZE {
        return Err(format!("expected {CART_SIZE} bytes"));
    }
    Ok(image)
}