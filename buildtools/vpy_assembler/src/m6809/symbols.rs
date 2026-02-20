//! Symbol table management and VECTREX.I loading

use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

// Global variable to store include directory (set before assembly)
static mut INCLUDE_DIR: Option<PathBuf> = None;

pub fn set_include_dir(dir: Option<PathBuf>) {
    unsafe {
        INCLUDE_DIR = dir;
    }
}

/// Load Vectrex BIOS symbols into equates table
#[allow(static_mut_refs)]
pub fn load_vectrex_symbols(equates: &mut HashMap<String, u16>) {
    // Try to load from VECTREX.I file first
    if let Some(ref include_dir) = unsafe { INCLUDE_DIR.as_ref() } {
        let vectrex_i = include_dir.join("VECTREX.I");

        if vectrex_i.exists() {
            match fs::read_to_string(&vectrex_i) {
                Ok(content) => {
                    parse_vectrex_symbols(&content, equates);
                    add_uppercase_aliases(equates);
                    return;
                }
                Err(e) => {
                    eprintln!("⚠️ [BIOS LOADER] Failed to read VECTREX.I: {}", e);
                }
            }
        }
    }

    // Fallback to embedded symbols
    load_vectrex_symbols_fallback(equates);
    add_uppercase_aliases(equates);
}

/// Parse VECTREX.I file and extract EQU definitions
fn parse_vectrex_symbols(content: &str, equates: &mut HashMap<String, u16>) {
    for line in content.lines() {
        let trimmed = line.trim();
        
        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with('*') {
            continue;
        }
        
        // Parse EQU directive
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 3 && parts[1].eq_ignore_ascii_case("EQU") {
            let name = parts[0].to_string();
            let value_str = parts[2];
            
            // Parse value (hex $HHHH or decimal)
            let value = if let Some(hex) = value_str.strip_prefix('$') {
                u16::from_str_radix(hex, 16).ok()
            } else {
                value_str.parse::<u16>().ok()
            };
            
            if let Some(v) = value {
                equates.insert(name, v);
            }
        }
    }
}

/// Ensure every symbol also has an uppercase alias to allow case-insensitive lookups
fn add_uppercase_aliases(equates: &mut HashMap<String, u16>) {
    let mut additions = Vec::new();
    for (name, addr) in equates.iter() {
        let upper = name.to_uppercase();
        if upper != *name && !equates.contains_key(&upper) {
            additions.push((upper, *addr));
        }
    }
    for (upper, addr) in additions {
        equates.insert(upper, addr);
    }
}

/// Fallback BIOS symbols (essential addresses)
fn load_vectrex_symbols_fallback(equates: &mut HashMap<String, u16>) {
    // VIA 6522 registers
    equates.insert("VIA_port_b".to_string(), 0xD000);
    equates.insert("VIA_port_a".to_string(), 0xD001);
    equates.insert("VIA_DDR_b".to_string(), 0xD002);
    equates.insert("VIA_DDR_a".to_string(), 0xD003);
    equates.insert("VIA_t1_cnt_lo".to_string(), 0xD004);
    equates.insert("VIA_t1_cnt_hi".to_string(), 0xD005);
    equates.insert("VIA_t1_lch_lo".to_string(), 0xD006);
    equates.insert("VIA_t1_lch_hi".to_string(), 0xD007);
    equates.insert("VIA_t2_lo".to_string(), 0xD008);
    equates.insert("VIA_t2_hi".to_string(), 0xD009);
    equates.insert("VIA_shift_reg".to_string(), 0xD00A);
    equates.insert("VIA_aux_cntl".to_string(), 0xD00B);
    equates.insert("VIA_per_cntl".to_string(), 0xD00C);
    equates.insert("VIA_cntl".to_string(), 0xD00C);
    equates.insert("VIA_int_flags".to_string(), 0xD00D);
    equates.insert("VIA_int_enable".to_string(), 0xD00E);
    equates.insert("VIA_port_a_nohs".to_string(), 0xD00F);
    
    // BIOS routines (essential subset)
    equates.insert("Wait_Recal".to_string(), 0xF192);
    equates.insert("Intensity_a".to_string(), 0xF2AB);
    equates.insert("Moveto_d".to_string(), 0xF312);
    equates.insert("Draw_Line_d".to_string(), 0xF3DF);
    equates.insert("Draw_VLc".to_string(), 0xF410);
    equates.insert("Print_Str_d".to_string(), 0xF37A);
    equates.insert("DP_to_D0".to_string(), 0xF1AA);
    equates.insert("DP_to_C8".to_string(), 0xF1AF);
    equates.insert("Read_Btns".to_string(), 0xF1BA);
    equates.insert("Reset_Pen".to_string(), 0xF35B);
    
    // RAM locations
    equates.insert("Vec_Btn_State".to_string(), 0xC80F);
    equates.insert("Vec_Prev_Btns".to_string(), 0xC80E);
    equates.insert("Vec_Misc_Count".to_string(), 0xC80A);
}

/// Resolve INCLUDE file path
#[allow(static_mut_refs)]
pub fn resolve_include_path(include_path: &str) -> Option<PathBuf> {
    // Try relative to include directory first
    if let Some(ref include_dir) = unsafe { INCLUDE_DIR.as_ref() } {
        let full_path = include_dir.join(include_path);
        if full_path.exists() {
            return Some(full_path);
        }
    }
    
    // Try as absolute path
    let path = PathBuf::from(include_path);
    if path.exists() {
        Some(path)
    } else {
        None
    }
}

/// Process INCLUDE file and load its EQU definitions
pub fn process_include_file(include_path: &str, equates: &mut HashMap<String, u16>) -> Result<(), String> {
    let path = resolve_include_path(include_path)
        .ok_or_else(|| format!("INCLUDE file not found: {}", include_path))?;
    
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read {}: {}", include_path, e))?;
    
    parse_vectrex_symbols(&content, equates);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_vectrex_symbols_fallback() {
        let mut equates = HashMap::new();
        load_vectrex_symbols_fallback(&equates);
        
        assert_eq!(equates.get("Wait_Recal"), Some(&0xF192));
        assert_eq!(equates.get("VIA_port_b"), Some(&0xD000));
    }

    #[test]
    fn test_parse_vectrex_symbols() {
        let content = r"
; Test VECTREX.I
Wait_Recal EQU $F192
VIA_port_b EQU $D000
        ";
        
        let mut equates = HashMap::new();
        parse_vectrex_symbols(content, &equates);
        
        assert_eq!(equates.get("Wait_Recal"), Some(&0xF192));
        assert_eq!(equates.get("VIA_port_b"), Some(&0xD000));
    }
}
