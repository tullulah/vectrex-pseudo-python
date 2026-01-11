// debug_info.rs - Estructuras para debug symbols (.pdb file generation)
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Function metadata for enhanced debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    /// Function name
    pub name: String,
    
    /// Start address in hex
    pub address: String,
    
    /// Start line in VPy source
    #[serde(rename = "startLine")]
    pub start_line: usize,
    
    /// End line in VPy source
    #[serde(rename = "endLine")]
    pub end_line: usize,
    
    /// Function type: "vpy" or "native"
    #[serde(rename = "type")]
    pub func_type: String,
}

/// ASM file function location for debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsmFunctionLocation {
    /// Function name
    pub name: String,
    
    /// File name (e.g., "main.asm")
    pub file: String,
    
    /// Start line in ASM file
    #[serde(rename = "startLine")]
    pub start_line: usize,
    
    /// End line in ASM file
    #[serde(rename = "endLine")]
    pub end_line: usize,
    
    /// Function type: "vpy", "native", or "bios"
    #[serde(rename = "type")]
    pub func_type: String,
}

/// Line map entry with file source information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineMapEntry {
    /// Source file name (e.g., "main.vpy" or "main.asm")
    pub file: String,
    
    /// Memory address in hex (e.g., "0x01A1")
    pub address: String,
    
    /// Line number in source file (1-based)
    pub line: usize,
}

/// Variable metadata for memory visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableInfo {
    /// Variable name (e.g., "player_x")
    pub name: String,
    
    /// Memory address in hex (e.g., "0xCF10")
    pub address: String,
    
    /// Size in bytes (2 for 16-bit variables, 4 for arrays, etc.)
    pub size: usize,
    
    /// Variable type: "int", "array", "struct", "const"
    #[serde(rename = "type")]
    pub var_type: String,
    
    /// Line where variable is declared in VPy source
    #[serde(rename = "declLine")]
    pub decl_line: Option<usize>,
}

/// Debug information collected during compilation for mapping VPy source to binary
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DebugInfo {
    /// Version of the debug format
    pub version: String,
    
    /// Source file name (e.g., "main.vpy")
    pub source: String,
    
    /// ASM file name (e.g., "main.asm")
    pub asm: String,
    
    /// Binary file name (e.g., "main.bin")
    pub binary: String,
    
    /// Entry point address in hex (e.g., "0x0000")
    #[serde(rename = "entryPoint")]
    pub entry_point: String,
    
    /// Symbol table: symbol name -> address in hex
    pub symbols: HashMap<String, String>,
    
    /// Line mapping: VPy line number (as string) -> address in hex (DEPRECATED - use vpyLineMap)
    #[serde(rename = "lineMap")]
    pub line_map: HashMap<String, String>,
    
    /// VPy line mapping with file info: address in hex -> LineMapEntry
    #[serde(rename = "vpyLineMap")]
    pub vpy_line_map: HashMap<String, LineMapEntry>,
    
    /// ASM line mapping with file info: address in hex -> LineMapEntry  
    #[serde(rename = "asmLineMap")]
    pub asm_line_map: HashMap<String, LineMapEntry>,
    
    /// Functions metadata for enhanced debugging
    pub functions: HashMap<String, FunctionInfo>,
    
    /// Native function calls mapping: VPy line (as string) -> native function name
    #[serde(rename = "nativeCalls")]
    pub native_calls: HashMap<String, String>,
    
    /// ASM file function locations: function name -> location info
    #[serde(rename = "asmFunctions")]
    pub asm_functions: HashMap<String, AsmFunctionLocation>,
    
    /// ASM address mapping: ASM line number (as string) -> binary address in hex
    #[serde(rename = "asmAddressMap")]
    pub asm_address_map: HashMap<String, String>,
    
    /// Variables metadata for memory visualization
    pub variables: HashMap<String, VariableInfo>,
}

impl DebugInfo {
    pub fn new(source: String, binary: String) -> Self {
        // Derive ASM name from binary name (not source) to match project naming
        // e.g., "test_bp_min.bin" -> "test_bp_min.asm"
        let asm = binary.replace(".bin", ".asm");
        Self {
            version: "1.0".to_string(),
            source,
            asm,
            binary,
            entry_point: "0x0000".to_string(),
            symbols: HashMap::new(),
            line_map: HashMap::new(),
            vpy_line_map: HashMap::new(),
            asm_line_map: HashMap::new(),
            functions: HashMap::new(),
            native_calls: HashMap::new(),
            asm_functions: HashMap::new(),
            asm_address_map: HashMap::new(),
            variables: HashMap::new(),
        }
    }
    
    /// Add a variable with metadata
    pub fn add_variable(&mut self, name: String, address: u16, size: usize, var_type: &str, decl_line: Option<usize>) {
        let info = VariableInfo {
            name: name.clone(),
            address: format!("0x{:04X}", address),
            size,
            var_type: var_type.to_string(),
            decl_line,
        };
        self.variables.insert(name, info);
    }
    
    /// Add a symbol (function name, label, etc.) with its address
    pub fn add_symbol(&mut self, name: String, address: u16) {
        self.symbols.insert(name, format!("0x{:04X}", address));
    }
    
    /// Add a line mapping from VPy source line to binary address
    pub fn add_line_mapping(&mut self, line: usize, address: u16) {
        self.line_map.insert(line.to_string(), format!("0x{:04X}", address));
    }
    
    /// Set the entry point address
    pub fn set_entry_point(&mut self, address: u16) {
        self.entry_point = format!("0x{:04X}", address);
    }
    
    /// Add a function with metadata
    pub fn add_function(&mut self, name: String, address: u16, start_line: usize, end_line: usize, func_type: &str) {
        let info = FunctionInfo {
            name: name.clone(),
            address: format!("0x{:04X}", address),
            start_line,
            end_line,
            func_type: func_type.to_string(),
        };
        self.functions.insert(name, info);
    }
    
    /// Add a native function call at specific VPy line
    pub fn add_native_call(&mut self, line: usize, function_name: String) {
        self.native_calls.insert(line.to_string(), function_name);
    }
    
    /// Add ASM function location information
    pub fn add_asm_function(&mut self, name: String, file: String, start_line: usize, end_line: usize, func_type: &str) {
        let location = AsmFunctionLocation {
            name: name.clone(),
            file,
            start_line,
            end_line,
            func_type: func_type.to_string(),
        };
        self.asm_functions.insert(name, location);
    }
    
    /// Add ASM line address mapping 
    pub fn add_asm_address(&mut self, line_number: usize, address: u16) {
        self.asm_address_map.insert(line_number.to_string(), format!("0x{:04X}", address));
    }
    
    /// Add VPy line to new line map with file info
    pub fn add_vpy_line(&mut self, line: usize, address: u16, file: &str) {
        let addr_str = format!("0x{:04X}", address);
        let entry = LineMapEntry {
            file: file.to_string(),
            address: addr_str.clone(),
            line,
        };
        self.vpy_line_map.insert(addr_str, entry);
    }
    
    /// Add ASM line to new line map with file info
    pub fn add_asm_line(&mut self, line: usize, address: u16, file: &str) {
        let addr_str = format!("0x{:04X}", address);
        let entry = LineMapEntry {
            file: file.to_string(),
            address: addr_str.clone(),
            line,
        };
        self.asm_line_map.insert(addr_str, entry);
    }
    
    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// Context for tracking line information during code generation
#[derive(Debug, Clone)]
pub struct LineTracker {
    /// Current address being generated (relative to ORG)
    pub current_address: u16,
    
    /// Current VPy source line (if known)
    pub current_line: Option<usize>,
    
    /// Accumulated debug info
    pub debug_info: DebugInfo,
}

impl LineTracker {
    pub fn new(source: String, binary: String, org: u16) -> Self {
        Self {
            current_address: org,
            current_line: None,
            debug_info: DebugInfo::new(source, binary),
        }
    }
    
    /// Update current source line
    pub fn set_line(&mut self, line: usize) {
        self.current_line = Some(line);
        // Record mapping when we first encounter this line
        self.debug_info.add_line_mapping(line, self.current_address);
    }
    
    /// Add bytes to current address (track code generation progress)
    #[allow(dead_code)]
    pub fn advance(&mut self, bytes: u16) {
        self.current_address = self.current_address.wrapping_add(bytes);
    }
    
    /// Add a symbol at current address
    #[allow(dead_code)]
    pub fn add_symbol(&mut self, name: String) {
        self.debug_info.add_symbol(name, self.current_address);
    }
    
    /// Add a function at current address
    #[allow(dead_code)]
    pub fn add_function(&mut self, name: String, start_line: usize, end_line: usize, func_type: &str) {
        self.debug_info.add_function(name, self.current_address, start_line, end_line, func_type);
    }
    
    /// Add a native function call at current line
    #[allow(dead_code)]
    pub fn add_native_call(&mut self, function_name: String) {
        if let Some(line) = self.current_line {
            self.debug_info.add_native_call(line, function_name);
        }
    }
    
    /// Get current address
    #[allow(dead_code)]
    pub fn address(&self) -> u16 {
        self.current_address
    }
    
    /// Consume tracker and return debug info
    #[allow(dead_code)]
    pub fn finish(self) -> DebugInfo {
        self.debug_info
    }
}

/// Parse hex or decimal number (supports $FFFF, 0xFFFF, and decimal)
fn parse_hex_or_decimal(s: &str) -> Result<u16, ()> {
    let trimmed = s.trim();
    if trimmed.starts_with('$') {
        let hex_str = trimmed.trim_start_matches('$');
        u16::from_str_radix(hex_str, 16).map_err(|_| ())
    } else if trimmed.starts_with("0x") || trimmed.starts_with("0X") {
        let hex_str = trimmed.trim_start_matches("0x").trim_start_matches("0X");
        u16::from_str_radix(hex_str, 16).map_err(|_| ())
    } else {
        trimmed.parse::<u16>().map_err(|_| ())
    }
}

/// DEPRECATED: Instruction size estimation is unreliable and causes confusion
/// DO NOT USE - All address calculations must come from BinaryEmitter (single source of truth)
#[allow(dead_code)]
fn estimate_instruction_size_deprecated(line: &str) -> u16 {
    let trimmed = line.trim();
    
    // Extract instruction mnemonic (first word)
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.is_empty() {
        return 0;
    }
    
    let instr = parts[0].to_uppercase();
    
    // Get operand (everything after first whitespace, before any comment)
    let operand = if parts.len() > 1 {
        // Join remaining parts and strip inline comments
        let rest = trimmed[trimmed.find(parts[0]).unwrap() + parts[0].len()..].trim();
        rest.split(';').next().unwrap_or("").trim()
    } else {
        ""
    };
    
    // Determine addressing mode by examining operand
    let is_immediate = operand.starts_with('#');
    let is_indexed = operand.contains(',');
    
    // Estimate size based on instruction type and addressing mode
    match instr.as_str() {
        // 1-byte instructions (inherent/implied) - no operand or register-only
        "NOP" | "INCA" | "INCB" | "DECA" | "DECB" | 
        "CLRA" | "CLRB" | "COMA" | "COMB" | "NEGA" | "NEGB" |
        "RTS" | "RTI" | "PULS" | "PSHS" | "PULU" | "PSHU" |
        "ABX" | "DAA" | "SEX" | "MUL" | "SWI" | "SWI2" | "SWI3" |
        "SYNC" | "CWAI" => 1,
        
        // TFR, EXG (register transfer) - always 2 bytes
        "TFR" | "EXG" => 2,
        
        // Branch instructions - always 2 bytes (relative)
        "BRA" | "BEQ" | "BNE" | "BCC" | "BCS" | "BPL" | "BMI" | "BVC" | "BVS" |
        "BHI" | "BLS" | "BGE" | "BLT" | "BGT" | "BLE" | "BSR" => 2,
        
        // Long branches - always 4 bytes
        "LBRA" | "LBEQ" | "LBNE" | "LBCC" | "LBHS" | "LBCS" | "LBLO" | "LBPL" | "LBMI" |
        "LBVC" | "LBVS" | "LBHI" | "LBLS" | "LBGE" | "LBLT" | "LBGT" | "LBLE" | "LBSR" => 4,
        
        // JSR, JMP - always 3 bytes (extended addressing)
        "JSR" | "JMP" => 3,
        
        // 8-bit accumulator instructions (LDA, STA, etc.)
        "LDA" | "LDB" | "STA" | "STB" | "ADDA" | "ADDB" | "SUBA" | "SUBB" |
        "SBCA" | "SBCB" |
        "CMPA" | "CMPB" | "ANDA" | "ANDB" | "ORA" | "ORB" | "EORA" | "EORB" |
        "BITA" | "BITB" => {
            if is_immediate {
                2  // Immediate: opcode + byte
            } else if is_indexed {
                2  // Indexed: opcode + postbyte (simplified, can be more)
            } else {
                3  // Extended: opcode + address (most common for labels)
            }
        }
        
        // 16-bit register instructions (LDD, STD, LDX, etc.)
        "LDD" | "STD" | "ADDD" | "SUBD" | "CMPD" |
        "LDX" | "LDY" | "LDU" | "LDS" | "STX" | "STY" | "STU" | "STS" |
        "CMPX" | "CMPY" | "CMPU" | "CMPS" => {
            if is_immediate {
                3  // Immediate 16-bit: opcode + word
            } else if is_indexed {
                2  // Indexed: opcode + postbyte (simplified)
            } else {
                3  // Extended: opcode + address
            }
        }
        
        // LEA instructions - usually indexed
        "LEAX" | "LEAY" | "LEAU" | "LEAS" => {
            if is_indexed {
                2  // Indexed: opcode + postbyte
            } else {
                3  // Extended (rare but possible)
            }
        }
        
        // Default: assume 3 bytes for safety (extended addressing)
        _ => 3,
    }
}

/// DEPRECATED: ASM address estimation is unreliable and causes confusion
/// DO NOT USE - All symbols must come from binary_symbol_table (Phase 6)
/// Use BinaryEmitter as single source of truth
#[allow(dead_code)]
pub fn parse_asm_addresses_deprecated(asm: &str, org: u16) -> HashMap<String, u16> {
    let mut addresses = HashMap::new();
    let mut current_address = org;
    let mut line_count = 0;
    const MAX_LINES: usize = 100_000; // Safety limit
    
    for line in asm.lines() {
        line_count += 1;
        if line_count > MAX_LINES {
            eprintln!("WARNING: parse_asm_addresses exceeded {} lines, stopping", MAX_LINES);
            break;
        }
        
        let trimmed = line.trim();
        
        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with('*') {
            continue;
        }
        
        // Detect labels (end with ':')
        if trimmed.ends_with(':') {
            let label = trimmed.trim_end_matches(':').trim().to_string();
            addresses.insert(label, current_address);
            continue;
        }
        
        // Detect ORG directive (changes current address)
        if trimmed.starts_with("ORG ") {
            if let Some(addr_str) = trimmed.split_whitespace().nth(1) {
                if let Ok(addr) = parse_hex_or_decimal(addr_str) {
                    current_address = addr;
                }
            }
            continue;
        }
        
        // Skip directives that don't generate code
        if trimmed.starts_with("INCLUDE ") || trimmed.starts_with("EQU ") {
            continue;
        }
        
        // Data directives that add bytes
        if trimmed.starts_with("FDB ") {
            // FDB adds 2 bytes per word
            current_address += 2;
            continue;
        }
        
        if trimmed.starts_with("FCB ") {
            // FCB adds 1 byte
            current_address += 1;
            continue;
        }
        
        if trimmed.starts_with("FCC ") {
            // FCC adds string length
            if let Some(start) = trimmed.find('"') {
                if let Some(end) = trimmed.rfind('"') {
                    if end > start {
                        current_address += (end - start - 1) as u16;
                    }
                }
            }
            continue;
        }
        
        if trimmed.starts_with("RMB ") {
            // RMB reserves bytes
            if let Some(count_str) = trimmed.split_whitespace().nth(1) {
                if let Ok(count) = parse_hex_or_decimal(count_str) {
                    current_address += count;
                }
            }
            continue;
        }
        
        // Regular instruction - DEPRECATED estimation (DO NOT USE)
        current_address += estimate_instruction_size_deprecated(line);
    }
    
    addresses
}

/// Parse variable definitions from ASM EQU directives
/// Format: "VAR_NAME EQU $CF10+0" or "RESULT EQU $C880+$00"
/// Returns: HashMap populated in debug_info.variables
pub fn parse_asm_variables(asm: &str) -> HashMap<String, VariableInfo> {
    let mut variables = HashMap::new();
    
    for line in asm.lines() {
        let trimmed = line.trim();
        
        // Skip comments and empty lines
        if trimmed.is_empty() || trimmed.starts_with(';') {
            continue;
        }
        
        // Look for EQU directives
        if !trimmed.contains(" EQU ") {
            continue;
        }
        
        // Parse: "VAR_NAME EQU $CF10+0  ; Array data (5 elements)" or "RESULT EQU $C880+$00"
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }
        
        let var_name = parts[0];
        let address_expr = parts[2];
        
        // Parse address expression (e.g., "$CF10+0" or "$C880+$00")
        let address = if address_expr.contains('+') {
            // Calculate base + offset
            let addr_parts: Vec<&str> = address_expr.split('+').collect();
            if addr_parts.len() == 2 {
                let base = parse_hex_or_decimal(addr_parts[0]).unwrap_or(0);
                let offset = parse_hex_or_decimal(addr_parts[1]).unwrap_or(0);
                base.wrapping_add(offset)
            } else {
                parse_hex_or_decimal(address_expr).unwrap_or(0)
            }
        } else {
            parse_hex_or_decimal(address_expr).unwrap_or(0)
        };
        
        // Determine variable type and size from name and comment
        let comment = trimmed.split(';').nth(1).unwrap_or("").trim();
        
        // First, try to extract explicit size from comment (e.g., "(320 bytes)" or "(1 byte)")
        let explicit_size = if let (Some(paren_start), Some(paren_end)) = (comment.rfind('('), comment.rfind(')')) {
            if paren_end > paren_start {
                let inside = &comment[paren_start + 1..paren_end];
                inside
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse::<usize>().ok())
            } else {
                None
            }
        } else {
            None
        };
        
        let (var_type, size) = if let Some(explicit_bytes) = explicit_size {
            // Use explicit size from comment
            if var_name.contains("BUFFER") || var_name.contains("LEVEL_") {
                ("system", explicit_bytes)
            } else if var_name.contains("_DATA") {
                ("array", explicit_bytes)
            } else {
                ("unknown", explicit_bytes)
            }
        } else if var_name.contains("_DATA") {
            // Array data - try to extract element count from comment
            let element_count = if comment.contains("elements") {
                // Extract number from "Array data (5 elements)"
                comment
                    .split('(')
                    .nth(1)
                    .and_then(|s| s.split(' ').next())
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(1)
            } else {
                1
            };
            ("array", element_count * 2)
        } else if var_name.starts_with("VAR_ARG") {
            ("arg", 2)
        } else if var_name.starts_with("VAR_") {
            ("int", 2)
        } else if var_name.contains("PSG_") || var_name.contains("SFX_") || var_name.contains("MUSIC_") {
            // Audio variables - some are 1 byte, some are 2
            let is_byte = comment.contains("(1 byte)") || 
                         var_name.ends_with("_ACTIVE") || 
                         var_name.ends_with("_PLAYING") ||
                         var_name.ends_with("_COUNT") ||
                         var_name.ends_with("_FRAMES") ||
                         var_name.ends_with("_PHASE") ||
                         var_name.ends_with("_VOL");
            ("audio", if is_byte { 1 } else { 2 })
        } else if var_name.starts_with("RESULT") || var_name.starts_with("TMP") || 
                  var_name.starts_with("MUL_") || var_name.starts_with("DIV_") {
            ("system", 2)
        } else {
            ("unknown", 2)
        };
        
        // Clean variable name (remove VAR_ prefix for user variables)
        let clean_name = if var_name.starts_with("VAR_") {
            var_name.strip_prefix("VAR_")
                .unwrap_or(var_name)
                .strip_suffix("_DATA")
                .unwrap_or(var_name.strip_prefix("VAR_").unwrap_or(var_name))
                .to_lowercase()
        } else {
            var_name.to_string()
        };
        
        variables.insert(clean_name.clone(), VariableInfo {
            name: clean_name,
            address: format!("0x{:04X}", address),
            size,
            var_type: var_type.to_string(),
            decl_line: None,
        });
    }
    
    variables
}

/// DEPRECATED: VPy line marker parsing with estimation is unreliable
/// DO NOT USE - Use generate_asm_address_map (Phase 6.5) + asmAddressMap instead
/// Real addresses come from binary disassembly, not estimation
#[allow(dead_code)]
pub fn parse_vpy_line_markers_deprecated(asm: &str, org: u16) -> HashMap<String, String> {
    let mut line_map = HashMap::new();
    let mut current_address = org;
    let mut pending_marker: Option<String> = None;
    
    for line in asm.lines() {
        let trimmed = line.trim();
        
        // Check for VPy_LINE marker
        if trimmed.starts_with("; VPy_LINE:") {
            // Parse: "; VPy_LINE:7"
            if let Some(line_num_str) = trimmed.strip_prefix("; VPy_LINE:") {
                let line_num = line_num_str.trim().to_string();
                // Don't record yet - wait for next instruction
                pending_marker = Some(line_num);
            }
            continue;
        }
        
        // Skip empty lines and pure comments (but NOT pseudo-labels)
        if trimmed.is_empty() || (trimmed.starts_with(';') && !trimmed.starts_with("; VPy_LINE:") && !trimmed.starts_with("; _")) || trimmed.starts_with('*') {
            continue;
        }
        
        // Handle pseudo-labels in comments (e.g., "; _CONST_DECL_0:" for const declarations)
        if trimmed.starts_with("; _") && trimmed.ends_with(':') {
            // Treat pseudo-labels like regular labels for pending marker registration
            if let Some(line_num) = pending_marker.take() {
                line_map.insert(line_num, format!("0x{:04X}", current_address));
            }
            continue;
        }
        
        // Handle labels - they may have a pending marker to record
        if trimmed.ends_with(':') {
            // If we have a pending marker and we're hitting a label, record the marker at the label address
            // (This happens for declarations like `main:` or `const` arrays that need line mapping)
            if let Some(line_num) = pending_marker.take() {
                line_map.insert(line_num, format!("0x{:04X}", current_address));
            }
            continue;
        }
        
        // Detect ORG directive (changes current address)
        if trimmed.starts_with("ORG ") {
            if let Some(addr_str) = trimmed.split_whitespace().nth(1) {
                if let Ok(addr) = parse_hex_or_decimal(addr_str) {
                    current_address = addr;
                }
            }
            continue;
        }
        
        // Skip non-code directives that don't generate bytes
        if trimmed.starts_with("INCLUDE ") || trimmed.starts_with("EQU ") {
            continue;
        }
        
        // IMPORTANT: Register pending_marker BEFORE processing data directives
        // This ensures const arrays and other data sections get mapped to current address
        if let Some(line_num) = pending_marker.take() {
            line_map.insert(line_num, format!("0x{:04X}", current_address));
        }
        
        // Data directives that add bytes
        if trimmed.starts_with("FDB ") {
            current_address += 2;
            continue;
        }
        
        if trimmed.starts_with("FCB ") {
            current_address += 1;
            continue;
        }
        
        if trimmed.starts_with("FCC ") {
            // FCC adds string length
            if let Some(start) = trimmed.find('"') {
                if let Some(end) = trimmed.rfind('"') {
                    if end > start {
                        let len = (end - start - 1) as u16;
                        current_address += len;
                    }
                }
            }
            continue;
        }
        
        if trimmed.starts_with("RMB ") {
            // RMB reserves bytes
            if let Some(count_str) = trimmed.split_whitespace().nth(1) {
                if let Ok(count) = parse_hex_or_decimal(count_str) {
                    current_address += count;
                }
            }
            continue;
        }
        
        // Regular instruction - DEPRECATED estimation (DO NOT USE)
        current_address += estimate_instruction_size_deprecated(line);
    }
    
    // Register any pending marker at end of file
    // (This handles const declarations that may be followed by blank lines/comments only)
    if let Some(line_num) = pending_marker.take() {
        line_map.insert(line_num, format!("0x{:04X}", current_address));
    }
    
    line_map
}

/// Parse native call comments from generated ASM
/// Format: "; NATIVE_CALL: FUNCTION_NAME at line N"
/// Returns: HashMap<line_number, function_name>
pub fn parse_native_call_comments(asm: &str) -> HashMap<usize, String> {
    let mut native_calls = HashMap::new();
    
    for line in asm.lines() {
        let trimmed = line.trim();
        
        // Look for NATIVE_CALL comments
        if trimmed.starts_with("; NATIVE_CALL:") {
            // Parse: "; NATIVE_CALL: VECTREX_PRINT_TEXT at line 42"
            if let Some(after_colon) = trimmed.strip_prefix("; NATIVE_CALL:") {
                let parts: Vec<&str> = after_colon.trim().split(" at line ").collect();
                if parts.len() == 2 {
                    let function_name = parts[0].trim().to_string();
                    if let Ok(line_num) = parts[1].trim().parse::<usize>() {
                        native_calls.insert(line_num, function_name);
                    }
                }
            }
        }
    }
    
    native_calls
}

/// DEPRECATED: ASM size estimation is unreliable
/// DO NOT USE - Get size from real binary instead
#[allow(dead_code)]
pub fn estimate_asm_size_deprecated(asm: &str) -> u16 {
    let mut size = 0u16;
    
    for line in asm.lines() {
        let trimmed = line.trim();
        
        // Skip comments, labels, and empty lines
        if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with('*') {
            continue;
        }
        
        // Skip labels (lines ending with ':')
        if trimmed.ends_with(':') {
            continue;
        }
        
        // Skip non-code directives
        if trimmed.starts_with("ORG ") || trimmed.starts_with("INCLUDE ") || trimmed.starts_with("EQU ") {
            continue;
        }
        
        // Data directives
        if trimmed.starts_with("FDB ") {
            size += 2;
            continue;
        }
        if trimmed.starts_with("FCB ") {
            size += 1;
            continue;
        }
        if trimmed.starts_with("FCC ") {
            // Count string length
            if let Some(start) = trimmed.find('"') {
                if let Some(end) = trimmed.rfind('"') {
                    if end > start {
                        size += (end - start - 1) as u16;
                    }
                }
            }
            continue;
        }
        
        // Regular instruction - DEPRECATED estimation (DO NOT USE)
        size += estimate_instruction_size_deprecated(line);
    }
    
    size
}
