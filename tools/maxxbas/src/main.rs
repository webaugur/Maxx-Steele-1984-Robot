use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use maxxbas::{
    compile_to_output, decode_cart, default_output, format_listing, format_rom_listing,
    input_kind, parse_source, program_bytes, resolve_input, run_upload, upload_command,
    validate_cart_image, CartImage, Copyright, InputKind, CART_SIZE,
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
        dry_run: bool,
    },
    /// Preview program steps (simulator placeholder — outputs trace summary)
    Simulate {
        image: PathBuf,
        #[arg(long)]
        json: bool,
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
        } => cmd_compile(&source, output.as_deref(), &copyright, listing),
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
            dry_run,
        } => cmd_upload(
            &file,
            output.as_deref(),
            &device,
            &size,
            persist,
            &copyright,
            dry_run,
        ),
        Commands::Simulate { image, json } => cmd_simulate(&image, json),
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
) -> Result<(), String> {
    let copyright = parse_copyright(copyright_key)?;
    let out_path = output
        .map(Path::to_path_buf)
        .unwrap_or_else(|| default_output(source));

    compile_to_output(source, &out_path, copyright)?;
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
    dry_run: bool,
) -> Result<(), String> {
    let copyright = parse_copyright(copyright_key)?;
    let resolved = resolve_input(file, copyright, output)?;

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

fn cmd_simulate(image: &Path, json: bool) -> Result<(), String> {
    let cart = CartImage::load(image)?;
    let trace = decode_cart(&cart)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&trace).map_err(|e| e.to_string())?);
        return Ok(());
    }

    println!("Maxx Steele program simulator (preview)");
    println!("Copyright: {}", trace.copyright);
    println!("Steps: {}", trace.steps.len());
    println!();

    for step in &trace.steps {
        println!("{:3}  {}", step.index, step.comment);
    }

    println!();
    println!(
        "Vector graphics renderer not yet implemented. \
         Use `maxx list --json` for machine-readable program trace."
    );
    Ok(())
}