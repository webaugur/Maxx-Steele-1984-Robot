use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use maxxbas::{compile, format_listing, parse_source, validate_cart, Copyright, CART_SIZE};

#[derive(Parser)]
#[command(
    name = "maxxbas",
    version,
    about = "Compile MaxxBAS source into 4 KB Maxx Steele cartridge ROM images"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile .bas / .maxx source to a .532 cartridge image
    Compile {
        /// MaxxBAS source file
        source: PathBuf,
        /// Output .532 path (default: same basename as source)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Copyright string: cbs or ultramaxx
        #[arg(long, default_value = "ultramaxx")]
        copyright: String,
        /// Print bytecode listing to stdout
        #[arg(long)]
        listing: bool,
    },
    /// Parse source without writing output
    Check {
        /// MaxxBAS source file
        source: PathBuf,
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
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile {
            source,
            output,
            copyright,
            listing,
        } => cmd_compile(&source, output.as_deref(), &copyright, listing),
        Commands::Check { source } => cmd_check(&source),
    }
}

fn cmd_compile(
    source: &Path,
    output: Option<&Path>,
    copyright_key: &str,
    listing: bool,
) -> Result<(), String> {
    let copyright = Copyright::from_str(copyright_key)
        .ok_or_else(|| format!("unknown copyright {copyright_key:?}; choose cbs or ultramaxx"))?;

    let text = fs::read_to_string(source)
        .map_err(|e| format!("{}: {e}", source.display()))?;

    let image = compile(&text, copyright).map_err(|e| e.to_string())?;

    let out_path = output
        .map(Path::to_path_buf)
        .unwrap_or_else(|| source.with_extension("532"));

    let issues = validate_cart(&image, 0xA000);
    if !issues.is_empty() {
        for issue in &issues {
            eprintln!("WARN: {issue}");
        }
        return Err("cartridge validation failed".into());
    }

    fs::write(&out_path, &image).map_err(|e| format!("{}: {e}", out_path.display()))?;
    println!("wrote {} ({CART_SIZE} bytes)", out_path.display());

    if listing {
        let program = parse_source(&text).map_err(|e| e.to_string())?;
        println!("{}", format_listing(&program));
    }

    Ok(())
}

fn cmd_check(source: &Path) -> Result<(), String> {
    let text = fs::read_to_string(source)
        .map_err(|e| format!("{}: {e}", source.display()))?;

    let program = parse_source(&text).map_err(|e| e.to_string())?;
    maxxbas::program_bytes(&program).map_err(|e| e.to_string())?;

    println!(
        "OK: {} instructions ({} bytes)",
        program.len(),
        program.len() * 2
    );
    Ok(())
}

