use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use maxxbas::{
    compile_to_output, decode_cart, default_output, format_listing, format_rom_listing,
    format_simulation, input_kind, parse_source, program_bytes, resolve_input,
    run_live_gui,
    run_simulation, run_upload, upload_command, validate_cart_image, CartImage, Copyright,
    InputKind, SimulationOptions, CART_SIZE,
};

#[derive(Parser)]
#[command(
    name = "maxx",
    version,
    about = "Maxx Steele toolchain — compile MaxxBAS, inspect ROMs, upload to PicoROM",
    long_about = "Unified CLI for Maxx Steele cartridge development.\n\
                  Compile .bas/.maxx sources, validate .532 images, list program steps \
                  (JSON for simulators), and upload to PicoROM."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile .bas / .maxx source to a .532 cartridge image
    Compile {
        source: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long, default_value = "ultramaxx")]
        copyright: String,
        #[arg(long)]
        listing: bool,
        /// Copy phrase/music tables from a reference .532 (factory SAY phrases)
        #[arg(long)]
        tables_from: Option<PathBuf>,
    },
    /// Parse MaxxBAS source without writing output
    Check {
        source: PathBuf,
    },
    /// Validate a .532 cartridge image structure
    Validate {
        image: PathBuf,
    },
    /// List program steps from a ROM image (text or JSON for simulators)
    List {
        image: PathBuf,
        /// Emit JSON program trace (for robot simulator)
        #[arg(long)]
        json: bool,
    },
    /// Compile (if needed) and upload to PicoROM
    Upload {
        /// .bas, .maxx, or .532 file
        file: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long, default_value = "maxx_cart")]
        device: String,
        #[arg(long, default_value = "4kb")]
        size: String,
        #[arg(short = 's', long)]
        persist: bool,
        #[arg(long, default_value = "ultramaxx")]
        copyright: String,
        #[arg(long)]
        tables_from: Option<PathBuf>,
        #[arg(long)]
        dry_run: bool,
    },
    /// Simulate program + robot model + patched internal ROM (unified simulator)
    #[command(visible_alias = "sim")]
    Simulate {
        /// .bas, .maxx, or .532 — omit with `--gui` to run internal ROM only (no cartridge)
        file: Option<PathBuf>,
        #[arg(long)]
        json: bool,
        /// Skip 65C02 firmware boot simulation
        #[arg(long)]
        no_firmware: bool,
        /// Max CPU cycles for firmware boot (default 25000)
        #[arg(long, default_value_t = 25000)]
        cycles: u64,
        /// Inject a fake keypad byte at $75 before firmware run
        #[arg(long)]
        key: Option<u8>,
        /// Write 64 KB patched memory image (masswerk virtual6502)
        #[arg(long)]
        image_out: Option<PathBuf>,
        #[arg(long, default_value = "ultramaxx")]
        copyright: String,
        #[arg(long)]
        tables_from: Option<PathBuf>,
        /// Text only — omit ASCII opcode visual storyboard
        #[arg(long)]
        plain: bool,
        /// Open interactive GUI (robot status + step playback)
        #[arg(long)]
        gui: bool,
    },
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    match Cli::parse().command {
        Commands::Compile {
            source,
            output,
            copyright,
            listing,
            tables_from,
        } => cmd_compile(
            &source,
            output.as_deref(),
            &copyright,
            listing,
            tables_from.as_deref(),
        ),
        Commands::Check { source } => cmd_check(&source),
        Commands::Validate { image } => cmd_validate(&image),
        Commands::List { image, json } => cmd_list(&image, json),
        Commands::Upload {
            file,
            output,
            device,
            size,
            persist,
            copyright,
            tables_from,
            dry_run,
        } => cmd_upload(
            &file,
            output.as_deref(),
            &device,
            &size,
            persist,
            &copyright,
            tables_from.as_deref(),
            dry_run,
        ),
        Commands::Simulate {
            file,
            json,
            no_firmware,
            cycles,
            key,
            image_out,
            copyright,
            tables_from,
            plain,
            gui,
        } => cmd_simulate(
            file.as_deref(),
            json,
            no_firmware,
            cycles,
            key,
            image_out.as_deref(),
            &copyright,
            tables_from.as_deref(),
            plain,
            gui,
        ),
    }
}

fn parse_copyright(key: &str) -> Result<Copyright, String> {
    Copyright::from_str(key)
        .ok_or_else(|| format!("unknown copyright {key:?}; choose cbs or ultramaxx"))
}

fn cmd_compile(
    source: &Path,
    output: Option<&Path>,
    copyright_key: &str,
    listing: bool,
    tables_from: Option<&Path>,
) -> Result<(), String> {
    let copyright = parse_copyright(copyright_key)?;
    let out_path = output
        .map(Path::to_path_buf)
        .unwrap_or_else(|| default_output(source));

    compile_to_output(source, &out_path, copyright, tables_from)?;
    println!("wrote {} ({CART_SIZE} bytes)", out_path.display());

    if listing {
        let text = fs::read_to_string(source)
            .map_err(|e| format!("{}: {e}", source.display()))?;
        let program = parse_source(&text).map_err(|e| e.to_string())?;
        println!("{}", format_listing(&program));
    }
    Ok(())
}

fn cmd_check(source: &Path) -> Result<(), String> {
    let text = fs::read_to_string(source)
        .map_err(|e| format!("{}: {e}", source.display()))?;
    let program = parse_source(&text).map_err(|e| e.to_string())?;
    program_bytes(&program).map_err(|e| e.to_string())?;
    println!(
        "OK: {} instructions ({} bytes)",
        program.len(),
        program.len() * 2
    );
    Ok(())
}

fn cmd_validate(image: &Path) -> Result<(), String> {
    let cart = CartImage::load(image)?;
    let issues = validate_cart_image(&cart);
    if issues.is_empty() {
        println!("OK: cartridge structure looks valid");
        Ok(())
    } else {
        for issue in issues {
            eprintln!("FAIL: {issue}");
        }
        Err("validation failed".into())
    }
}

fn cmd_list(image: &Path, json: bool) -> Result<(), String> {
    let cart = CartImage::load(image)?;
    let trace = decode_cart(&cart)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&trace).map_err(|e| e.to_string())?);
    } else {
        println!("{}", format_rom_listing(&trace));
    }
    Ok(())
}

fn cmd_upload(
    file: &Path,
    output: Option<&Path>,
    device: &str,
    size: &str,
    persist: bool,
    copyright_key: &str,
    tables_from: Option<&Path>,
    dry_run: bool,
) -> Result<(), String> {
    let copyright = parse_copyright(copyright_key)?;
    let resolved = resolve_input(file, copyright, output, tables_from)?;

    if input_kind(file) == InputKind::MaxxBas {
        if let Some(out) = output {
            println!("compiled {} -> {}", file.display(), out.display());
        } else {
            println!("compiled {} (temp ROM for upload)", file.display());
        }
    }

    let cart = CartImage::load(&resolved.path)?;
    let issues = validate_cart_image(&cart);
    if !issues.is_empty() {
        return Err(format!("validation failed: {}", issues.join("; ")));
    }

    let cmd = upload_command(&resolved.path, device, size, persist)?;
    if !dry_run {
        println!("{}", cmd.join(" "));
    }
    run_upload(&cmd, dry_run)
}

fn cmd_simulate(
    file: Option<&Path>,
    json: bool,
    no_firmware: bool,
    cycles: u64,
    key: Option<u8>,
    image_out: Option<&Path>,
    copyright_key: &str,
    tables_from: Option<&Path>,
    plain: bool,
    gui: bool,
) -> Result<(), String> {
    if gui {
        if json {
            eprintln!("note: --json ignored when --gui is set");
        }
        if no_firmware {
            return Err("--no-firmware is not supported with --gui".into());
        }
        return match file {
            Some(path) => {
                let copyright = parse_copyright(copyright_key)?;
                let resolved = resolve_input(path, copyright, None, tables_from)?;
                let cart = CartImage::load(&resolved.path)?;
                run_live_gui(Some(cart), path.display().to_string())
            }
            None => run_live_gui(None, "Internal ROM".to_string()),
        };
    }

    let path = file.ok_or(
        "simulate requires a firmware file, or use `maxx simulate --gui` for internal ROM only",
    )?;
    let copyright = parse_copyright(copyright_key)?;
    let resolved = resolve_input(path, copyright, None, tables_from)?;
    let cart = CartImage::load(&resolved.path)?;

    let report = run_simulation(
        &cart,
        &path.display().to_string(),
        &SimulationOptions {
            max_cycles: cycles,
            inject_key: key,
            run_firmware: !no_firmware,
            cart_bootstrap: !no_firmware,
            image_out: image_out.map(Path::to_path_buf),
            plain,
        },
    )?;

    if json {
        println!("{}", serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?);
    } else {
        print!("{}", format_simulation(&report, plain));
        if let Some(path) = image_out {
            println!("Wrote 64 KB sim image: {}", path.display());
        }
    }
    Ok(())
}