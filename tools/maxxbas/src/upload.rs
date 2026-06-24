use std::path::Path;
use std::process::Command;

pub const PICOROM_SIZES: &[(&str, &str)] = &[
    ("4kb", "32KBit"),
    ("27c512", "512KBit"),
];

pub fn picorom_size_token(size_key: &str) -> Result<&'static str, String> {
    PICOROM_SIZES
        .iter()
        .find(|(key, _)| *key == size_key)
        .map(|(_, token)| *token)
        .ok_or_else(|| {
            format!(
                "unknown size {size_key:?}; choose from: {}",
                PICOROM_SIZES
                    .iter()
                    .map(|(k, _)| *k)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
}

pub fn upload_command(
    rom_path: &Path,
    device: &str,
    size_key: &str,
    persist: bool,
) -> Result<Vec<String>, String> {
    let size_token = picorom_size_token(size_key)?;
    let mut cmd = vec![
        "picorom".into(),
        "upload".into(),
        device.into(),
        rom_path.display().to_string(),
        size_token.into(),
    ];
    if persist {
        cmd.push("-s".into());
    }
    Ok(cmd)
}

pub fn run_upload(cmd: &[String], dry_run: bool) -> Result<(), String> {
    if dry_run {
        println!("{}", cmd.join(" "));
        return Ok(());
    }

    let picorom = which::which("picorom").map_err(|_| {
        format!(
            "picorom not found in PATH.\nInstall from: https://github.com/wickerwaka/PicoROM/releases\n\nRun manually:\n  {}",
            cmd.join(" ")
        )
    })?;

    let args: Vec<&str> = cmd.iter().skip(1).map(String::as_str).collect();
    let status = Command::new(picorom)
        .args(&args)
        .status()
        .map_err(|e| format!("failed to run picorom: {e}"))?;

    if !status.success() {
        return Err(format!("picorom exited with status {status}"));
    }
    Ok(())
}