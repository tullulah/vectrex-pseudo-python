// Bank Switching - Cross-Bank Call Wrapper Generator (Phase 2.6 - TODO #8)
//
// Purpose: Automatically detect and generate wrapper functions for cross-bank calls
//
// When a function in Bank X calls a function in Bank Y (X ≠ Y), we need:
// 1. Detect the cross-bank call during emission
// 2. Generate a wrapper function that:
//    - Saves current bank
//    - Switches to target bank
//    - Calls real function
//    - Restores original bank
// 3. Modify call site to use wrapper instead of direct call
//
// Example:
//   func_a (Bank #0) calls func_b (Bank #31)
//   Generated wrapper:
//     func_b_BANK_WRAPPER:
//         PSHS A              ; Save registers
//         LDA $4000           ; Read current bank
//         PSHS A              ; Save current bank
//         LDA #31             ; Load target bank
//         STA $4000           ; Switch to bank #31
//         JSR func_b          ; Call real function
//         PULS A              ; Restore original bank
//         STA $4000           ; Switch back
//         PULS A              ; Restore registers
//         RTS

use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use once_cell::sync::Lazy;

/// Global wrapper generator (thread-safe singleton)
/// Used during emission to determine if JSR calls need wrappers
static WRAPPER_GENERATOR: Lazy<Mutex<Option<BankWrapperGenerator>>> = Lazy::new(|| Mutex::new(None));

/// Initialize global wrapper generator (called from emit_with_debug)
pub fn init_global_generator(generator: BankWrapperGenerator) {
    let mut global = WRAPPER_GENERATOR.lock().unwrap();
    *global = Some(generator);
}

/// Clear global wrapper generator (cleanup after emission)
pub fn clear_global_generator() {
    let mut global = WRAPPER_GENERATOR.lock().unwrap();
    *global = None;
}

/// Check if a call from caller_func to callee_func needs a wrapper
/// Returns wrapper name if needed, None if same-bank or no generator
pub fn get_wrapper_for_call(caller_func: &str, callee_func: &str) -> Option<String> {
    let global = WRAPPER_GENERATOR.lock().unwrap();
    if let Some(ref gen) = *global {
        if gen.is_cross_bank_call(caller_func, callee_func).is_some() {
            return Some(gen.wrapper_name(callee_func));
        }
    }
    None
}

/// Information about a cross-bank call that needs a wrapper
#[derive(Debug, Clone)]
pub struct CrossBankCall {
    pub caller_func: String,      // Name of calling function
    pub caller_bank: u8,           // Bank of caller
    pub callee_func: String,       // Name of called function
    pub callee_bank: u8,           // Bank of callee
}

/// Tracks cross-bank calls and generates wrappers
#[derive(Clone)]
pub struct BankWrapperGenerator {
    /// Function name → bank ID mapping (from bank optimizer)
    function_banks: HashMap<String, u8>,
    
    /// Function name → source line number (for debugging line markers)
    function_lines: HashMap<String, usize>,
    
    /// Set of cross-bank calls detected
    pub cross_bank_calls: Vec<CrossBankCall>,
    
    /// Set of wrapper names already generated (avoid duplicates)
    generated_wrappers: HashSet<String>,
    
    /// Bank register address (hardware-dependent)
    bank_register: u16,
}

impl BankWrapperGenerator {
    /// Create new wrapper generator with function→bank mapping
    pub fn new(function_banks: HashMap<String, u8>, bank_register: u16) -> Self {
        Self {
            function_banks,
            function_lines: HashMap::new(),
            cross_bank_calls: Vec::new(),
            generated_wrappers: HashSet::new(),
            bank_register,
        }
    }
    
    /// Register the source line number for a function (for debugging)
    pub fn register_function_line(&mut self, func_name: &str, line: usize) {
        self.function_lines.insert(func_name.to_string(), line);
    }
    
    /// Detect if a function call is cross-bank
    /// Returns Some(target_bank) if cross-bank, None if same-bank or unknown
    pub fn is_cross_bank_call(&self, caller_func: &str, callee_func: &str) -> Option<u8> {
        let caller_bank = self.function_banks.get(caller_func)?;
        let callee_bank = self.function_banks.get(callee_func)?;
        
        if caller_bank != callee_bank {
            Some(*callee_bank)
        } else {
            None
        }
    }
    
    /// Record a cross-bank call for later wrapper generation
    pub fn record_cross_bank_call(&mut self, caller_func: String, callee_func: String) {
        if let (Some(&caller_bank), Some(&callee_bank)) = 
            (self.function_banks.get(&caller_func), self.function_banks.get(&callee_func)) 
        {
            if caller_bank != callee_bank {
                self.cross_bank_calls.push(CrossBankCall {
                    caller_func,
                    caller_bank,
                    callee_func,
                    callee_bank,
                });
            }
        }
    }
    
    /// Get wrapper name for a function
    pub fn wrapper_name(&self, func_name: &str) -> String {
        format!("{}_BANK_WRAPPER", func_name)
    }
    
    /// Generate ASM code for a cross-bank wrapper function
    pub fn generate_wrapper(&mut self, func_name: &str, target_bank: u8) -> String {
        let wrapper_name = self.wrapper_name(func_name);
        
        // Avoid generating duplicate wrappers
        if self.generated_wrappers.contains(&wrapper_name) {
            return String::new();
        }
        
        self.generated_wrappers.insert(wrapper_name.clone());
        
        // Generate wrapper ASM (CURRENT_ROM_BANK = RAM tracker, $D000 = hardware register)
        // NOTE: Hardware intercepts writes to $D000 to perform actual bank switching
        //       CURRENT_ROM_BANK keeps RAM copy for debugging/inspection
        
        // Get source line number for debugging (if available)
        let line_marker = if let Some(&line) = self.function_lines.get(func_name) {
            format!("    ; VPy_LINE:{}\n", line)
        } else {
            String::new()
        };
        
        format!(
r#"
; Cross-bank wrapper for {fname} (Bank #{tbank})
{wname}:
    PSHS A              ; Save A register
    LDA CURRENT_ROM_BANK ; Read current bank from RAM
    PSHS A              ; Save current bank on stack
    LDA #{tbank}             ; Load target bank ID
    STA CURRENT_ROM_BANK ; Switch to target bank (RAM tracker)
    STA $D000            ; Hardware bank switch register (cartucho intercepts)
{line_marker}    JSR {upper_fname}              ; Call real function
    PULS A              ; Restore original bank from stack
    STA CURRENT_ROM_BANK ; Switch back to original bank (RAM tracker)
    STA $D000            ; Hardware bank switch register (cartucho intercepts)
    PULS A              ; Restore A register
    RTS
"#,
            fname=func_name,
            tbank=target_bank,
            wname=wrapper_name,
            upper_fname=func_name.to_uppercase(),
            line_marker=line_marker,
        )
    }
    
    /// Generate all wrapper functions for detected cross-bank calls
    pub fn generate_all_wrappers(&mut self) -> String {
        let mut output = String::new();
        
        if self.cross_bank_calls.is_empty() {
            return output;
        }
        
        output.push_str("\n; ===== CROSS-BANK CALL WRAPPERS =====\n");
        output.push_str("; Auto-generated wrappers for bank switching\n\n");
        
        // Collect unique callee functions
        let mut unique_callees: HashMap<String, u8> = HashMap::new();
        for call in &self.cross_bank_calls {
            unique_callees.insert(call.callee_func.clone(), call.callee_bank);
        }
        
        // Generate wrapper for each unique callee
        for (func_name, target_bank) in unique_callees {
            let wrapper = self.generate_wrapper(&func_name, target_bank);
            output.push_str(&wrapper);
        }
        
        output.push_str("; ===== END CROSS-BANK WRAPPERS =====\n\n");
        output
    }
    
    /// Print statistics about cross-bank calls
    pub fn print_statistics(&self) {
        if self.cross_bank_calls.is_empty() {
            eprintln!("   [Bank Wrappers] No cross-bank calls detected");
            return;
        }
        
        eprintln!("   [Bank Wrappers] Statistics:");
        eprintln!("     - Total cross-bank calls: {}", self.cross_bank_calls.len());
        eprintln!("     - Unique wrappers generated: {}", self.generated_wrappers.len());
        
        // Group by caller→callee bank transition
        let mut transitions: HashMap<(u8, u8), usize> = HashMap::new();
        for call in &self.cross_bank_calls {
            *transitions.entry((call.caller_bank, call.callee_bank)).or_insert(0) += 1;
        }
        
        eprintln!("     - Bank transitions:");
        for ((from_bank, to_bank), count) in transitions {
            eprintln!("       Bank #{} → Bank #{}: {} call(s)", from_bank, to_bank, count);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_cross_bank_call() {
        let mut function_banks = HashMap::new();
        function_banks.insert("func_a".to_string(), 0);
        function_banks.insert("func_b".to_string(), 31);
        function_banks.insert("func_c".to_string(), 0);
        
        let generator = BankWrapperGenerator::new(function_banks, 0x4000);
        
        // Cross-bank call (Bank 0 → Bank 31)
        assert_eq!(generator.is_cross_bank_call("func_a", "func_b"), Some(31));
        
        // Same-bank call (Bank 0 → Bank 0)
        assert_eq!(generator.is_cross_bank_call("func_a", "func_c"), None);
        
        // Unknown function
        assert_eq!(generator.is_cross_bank_call("func_a", "unknown"), None);
    }
    
    #[test]
    fn test_wrapper_generation() {
        let mut function_banks = HashMap::new();
        function_banks.insert("target_func".to_string(), 31);
        
        let mut generator = BankWrapperGenerator::new(function_banks, 0x4000);
        
        let wrapper = generator.generate_wrapper("target_func", 31);
        
        // Verify wrapper contains expected elements
        assert!(wrapper.contains("target_func_BANK_WRAPPER:"));
        assert!(wrapper.contains("LDA #31"));
        assert!(wrapper.contains("STA CURRENT_ROM_BANK"));
        assert!(wrapper.contains("JSR target_func"));
        
        // Second generation should return empty (already generated)
        let wrapper2 = generator.generate_wrapper("target_func", 31);
        assert!(wrapper2.is_empty());
    }
    
    #[test]
    fn test_record_cross_bank_call() {
        let mut function_banks = HashMap::new();
        function_banks.insert("caller".to_string(), 0);
        function_banks.insert("callee".to_string(), 31);
        
        let mut generator = BankWrapperGenerator::new(function_banks, 0x4000);
        
        generator.record_cross_bank_call("caller".to_string(), "callee".to_string());
        
        assert_eq!(generator.cross_bank_calls.len(), 1);
        assert_eq!(generator.cross_bank_calls[0].caller_func, "caller");
        assert_eq!(generator.cross_bank_calls[0].callee_func, "callee");
        assert_eq!(generator.cross_bank_calls[0].caller_bank, 0);
        assert_eq!(generator.cross_bank_calls[0].callee_bank, 31);
    }
}
