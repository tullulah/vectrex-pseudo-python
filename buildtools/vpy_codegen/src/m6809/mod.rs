//! M6809 Code Generator for Vectrex
//! 
//! Modular architecture:
//! - header: Vectrex cartridge header generation
//! - variables: RAM variable allocation
//! - functions: Function code generation
//! - expressions: Expression compilation
//! - builtins: Builtin function code
//! - helpers: Runtime helpers (MUL16, DIV16, etc.)
//! - assets: Asset discovery and generation

pub mod header;
pub mod variables;
pub mod functions;
pub mod expressions;
pub mod builtins;
pub mod helpers;
pub mod math;
pub mod joystick;
pub mod debug;
pub mod math_extended;
pub mod drawing;
pub mod level;
pub mod utilities;
pub mod ram_layout;
pub mod assets;

use vpy_parser::{Item, Expr, Stmt, CallInfo};

/// Extract vector names referenced by a level asset
/// Scans level JSON for vectorName fields in all layers
#[allow(dead_code)]
fn extract_level_vectors(level_name: &str, assets: &[crate::AssetInfo]) -> Vec<String> {
    use crate::AssetType;
    
    // Find the level asset
    let level_asset = assets.iter().find(|a| {
        matches!(a.asset_type, AssetType::Level) && a.name == level_name
    });
    
    if let Some(level_asset) = level_asset {
        // Load and parse the level JSON
        if let Ok(json_str) = std::fs::read_to_string(&level_asset.path) {
            if let Ok(level_data) = serde_json::from_str::<serde_json::Value>(&json_str) {
                let mut vectors = Vec::new();
                
                // Extract vectorName from all layers
                if let Some(layers) = level_data.get("layers") {
                    for layer_name in ["background", "gameplay", "foreground"] {
                        if let Some(layer) = layers.get(layer_name) {
                            if let Some(objects) = layer.as_array() {
                                for obj in objects {
                                    if let Some(vector_name) = obj.get("vectorName").and_then(|v| v.as_str()) {
                                        vectors.push(vector_name.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
                return vectors;
            }
        }
    }
    Vec::new()
}

/// analyze_used_assets: Scan module for DRAW_VECTOR() and PLAY_MUSIC() calls
/// Returns set of asset names that are actually used in the code
#[allow(dead_code)]
fn analyze_used_assets(module: &Module, assets: &[crate::AssetInfo]) -> std::collections::HashSet<String> {
    use std::collections::HashSet;
    let mut used = HashSet::new();
    
    fn scan_expr(expr: &Expr, used: &mut HashSet<String>, assets: &[crate::AssetInfo], depth: usize) {
        const MAX_DEPTH: usize = 500;
        if depth > MAX_DEPTH {
            panic!("Maximum expression nesting depth ({}) exceeded during asset analysis.", MAX_DEPTH);
        }
        match expr {
            Expr::Call(call_info) => {
                let name_upper = call_info.name.to_uppercase();
                // Check for DRAW_VECTOR("asset_name", x, y), DRAW_VECTOR_EX("asset_name", x, y, mirror, intensity), 
                // PLAY_MUSIC("asset_name"), PLAY_SFX("asset_name"), or LOAD_LEVEL("level_name")
                if (name_upper == "DRAW_VECTOR" && call_info.args.len() == 3) || 
                   (name_upper == "DRAW_VECTOR_EX" && call_info.args.len() == 5) ||
                   (name_upper == "PLAY_MUSIC" && call_info.args.len() == 1) ||
                   (name_upper == "PLAY_SFX" && call_info.args.len() == 1) ||
                   (name_upper == "LOAD_LEVEL" && call_info.args.len() == 1) {
                    if let Expr::StringLit(asset_name) = &call_info.args[0] {
                        eprintln!("[DEBUG] Found asset usage: {} ({})", asset_name, name_upper);
                        used.insert(asset_name.clone());
                        
                        // If loading a level, also mark vectors it references as used
                        if name_upper == "LOAD_LEVEL" {
                            let level_vectors = extract_level_vectors(asset_name, assets);
                            for vec_name in level_vectors {
                                eprintln!("[DEBUG] Level '{}' references vector: {}", asset_name, vec_name);
                                used.insert(vec_name);
                            }
                        }
                    }
                }
                // Recursively scan arguments
                for arg in &call_info.args {
                    scan_expr(arg, used, assets, depth + 1);
                }
            },
            Expr::MethodCall(mc) => {
                // Scan target and arguments for nested asset usages
                scan_expr(&mc.target, used, assets, depth + 1);
                for arg in &mc.args {
                    scan_expr(arg, used, assets, depth + 1);
                }
            },
            Expr::Binary { left, right, .. } | Expr::Compare { left, right, .. } | Expr::Logic { left, right, .. } => {
                scan_expr(left, used, assets, depth + 1);
                scan_expr(right, used, assets, depth + 1);
            },
            Expr::Not(inner) | Expr::BitNot(inner) => scan_expr(inner, used, assets, depth + 1),
            Expr::List(elements) => {
                for elem in elements {
                    scan_expr(elem, used, assets, depth + 1);
                }
            },
            Expr::Index { target, index } => {
                scan_expr(target, used, assets, depth + 1);
                scan_expr(index, used, assets, depth + 1);
            },
            _ => {}
        }
    }
    
    fn scan_stmt(stmt: &Stmt, used: &mut HashSet<String>, assets: &[crate::AssetInfo], depth: usize) {
        const MAX_DEPTH: usize = 500;
        if depth > MAX_DEPTH {
            panic!("Maximum statement nesting depth ({}) exceeded during asset analysis.", MAX_DEPTH);
        }
        match stmt {
            Stmt::Assign { value, .. } => scan_expr(value, used, assets, depth + 1),
            Stmt::Let { value, .. } => scan_expr(value, used, assets, depth + 1),
            Stmt::CompoundAssign { value, .. } => scan_expr(value, used, assets, depth + 1),
            Stmt::Expr(expr, _line) => scan_expr(expr, used, assets, depth + 1),
            Stmt::If { cond, body, elifs, else_body, .. } => {
                scan_expr(cond, used, assets, depth + 1);
                for s in body { scan_stmt(s, used, assets, depth + 1); }
                for (elif_cond, elif_body) in elifs {
                    scan_expr(elif_cond, used, assets, depth + 1);
                    for s in elif_body { scan_stmt(s, used, assets, depth + 1); }
                }
                if let Some(els) = else_body {
                    for s in els { scan_stmt(s, used, assets, depth + 1); }
                }
            },
            Stmt::While { cond, body, .. } => {
                scan_expr(cond, used, assets, depth + 1);
                for s in body { scan_stmt(s, used, assets, depth + 1); }
            },
            Stmt::For { start, end, step, body, .. } => {
                scan_expr(start, used, assets, depth + 1);
                scan_expr(end, used, assets, depth + 1);
                if let Some(step_expr) = step {
                    scan_expr(step_expr, used, assets, depth + 1);
                }
                for s in body { scan_stmt(s, used, assets, depth + 1); }
            },
            Stmt::ForIn { iterable, body, .. } => {
                scan_expr(iterable, used, assets, depth + 1);
                for s in body { scan_stmt(s, used, assets, depth + 1); }
            },
            Stmt::Switch { expr, cases, default, .. } => {
                scan_expr(expr, used, assets, depth + 1);
                for (case_expr, case_body) in cases {
                    scan_expr(case_expr, used, assets, depth + 1);
                    for s in case_body { scan_stmt(s, used, assets, depth + 1); }
                }
                if let Some(default_body) = default {
                    for s in default_body { scan_stmt(s, used, assets, depth + 1); }
                }
            },
            Stmt::Return(Some(expr), _line) => scan_expr(expr, used, assets, depth + 1),
            _ => {}
        }
    }
    
    // Scan all functions and top-level items in module
    for item in &module.items {
        match item {
            Item::Function(func) => {
                for stmt in &func.body {
                    scan_stmt(stmt, &mut used, assets, 0);
                }
            },
            Item::Const { value, .. } | Item::GlobalLet { value, .. } => {
                scan_expr(value, &mut used, assets, 0);
            },
            Item::ExprStatement(expr) => {
                scan_expr(expr, &mut used, assets, 0);
            },
            _ => {}
        }
    }
    
    used
}

/// Check if trigonometric functions (SIN, COS, TAN) are used in statements
fn check_trig_usage(stmts: &[Stmt]) -> bool {
    for stmt in stmts {
        if check_stmt_trig(stmt) {
            return true;
        }
    }
    false
}

fn check_stmt_trig(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Expr(expr, _) => check_expr_trig(expr),
        Stmt::Assign { value, .. } => check_expr_trig(value),
        Stmt::If { cond, body, elifs, else_body, .. } => {
            check_expr_trig(cond)
                || check_trig_usage(body)
                || elifs.iter().any(|(c, b)| check_expr_trig(c) || check_trig_usage(b))
                || else_body.as_ref().map_or(false, |b| check_trig_usage(b))
        }
        Stmt::While { cond, body, .. } => check_expr_trig(cond) || check_trig_usage(body),
        _ => false,
    }
}

fn check_expr_trig(expr: &Expr) -> bool {
    match expr {
        Expr::Call(CallInfo { name, args, .. }) => {
            let upper = name.to_uppercase();
            upper == "SIN" || upper == "COS" || upper == "TAN"
                || args.iter().any(check_expr_trig)
        }
        Expr::Binary { left, right, .. } => check_expr_trig(left) || check_expr_trig(right),
        Expr::Not(operand) | Expr::BitNot(operand) => check_expr_trig(operand),
        Expr::Index { target, index, .. } => check_expr_trig(target) || check_expr_trig(index),
        Expr::List(elements) => elements.iter().any(check_expr_trig),
        _ => false,
    }
}

use vpy_parser::Module;

/// Main entry point for M6809 code generation
pub fn generate_m6809_asm(
    module: &Module,
    title: &str,
    rom_size: usize,
    _bank_size: usize,
    assets: &[crate::AssetInfo],
) -> Result<String, String> {
    let mut asm = String::new();
    
    // Detect if this is a multibank ROM (>32KB)
    let is_multibank = rom_size > 32768;
    
    // Calculate bank configuration dynamically
    let bank_size = 16384; // Standard Vectrex bank size (16KB)
    let num_banks = if is_multibank { rom_size / bank_size } else { 1 };
    let helpers_bank = if is_multibank { num_banks - 1 } else { 0 };
    
    // Generate header comments
    asm.push_str(&format!("; VPy M6809 Assembly (Vectrex)\n"));
    asm.push_str(&format!("; ROM: {} bytes\n", rom_size));
    if is_multibank {
        asm.push_str(&format!("; Multibank cartridge: {} banks ({}KB each)\n", num_banks, bank_size / 1024));
        asm.push_str(&format!("; Helpers bank: {} (fixed bank at $4000-$7FFF)\n", helpers_bank));
    }
    asm.push_str("\n");
    
    // For multibank: Emit Bank 0 marker
    if is_multibank {
        asm.push_str("; ================================================\n");
        asm.push_str("; BANK #0 - Entry point and main code\n");
        asm.push_str("; ================================================\n");
    }
    
    // NOW start of ROM code
    asm.push_str("\n");
    asm.push_str("    ORG $0000\n\n");
    
    // Include VECTREX.I for BIOS definitions
    asm.push_str(";***************************************************************************\n");
    asm.push_str("; DEFINE SECTION\n");
    asm.push_str(";***************************************************************************\n");
    asm.push_str("    INCLUDE \"VECTREX.I\"\n\n");
    
    // Generate Vectrex header
    let header_asm = header::generate_header(title, &module.meta)?;
    asm.push_str(&header_asm);
    
    // Generate code section
    asm.push_str(";***************************************************************************\n");
    asm.push_str("; CODE SECTION\n");
    asm.push_str(";***************************************************************************\n\n");
    
    // Generate START initialization
    asm.push_str("START:\n");
    asm.push_str("    LDA #$D0\n");
    asm.push_str("    TFR A,DP        ; Set Direct Page for BIOS\n");
    asm.push_str("    CLR $C80E        ; Initialize Vec_Prev_Btns\n");
    asm.push_str("    LDA #$80\n");
    asm.push_str("    STA VIA_t1_cnt_lo\n");
    asm.push_str("    LDS #$CBFF       ; Initialize stack\n");
    
    // CRITICAL: Initialize SFX system variables to prevent garbage data interference
    use crate::m6809::functions::has_audio_calls;
    if has_audio_calls(module) {
        asm.push_str("    ; Initialize SFX variables to prevent random noise on startup\n");
        asm.push_str("    CLR >SFX_ACTIVE         ; Mark SFX as inactive (0=off)\n");
        asm.push_str("    LDD #$0000\n");
        asm.push_str("    STD >SFX_PTR            ; Clear SFX pointer\n");
    }
    
    // For multibank: Fixed bank is ALWAYS visible at $4000-$7FFF
    // No need to write bank register - cartridge hardware has it configured
    // from factory. Bank 0 is at $0000, fixed bank at $4000.
    if is_multibank {
        asm.push_str(&format!("; Bank 0 ($0000) is active; fixed bank {} ($4000-$7FFF) always visible\n", helpers_bank));
    }
    
    asm.push_str("    JMP MAIN\n\n");
    
    // CRITICAL FIX (2026-01-18): Generate RAM definitions and arrays BEFORE user functions
    // This ensures arrays are defined before first use (fixes forward reference errors)
    let ram_and_arrays_asm = helpers::generate_ram_and_arrays(module)?;
    asm.push_str(&ram_and_arrays_asm);
    
    // Generate user functions in Bank 0
    let functions_asm = functions::generate_functions(module)?;
    asm.push_str(&functions_asm);
    
    // CRITICAL FIX (2026-01-17): Collect PRINT_TEXT strings here but emit LATER
    // Problem: If strings are emitted immediately after functions, they get addresses
    // in the middle of code, and LDX #PRINT_TEXT_STR references fail in assembler
    // Solution: Collect now, emit at END (after helpers, before vectors) like CORE does
    let print_text_strings = builtins::collect_print_text_strings(module);
    
    // For multibank: Emit ALL intermediate banks as empty placeholders
    // multi_bank_linker requires ALL banks to be marked in the ASM
    // Format: "; ================================================"
    //         "; BANK #N - 0 function(s) [EMPTY]"
    //         "; ================================================"
    //         "    ORG $0000  ; Sequential bank model"
    if is_multibank {
        // Emit banks 1 through (helpers_bank - 1) as empty
        for bank_id in 1..(helpers_bank as usize) {
            asm.push_str(&format!("\n; ================================================\n"));
            asm.push_str(&format!("; BANK #{} - 0 function(s) [EMPTY]\n", bank_id));
            asm.push_str(&format!("; ================================================\n"));
            asm.push_str("    ORG $0000  ; Sequential bank model\n");
            asm.push_str(&format!("    ; Reserved for future code overflow\n\n"));
        }
        
        // Emit helpers bank (last bank) with proper marker
        asm.push_str(&format!("\n; ================================================\n"));
        asm.push_str(&format!("; BANK #{} - 0 function(s) [HELPERS ONLY]\n", helpers_bank));
        asm.push_str(&format!("; ================================================\n"));
        asm.push_str("    ORG $4000  ; Fixed bank (always visible at $4000-$7FFF)\n");
        asm.push_str(&format!("    ; Runtime helpers (accessible from all banks)\n\n"));
        
        // NOTE: VAR_ARG0-4 are already defined in SYSTEM RAM VARIABLES section above
        // (before bank split). No need to redefine them here in Bank #31.
        
        let helpers_asm = helpers::generate_helpers(module)?;
        asm.push_str(&helpers_asm);
    }
    
    // For single-bank: Emit helpers normally
    if !is_multibank {
        let helpers_asm = helpers::generate_helpers(module)?;
        asm.push_str(&helpers_asm);
    }
    
    // Emit trigonometry lookup tables (SIN, COS, TAN) - CONDITIONAL
    // Only emit if SIN, COS, or TAN functions are actually used
    if module.items.iter().any(|item| {
        if let Item::Function(f) = item {
            check_trig_usage(&f.body)
        } else {
            false
        }
    }) {
        asm.push_str(&math_extended::generate_trig_tables());
    }
    
    // CRITICAL FIX (2026-01-17): Emit PRINT_TEXT strings AFTER all code/helpers
    // This ensures labels have stable final addresses that assembler can resolve
    // Matches CORE architecture where strings are emitted at end
    if !print_text_strings.is_empty() {
        builtins::emit_print_text_strings(&print_text_strings, &mut asm);
    }
    
    // Generate embedded assets (vectors, music, levels, SFX)
    if !assets.is_empty() {
        let assets_asm = assets::generate_assets_asm(assets)
            .map_err(|e| format!("Asset generation failed: {}", e))?;
        asm.push_str(&assets_asm);
    }
    
    // NOTE: Cartridge ROM ($0000-$7FFF) does NOT contain interrupt vectors
    // Hardware vectors ($FFF0-$FFFF) are in BIOS ROM
    // BIOS vectors point to RAM vectors ($CBF2-$CBFB) as defined in VECTREX.I
    // Cartridge starts at $0000 and BIOS jumps there after verification
    
    Ok(asm)
}
