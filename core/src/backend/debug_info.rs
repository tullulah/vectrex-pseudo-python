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
    
    /// Line mapping: VPy line number (as string) -> address in hex
    #[serde(rename = "lineMap")]
    pub line_map: HashMap<String, String>,
    
    /// Functions metadata for enhanced debugging
    pub functions: HashMap<String, FunctionInfo>,
    
    /// Native function calls mapping: VPy line (as string) -> native function name
    #[serde(rename = "nativeCalls")]
    pub native_calls: HashMap<String, String>,
}

impl DebugInfo {
    pub fn new(source: String, binary: String) -> Self {
        let asm = source.replace(".vpy", ".asm");
        Self {
            version: "1.0".to_string(),
            source,
            asm,
            binary,
            entry_point: "0x0000".to_string(),
            symbols: HashMap::new(),
            line_map: HashMap::new(),
            functions: HashMap::new(),
            native_calls: HashMap::new(),
        }
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
    pub fn advance(&mut self, bytes: u16) {
        self.current_address = self.current_address.wrapping_add(bytes);
    }
    
    /// Add a symbol at current address
    pub fn add_symbol(&mut self, name: String) {
        self.debug_info.add_symbol(name, self.current_address);
    }
    
    /// Add a function at current address
    pub fn add_function(&mut self, name: String, start_line: usize, end_line: usize, func_type: &str) {
        self.debug_info.add_function(name, self.current_address, start_line, end_line, func_type);
    }
    
    /// Add a native function call at current line
    pub fn add_native_call(&mut self, function_name: String) {
        if let Some(line) = self.current_line {
            self.debug_info.add_native_call(line, function_name);
        }
    }
    
    /// Get current address
    pub fn address(&self) -> u16 {
        self.current_address
    }
    
    /// Consume tracker and return debug info
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

/// Estimate the size in bytes of a single ASM instruction line
/// This is a rough approximation based on typical 6809 instruction sizes
fn estimate_instruction_size(line: &str) -> u16 {
    let trimmed = line.trim();
    
    // Extract instruction mnemonic (first word)
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.is_empty() {
        return 0;
    }
    
    let instr = parts[0].to_uppercase();
    
    // Estimate size based on instruction type
    // This is approximate - real sizes depend on addressing modes
    match instr.as_str() {
        // 1-byte instructions (inherent/implied)
        "NOP" | "INCA" | "INCB" | "DECA" | "DECB" | 
        "CLRA" | "CLRB" | "COMA" | "COMB" | "NEGA" | "NEGB" |
        "RTS" | "RTI" | "PULS" | "PSHS" | "PULU" | "PSHU" |
        "ABX" | "DAA" | "SEX" | "MUL" | "SWI" | "SWI2" | "SWI3" |
        "SYNC" | "CWAI" => 1,
        
        // 2-byte instructions (immediate/direct/indexed simple)
        "LDA" | "LDB" | "STA" | "STB" | "ADDA" | "ADDB" | "SUBA" | "SUBB" |
        "CMPA" | "CMPB" | "ANDA" | "ANDB" | "ORA" | "ORB" | "EORA" | "EORB" |
        "BITA" | "BITB" | "LDX" | "LDY" | "LDU" | "LDS" |
        "STX" | "STY" | "STU" | "STS" | "LEAX" | "LEAY" | "LEAU" | "LEAS" |
        "BRA" | "BEQ" | "BNE" | "BCC" | "BCS" | "BPL" | "BMI" | "BVC" | "BVS" |
        "BHI" | "BLS" | "BGE" | "BLT" | "BGT" | "BLE" | "BSR" => 2,
        
        // 3-byte instructions (16-bit immediate/extended/long branches)
        "LDD" | "STD" | "ADDD" | "SUBD" | "CMPD" | "CMPX" | "CMPY" | "CMPU" | "CMPS" |
        "JSR" | "JMP" | "LBRA" | "LBEQ" | "LBNE" | "LBCC" | "LBCS" | "LBPL" | "LBMI" |
        "LBVC" | "LBVS" | "LBHI" | "LBLS" | "LBGE" | "LBLT" | "LBGT" | "LBLE" | "LBSR" => 3,
        
        // TFR, EXG (register transfer) - 2 bytes
        "TFR" | "EXG" => 2,
        
        // Default: assume 2 bytes for unknown instructions
        _ => 2,
    }
}

/// Parse ASM output and build label-to-address map
/// Returns HashMap of label names to their addresses
pub fn parse_asm_addresses(asm: &str, org: u16) -> HashMap<String, u16> {
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
        
        // Regular instruction - estimate size
        current_address += estimate_instruction_size(line);
    }
    
    addresses
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

/// Estimate the size in bytes of generated ASM code
/// This is a rough approximation based on typical 6809 instruction sizes
pub fn estimate_asm_size(asm: &str) -> u16 {
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
        
        // Regular instruction
        size += estimate_instruction_size(line);
    }
    
    size
}
