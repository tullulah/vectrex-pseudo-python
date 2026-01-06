// Address tracking for generating ASM listing with address comments
// This allows the asm_address_mapper to build an accurate PDB without guessing

use std::sync::Mutex;

/// Global address tracker for ASM emission
static ADDRESS_TRACKER: Mutex<Option<AddressTracker>> = Mutex::new(None);

#[derive(Debug, Clone)]
pub struct AddressTracker {
    current_address: u16,
    enabled: bool,
}

impl AddressTracker {
    pub fn new(start_address: u16) -> Self {
        Self {
            current_address: start_address,
            enabled: true,
        }
    }
    
    /// Initialize the global tracker
    pub fn init_global(start_address: u16) {
        let mut tracker = ADDRESS_TRACKER.lock().unwrap();
        *tracker = Some(AddressTracker::new(start_address));
    }
    
    /// Get current address
    pub fn current() -> u16 {
        let tracker = ADDRESS_TRACKER.lock().unwrap();
        tracker.as_ref().map(|t| t.current_address).unwrap_or(0)
    }
    
    /// Advance address by N bytes
    pub fn advance(bytes: u16) {
        let mut tracker = ADDRESS_TRACKER.lock().unwrap();
        if let Some(ref mut t) = *tracker {
            if t.enabled {
                t.current_address = t.current_address.wrapping_add(bytes);
            }
        }
    }
    
    /// Set address explicitly (for ORG directives)
    pub fn set(address: u16) {
        let mut tracker = ADDRESS_TRACKER.lock().unwrap();
        if let Some(ref mut t) = *tracker {
            t.current_address = address;
        }
    }
    
    /// Disable tracking (for data sections)
    pub fn disable() {
        let mut tracker = ADDRESS_TRACKER.lock().unwrap();
        if let Some(ref mut t) = *tracker {
            t.enabled = false;
        }
    }
    
    /// Enable tracking
    pub fn enable() {
        let mut tracker = ADDRESS_TRACKER.lock().unwrap();
        if let Some(ref mut t) = *tracker {
            t.enabled = true;
        }
    }
    
    /// Generate address comment for current line
    pub fn comment() -> String {
        let addr = Self::current();
        format!("; 0x{:04X}\n", addr)
    }
}

/// Helper to emit instruction with address comment
pub fn emit_with_addr(instruction: &str) -> String {
    let comment = AddressTracker::comment();
    let size = estimate_instruction_size(instruction);
    AddressTracker::advance(size);
    format!("{}{}", comment, instruction)
}

/// Estimate instruction size based on mnemonic
/// This is used to advance the address counter - must be accurate
fn estimate_instruction_size(instruction: &str) -> u16 {
    let trimmed = instruction.trim();
    
    // Skip empty lines and comments
    if trimmed.is_empty() || trimmed.starts_with(';') {
        return 0;
    }
    
    // Labels don't consume space
    if trimmed.ends_with(':') {
        return 0;
    }
    
    // Directives (FCB, FDB, FCC, etc.) - parse the data
    if trimmed.starts_with("FCB") {
        // Count comma-separated values
        let data_part = trimmed.trim_start_matches("FCB").trim();
        let values = data_part.split(',').count();
        return values as u16;
    }
    if trimmed.starts_with("FDB") {
        let data_part = trimmed.trim_start_matches("FDB").trim();
        let values = data_part.split(',').count();
        return (values * 2) as u16;
    }
    if trimmed.starts_with("FCC") {
        // Parse string length - format: FCC "text"
        if let Some(start) = trimmed.find('"') {
            if let Some(end) = trimmed[start+1..].find('"') {
                return (end + 1) as u16; // +1 for null terminator typically
            }
        }
        return 1; // Fallback
    }
    
    // Extract mnemonic (first word)
    let mnemonic = trimmed.split_whitespace().next().unwrap_or("");
    let upper_mnemonic = mnemonic.to_uppercase();
    
    // Check for addressing modes
    let has_immediate = trimmed.contains('#');
    let has_extended = trimmed.contains('>') || trimmed.contains('<');
    let has_indexed = trimmed.contains(',');
    
    // Common instruction sizes
    match upper_mnemonic.as_str() {
        // Inherent mode (1 byte)
        "NOP" | "DAA" | "SEX" | "RTS" | "RTI" | "ABX" | "MUL" | "SWI" | "SYNC" => 1,
        "NEGA" | "COMA" | "LSRA" | "RORA" | "ASRA" | "ASLA" | "ROLA" | "DECA" | "INCA" | "TSTA" | "CLRA" => 1,
        "NEGB" | "COMB" | "LSRB" | "RORB" | "ASRB" | "ASLB" | "ROLB" | "DECB" | "INCB" | "TSTB" | "CLRB" => 1,
        
        // Register operations (2 bytes)
        "TFR" | "EXG" | "PSHS" | "PSHU" | "PULS" | "PULU" => 2,
        
        // Branches (2 bytes for short, 3 for long)
        "BRA" | "BRN" | "BHI" | "BLS" | "BCC" | "BCS" | "BNE" | "BEQ" | "BVC" | "BVS" | "BPL" | "BMI" | "BGE" | "BLT" | "BGT" | "BLE" | "BSR" => 2,
        "LBRA" | "LBRN" | "LBHI" | "LBLS" | "LBCC" | "LBCS" | "LBNE" | "LBEQ" | "LBVC" | "LBVS" | "LBPL" | "LBMI" | "LBGE" | "LBLT" | "LBGT" | "LBLE" | "LBSR" => 3,
        
        // Load/Store - depends on addressing mode
        "LDA" | "LDB" | "STA" | "STB" | "CMPA" | "CMPB" | "ADDA" | "ADDB" | "SUBA" | "SUBB" | "ANDA" | "ANDB" | "ORA" | "ORB" | "EORA" | "EORB" => {
            if has_immediate {
                2 // Immediate: opcode + byte
            } else if has_extended {
                3 // Extended: opcode + 16-bit address
            } else if has_indexed {
                2 // Indexed: opcode + post-byte (simplified)
            } else {
                2 // Direct: opcode + 8-bit address
            }
        }
        
        "LDD" | "LDX" | "LDY" | "LDU" | "LDS" | "STD" | "STX" | "STY" | "STU" | "STS" | "CMPD" | "CMPX" | "CMPY" | "CMPU" | "CMPS" | "ADDD" | "SUBD" => {
            if has_immediate {
                3 // Immediate 16-bit: opcode + word
            } else if has_extended {
                3 // Extended: opcode + 16-bit address
            } else if has_indexed {
                2 // Indexed: opcode + post-byte (simplified)
            } else {
                2 // Direct: opcode + 8-bit address
            }
        }
        
        // LEA always indexed (2 bytes)
        "LEAX" | "LEAY" | "LEAU" | "LEAS" => 2,
        
        // JSR/JMP
        "JSR" => {
            if has_extended {
                3
            } else if has_indexed {
                2
            } else {
                2 // Direct
            }
        }
        "JMP" => {
            if has_extended {
                3
            } else {
                2
            }
        }
        
        // Default for unknown instructions
        _ => 2,
    }
}
