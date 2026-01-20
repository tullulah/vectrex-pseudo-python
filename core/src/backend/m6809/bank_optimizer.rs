/// Bank Optimizer - Sequential bank assignment for multi-bank ROM
/// 
/// Sequential Model (2025-01-02):
/// - Banks #0 to #(N-2): Code fills sequentially (fill #0 first, overflow to #1, etc.)
/// - Bank #(N-1): Reserved for runtime helpers (DRAW_LINE_WRAPPER, MUL16, DIV_A, etc.)
/// 
/// This eliminates the complexity of "fixed bank" concept and matches how Vectrex BIOS
/// actually works: it boots from bank #0, loads header there, and code continues naturally.

use crate::codegen::BankConfig;
use super::call_graph::{CallGraph, FunctionNode};
use std::collections::HashMap;

/// Bank assignment optimizer
pub struct BankOptimizer {
    config: BankConfig,
    graph: CallGraph,
}

/// Information about a single bank
#[derive(Debug, Clone)]
struct BankInfo {
    id: u8,
    used_bytes: usize,
    functions: Vec<String>,
}

impl BankInfo {
    fn new(id: u8) -> Self {
        BankInfo {
            id,
            used_bytes: 0,
            functions: Vec::new(),
        }
    }
    
    fn available_bytes(&self, bank_size: u32) -> usize {
        (bank_size as usize).saturating_sub(self.used_bytes)
    }
    
    fn can_fit(&self, function_size: usize, bank_size: u32) -> bool {
        self.available_bytes(bank_size) >= function_size
    }
    
    fn add_function(&mut self, name: String, size: usize) {
        self.functions.push(name);
        self.used_bytes += size;
    }
}

impl BankOptimizer {
    pub fn new(config: BankConfig, graph: CallGraph) -> Self {
        BankOptimizer { config, graph }
    }
    
    /// Assign functions to banks using sequential model
    /// 
    /// Sequential Model Algorithm:
    /// 1. Sort functions by size (largest first) - helps pack efficiently
    /// 2. Fill banks sequentially: Bank #0 first, then #1, #2, etc.
    /// 3. NEVER touch bank #(N-1) - reserved for runtime helpers
    /// 4. Assign each function to first bank with available space
    /// 5. If function doesn't fit anywhere, error
    /// 
    /// Benefits:
    /// - No artificial "fixed bank" concept
    /// - Code fills naturally from beginning
    /// - Helpers in predictable last location
    /// - Matches hardware boot sequence (BIOS loads from bank #0)
    /// 
    /// Returns: HashMap<function_name, bank_id>
    pub fn assign_banks(&self) -> Result<HashMap<String, u8>, String> {
        let bank_size = self.config.rom_bank_size;
        let total_banks = self.config.num_banks();
        
        // Code banks: #0 to #(N-2)
        // Helper bank: #(N-1) - reserved, don't allocate here
        let code_banks_count = (total_banks as u8).saturating_sub(1);
        
        if code_banks_count == 0 {
            return Err("Need at least 2 banks (1 for code, 1 for helpers)".to_string());
        }
        
        // Initialize banks (code banks #0 to #(N-2))
        let mut banks: Vec<BankInfo> = (0..code_banks_count as usize)
            .map(|i| BankInfo::new(i as u8))
            .collect();
        
        let mut assignments: HashMap<String, u8> = HashMap::new();
        
        // Sort all functions by size (largest first) - helps pack efficiently
        let mut all_functions: Vec<_> = self.graph.nodes.values().collect();
        all_functions.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
        
        // Sequential assignment: fill banks in order
        for func in all_functions {
            let mut assigned = false;
            
            // Try to fit in each bank sequentially (starting from bank #0)
            for bank in &mut banks {
                if bank.can_fit(func.size_bytes, bank_size) {
                    bank.add_function(func.name.clone(), func.size_bytes);
                    assignments.insert(func.name.clone(), bank.id);
                    assigned = true;
                    break; // Found a home, move to next function
                }
            }
            
            if !assigned {
                return Err(format!(
                    "Function '{}' ({} bytes) doesn't fit in any code bank (bank size: {} bytes, available code banks: {})",
                    func.name, func.size_bytes, bank_size, code_banks_count
                ));
            }
        }
        
        // Step 4: Validate and report
        self.validate_assignments(&banks)?;
        
        Ok(assignments)
    }
    
    /// Validate that bank assignments are valid
    fn validate_assignments(&self, banks: &[BankInfo]) -> Result<(), String> {
        let bank_size = self.config.rom_bank_size as usize;
        
        for bank in banks {
            if bank.used_bytes > bank_size {
                return Err(format!(
                    "Bank #{} overflow: {} bytes used (max: {} bytes)",
                    bank.id, bank.used_bytes, bank_size
                ));
            }
        }
        
        Ok(())
    }
    
    /// Get assignment statistics for debugging
    pub fn assignment_stats(&self, assignments: &HashMap<String, u8>) -> BankStats {
        let bank_size = self.config.rom_bank_size as usize;
        let total_banks = self.config.num_banks();
        
        // Code banks: #0 to #(N-2)
        let code_banks_count = total_banks.saturating_sub(1);
        
        // Calculate per-bank statistics
        let mut banks: Vec<BankInfo> = (0..code_banks_count)
            .map(|i| BankInfo::new(i as u8))
            .collect();
        
        for (func_name, bank_id) in assignments {
            if let Some(node) = self.graph.nodes.get(func_name) {
                if (*bank_id as usize) < code_banks_count {
                    banks[*bank_id as usize].add_function(func_name.clone(), node.size_bytes);
                }
            }
        }
        
        let used_banks = banks.iter()
            .filter(|b| !b.functions.is_empty())
            .count();
        
        let total_used_bytes: usize = banks.iter()
            .map(|b| b.used_bytes)
            .sum();
        
        let total_available_bytes = bank_size * code_banks_count;
        let utilization = (total_used_bytes as f64 / total_available_bytes as f64) * 100.0;
        
        BankStats {
            total_banks: total_banks as usize,
            code_banks: code_banks_count,
            helper_bank: (total_banks - 1) as u8,
            used_banks,
            total_functions: assignments.len(),
            total_used_bytes,
            total_available_bytes,
            utilization,
            banks: banks.into_iter()
                .filter(|b| !b.functions.is_empty())
                .collect(),
        }
    }
}

/// Statistics about bank assignments
#[derive(Debug)]
pub struct BankStats {
    pub total_banks: usize,
    pub code_banks: usize,          // Code banks #0 to #(N-2)
    pub helper_bank: u8,             // Helper bank #(N-1)
    pub used_banks: usize,
    pub total_functions: usize,
    pub total_used_bytes: usize,
    pub total_available_bytes: usize,
    pub utilization: f64,
    pub banks: Vec<BankInfo>,
}

impl BankStats {
    /// Print bank statistics in a human-readable format
    pub fn print(&self) {
        eprintln!("   [Bank Assignment] Sequential Model Statistics:");
        eprintln!("     - Total banks: {}", self.total_banks);
        eprintln!("     - Code banks: #0-#{}", self.code_banks - 1);
        eprintln!("     - Helper bank: #{}", self.helper_bank);
        eprintln!("     - Used banks: {}", self.used_banks);
        eprintln!("     - Total functions: {}", self.total_functions);
        eprintln!("     - Total used: {} bytes ({:.1} KB)", 
            self.total_used_bytes, 
            self.total_used_bytes as f64 / 1024.0);
        eprintln!("     - Utilization: {:.1}%", self.utilization);
        eprintln!("     - Banks:");
        
        for bank in &self.banks {
            let bank_size_kb = bank.used_bytes as f64 / 1024.0;
            eprintln!("       Bank #{}: {} bytes ({:.1} KB) - {} functions", 
                bank.id, bank.used_bytes, bank_size_kb, bank.functions.len());
            
            // Show functions in this bank
            for func in &bank.functions {
                eprintln!("         - {}", func);
            }
        }
        eprintln!("     - Bank #{} (helpers): [Reserved for DRAW_LINE_WRAPPER, MUL16, etc.]", self.helper_bank);
    }
}
