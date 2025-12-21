mod lexer;    // Lexical analysis
mod ast;      // AST definitions
mod parser;   // Parsing logic
mod codegen;  // Optimization + backend dispatch
mod target;   // Target info & selection
mod backend;  // Backend modules declared in src/backend/mod.rs
mod resolver; // Multi-file import resolution
mod unifier;  // AST unification for multi-file projects
mod library;  // Library system
mod vecres;   // Vector resources (.vec)
mod musres;   // Music resources (.vmus)
mod sfxres;   // Sound effects resources (.vsfx)
mod struct_layout; // Struct layout computation

use std::fs;
use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use toml;

#[allow(dead_code)]
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

/// Discover assets (.vec and .vmus files) in project directory
/// Phase 0: Asset Discovery
fn discover_assets(source_path: &Path) -> Vec<codegen::AssetInfo> {
    let mut assets = Vec::new();
    
    // Determine project root - convert to absolute path first to avoid cwd confusion
    let abs_source = source_path.canonicalize().unwrap_or_else(|_| source_path.to_path_buf());
    
    let project_root: PathBuf = if let Some(parent) = abs_source.parent() {
        if parent.file_name().and_then(|n| n.to_str()) == Some("src") {
            // Source is in src/ directory, project root is parent
            parent.parent().unwrap_or(parent).to_path_buf()
        } else {
            // Source is not in src/, assume parent is project root
            parent.to_path_buf()
        }
    } else {
        // No parent (shouldn't happen with absolute path), use source itself
        abs_source.clone()
    };
    
    // Search for vector assets (assets/vectors/*.vec)
    let vectors_dir = project_root.join("assets").join("vectors");
    if vectors_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&vectors_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("vec") {
                    if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                        assets.push(codegen::AssetInfo {
                            name: name.to_string(),
                            path: path.display().to_string(),
                            asset_type: codegen::AssetType::Vector,
                        });
                    }
                }
            }
        }
    }
    
    // Search for music assets (assets/music/*.vmus)
    let music_dir = project_root.join("assets").join("music");
    if music_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&music_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("vmus") {
                    if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                        assets.push(codegen::AssetInfo {
                            name: name.to_string(),
                            path: path.display().to_string(),
                            asset_type: codegen::AssetType::Music,
                        });
                    }
                }
            }
        }
    }
    
    // Search for sound effects (assets/sfx/*.vsfx)
    let sfx_dir = project_root.join("assets").join("sfx");
    if sfx_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&sfx_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("vsfx") {
                    if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                        assets.push(codegen::AssetInfo {
                            name: name.to_string(),
                            path: path.display().to_string(),
                            asset_type: codegen::AssetType::Sfx,
                        });
                    }
                }
            }
        }
    }
    
    // Log discovered assets
    if !assets.is_empty() {
        eprintln!("✓ Discovered {} asset(s):", assets.len());
        for asset in &assets {
            let type_str = match asset.asset_type {
                codegen::AssetType::Vector => "Vector",
                codegen::AssetType::Music => "Music",
                codegen::AssetType::Sfx => "SFX",
            };
            eprintln!("  - {} ({})", asset.name, type_str);
        }
    }
    
    assets
}

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
        #[arg(long, help="Generar también binario raw (.bin) con ensamblador nativo M6809")] bin: bool,
        #[arg(long, help="Usar lwasm externo en lugar del ensamblador nativo (útil para comparar/diagnosticar)")] use_lwasm: bool,
        #[arg(long, help="Compilar con AMBOS ensambladores y comparar resultados (requiere lwasm instalado)")] dual: bool,
        #[arg(short = 'p', long, help="Compilar proyecto .vpyproj (ignora -f si se especifica)")] project: bool,
        #[arg(short = 'f', long, help="Compilar archivo .vpy individual (default)")] file: bool,
        #[arg(long = "include-dir", help="Directorio con archivos include (VECTREX.I, etc)")] include_dir: Option<PathBuf>,
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
        Commands::Build { input, out, target, title, bin, use_lwasm, dual, project, file: _, include_dir } => {
            // Si -p está especificado o el input es .vpyproj, compilar como proyecto
            if project || input.extension().and_then(|e| e.to_str()) == Some("vpyproj") {
                build_project_cmd(&input, bin, use_lwasm, dual, include_dir.as_ref())
            } else {
                build_cmd(&input, out.as_ref(), target, &title, bin, use_lwasm, dual, include_dir.as_ref())
            }
        },
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
            vecres::Point { x: 0, y: 20, intensity: None },
            vecres::Point { x: -15, y: -10, intensity: None },
            vecres::Point { x: 15, y: -10, intensity: None },
        ],
    });
    
    resource.save(&file_path)?;
    
    eprintln!("✓ Created: {:?}", file_path);
    eprintln!("\nEdit the file to add your vector graphics.");
    eprintln!("Then compile with: vectrexc vec2asm {}.vec", name);
    
    Ok(())
}

// build_project_cmd: compile a .vpyproj project file
fn build_project_cmd(project_path: &PathBuf, bin: bool, use_lwasm: bool, dual: bool, include_dir: Option<&PathBuf>) -> Result<()> {
    eprintln!("=== PROJECT COMPILATION START ===");
    eprintln!("Project file: {}", project_path.display());
    
    // Read and parse .vpyproj file
    let project_content = fs::read_to_string(project_path)?;
    let project_toml: toml::Value = toml::from_str(&project_content)?;
    
    // Extract project root (parent directory of .vpyproj)
    let project_root = project_path.parent()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine project root"))?;
    
    // Extract entry file from [project] section
    let entry_relative = project_toml.get("project")
        .and_then(|p| p.get("entry"))
        .and_then(|e| e.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing [project] entry field in .vpyproj"))?;
    
    let entry_file = project_root.join(entry_relative);
    
    // Extract output path from [build] section
    // Note: .vpyproj defines bin path, but build_cmd expects ASM path
    let output_relative = project_toml.get("build")
        .and_then(|b| b.get("output"))
        .and_then(|o| o.as_str());
    
    let output_path = output_relative.map(|o| {
        let bin_path = project_root.join(o);
        // Derive ASM path from bin path (change .bin to .asm)
        bin_path.with_extension("asm")
    });
    
    // Extract target from [build] section
    let target_str = project_toml.get("build")
        .and_then(|b| b.get("target"))
        .and_then(|t| t.as_str())
        .unwrap_or("vectrex");
    
    let target = match target_str {
        "vectrex" => target::Target::Vectrex,
        "pitrex" => target::Target::Pitrex,
        "vecfever" => target::Target::Vecfever,
        "vextreme" => target::Target::Vextreme,
        "all" => target::Target::All,
        _ => target::Target::Vectrex,
    };
    
    // Extract title from [project] section
    let title = project_toml.get("project")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("UNTITLED");
    
    eprintln!("✓ Project: {}", title);
    eprintln!("✓ Entry file: {}", entry_file.display());
    if let Some(ref out) = output_path {
        eprintln!("✓ Output: {}", out.display());
    }
    
    // Call regular build_cmd with project-resolved paths
    build_cmd(&entry_file, output_path.as_ref(), target, title, bin, use_lwasm, dual, include_dir)
}

// build_cmd: run full pipeline (lex/parse/opt/codegen) and write assembly.
fn build_cmd(path: &PathBuf, out: Option<&PathBuf>, tgt: target::Target, title: &str, bin: bool, use_lwasm: bool, dual: bool, include_dir: Option<&PathBuf>) -> Result<()> {
    eprintln!("=== COMPILATION PIPELINE START ===");
    eprintln!("Input file: {}", path.display());
    eprintln!("Target: {:?}", tgt);
    eprintln!("Binary generation: {}", if bin { "enabled" } else { "disabled" });
    if bin && dual {
        eprintln!("Assembler: DUAL MODE (native + lwasm comparison)");
    } else if bin && use_lwasm {
        eprintln!("Assembler: lwasm (external)");
    } else if bin {
        eprintln!("Assembler: native M6809 (integrated)");
    }
    
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
                assets: vec![], // TODO: Implement asset discovery
                const_values: std::collections::BTreeMap::new(), // Will be populated by backend
                structs: std::collections::HashMap::new(), // Empty registry for non-struct code
                type_context: std::collections::HashMap::new(), // Empty type context for non-struct code
            });
                let base = path.file_stem().unwrap().to_string_lossy();
                let out_path = out.cloned().unwrap_or_else(|| path.with_file_name(format!("{}-{}.asm", base, ct)));
                fs::write(&out_path, &asm)?;
                eprintln!("Generated: {} (target={})", out_path.display(), ct);
            // fast_wait desactivado en modo minimal
            if bin && *ct == target::Target::Vectrex {
                // When generating for all targets, always use native assembler
                assemble_bin(&out_path, false, include_dir)?;
            }
        }
        Ok(())
    } else {
        // Phase 0: Asset discovery
        eprintln!("Phase 0: Asset discovery...");
        let assets = discover_assets(&path);
        
        // Phase 4: Code generation
        eprintln!("Phase 4: Code generation (ASM emission)...");
        let (asm, debug_info, diagnostics) = codegen::emit_asm_with_debug(&final_module, tgt, &codegen::CodegenOptions {
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
            assets,
            const_values: std::collections::BTreeMap::new(), // Will be populated by backend
            structs: std::collections::HashMap::new(), // Will be populated by emit_asm_with_debug
            type_context: std::collections::HashMap::new(), // Will be populated during semantic validation
        });
        
        // Phase 4 validation: Check if assembly was actually generated
        if asm.is_empty() {
            eprintln!("❌ PHASE 4 FAILED: Empty assembly generated (0 bytes)");
            if !diagnostics.is_empty() {
                eprintln!("   Semantic errors detected:");
                for diag in &diagnostics {
                    if let (Some(line), Some(col)) = (diag.line, diag.col) {
                        eprintln!("   error {}:{} - {}", line, col, diag.message);
                    } else {
                        eprintln!("   error - {}", diag.message);
                    }
                }
            } else {
                eprintln!("   This usually indicates:");
                eprintln!("   - Missing main() function or entry point");
                eprintln!("   - All code was filtered out or not executed");
                eprintln!("   - Internal codegen error (no assembly emitted)");
            }
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
            if dual {
                assemble_dual(&out_path, include_dir).map_err(|e| {
                    eprintln!("❌ PHASE 6 FAILED: Dual assembly error");
                    eprintln!("   Error: {}", e);
                    e
                })?;
            } else {
                assemble_bin(&out_path, use_lwasm, include_dir).map_err(|e| {
                    eprintln!("❌ PHASE 6 FAILED: Binary assembly error");
                    eprintln!("   Error: {}", e);
                    e
                })?;
            }
            eprintln!("✓ Phase 6 SUCCESS: Binary generation complete");
            
            // Phase 6.5: Generate ASM address mapping (DISABLED - BinaryEmitter already has correct addresses)
            // TODO: Export emitter.symbols from assemble_asm_to_binary() and use those instead of re-estimating
            if let Some(ref mut dbg) = debug_info_mut {
                eprintln!("Phase 6.5: ASM address mapping (skipped - using BinaryEmitter addresses)...");
                let _bin_path = out_path.with_extension("bin");
                
                // DISABLED: This re-estimates addresses and gets them wrong for large binaries
                // match backend::asm_address_mapper::generate_asm_address_map(&out_path, &bin_path, dbg) {
                if false {
                    // Ok(()) => {
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
                    // },
                    // Err(e) => {
                    //     eprintln!("⚠ Phase 6.5 WARNING: ASM address mapping failed: {}", e);
                    //     eprintln!("   Debugging will work but without precise ASM line mapping");
                    // }
                }
                eprintln!("✓ Phase 6.5 SKIPPED - Using BinaryEmitter symbol addresses (more accurate)");
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

fn assemble_bin(asm_path: &PathBuf, use_lwasm: bool, include_dir: Option<&PathBuf>) -> Result<()> {
    let bin_path = asm_path.with_extension("bin");
    eprintln!("=== BINARY ASSEMBLY PHASE ===");
    eprintln!("ASM input: {}", asm_path.display());
    eprintln!("BIN output: {}", bin_path.display());
    
    // Read ASM source
    let asm_source = fs::read_to_string(asm_path).map_err(|e| {
        eprintln!("❌ Failed to read ASM file: {}", e);
        e
    })?;
    
    let binary = if use_lwasm {
        // Option 1: External lwasm assembler
        eprintln!("Using external lwasm assembler...");
        eprintln!("NOTE: lwasm does NOT generate debug symbols (.pdb)");
        eprintln!("      Breakpoints and line mapping will NOT work with this option");
        
        // Check if lwasm is installed
        let lwasm_check = std::process::Command::new("lwasm")
            .arg("--version")
            .output();
        
        if lwasm_check.is_err() {
            eprintln!("❌ ERROR: lwasm not found in PATH");
            eprintln!("   Install lwasm from: http://www.lwtools.ca/");
            eprintln!("   macOS: brew install lwtools");
            eprintln!("   Linux: apt-get install lwtools or build from source");
            eprintln!("   Or use native assembler (remove --use-lwasm flag)");
            return Err(anyhow::anyhow!("lwasm not installed"));
        }
        
        // Use temporary file for lwasm output (we'll add padding later)
        let temp_bin = bin_path.with_extension("bin.tmp");
        
        // Determine include directory (use provided or current working directory)
        let inc_dir = include_dir
            .map(|p| p.to_path_buf())
            .or_else(|| std::env::current_dir().ok())
            .unwrap_or_else(|| PathBuf::from("."));
        
        // Run lwasm with include directory (use include_dir so "include/VECTREX.I" works)
        let output = std::process::Command::new("lwasm")
            .arg("--format=raw")
            .arg("-I")
            .arg(&inc_dir)
            .arg(format!("--output={}", temp_bin.display()))
            .arg(asm_path)
            .output()
            .map_err(|e| {
                eprintln!("❌ Failed to execute lwasm: {}", e);
                e
            })?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            eprintln!("❌ lwasm assembly failed:");
            if !stdout.is_empty() {
                eprintln!("STDOUT:\n{}", stdout);
            }
            if !stderr.is_empty() {
                eprintln!("STDERR:\n{}", stderr);
            }
            return Err(anyhow::anyhow!("lwasm assembly failed"));
        }
        
        eprintln!("✓ lwasm assembly successful");
        
        // Read generated binary (will be padded later)
        let bin_data = fs::read(&temp_bin).map_err(|e| {
            eprintln!("❌ Failed to read lwasm output: {}", e);
            e
        })?;
        
        // Clean up temp file
        let _ = fs::remove_file(&temp_bin);
        
        bin_data
    } else {
        // Option 2: Native M6809 assembler (default)
        eprintln!("Using native M6809 assembler (integrated)...");
        
        // Extract ORG directive (default to Vectrex RAM start 0xC800)
        let org = extract_org_directive(&asm_source).unwrap_or(0xC800);
        eprintln!("Detected ORG address: 0x{:04X}", org);
        
        // Set include directory for assembler
        backend::asm_to_binary::set_include_dir(include_dir.map(|p| p.to_path_buf()));
        
        // Assemble with native assembler
        let (binary, _line_map) = backend::asm_to_binary::assemble_m6809(&asm_source, org)
            .map_err(|e| {
                eprintln!("❌ Native assembler failed: {}", e);
                eprintln!("\nPlease fix the assembly errors above.");
                eprintln!("\nAlternative: Use --use-lwasm flag to try external lwasm assembler");
                eprintln!("             (WARNING: No debug symbols with lwasm)");
                anyhow::anyhow!("Native assembler failed: {}", e)
            })?;
        
        eprintln!("✓ Native assembler successful");
        binary
    };
    
    // Validate binary is not empty
    if binary.is_empty() {
        eprintln!("❌ CRITICAL ERROR: Binary is EMPTY (0 bytes)");
        eprintln!("   This usually indicates ASM syntax errors or missing ORG directive");
        return Err(anyhow::anyhow!("Empty binary generated"));
    }
    
    let original_size = binary.len();
    eprintln!("✓ Assembler generated: {} bytes", original_size);
    
    // Pad to minimum 8K so BIOS detects cartridge instead of launching MineStorm
    let mut data = binary;
    if original_size < 0x2000 { 
        data.resize(0x2000, 0); 
        let remaining = 0x2000 - original_size;
        eprintln!("✓ Padded to 8192 bytes (available space: {} bytes / {} KB)", 
            remaining, remaining / 1024);
    } else if original_size == 0x2000 {
        eprintln!("⚠ Cartridge is at maximum size (8KB)");
    } else {
        eprintln!("❌ Binary size exceeds 8KB cartridge limit by {} bytes", 
            original_size - 0x2000);
    }
    
    // Write final binary to file
    fs::write(&bin_path, &data).map_err(|e| {
        eprintln!("❌ Failed to write binary file: {}", e);
        e
    })?;
    
    eprintln!("✓ NATIVE ASSEMBLER SUCCESS: {} -> {}", 
        bin_path.display(), data.len());
    eprintln!("=== BINARY ASSEMBLY COMPLETE ===");
    Ok(())
}

fn assemble_dual(asm_path: &PathBuf, include_dir: Option<&PathBuf>) -> Result<()> {
    eprintln!("=== DUAL ASSEMBLER MODE ===");
    eprintln!("Compiling with BOTH native and lwasm, then comparing...");
    
    let bin_path = asm_path.with_extension("bin");
    let native_path = asm_path.with_file_name(
        asm_path.file_stem().unwrap().to_str().unwrap().to_string() + ".native.bin"
    );
    let lwasm_path = asm_path.with_file_name(
        asm_path.file_stem().unwrap().to_str().unwrap().to_string() + ".lwasm.bin"
    );
    
    // Read ASM source
    let asm_source = fs::read_to_string(asm_path).map_err(|e| {
        eprintln!("❌ Failed to read ASM file: {}", e);
        e
    })?;
    
    // === NATIVE ASSEMBLER ===
    eprintln!("\n[1/2] Compiling with NATIVE assembler...");
    let org = extract_org_directive(&asm_source).unwrap_or(0xC800);
    eprintln!("    Detected ORG: 0x{:04X}", org);
    
    let (native_binary, _line_map) = backend::asm_to_binary::assemble_m6809(&asm_source, org)
        .map_err(|e| {
            eprintln!("❌ Native assembler failed: {}", e);
            anyhow::anyhow!("Native assembler failed: {}", e)
        })?;
    
    eprintln!("    ✓ Native: {} bytes", native_binary.len());
    fs::write(&native_path, &native_binary)?;
    
    // === LWASM ASSEMBLER ===
    eprintln!("\n[2/2] Compiling with LWASM assembler...");
    
    // Check if lwasm is installed
    let lwasm_check = std::process::Command::new("lwasm")
        .arg("--version")
        .output();
    
    if lwasm_check.is_err() {
        eprintln!("❌ ERROR: lwasm not found in PATH");
        eprintln!("   Install: brew install lwtools (macOS)");
        return Err(anyhow::anyhow!("lwasm not installed"));
    }
    
    // Determine project root for include path
    let project_root = std::env::current_dir()?;
    
    // Run lwasm with include directory (use project root so "include/VECTREX.I" works)
    let output = std::process::Command::new("lwasm")
        .arg("--format=raw")
        .arg("-I")
        .arg(&project_root)
        .arg(format!("--output={}", lwasm_path.display()))
        .arg(asm_path)
        .output()
        .map_err(|e| {
            eprintln!("❌ Failed to execute lwasm: {}", e);
            e
        })?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("❌ lwasm failed:\n{}", stderr);
        return Err(anyhow::anyhow!("lwasm assembly failed"));
    }
    
    let lwasm_binary = fs::read(&lwasm_path)?;
    eprintln!("    ✓ lwasm: {} bytes", lwasm_binary.len());
    
    // === COMPARISON ===
    eprintln!("\n=== BINARY COMPARISON ===");
    eprintln!("Native: {} bytes", native_binary.len());
    eprintln!("lwasm:  {} bytes", lwasm_binary.len());
    
    if native_binary.len() != lwasm_binary.len() {
        eprintln!("⚠️  SIZE MISMATCH: {} bytes difference", 
            (native_binary.len() as i32 - lwasm_binary.len() as i32).abs());
    }
    
    let min_len = native_binary.len().min(lwasm_binary.len());
    let mut differences = 0;
    let mut first_diff = None;
    
    for i in 0..min_len {
        if native_binary[i] != lwasm_binary[i] {
            differences += 1;
            if first_diff.is_none() {
                first_diff = Some(i);
            }
        }
    }
    
    if differences == 0 && native_binary.len() == lwasm_binary.len() {
        eprintln!("✅ BINARIES ARE IDENTICAL!");
        eprintln!("   Both assemblers produced exactly the same output.");
    } else {
        eprintln!("❌ BINARIES DIFFER!");
        eprintln!("   Differences: {} bytes", differences);
        if let Some(offset) = first_diff {
            eprintln!("   First diff at offset 0x{:04X}:", offset);
            eprintln!("     Native: 0x{:02X}", native_binary[offset]);
            eprintln!("     lwasm:  0x{:02X}", lwasm_binary[offset]);
        }
    }
    
    // Pad both binaries to 8KB for emulator compatibility
    let mut native_padded = native_binary.clone();
    let mut lwasm_padded = lwasm_binary.clone();
    
    if native_padded.len() < 0x2000 {
        native_padded.resize(0x2000, 0);
    }
    if lwasm_padded.len() < 0x2000 {
        lwasm_padded.resize(0x2000, 0);
    }
    
    eprintln!("\n✓ Padded both binaries to 8KB");
    
    // Write all three binaries
    fs::write(&bin_path, &native_padded)?;      // Default .bin (native)
    fs::write(&native_path, &native_padded)?;   // .native.bin
    fs::write(&lwasm_path, &lwasm_padded)?;     // .lwasm.bin
    
    eprintln!("✓ Generated 3 binaries:");
    eprintln!("  - {} (default, native)", bin_path.display());
    eprintln!("  - {} (native assembler)", native_path.display());
    eprintln!("  - {} (lwasm assembler)", lwasm_path.display());
    
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
