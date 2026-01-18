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

/// Generate assembly code for all assets
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
                // vecres::compile_to_asm() no toma parámetros, usa nombre interno
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
                // musres::compile_to_asm() REQUIERE asset_name
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
                // levelres::compile_to_asm() no toma parámetros, usa nombre interno
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
                // sfxres::compile_to_asm() no toma parámetros, usa nombre interno
                out.push_str(&resource.compile_to_asm());
            },
            Err(e) => {
                eprintln!("[WARNING] Failed to load SFX asset '{}': {}", asset.name, e);
            }
        }
    }
    
    Ok(out)
}
