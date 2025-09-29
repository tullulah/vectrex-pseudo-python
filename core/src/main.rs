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
    eprintln!("=== COMPILATION PIPELINE START ===");
    eprintln!("Input file: {}", path.display());
    eprintln!("Target: {:?}", tgt);
    eprintln!("Binary generation: {}", if bin { "enabled" } else { "disabled" });
    
    // Phase 1: Read source
    eprintln!("Phase 1: Reading source file...");
    let src = read_source(path).map_err(|e| {
        eprintln!("❌ PHASE 1 FAILED: Cannot read source file");
        eprintln!("   Error: {}", e);
        e
    })?;
    eprintln!("✓ Phase 1 SUCCESS: Read {} characters", src.len());
    
    // Phase 2: Lexical analysis
    eprintln!("Phase 2: Lexical analysis (tokenization)...");
    let tokens = lexer::lex(&src).map_err(|e| {
        eprintln!("❌ PHASE 2 FAILED: Lexical analysis error");
        eprintln!("   Error: {}", e);
        eprintln!("   This usually means syntax errors in the source code (invalid characters, unclosed strings, etc.)");
        e
    })?;
    eprintln!("✓ Phase 2 SUCCESS: Generated {} tokens", tokens.len());
    
    // Phase 3: Syntax analysis (parsing)
    eprintln!("Phase 3: Syntax analysis (parsing)...");
    let module = parser::parse_with_filename(&tokens, &path.display().to_string()).map_err(|e| {
        eprintln!("❌ PHASE 3 FAILED: Syntax analysis error");
        eprintln!("   Error: {}", e);
        eprintln!("   This usually means syntax errors in the source code (invalid grammar, missing tokens, etc.)");
        e
    })?;
    eprintln!("✓ Phase 3 SUCCESS: Parsed module with {} top-level items", module.items.len());
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
        // Phase 4: Code generation
        eprintln!("Phase 4: Code generation (ASM emission)...");
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
        eprintln!("✓ Phase 4 SUCCESS: Generated {} bytes of assembly", asm.len());
        
        // Phase 5: Write ASM file
        eprintln!("Phase 5: Writing assembly file...");
        let out_path = out.cloned().unwrap_or_else(|| path.with_extension("asm"));
        fs::write(&out_path, &asm).map_err(|e| {
            eprintln!("❌ PHASE 5 FAILED: Cannot write assembly file");
            eprintln!("   Output path: {}", out_path.display());
            eprintln!("   Error: {}", e);
            e
        })?;
        eprintln!("✓ Phase 5 SUCCESS: Written to {} (target={})", out_path.display(), tgt);
        
        // Phase 6: Binary assembly (if requested)
        if bin && tgt == target::Target::Vectrex { 
            eprintln!("Phase 6: Binary assembly requested...");
            assemble_bin(&out_path).map_err(|e| {
                eprintln!("❌ PHASE 6 FAILED: Binary assembly error");
                eprintln!("   Error: {}", e);
                e
            })?; 
            eprintln!("✓ Phase 6 SUCCESS: Binary generation complete");
        } else {
            eprintln!("Phase 6: Binary assembly skipped (not requested or target not Vectrex)");
        }
        
        eprintln!("=== COMPILATION PIPELINE COMPLETE ===");
        Ok(())
    }
}

fn assemble_bin(asm_path: &PathBuf) -> Result<()> {
    let bin_path = asm_path.with_extension("bin");
    eprintln!("=== BINARY ASSEMBLY PHASE ===");
    eprintln!("ASM input: {}", asm_path.display());
    eprintln!("BIN output: {}", bin_path.display());
    
    // Try lwasm first (system PATH)
    eprintln!("Attempting lwasm assembly...");
    let mut attempt_local = std::process::Command::new("lwasm")
        .arg("--6809")
        .arg("--format=raw")
        .arg(format!("--output={}", bin_path.display()))
        .arg(asm_path)
        .output();
    
    // If system lwasm failed, try local lwasm in tools directory
    if attempt_local.is_err() {
        let local_lwasm = PathBuf::from("ide/frontend/public/tools/lwasm.exe");
        if local_lwasm.exists() {
            eprintln!("System lwasm not found, trying local lwasm...");
            attempt_local = std::process::Command::new(&local_lwasm)
                .arg("--6809")
                .arg("--format=raw")
                .arg(format!("--output={}", bin_path.display()))
                .arg(asm_path)
                .current_dir(".") // Ensure we're in project root
                .output();
        }
    }
        
    let mut assembled_success = false;
    let mut lwasm_error_details = String::new();
    
    match attempt_local {
        Ok(res) => {
            if res.status.success() {
                eprintln!("✓ lwasm SUCCESS: {}", bin_path.display());
                assembled_success = true;
            } else {
                let stderr_text = String::from_utf8_lossy(&res.stderr);
                let stdout_text = String::from_utf8_lossy(&res.stdout);
                lwasm_error_details = format!("lwasm failed (exit code: {})\nSTDERR:\n{}\nSTDOUT:\n{}", 
                    res.status, stderr_text, stdout_text);
                eprintln!("❌ {}", lwasm_error_details);
            }
        }
        Err(e) => {
            lwasm_error_details = format!("Failed to execute lwasm: {} (Is lwasm installed and in PATH?)", e);
            eprintln!("❌ {}", lwasm_error_details);
        }
    }
    
    // Try PowerShell fallback if lwasm failed
    if !assembled_success {
        eprintln!("Trying PowerShell fallback script...");
        let script = PathBuf::from("ide/frontend/public/tools/lwasm.ps1");
        if script.exists() {
            let pw = std::process::Command::new("powershell")
                .arg("-NoProfile")
                .arg("-ExecutionPolicy").arg("Bypass")
                .arg("-File")
                .arg(&script)
                .arg(asm_path)
                .arg(&bin_path)
                .output();
            match pw {
                Ok(r) => {
                    if r.status.success() {
                        eprintln!("✓ PowerShell fallback SUCCESS: {}", bin_path.display());
                        assembled_success = true;
                    } else {
                        let stderr_text = String::from_utf8_lossy(&r.stderr);
                        let stdout_text = String::from_utf8_lossy(&r.stdout);
                        eprintln!("❌ PowerShell fallback failed (exit code: {})\nSTDERR:\n{}\nSTDOUT:\n{}", 
                            r.status, stderr_text, stdout_text);
                    }
                }
                Err(e) => {
                    eprintln!("❌ PowerShell execution failed: {}", e);
                }
            }
        } else {
            eprintln!("❌ PowerShell fallback script not found: {}", script.display());
        }
    }
    
    // Final validation
    if assembled_success {
        // Check if binary was actually created and is not empty
        match std::fs::metadata(&bin_path) {
            Ok(metadata) => {
                if metadata.len() == 0 {
                    eprintln!("❌ CRITICAL ERROR: Binary file created but is EMPTY (0 bytes)");
                    eprintln!("   This usually indicates ASM syntax errors or missing ORG directive");
                    return Err(anyhow::anyhow!("Empty binary file generated"));
                } else {
                    eprintln!("✓ Binary validation passed: {} bytes", metadata.len());
                }
            }
            Err(e) => {
                eprintln!("❌ CRITICAL ERROR: Binary file not found after successful assembly: {}", e);
                return Err(anyhow::anyhow!("Binary file missing after assembly: {}", e));
            }
        }
        
        // Pad to minimum 8K so BIOS detects cartridge instead of launching MineStorm
        if let Ok(mut data) = std::fs::read(&bin_path) {
            let original_size = data.len();
            if original_size < 0x2000 { 
                data.resize(0x2000, 0); 
                std::fs::write(&bin_path, &data)?; 
                let remaining = 0x2000 - original_size;
                eprintln!("✓ Binary size: {} bytes (padded to 8192 bytes)", original_size); 
                eprintln!("✓ Available space: {} bytes ({} KB) remaining", remaining, remaining / 1024);
            } else if original_size == 0x2000 {
                eprintln!("✓ Binary size: {} bytes (8192 bytes - cartridge size limit reached)", original_size);
                eprintln!("⚠ Warning: Cartridge is at maximum size (8KB)");
            } else {
                eprintln!("❌ Binary size: {} bytes exceeds 8KB cartridge limit by {} bytes", original_size, original_size - 0x2000);
            }
        }
        eprintln!("=== BINARY ASSEMBLY COMPLETE ===");
    } else {
        eprintln!("=== BINARY ASSEMBLY FAILED ===");
        eprintln!("All assembly methods failed. Details:");
        eprintln!("{}", lwasm_error_details);
        return Err(anyhow::anyhow!("Binary assembly failed - see details above"));
    }
    
    Ok(())
}
