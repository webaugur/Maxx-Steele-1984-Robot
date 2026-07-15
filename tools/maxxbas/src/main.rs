use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use maxxbas::{
    compile_to_output, decode_cart, default_output, find_voice, format_listing, format_rom_listing,
    format_simulation, format_voice_list, input_kind, parse_source, play_samples, program_bytes,
    resolve_input, resolve_phrase_text, run_live_gui, run_simulation, run_upload, seed_boop_rng,
    split_statements, synthesize_boop, synthesize_boop_for_statement, synthesize_text_voice,
    upload_command, validate_cart_image, write_wav, CartImage, Copyright, InputKind,
    SimulationOptions, CART_SIZE,
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
    /// Speak text with Maxx SAM voice (Software Automatic Mouth; drop-in for macOS `say`)
    Say {
        /// Words to speak (joined with spaces). If empty, reads stdin.
        text: Vec<String>,
        /// Read text from a file instead of arguments / stdin
        #[arg(short = 'f', long)]
        file: Option<PathBuf>,
        /// Write mono 16-bit WAV instead of playing audio
        #[arg(short = 'o', long, value_name = "WAV")]
        output: Option<PathBuf>,
        /// Voice name (macOS `say -v` style). Use `-v ?` to list. Default: robot.
        /// Classic SAM: sam, elf, robot, stuffy, lady, alien. Also accepts common macOS names (SAM approx).
        #[arg(
            short = 'v',
            long = "voice",
            value_name = "NAME",
            default_value = "robot",
            visible_alias = "preset"
        )]
        voice: String,
        /// SAM sing mode (steadier pitch for melodic speech)
        #[arg(short = 's', long)]
        sing: bool,
        /// After each statement, play an emotive beep-boop (punctuation-aware; else random).
        /// If the text contains !, use single quotes: bash expands !! inside "..." before say runs.
        #[arg(long)]
        boop: bool,
        /// Force a named boop pattern (implies --boop): greet, curious, happy, affirm, …
        #[arg(long, value_name = "NAME")]
        boop_pattern: Option<String>,
        /// Speak a built-in Maxx phrase by index (e.g. 0x10 or 16)
        #[arg(long, value_name = "INDEX")]
        phrase: Option<String>,
        /// Print the resolved text on stdout (still speaks / writes unless --dry-run)
        #[arg(long)]
        print: bool,
        /// Resolve text only; do not play or write audio
        #[arg(long)]
        dry_run: bool,
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
        Commands::Say {
            text,
            file,
            output,
            voice,
            sing,
            boop,
            boop_pattern,
            phrase,
            print,
            dry_run,
        } => cmd_say(
            text,
            file.as_deref(),
            output.as_deref(),
            &voice,
            sing,
            boop,
            boop_pattern.as_deref(),
            phrase.as_deref(),
            print,
            dry_run,
        ),
    }
}

fn parse_phrase_index(s: &str) -> Result<u8, String> {
    let t = s.trim();
    if let Some(hex) = t
        .strip_prefix("0x")
        .or_else(|| t.strip_prefix("0X"))
        .or_else(|| t.strip_prefix('$'))
    {
        u8::from_str_radix(hex, 16).map_err(|e| format!("invalid phrase index {s:?}: {e}"))
    } else {
        t.parse::<u8>()
            .map_err(|e| format!("invalid phrase index {s:?}: {e}"))
    }
}

fn cmd_say(
    words: Vec<String>,
    file: Option<&Path>,
    output: Option<&Path>,
    voice_name: &str,
    sing: bool,
    boop: bool,
    boop_pattern: Option<&str>,
    phrase: Option<&str>,
    print: bool,
    dry_run: bool,
) -> Result<(), String> {
    // macOS: `say -v ?` lists voices
    let trimmed_voice = voice_name.trim();
    if trimmed_voice == "?" || trimmed_voice.eq_ignore_ascii_case("list") {
        print!("{}", format_voice_list());
        return Ok(());
    }

    let voice = find_voice(voice_name).map_err(|e| {
        if e == "__list_voices__" {
            format_voice_list()
        } else {
            e
        }
    })?;

    let want_boop = boop || boop_pattern.is_some();
    let fixed_boop = if let Some(name) = boop_pattern {
        if name.trim() == "?" || name.eq_ignore_ascii_case("list") {
            eprintln!("Boop patterns:");
            for p in maxxbas::BOOP_PATTERNS {
                eprintln!("  {:<10} {}", p.name, p.mood);
            }
            return Ok(());
        }
        Some(
            maxxbas::find_boop(name).ok_or_else(|| {
                format!(
                    "unknown boop pattern {name:?}; try --boop-pattern ? (greet, curious, happy, …)"
                )
            })?,
        )
    } else {
        None
    };

    // Mild entropy so successive `say --boop` runs differ
    seed_boop_rng(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(1)
            ^ std::process::id() as u64,
    );

    let text = if let Some(idx_s) = phrase {
        if file.is_some() || !words.is_empty() {
            return Err("use either --phrase or text/file, not both".into());
        }
        let idx = parse_phrase_index(idx_s)?;
        resolve_phrase_text(idx)?.to_string()
    } else if let Some(path) = file {
        if !words.is_empty() {
            return Err("use either --file or text arguments, not both".into());
        }
        fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?
    } else if !words.is_empty() {
        words.join(" ")
    } else {
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| format!("stdin: {e}"))?;
        buf
    };

    let text = text.trim();
    if text.is_empty() {
        return Err("no text to speak (pass words, --file, --phrase, or stdin)".into());
    }

    let statements = if want_boop {
        split_statements(text)
    } else {
        vec![text.to_string()]
    };

    if print || dry_run {
        for stmt in &statements {
            println!("{stmt}");
        }
        if dry_run {
            let sing_note = if sing { " sing" } else { "" };
            let boop_note = if want_boop {
                if let Some(p) = fixed_boop {
                    format!(" boop={}", p.name)
                } else {
                    let preview: Vec<_> = statements
                        .iter()
                        .map(|s| maxxbas::select_boop_for_statement(s).name)
                        .collect();
                    format!(" boop=auto[{}]", preview.join(", "))
                }
            } else {
                String::new()
            };
            eprintln!(
                "voice={} ({}){sing_note}{boop_note} statements={}",
                voice.name,
                voice.note,
                statements.len()
            );
        }
    }
    if dry_run {
        return Ok(());
    }

    let mut samples: Vec<f32> = Vec::new();
    let mut boop_names: Vec<&str> = Vec::new();
    for (i, stmt) in statements.iter().enumerate() {
        if i > 0 {
            // short pause between statements
            let gap = (maxxbas::SAY_SAMPLE_RATE as f32 * 0.12) as usize;
            samples.extend(std::iter::repeat_n(0.0, gap));
        }
        samples.extend(synthesize_text_voice(stmt, voice, sing)?);
        if want_boop {
            let (name, boop_samples) = if let Some(p) = fixed_boop {
                (p.name, synthesize_boop(p))
            } else {
                let (p, s) = synthesize_boop_for_statement(stmt);
                (p.name, s)
            };
            boop_names.push(name);
            samples.extend(boop_samples);
        }
    }

    if let Some(path) = output {
        write_wav(path, &samples)?;
        eprintln!(
            "wrote {} ({} samples @ {} Hz, voice={}, sing={sing}{})",
            path.display(),
            samples.len(),
            maxxbas::SAY_SAMPLE_RATE,
            voice.name,
            if boop_names.is_empty() {
                String::new()
            } else {
                format!(", boops=[{}]", boop_names.join(", "))
            }
        );
    } else {
        if !boop_names.is_empty() {
            eprintln!("boops: {}", boop_names.join(", "));
        }
        play_samples(&samples)?;
    }
    Ok(())
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