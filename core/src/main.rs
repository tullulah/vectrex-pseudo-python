mod lexer;    // Lexical analysis
mod ast;      // AST definitions
mod parser;   // Parsing logic
mod codegen;  // Optimization + backend dispatch
mod target;   // Target info & selection
mod backend;  // Backend modules declared in src/backend/mod.rs
mod resolver; // Multi-file import resolution
mod unifier;  // AST unification for multi-file projects
mod project;  // Project system
mod library;  // Library system
mod vecres;   // Vector resources (.vec)

use std::fs;
use std::path::PathBuf;
use clap::{Parser, Subcommand};

fn find_project_root() -> anyhow::Result<PathBuf> {
    let mut current = std::env::current_dir()?;
    loop {
        if current.join("Cargo.toml").exists() {
            return Ok(current);
        }
        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => return Err(anyhow::anyhow!("Could not find project root (no Cargo.toml found)")),
        }
    }
}
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
    /// Create a new library
    #[command(name = "lib-new")]
    LibNew {
        /// Name of the library to create
        name: String,
        /// Directory to create the library in (default: current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Initialize a new project
    Init {
        /// Project name
        name: String,
        /// Directory to create the project in (default: current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Compile a vector resource (.vec) to ASM
    #[command(name = "vec2asm")]
    Vec2Asm {
        /// Input .vec file
        input: PathBuf,
        /// Output .asm file (default: same name with .asm extension)
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    /// Create a new vector resource
    #[command(name = "vec-new")]
    VecNew {
        /// Resource name
        name: String,
        /// Output directory (default: current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
}

// main: parse CLI and dispatch subcommands.
fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build { input, out, target, title, bin } => build_cmd(&input, out.as_ref(), target, &title, bin),
        Commands::Lex { input } => lex_cmd(&input),
        Commands::Ast { input } => ast_cmd(&input),
        Commands::LibNew { name, path } => lib_new_cmd(&name, path.as_ref()),
        Commands::Init { name, path } => init_cmd(&name, path.as_ref()),
        Commands::Vec2Asm { input, out } => vec2asm_cmd(&input, out.as_ref()),
        Commands::VecNew { name, path } => vec_new_cmd(&name, path.as_ref()),
    }
}

// read_source: utility to load file contents.
fn read_source(path: &PathBuf) -> Result<String> { Ok(fs::read_to_string(path)?) }

// lib_new_cmd: create a new library skeleton
fn lib_new_cmd(name: &str, path: Option<&PathBuf>) -> Result<()> {
    let base_path = path.cloned().unwrap_or_else(|| std::env::current_dir().unwrap());
    eprintln!("Creating library '{}' in {:?}...", name, base_path);
    
    let lib_path = library::create_library(name, &base_path)?;
    
    eprintln!("✓ Library created at: {:?}", lib_path);
    eprintln!("\nStructure:");
    eprintln!("  {}/", name);
    eprintln!("  ├── library.vpylib");
    eprintln!("  ├── src/");
    eprintln!("  │   └── lib.vpy");
    eprintln!("  └── README.md");
    eprintln!("\nNext steps:");
    eprintln!("  1. Edit src/lib.vpy to add your library code");
    eprintln!("  2. Add more modules in src/");
    eprintln!("  3. Update library.vpylib with your info");
    
    Ok(())
}

// init_cmd: initialize a new project
fn init_cmd(name: &str, path: Option<&PathBuf>) -> Result<()> {
    let base_path = path.cloned().unwrap_or_else(|| std::env::current_dir().unwrap());
    let project_dir = base_path.join(name);
    
    eprintln!("Initializing project '{}' in {:?}...", name, project_dir);
    
    // Create directories
    fs::create_dir_all(&project_dir)?;
    fs::create_dir_all(project_dir.join("src"))?;
    
    // Create project.vpyproj
    let project_content = format!(r#"[project]
name = "{name}"
version = "0.1.0"
entry = "src/main.vpy"

[build]
target = "vectrex"
output = "build/{name}.bin"

[dependencies]
# Add libraries here:
# vectrex-stdlib = {{ path = "../vectrex-stdlib" }}
"#, name = name);
    
    fs::write(project_dir.join("project.vpyproj"), project_content)?;
    
    // Create main.vpy
    let main_content = format!(r#"# {name} - Main entry point
#
# This is the main file for your VPy project.

META TITLE = "{name}"

def main():
    # Initialization code
    Set_Intensity(127)

def loop():
    # Main game loop - runs every frame
    Wait_Recal()
    
    # Your game code here
    Move(0, 0)
    Draw_To(50, 50)
"#, name = name);
    
    fs::write(project_dir.join("src").join("main.vpy"), main_content)?;
    
    eprintln!("✓ Project created at: {:?}", project_dir);
    eprintln!("\nStructure:");
    eprintln!("  {}/", name);
    eprintln!("  ├── project.vpyproj");
    eprintln!("  └── src/");
    eprintln!("      └── main.vpy");
    eprintln!("\nNext steps:");
    eprintln!("  1. cd {}", name);
    eprintln!("  2. vectrexc build src/main.vpy");
    
    Ok(())
}

// lex_cmd: dump tokens for a source file.
fn lex_cmd(path: &PathBuf) -> Result<()> { let src = read_source(path)?; let tokens = lexer::lex(&src)?; for t in tokens { println!("{:?}", t); } Ok(()) }

// ast_cmd: pretty-print the parsed AST.
fn ast_cmd(path: &PathBuf) -> Result<()> { let src = read_source(path)?; let tokens = lexer::lex(&src)?; let module = parser::parse_with_filename(&tokens, &path.display().to_string())?; println!("{:#?}", module); Ok(()) }

// vec2asm_cmd: compile a .vec resource to ASM
fn vec2asm_cmd(input: &PathBuf, out: Option<&PathBuf>) -> Result<()> {
    eprintln!("Compiling vector resource: {:?}", input);
    
    let resource = vecres::VecResource::load(input)?;
    let asm = resource.compile_to_asm();
    
    let output_path = out.cloned().unwrap_or_else(|| input.with_extension("asm"));
    std::fs::write(&output_path, &asm)?;
    
    eprintln!("✓ Generated: {:?}", output_path);
    eprintln!("  Paths: {}, Points: {}", resource.visible_paths().len(), resource.point_count());
    
    Ok(())
}

// vec_new_cmd: create a new .vec resource
fn vec_new_cmd(name: &str, path: Option<&PathBuf>) -> Result<()> {
    let base_path = path.cloned().unwrap_or_else(|| std::env::current_dir().unwrap());
    let file_path = base_path.join(format!("{}.vec", name));
    
    eprintln!("Creating vector resource: {:?}", file_path);
    
    // Create a sample resource with a simple shape
    let mut resource = vecres::VecResource::new(name);
    resource.layers[0].paths.push(vecres::VecPath {
        name: "shape".to_string(),
        intensity: 127,
        closed: true,
        points: vec![
            vecres::Point { x: 0, y: 20 },
            vecres::Point { x: -15, y: -10 },
            vecres::Point { x: 15, y: -10 },
        ],
    });
    
    resource.save(&file_path)?;
    
    eprintln!("✓ Created: {:?}", file_path);
    eprintln!("\nEdit the file to add your vector graphics.");
    eprintln!("Then compile with: vectrexc vec2asm {}.vec", name);
    
    Ok(())
}
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
    
    // Phase 3.5: Multi-file resolution (if module has imports)
    let final_module = if !module.imports.is_empty() {
        eprintln!("Phase 3.5: Multi-file import resolution...");
        eprintln!("   Found {} import declarations", module.imports.len());
        
        // Determine project root (parent of src/ or same as file's parent)
        let file_dir = path.parent().unwrap_or(std::path::Path::new("."));
        let project_root = if file_dir.ends_with("src") {
            file_dir.parent().unwrap_or(file_dir).to_path_buf()
        } else {
            file_dir.to_path_buf()
        };
        
        eprintln!("   Project root: {}", project_root.display());
        
        // Create resolver and load all modules
        let mut resolver = resolver::ModuleResolver::new(project_root);
        resolver.load_project(path).map_err(|e| {
            eprintln!("❌ PHASE 3.5 FAILED: Import resolution error");
            eprintln!("   Error: {}", e);
            e
        })?;
        
        let loaded_count = resolver.get_all_modules().len();
        eprintln!("   Loaded {} module(s)", loaded_count);
        
        // Unify modules into single AST
        let entry_name = path.file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "main".to_string());
        
        let options = unifier::UnifyOptions::default();
        let unified = unifier::unify_modules(&resolver, &entry_name, &options).map_err(|e| {
            eprintln!("❌ PHASE 3.5 FAILED: AST unification error");
            eprintln!("   Error: {}", e);
            e
        })?;
        
        eprintln!("✓ Phase 3.5 SUCCESS: Unified {} items from {} modules", 
            unified.module.items.len(), loaded_count);
        
        unified.module
    } else {
        module
    };
    
    if tgt == target::Target::All {
        for ct in target::concrete_targets() {
            let asm = codegen::emit_asm(&final_module, *ct, &codegen::CodegenOptions {
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
                source_path: Some(path.canonicalize().unwrap_or_else(|_| path.clone()).display().to_string()),
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
        let (asm, debug_info, _diagnostics) = codegen::emit_asm_with_debug(&final_module, tgt, &codegen::CodegenOptions {
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
            source_path: Some(path.canonicalize().unwrap_or_else(|_| path.clone()).display().to_string()),
        });
        
        // Phase 4 validation: Check if assembly was actually generated
        if asm.is_empty() {
            eprintln!("❌ PHASE 4 FAILED: Empty assembly generated (0 bytes)");
            eprintln!("   This usually indicates:");
            eprintln!("   - Missing main() function or entry point");
            eprintln!("   - All code was filtered out or not executed");
            eprintln!("   - Internal codegen error (no assembly emitted)");
            return Err(anyhow::anyhow!("Code generation produced empty assembly"));
        }
        
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
        
        // Phase 5.5: Write .pdb file if debug info available
        let mut debug_info_mut = debug_info;
        if let Some(ref mut dbg) = debug_info_mut {
            eprintln!("Phase 5.5: Writing debug symbols file (.pdb)...");
            let pdb_path = out_path.with_extension("pdb");
            
            // If binary generation is requested, we'll update the PDB after Phase 6.5
            // Otherwise, write it now
            if !(bin && tgt == target::Target::Vectrex) {
                match dbg.to_json() {
                    Ok(json) => {
                        fs::write(&pdb_path, json).map_err(|e| {
                            eprintln!("⚠ Warning: Cannot write debug symbols file");
                            eprintln!("   Output path: {}", pdb_path.display());
                            eprintln!("   Error: {}", e);
                            e
                        })?;
                        eprintln!("✓ Phase 5.5 SUCCESS: Debug symbols written to {}", pdb_path.display());
                    },
                    Err(e) => {
                        eprintln!("⚠ Warning: Failed to serialize debug symbols: {}", e);
                    }
                }
            } else {
                eprintln!("Phase 5.5: Debug symbols write deferred until after binary generation");
            }
        } else {
            eprintln!("Phase 5.5: Debug symbols generation skipped (not supported for target={})", tgt);
        }
        
        // Phase 6: Binary assembly (if requested)
        if bin && tgt == target::Target::Vectrex { 
            eprintln!("Phase 6: Binary assembly requested...");
            assemble_bin(&out_path).map_err(|e| {
                eprintln!("❌ PHASE 6 FAILED: Binary assembly error");
                eprintln!("   Error: {}", e);
                e
            })?; 
            eprintln!("✓ Phase 6 SUCCESS: Binary generation complete");
            
            // Phase 6.5: Generate ASM address mapping (if debug info exists)
            if let Some(ref mut dbg) = debug_info_mut {
                eprintln!("Phase 6.5: Generating ASM address mapping...");
                let bin_path = out_path.with_extension("bin");
                
                match backend::asm_address_mapper::generate_asm_address_map(&out_path, &bin_path, dbg) {
                    Ok(()) => {
                        eprintln!("✓ Phase 6.5 SUCCESS: ASM address mapping complete");
                        
                        // Write updated PDB with address mappings
                        let pdb_path = out_path.with_extension("pdb");
                        match dbg.to_json() {
                            Ok(json) => {
                                fs::write(&pdb_path, json).map_err(|e| {
                                    eprintln!("⚠ Warning: Cannot write updated debug symbols file");
                                    eprintln!("   Output path: {}", pdb_path.display());
                                    eprintln!("   Error: {}", e);
                                    e
                                })?;
                                eprintln!("✓ Updated debug symbols with ASM address mappings: {}", pdb_path.display());
                            },
                            Err(e) => {
                                eprintln!("⚠ Warning: Failed to serialize updated debug symbols: {}", e);
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("⚠ Phase 6.5 WARNING: ASM address mapping failed: {}", e);
                        eprintln!("   Debugging will work but without precise ASM line mapping");
                    }
                }
            } else {
                eprintln!("Phase 6.5: ASM address mapping skipped (no debug info available)");
            }
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
    
    // PRIORITY 1: Try native M6809 assembler (integrated, no external dependencies)
    eprintln!("Attempting native M6809 assembler...");
    
    // Read ASM source
    let asm_source = fs::read_to_string(asm_path).map_err(|e| {
        eprintln!("❌ Failed to read ASM file: {}", e);
        e
    })?;
    
    // Extract ORG directive (default to Vectrex RAM start 0xC800)
    let org = extract_org_directive(&asm_source).unwrap_or(0xC800);
    eprintln!("Detected ORG address: 0x{:04X}", org);
    
    // Attempt native assembly
    match backend::asm_to_binary::assemble_m6809(&asm_source, org) {
        Ok((binary, _line_map)) => {
            // Write binary file
            fs::write(&bin_path, &binary).map_err(|e| {
                eprintln!("❌ Failed to write binary file: {}", e);
                e
            })?;
            eprintln!("✓ NATIVE ASSEMBLER SUCCESS: Generated {} bytes -> {}", 
                binary.len(), bin_path.display());
            return Ok(());
        },
        Err(e) => {
            eprintln!("⚠ Native assembler failed: {}", e);
            eprintln!("  Falling back to lwasm...");
        }
    }
    
    // FALLBACK: Use lwasm (original behavior)
    // Find project root by looking for Cargo.toml
    let project_root = find_project_root()?;
    eprintln!("Project root detected: {}", project_root.display());
    
    // Convert asm_path to absolute if it's relative
    let asm_path_abs = if asm_path.is_absolute() {
        asm_path.clone()
    } else {
        std::env::current_dir()?.join(asm_path)
    };
    
    // Convert bin_path to absolute if it's relative  
    let bin_path_abs = if bin_path.is_absolute() {
        bin_path.clone()
    } else {
        std::env::current_dir()?.join(&bin_path)
    };
    
    // Try lwasm first (system PATH)
    eprintln!("Attempting lwasm assembly...");
    let mut attempt_local = std::process::Command::new("lwasm")
        .arg("--6809")
        .arg("--format=raw")
        .arg(format!("--output={}", bin_path_abs.display()))
        .arg(&asm_path_abs)
        .current_dir(&project_root) // Always run from project root
        .output();
    
    // If system lwasm failed, try local lwasm in tools directory
    if attempt_local.is_err() {
        let local_lwasm = project_root.join("ide/frontend/public/tools/lwasm.exe");
        if local_lwasm.exists() {
            eprintln!("System lwasm not found, trying local lwasm...");
            attempt_local = std::process::Command::new(&local_lwasm)
                .arg("--6809")
                .arg("--format=raw")
                .arg(format!("--output={}", bin_path_abs.display()))
                .arg(&asm_path_abs)
                .current_dir(&project_root) // Always run from project root
                .output();
        }
    }
        
    let mut assembled_success = false;
    let mut lwasm_error_details = String::new();
    
    match attempt_local {
        Ok(res) => {
            if res.status.success() {
                eprintln!("✓ lwasm SUCCESS: {}", bin_path_abs.display());
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
        let script = project_root.join("ide/frontend/public/tools/lwasm.ps1");
        if script.exists() {
            let pw = std::process::Command::new("powershell")
                .arg("-NoProfile")
                .arg("-ExecutionPolicy").arg("Bypass")
                .arg("-File")
                .arg(&script)
                .arg(&asm_path_abs)
                .arg(&bin_path_abs)
                .current_dir(&project_root)
                .output();
            match pw {
                Ok(r) => {
                    if r.status.success() {
                        eprintln!("✓ PowerShell fallback SUCCESS: {}", bin_path_abs.display());
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
        match std::fs::metadata(&bin_path_abs) {
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
        if let Ok(mut data) = std::fs::read(&bin_path_abs) {
            let original_size = data.len();
            if original_size < 0x2000 { 
                data.resize(0x2000, 0); 
                std::fs::write(&bin_path_abs, &data)?; 
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

/// Extrae directiva ORG del código ASM (formato: ORG $C800 o ORG 0xC800)
fn extract_org_directive(asm: &str) -> Option<u16> {
    for line in asm.lines() {
        let trimmed = line.trim().to_uppercase();
        if trimmed.starts_with("ORG") {
            let addr_part = trimmed.trim_start_matches("ORG").trim();
            
            // Probar formato hex con $
            if let Some(hex_part) = addr_part.strip_prefix('$') {
                if let Ok(addr) = u16::from_str_radix(hex_part, 16) {
                    return Some(addr);
                }
            }
            
            // Probar formato hex con 0x
            if let Some(hex_part) = addr_part.strip_prefix("0X") {
                if let Ok(addr) = u16::from_str_radix(hex_part, 16) {
                    return Some(addr);
                }
            }
            
            // Probar formato decimal
            if let Ok(addr) = addr_part.parse::<u16>() {
                return Some(addr);
            }
        }
    }
    None
}
