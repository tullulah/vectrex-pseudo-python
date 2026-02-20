//! VPy Debug Gen: Phase 9 of buildtools compiler pipeline
//!
//! Generates debug symbols (.pdb) from linker output
//!
//! **Depends on Phase 7 (linker) for correct addresses**
//!
//! # Module Structure
//!
//! - `error.rs`: Error types
//! - `format.rs`: PDB format handling
//! - `generator.rs`: Symbol extraction and PDB creation
//!
//! # Typical workflow
//!
//! ```text
//! 1. generate_pdb(asm_source, vectrex_i, config)  → PdbFile (addresses = 0)
//! 2. update_pdb_addresses(&mut pdb, &linker_symbols) → fills real addresses
//! 3. populate_line_maps(&mut pdb, asm_source)     → builds line-number maps
//! 4. pdb.to_json()                                → serialize for IDE
//! ```

pub mod error;
pub mod format;
pub mod generator;

pub use error::{DebugError, DebugResult};
pub use format::PdbFile;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RomConfig {
    pub total_size: u32,
    pub bank_size: u32,
    pub bank_count: u32,
    pub is_multibank: bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// Primary API
// ─────────────────────────────────────────────────────────────────────────────

/// Phase 9, step 1 – parse ASM source and produce a `PdbFile`.
///
/// * Labels found in the ASM are registered with address `0` (placeholder).
/// * If `vectrex_i` is provided, BIOS symbol EQU values are parsed and stored
///   in `pdb.bios_symbols` with their real absolute addresses.
///
/// Call `update_pdb_addresses` afterwards to fill in the real addresses from
/// the Phase-7 symbol table.
pub fn generate_pdb(
    asm_source: &str,
    vectrex_i: Option<&str>,
    _rom_config: RomConfig,
) -> DebugResult<format::PdbFile> {
    let mut pdb = format::PdbFile::new();

    // ── Extract labels from ASM ──────────────────────────────────────────────
    for line in asm_source.lines() {
        let trimmed = line.trim_end();
        if let Some(label) = generator::parse_label_definition(trimmed) {
            pdb.labels.insert(label.to_string(), 0);
        }
    }

    // ── Parse BIOS symbols from VECTREX.I ───────────────────────────────────
    if let Some(content) = vectrex_i {
        for line in content.lines() {
            parse_equ_line(line, &mut pdb.bios_symbols);
        }
    }

    Ok(pdb)
}

/// Phase 9, step 2 – update all PDB symbol addresses from the linker's
/// resolved symbol table.
///
/// Any symbol present in both `pdb` and `symbols` gets its address replaced
/// with the linker-assigned value.  Extra symbols in `symbols` that are not
/// yet in the PDB are added to `pdb.labels`.
pub fn update_pdb_addresses(
    pdb: &mut format::PdbFile,
    symbols: &HashMap<String, u32>,
) {
    for (name, &addr) in symbols {
        // Update existing entries
        if let Some(entry) = pdb.labels.get_mut(name) {
            *entry = addr;
            continue;
        }
        if let Some(entry) = pdb.functions.get_mut(name) {
            *entry = addr;
            continue;
        }
        if let Some(sym) = pdb.symbols.get_mut(name) {
            sym.address = addr;
            continue;
        }
        if let Some(sym) = pdb.variables.get_mut(name) {
            sym.address = addr;
            continue;
        }
        // Symbol came from the linker but wasn't in the ASM labels → add it
        pdb.labels.insert(name.clone(), addr);
    }

    // Promote labels that look like functions (uppercase identifiers) into
    // the functions map so the IDE can show them separately.
    let candidates: Vec<(String, u32)> = pdb
        .labels
        .iter()
        .filter(|(n, _)| looks_like_function(n))
        .map(|(n, &a)| (n.clone(), a))
        .collect();

    for (name, addr) in candidates {
        pdb.functions.entry(name).or_insert(addr);
    }
}

/// Phase 9, step 3 – scan the ASM source for `; VPy_LINE:N` annotations and
/// build bidirectional line maps:
///
/// * `pdb.asm_line_map[asm_line] = vpy_line`
/// * `pdb.vpy_line_map[vpy_line] = asm_line`
pub fn populate_line_maps(
    pdb: &mut format::PdbFile,
    asm_source: &str,
) {
    for (asm_line, line_content) in asm_source.lines().enumerate() {
        if let Some(vpy_line) = generator::parse_vpy_line_annotation(line_content) {
            pdb.asm_line_map.insert(asm_line, vpy_line);
            // Only record first ASM line for each VPy line
            pdb.vpy_line_map.entry(vpy_line).or_insert(asm_line);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Legacy / convenience helpers
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DebugInfo {
    pub symbols: HashMap<String, SymbolInfo>,
    pub source_lines: HashMap<usize, String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SymbolInfo {
    pub address: u32,
    pub size: usize,
    pub source_line: usize,
}

/// Generate a `DebugInfo` summary from an ASM string (convenience wrapper).
///
/// `binary` is unused in this implementation — symbol names cannot be derived
/// from a raw binary without an embedded symbol table.
pub fn generate_debug_info(_binary: &str, asm: &str) -> DebugResult<DebugInfo> {
    let mut info = DebugInfo {
        symbols: HashMap::new(),
        source_lines: HashMap::new(),
    };

    for (line_num, line_content) in asm.lines().enumerate() {
        let trimmed = line_content.trim_end();
        if let Some(label) = generator::parse_label_definition(trimmed) {
            info.symbols.insert(
                label.to_string(),
                SymbolInfo {
                    address: 0,
                    size: 0,
                    source_line: line_num,
                },
            );
        }
        if !trimmed.is_empty() {
            info.source_lines.insert(line_num, trimmed.to_string());
        }
    }

    Ok(info)
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Parse one line of VECTREX.I looking for `SYMBOL  EQU  $XXXX` (or `equ`/`SET`).
fn parse_equ_line(line: &str, out: &mut HashMap<String, u32>) {
    // Strip inline comments then tokenize — handles any amount of whitespace
    let code = line.split(';').next().unwrap_or("").split('*').next().unwrap_or("");
    let mut tokens = code.split_whitespace();

    let name = match tokens.next() {
        Some(t) => t,
        None => return,
    };
    let directive = match tokens.next() {
        Some(t) => t.to_ascii_uppercase(),
        None => return,
    };
    if directive != "EQU" && directive != "SET" {
        return;
    }
    let value_str = match tokens.next() {
        Some(t) => t,
        None => return,
    };

    if let Some(addr) = parse_integer_value(value_str) {
        out.insert(name.to_string(), addr);
    }
}

/// Parse `$XXXX`, `0xXXXX`, `%BBBB`, or decimal integer.
fn parse_integer_value(s: &str) -> Option<u32> {
    if let Some(hex) = s.strip_prefix('$') {
        u32::from_str_radix(hex, 16).ok()
    } else if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16).ok()
    } else if let Some(bin) = s.strip_prefix('%') {
        u32::from_str_radix(bin, 2).ok()
    } else {
        s.parse::<u32>().ok()
    }
}

/// Heuristic: labels that are all-uppercase and start with a letter are
/// likely function entry points (Vectrex convention).
fn looks_like_function(name: &str) -> bool {
    !name.is_empty()
        && name.chars().next().map(|c| c.is_alphabetic()).unwrap_or(false)
        && name.chars().all(|c| c.is_uppercase() || c.is_ascii_digit() || c == '_')
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_ASM: &str = "\
    ORG $0000\n\
START:\n\
    LDX #$CAFE\n\
LOOP:\n\
    LEAX 1,X\n\
    BRA LOOP\n\
; VPy_LINE:10\n\
    NOP\n\
; VPy_LINE:11\n\
    RTS\n\
helper_fn:\n\
    RTS\n";

    #[test]
    fn test_placeholder() {
        let info = generate_debug_info("", "").unwrap();
        assert!(info.symbols.is_empty());
    }

    #[test]
    fn test_generate_pdb_labels() {
        let config = RomConfig { total_size: 32768, bank_size: 32768, bank_count: 1, is_multibank: false };
        let pdb = generate_pdb(SAMPLE_ASM, None, config).unwrap();
        assert!(pdb.labels.contains_key("START"), "must have START");
        assert!(pdb.labels.contains_key("LOOP"), "must have LOOP");
        assert!(pdb.labels.contains_key("helper_fn"), "must have helper_fn");
    }

    #[test]
    fn test_update_pdb_addresses() {
        let config = RomConfig { total_size: 32768, bank_size: 32768, bank_count: 1, is_multibank: false };
        let mut pdb = generate_pdb(SAMPLE_ASM, None, config).unwrap();

        let mut syms = HashMap::new();
        syms.insert("START".to_string(), 0x0000u32);
        syms.insert("LOOP".to_string(), 0x0003u32);
        update_pdb_addresses(&mut pdb, &syms);

        assert_eq!(pdb.labels["START"], 0x0000);
        assert_eq!(pdb.labels["LOOP"], 0x0003);
        // START is all-caps → promoted to functions too
        assert!(pdb.functions.contains_key("START"));
    }

    #[test]
    fn test_populate_line_maps() {
        let config = RomConfig { total_size: 32768, bank_size: 32768, bank_count: 1, is_multibank: false };
        let mut pdb = generate_pdb(SAMPLE_ASM, None, config).unwrap();
        populate_line_maps(&mut pdb, SAMPLE_ASM);

        // "; VPy_LINE:10" is on asm line 6 (0-indexed)
        assert!(pdb.vpy_line_map.contains_key(&10), "vpy line 10 mapped");
        assert!(pdb.vpy_line_map.contains_key(&11), "vpy line 11 mapped");
    }

    #[test]
    fn test_parse_equ_line_bios() {
        let vectrex_i = "\
Wait_Recal   EQU  $F192\n\
Intensity_a  EQU  $F2C8\n\
; comment line\n\
Sound_Byte_2 SET  $F148\n";

        let config = RomConfig { total_size: 32768, bank_size: 32768, bank_count: 1, is_multibank: false };
        let pdb = generate_pdb("", Some(vectrex_i), config).unwrap();

        assert_eq!(pdb.bios_symbols["Wait_Recal"], 0xF192);
        assert_eq!(pdb.bios_symbols["Intensity_a"], 0xF2C8);
        assert_eq!(pdb.bios_symbols["Sound_Byte_2"], 0xF148);
    }
}
