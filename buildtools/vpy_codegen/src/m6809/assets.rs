//! Asset discovery and generation
//! Handles .vec, .vmus, .vlevel, and .vsfx resources

use std::path::{Path, PathBuf};
use std::fs;
use crate::{AssetInfo, AssetType};

/// Discover all assets in a project
/// 
/// Searches for:
/// - assets/vectors/*.vec (vector graphics)
/// - assets/music/*.vmus (music)
/// - assets/levels/*.vlevel (level data)
/// - assets/sfx/*.vsfx (sound effects)
pub fn discover_assets(source_path: &Path) -> Vec<AssetInfo> {
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
    
    eprintln!("[DEBUG ASSETS] Project root: {:?}", project_root);
    
    // Search for vector assets (assets/vectors/*.vec)
    let vectors_dir = project_root.join("assets").join("vectors");
    if vectors_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&vectors_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("vec") {
                    if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                        eprintln!("[DEBUG ASSETS] Found vector: {} at {:?}", name, path);
                        assets.push(AssetInfo {
                            name: name.to_string(),
                            path: path.display().to_string(),
                            asset_type: AssetType::Vector,
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
                        eprintln!("[DEBUG ASSETS] Found music: {} at {:?}", name, path);
                        assets.push(AssetInfo {
                            name: name.to_string(),
                            path: path.display().to_string(),
                            asset_type: AssetType::Music,
                        });
                    }
                }
            }
        }
    }
    
    // Search for level assets (assets/levels/*.vlevel)
    let levels_dir = project_root.join("assets").join("levels");
    if levels_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&levels_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("vlevel") {
                    if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                        eprintln!("[DEBUG ASSETS] Found level: {} at {:?}", name, path);
                        assets.push(AssetInfo {
                            name: name.to_string(),
                            path: path.display().to_string(),
                            asset_type: AssetType::Level,
                        });
                    }
                }
            }
        }
    }
    
    // Search for SFX assets (assets/sfx/*.vsfx)
    let sfx_dir = project_root.join("assets").join("sfx");
    if sfx_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&sfx_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("vsfx") {
                    if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                        eprintln!("[DEBUG ASSETS] Found SFX: {} at {:?}", name, path);
                        assets.push(AssetInfo {
                            name: name.to_string(),
                            path: path.display().to_string(),
                            asset_type: AssetType::Sfx,
                        });
                    }
                }
            }
        }
    }
    
    eprintln!("[DEBUG ASSETS] Total assets found: {}", assets.len());
    assets
}

/// Asset with calculated size for bin-packing
#[derive(Debug, Clone)]
pub struct SizedAsset {
    pub info: AssetInfo,
    pub binary_size: usize,
    pub asm_code: String,
}

/// Asset distribution result for multi-bank support
#[derive(Debug, Clone)]
pub struct AssetDistribution {
    /// Assets assigned to each bank (bank_id -> list of assets)
    pub bank_assignments: std::collections::HashMap<u8, Vec<SizedAsset>>,
    /// Total assets distributed
    pub total_assets: usize,
    /// Total bytes distributed
    pub total_bytes: usize,
}

/// Calculate sizes and generate ASM for all assets
pub fn prepare_assets_with_sizes(assets: &[AssetInfo]) -> Vec<SizedAsset> {
    let mut sized_assets = Vec::new();
    
    for asset in assets.iter().filter(|a| matches!(a.asset_type, AssetType::Vector)) {
        match crate::vecres::VecResource::load(Path::new(&asset.path)) {
            Ok(resource) => {
                let binary_size = resource.estimate_binary_size();
                let asm_code = resource.compile_to_asm_with_name(Some(&asset.name));
                eprintln!("[DEBUG ASSETS] Vector '{}': {} bytes estimated", asset.name, binary_size);
                sized_assets.push(SizedAsset {
                    info: asset.clone(),
                    binary_size,
                    asm_code,
                });
            },
            Err(e) => {
                eprintln!("[WARNING] Failed to load vector asset '{}': {}", asset.name, e);
            }
        }
    }
    
    for asset in assets.iter().filter(|a| matches!(a.asset_type, AssetType::Music)) {
        match crate::musres::MusicResource::load(Path::new(&asset.path)) {
            Ok(resource) => {
                let asm_code = resource.compile_to_asm(&asset.name);
                // Estimate music size: count FCB/FDB bytes in ASM (rough approximation)
                let binary_size = estimate_asm_size(&asm_code);
                eprintln!("[DEBUG ASSETS] Music '{}': {} bytes estimated", asset.name, binary_size);
                sized_assets.push(SizedAsset {
                    info: asset.clone(),
                    binary_size,
                    asm_code,
                });
            },
            Err(e) => {
                eprintln!("[WARNING] Failed to load music asset '{}': {}", asset.name, e);
            }
        }
    }
    
    for asset in assets.iter().filter(|a| matches!(a.asset_type, AssetType::Level)) {
        match crate::levelres::VPlayLevel::load(Path::new(&asset.path)) {
            Ok(resource) => {
                let asm_code = resource.compile_to_asm();
                let binary_size = estimate_asm_size(&asm_code);
                eprintln!("[DEBUG ASSETS] Level '{}': {} bytes estimated", asset.name, binary_size);
                sized_assets.push(SizedAsset {
                    info: asset.clone(),
                    binary_size,
                    asm_code,
                });
            },
            Err(e) => {
                eprintln!("[WARNING] Failed to load level asset '{}': {}", asset.name, e);
            }
        }
    }
    
    for asset in assets.iter().filter(|a| matches!(a.asset_type, AssetType::Sfx)) {
        match crate::sfxres::SfxResource::load(Path::new(&asset.path)) {
            Ok(resource) => {
                let asm_code = resource.compile_to_asm();
                let binary_size = estimate_asm_size(&asm_code);
                eprintln!("[DEBUG ASSETS] SFX '{}': {} bytes estimated", asset.name, binary_size);
                sized_assets.push(SizedAsset {
                    info: asset.clone(),
                    binary_size,
                    asm_code,
                });
            },
            Err(e) => {
                eprintln!("[WARNING] Failed to load SFX asset '{}': {}", asset.name, e);
            }
        }
    }
    
    // Sort by size descending (best for bin-packing)
    sized_assets.sort_by(|a, b| b.binary_size.cmp(&a.binary_size));
    
    sized_assets
}

/// Estimate binary size from ASM code (rough approximation)
fn estimate_asm_size(asm: &str) -> usize {
    let mut size = 0;
    for line in asm.lines() {
        let trimmed = line.trim().to_uppercase();
        if trimmed.starts_with("FCB ") {
            // Count comma-separated values
            let values = trimmed[4..].split(',').count();
            size += values;
        } else if trimmed.starts_with("FDB ") {
            // Each FDB is 2 bytes per value
            let values = trimmed[4..].split(',').count();
            size += values * 2;
        } else if trimmed.starts_with("FCC ") {
            // String length (approximate)
            if let Some(start) = trimmed.find('"') {
                if let Some(end) = trimmed.rfind('"') {
                    size += end - start - 1;
                }
            }
        }
    }
    size
}

/// Distribute assets across banks using First-Fit Decreasing bin-packing
/// 
/// Parameters:
/// - assets: List of assets to distribute
/// - bank_size: Maximum bytes per bank (default 16384 = 16KB)
/// - start_bank: First bank ID to use for assets (default 1, Bank 0 has code)
/// - max_banks: Maximum number of banks to use for assets (helpers bank excluded)
/// 
/// Returns AssetDistribution with bank assignments
pub fn distribute_assets(
    assets: &[AssetInfo],
    bank_size: usize,
    start_bank: u8,
    max_banks: u8,
) -> AssetDistribution {
    use std::collections::HashMap;
    
    let sized_assets = prepare_assets_with_sizes(assets);
    let mut bank_assignments: HashMap<u8, Vec<SizedAsset>> = HashMap::new();
    let mut bank_used: HashMap<u8, usize> = HashMap::new();
    
    // Reserve space in each bank for overhead (labels, alignment, etc.)
    let overhead_per_bank = 256; // 256 bytes reserved for bank header/overhead
    let effective_bank_size = bank_size.saturating_sub(overhead_per_bank);
    
    let total_assets = sized_assets.len();
    let total_bytes: usize = sized_assets.iter().map(|a| a.binary_size).sum();
    
    eprintln!("[ASSET DISTRIBUTION] {} assets, {} bytes total, {} bytes/bank effective", 
        total_assets, total_bytes, effective_bank_size);
    
    // First-Fit Decreasing: try to fit each asset in first bank with space
    for asset in sized_assets {
        let mut assigned = false;
        
        // Try existing banks first
        for bank_id in start_bank..start_bank.saturating_add(max_banks) {
            let used = *bank_used.get(&bank_id).unwrap_or(&0);
            if used + asset.binary_size <= effective_bank_size {
                // Fits in this bank
                bank_assignments.entry(bank_id).or_insert_with(Vec::new).push(asset.clone());
                *bank_used.entry(bank_id).or_insert(0) += asset.binary_size;
                eprintln!("[ASSET DISTRIBUTION] '{}' ({} bytes) -> Bank #{} (used: {}/{})", 
                    asset.info.name, asset.binary_size, bank_id, 
                    bank_used.get(&bank_id).unwrap_or(&0), effective_bank_size);
                assigned = true;
                break;
            }
        }
        
        if !assigned {
            // No existing bank has space - use Bank 0 as overflow (may cause overflow error later)
            eprintln!("[ASSET DISTRIBUTION] WARNING: '{}' ({} bytes) -> Bank #0 (OVERFLOW)", 
                asset.info.name, asset.binary_size);
            bank_assignments.entry(0).or_insert_with(Vec::new).push(asset.clone());
            *bank_used.entry(0).or_insert(0) += asset.binary_size;
        }
    }
    
    // Report distribution
    for bank_id in start_bank..start_bank.saturating_add(max_banks) {
        if let Some(assets) = bank_assignments.get(&bank_id) {
            let used = bank_used.get(&bank_id).unwrap_or(&0);
            eprintln!("[ASSET DISTRIBUTION] Bank #{}: {} assets, {} bytes", bank_id, assets.len(), used);
        }
    }
    
    AssetDistribution {
        bank_assignments,
        total_assets,
        total_bytes,
    }
}

/// Generate assembly code for all assets (single-bank mode - all in one place)
pub fn generate_assets_asm(assets: &[AssetInfo]) -> Result<String, String> {
    let mut out = String::new();
    
    out.push_str(";***************************************************************************\n");
    out.push_str("; EMBEDDED ASSETS (vectors, music, levels, SFX)\n");
    out.push_str(";***************************************************************************\n\n");
    
    // Generate vector assets
    for asset in assets.iter().filter(|a| matches!(a.asset_type, AssetType::Vector)) {
        eprintln!("[DEBUG ASSETS] Generating ASM for vector: {}", asset.name);
        match crate::vecres::VecResource::load(Path::new(&asset.path)) {
            Ok(resource) => {
                out.push_str(&resource.compile_to_asm_with_name(Some(&asset.name)));
            },
            Err(e) => {
                eprintln!("[WARNING] Failed to load vector asset '{}': {}", asset.name, e);
            }
        }
    }
    
    // Generate music assets
    for asset in assets.iter().filter(|a| matches!(a.asset_type, AssetType::Music)) {
        eprintln!("[DEBUG ASSETS] Generating ASM for music: {}", asset.name);
        match crate::musres::MusicResource::load(Path::new(&asset.path)) {
            Ok(resource) => {
                out.push_str(&resource.compile_to_asm(&asset.name));
            },
            Err(e) => {
                eprintln!("[WARNING] Failed to load music asset '{}': {}", asset.name, e);
            }
        }
    }
    
    // Generate level assets
    for asset in assets.iter().filter(|a| matches!(a.asset_type, AssetType::Level)) {
        eprintln!("[DEBUG ASSETS] Generating ASM for level: {}", asset.name);
        match crate::levelres::VPlayLevel::load(Path::new(&asset.path)) {
            Ok(resource) => {
                out.push_str(&resource.compile_to_asm());
            },
            Err(e) => {
                eprintln!("[WARNING] Failed to load level asset '{}': {}", asset.name, e);
            }
        }
    }
    
    // Generate SFX assets
    for asset in assets.iter().filter(|a| matches!(a.asset_type, AssetType::Sfx)) {
        eprintln!("[DEBUG ASSETS] Generating ASM for SFX: {}", asset.name);
        match crate::sfxres::SfxResource::load(Path::new(&asset.path)) {
            Ok(resource) => {
                out.push_str(&resource.compile_to_asm());
            },
            Err(e) => {
                eprintln!("[WARNING] Failed to load SFX asset '{}': {}", asset.name, e);
            }
        }
    }
    
    Ok(out)
}

/// Generate assembly code for assets distributed across multiple banks
/// 
/// Returns a tuple: (bank_asm_map, lookup_tables_asm)
/// - bank_asm_map: HashMap<bank_id, asm_code> for each bank's assets
/// - lookup_tables_asm: ASM code for ASSET_BANK_TABLE and ASSET_ADDR_TABLE (goes in helpers bank)
pub fn generate_distributed_assets_asm(
    assets: &[AssetInfo],
    bank_size: usize,
    helpers_bank: u8,
) -> Result<(std::collections::HashMap<u8, String>, String), String> {
    use std::collections::HashMap;
    
    // Distribute assets across banks 1..(helpers_bank-1)
    // Bank 0 has main code, helpers_bank has runtime
    let distribution = distribute_assets(assets, bank_size, 1, helpers_bank.saturating_sub(1));
    
    let mut bank_asm: HashMap<u8, String> = HashMap::new();
    let _asset_index = 0u16;
    
    // Track asset info for lookup table generation
    let mut asset_entries: Vec<(String, u8, String, AssetType)> = Vec::new(); // (name, bank_id, label, type)
    
    // Generate ASM for each bank
    for (bank_id, sized_assets) in &distribution.bank_assignments {
        let mut asm = String::new();
        asm.push_str(&format!(";***************************************************************************\n"));
        asm.push_str(&format!("; ASSETS IN BANK #{} ({} assets)\n", bank_id, sized_assets.len()));
        asm.push_str(&format!(";***************************************************************************\n\n"));
        
        for asset in sized_assets {
            // Use pre-generated ASM code
            asm.push_str(&asset.asm_code);
            asm.push_str("\n");
            
            // Track for lookup table with correct label suffix based on type
            let symbol_name = asset.info.name.to_uppercase().replace("-", "_").replace(" ", "_");
            let label = match asset.info.asset_type {
                AssetType::Vector => format!("_{}_VECTORS", symbol_name),
                AssetType::Music => format!("_{}_MUSIC", symbol_name),
                AssetType::Sfx => format!("_{}_SFX", symbol_name),
                AssetType::Level => format!("_{}_LEVEL", symbol_name),
            };
            asset_entries.push((asset.info.name.clone(), *bank_id, label, asset.info.asset_type.clone()));
        }
        
        bank_asm.insert(*bank_id, asm);
    }
    
    // Generate lookup tables for helpers bank
    let mut lookup_asm = String::new();
    lookup_asm.push_str(";***************************************************************************\n");
    lookup_asm.push_str("; ASSET LOOKUP TABLES (for banked asset access)\n");
    lookup_asm.push_str(&format!("; Total assets: {}\n", asset_entries.len()));
    lookup_asm.push_str(";***************************************************************************\n\n");
    
    // Generate name-to-index mapping as comments (for documentation)
    lookup_asm.push_str("; Asset Index Mapping:\n");
    for (idx, (name, bank_id, _label, asset_type)) in asset_entries.iter().enumerate() {
        lookup_asm.push_str(&format!(";   {} = {} (Bank #{}, {:?})\n", idx, name, bank_id, asset_type));
    }
    lookup_asm.push_str("\n");
    
    // ASSET_BANK_TABLE: FCB bank_id for each asset
    lookup_asm.push_str("ASSET_BANK_TABLE:\n");
    for (_, bank_id, _, _) in &asset_entries {
        lookup_asm.push_str(&format!("    FCB {}              ; Bank ID\n", bank_id));
    }
    lookup_asm.push_str("\n");
    
    // ASSET_ADDR_TABLE: FDB address for each asset (within its bank)
    lookup_asm.push_str("ASSET_ADDR_TABLE:\n");
    for (name, _, label, _) in &asset_entries {
        lookup_asm.push_str(&format!("    FDB {}       ; {}\n", label, name));
    }
    lookup_asm.push_str("\n");
    
    // Generate DRAW_VECTOR_BANKED wrapper
    lookup_asm.push_str(&generate_draw_vector_banked_wrapper());
    
    Ok((bank_asm, lookup_asm))
}

/// Generate the DRAW_VECTOR_BANKED runtime wrapper for helpers bank
fn generate_draw_vector_banked_wrapper() -> String {
    let mut asm = String::new();
    
    asm.push_str(";***************************************************************************\n");
    asm.push_str("; DRAW_VECTOR_BANKED - Draw vector asset with automatic bank switching\n");
    asm.push_str("; Input: X = asset index (0-based), DRAW_VEC_X/Y set for position\n");
    asm.push_str("; Uses: A, B, X, Y, TMPPTR\n");
    asm.push_str("; Preserves: CURRENT_ROM_BANK (restored after drawing)\n");
    asm.push_str(";***************************************************************************\n");
    asm.push_str("DRAW_VECTOR_BANKED:\n");
    asm.push_str("    PSHS A,B,X,Y,U       ; Save all registers\n");
    asm.push_str("\n");
    asm.push_str("    ; Save current bank\n");
    asm.push_str("    LDA CURRENT_ROM_BANK\n");
    asm.push_str("    PSHS A               ; Save on stack\n");
    asm.push_str("\n");
    asm.push_str("    ; Get asset's bank from lookup table\n");
    asm.push_str("    TFR X,D              ; X = asset index -> D\n");
    asm.push_str("    LDX #ASSET_BANK_TABLE\n");
    asm.push_str("    LDA D,X              ; A = bank ID for this asset\n");
    asm.push_str("    STA CURRENT_ROM_BANK ; Update RAM tracker\n");
    asm.push_str("    STA $DF00            ; Switch bank hardware register\n");
    asm.push_str("\n");
    asm.push_str("    ; Get asset's address from lookup table\n");
    asm.push_str("    PULS X               ; Restore asset index (was saved as A, but reuse)\n");
    asm.push_str("    ; Need to recalculate X offset for address table (2 bytes per entry)\n");
    asm.push_str("    PSHS A               ; Re-save bank (need it for later)\n");
    asm.push_str("    TFR X,D              ; asset index in D\n");
    asm.push_str("    ASLB                 ; *2 for FDB entries\n");
    asm.push_str("    ROLA\n");
    asm.push_str("    LDX #ASSET_ADDR_TABLE\n");
    asm.push_str("    LEAX D,X             ; X points to address entry\n");
    asm.push_str("    LDX ,X               ; X = actual vector address in banked ROM\n");
    asm.push_str("\n");
    asm.push_str("    ; Set up for drawing\n");
    asm.push_str("    CLR MIRROR_X\n");
    asm.push_str("    CLR MIRROR_Y\n");
    asm.push_str("    CLR DRAW_VEC_INTENSITY\n");
    asm.push_str("    JSR $F1AA            ; DP_to_D0\n");
    asm.push_str("\n");
    asm.push_str("    ; Draw the vector (X already has address)\n");
    asm.push_str("    JSR Draw_Sync_List_At_With_Mirrors\n");
    asm.push_str("\n");
    asm.push_str("    JSR $F1AF            ; DP_to_C8\n");
    asm.push_str("\n");
    asm.push_str("    ; Restore original bank\n");
    asm.push_str("    PULS A               ; Get saved bank\n");
    asm.push_str("    STA CURRENT_ROM_BANK\n");
    asm.push_str("    STA $DF00            ; Restore bank\n");
    asm.push_str("\n");
    asm.push_str("    PULS A,B,X,Y,U       ; Restore all registers\n");
    asm.push_str("    RTS\n");
    asm.push_str("\n");
    
    asm
}
