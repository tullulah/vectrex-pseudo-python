mod lexer;    // Lexical analysis
mod ast;      // AST definitions
mod parser;   // Parsing logic
mod codegen;  // Optimization + backend dispatch
mod target;   // Target info & selection
mod backend; // Backend modules declared in src/backend/mod.rs

use std::fs;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "vectrexc", about = "Pseudo-Python multi-target assembler compiler (prototype)")]
struct Cli { #[command(subcommand)] command: Commands }

#[derive(Subcommand)]
enum Commands {
    Build {
        input: PathBuf,
        #[arg(short, long)] out: Option<PathBuf>,
        #[arg(long, default_value="vectrex")] target: target::Target,
        #[arg(long, default_value="UNTITLED")] title: String,
    #[arg(long, help="Generar también binario raw (.bin) con lwasm si está disponible")] bin: bool,
    },
    Lex { input: PathBuf },
    Ast { input: PathBuf },
}

// main: parse CLI and dispatch subcommands.
fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
    Commands::Build { input, out, target, title, bin } => build_cmd(&input, out.as_ref(), target, &title, bin),
        Commands::Lex { input } => lex_cmd(&input),
        Commands::Ast { input } => ast_cmd(&input),
    }
}

// read_source: utility to load file contents.
fn read_source(path: &PathBuf) -> Result<String> { Ok(fs::read_to_string(path)?) }

// lex_cmd: dump tokens for a source file.
fn lex_cmd(path: &PathBuf) -> Result<()> { let src = read_source(path)?; let tokens = lexer::lex(&src)?; for t in tokens { println!("{:?}", t); } Ok(()) }

// ast_cmd: pretty-print the parsed AST.
fn ast_cmd(path: &PathBuf) -> Result<()> { let src = read_source(path)?; let tokens = lexer::lex(&src)?; let module = parser::parse_with_filename(&tokens, &path.display().to_string())?; println!("{:#?}", module); Ok(()) }

// build_cmd: run full pipeline (lex/parse/opt/codegen) and write assembly.
fn build_cmd(path: &PathBuf, out: Option<&PathBuf>, tgt: target::Target, title: &str, bin: bool) -> Result<()> {
    let src = read_source(path)?;
    let tokens = lexer::lex(&src)?;
    let module = parser::parse_with_filename(&tokens, &path.display().to_string())?;
    if tgt == target::Target::All {
        for ct in target::concrete_targets() {
            let asm = codegen::emit_asm(&module, *ct, &codegen::CodegenOptions {
                title: title.to_string(),
                auto_loop: true,
                diag_freeze: false,
                force_extended_jsr: false,
                _bank_size: 0,
                per_frame_silence: false, // default off for minimal output
                debug_init_draw: false,   // default off for minimal output
                blink_intensity: false,
                exclude_ram_org: true,
                fast_wait: false,
            });
                let base = path.file_stem().unwrap().to_string_lossy();
                let out_path = out.cloned().unwrap_or_else(|| path.with_file_name(format!("{}-{}.asm", base, ct)));
                fs::write(&out_path, &asm)?;
                eprintln!("Generated: {} (target={})", out_path.display(), ct);
            // fast_wait desactivado en modo minimal
            if bin && *ct == target::Target::Vectrex {
                assemble_bin(&out_path)?;
            }
        }
        Ok(())
    } else {
        let asm = codegen::emit_asm(&module, tgt, &codegen::CodegenOptions {
            title: title.to_string(),
            auto_loop: true,
            diag_freeze: false,
            force_extended_jsr: false,
            _bank_size: 0,
            per_frame_silence: false,
            debug_init_draw: false,
            blink_intensity: false,
            exclude_ram_org: true,
            fast_wait: false,
        });
        let out_path = out.cloned().unwrap_or_else(|| path.with_extension("asm"));
    fs::write(&out_path, &asm)?;
        eprintln!("Generated: {} (target={})", out_path.display(), tgt);
        // fast_wait desactivado en modo minimal
        if bin && tgt == target::Target::Vectrex { assemble_bin(&out_path)?; }
        Ok(())
    }
}

fn assemble_bin(asm_path: &PathBuf) -> Result<()> {
    let bin_path = asm_path.with_extension("bin");
    let attempt_local = std::process::Command::new("lwasm")
        .arg("--format=raw")
        .arg("--output")
        .arg(&bin_path)
        .arg(asm_path)
        .output();
    let mut assembled_success = false;
    if let Ok(res) = &attempt_local {
        if res.status.success() {
            eprintln!("Assembled (lwasm): {}", bin_path.display());
            assembled_success = true;
        } else {
            eprintln!("lwasm fallo (status {}), stderr:\n{}", res.status, String::from_utf8_lossy(&res.stderr));
        }
    }
    if !assembled_success {
        let script = PathBuf::from("tools/lwasm.ps1");
        if script.exists() {
            let pw = std::process::Command::new("powershell")
                .arg("-NoProfile")
                .arg("-ExecutionPolicy").arg("Bypass")
                .arg("-File")
                .arg(&script)
                .arg(asm_path)
                .arg(&bin_path)
                .output();
            if let Ok(r) = pw {
                if r.status.success() {
                    eprintln!("Assembled (script fallback): {}", bin_path.display());
                    assembled_success = true;
                } else {
                    eprintln!("Script fallback fallo (status {}), stderr:\n{}", r.status, String::from_utf8_lossy(&r.stderr));
                }
            }
        }
    }
    if assembled_success {
        // Pad to minimum 8K so BIOS detects cartridge instead of launching MineStorm
        if let Ok(mut data) = std::fs::read(&bin_path) {
            if data.len() < 0x2000 { data.resize(0x2000, 0); std::fs::write(&bin_path, &data)?; eprintln!("Padded binary to 8192 bytes"); }
        }
    }
    Ok(())
}
