//! Bank allocation algorithm
//!
//! Implements the main algorithm for assigning functions to banks
//! 
//! Sequential Model (ported from core/src/backend/m6809/bank_optimizer.rs):
//! - Banks #0 to #(N-2): Code fills sequentially (fill #0 first, overflow to #1, etc.)
//! - Bank #(N-1): Reserved for runtime helpers (DRAW_LINE_WRAPPER, MUL16, etc.)

use crate::graph::CallGraph;
use crate::error::{BankAllocatorError, BankAllocatorResult};
use std::collections::HashMap;

/// Configuration for bank allocation
#[derive(Debug, Clone)]
pub struct BankConfig {
    pub rom_total_size: usize,
    pub rom_bank_size: usize,
    pub rom_bank_count: usize,
    pub helpers_bank: usize, // Always last bank (rom_bank_count - 1)
}

impl BankConfig {
    /// Create bank config from total size and bank size
    pub fn new(rom_total_size: usize, rom_bank_size: usize) -> Self {
        let rom_bank_count = rom_total_size / rom_bank_size;
        let helpers_bank = rom_bank_count.saturating_sub(1);
        
        Self {
            rom_total_size,
            rom_bank_size,
            rom_bank_count,
            helpers_bank,
        }
    }
    
    /// Single bank configuration (32KB cartridge)
    pub fn single_bank() -> Self {
        Self {
            rom_total_size: 32768,
            rom_bank_size: 32768,
            rom_bank_count: 1,
            helpers_bank: 0,
        }
    }
    
    /// Multibank configuration (512KB = 32 banks × 16KB)
    pub fn multibank_512kb() -> Self {
        Self::new(524288, 16384)
    }
}

/// Information about a single bank
#[derive(Debug, Clone)]
pub struct BankInfo {
    pub id: u8,
    pub used_bytes: usize,
    pub functions: Vec<String>,
}

impl BankInfo {
    fn new(id: u8) -> Self {
        BankInfo {
            id,
            used_bytes: 0,
            functions: Vec::new(),
        }
    }
    
    fn available_bytes(&self, bank_size: usize) -> usize {
        bank_size.saturating_sub(self.used_bytes)
    }
    
    fn can_fit(&self, function_size: usize, bank_size: usize) -> bool {
        self.available_bytes(bank_size) >= function_size
    }
    
    fn add_function(&mut self, name: String, size: usize) {
        self.functions.push(name);
        self.used_bytes += size;
    }
}

/// Bank assignment optimizer
pub struct BankAllocator {
    config: BankConfig,
    graph: CallGraph,
}

impl BankAllocator {
    pub fn new(config: BankConfig, graph: CallGraph) -> Self {
        BankAllocator { config, graph }
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
    pub fn assign_banks(&self) -> BankAllocatorResult<HashMap<String, u8>> {
        let bank_size = self.config.rom_bank_size;
        let total_banks = self.config.rom_bank_count;
        
        // Code banks: #0 to #(N-2)
        // Helper bank: #(N-1) - reserved, don't allocate here
        let code_banks_count = (total_banks as u8).saturating_sub(1);
        
        if code_banks_count == 0 {
            return Err(BankAllocatorError::Generic(
                "Need at least 2 banks (1 for code, 1 for helpers)".to_string()
            ));
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
                return Err(BankAllocatorError::FunctionTooLarge(
                    func.name.clone(),
                    func.size_bytes
                ));
            }
        }
        
        // Validate assignments
        self.validate_assignments(&banks)?;
        
        Ok(assignments)
    }
    
    /// Validate that bank assignments are valid
    fn validate_assignments(&self, banks: &[BankInfo]) -> BankAllocatorResult<()> {
        let bank_size = self.config.rom_bank_size;
        
        for bank in banks {
            if bank.used_bytes > bank_size {
                return Err(BankAllocatorError::CodeTooLarge(bank.used_bytes));
            }
        }
        
        Ok(())
    }
    
    /// Get assignment statistics for debugging
    pub fn assignment_stats(&self, assignments: &HashMap<String, u8>) -> BankStats {
        let bank_size = self.config.rom_bank_size;
        let total_banks = self.config.rom_bank_count;
        
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
            total_banks,
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
    /// Format statistics as human-readable string
    pub fn summary(&self) -> String {
        format!(
            "Bank Allocation Summary:\n\
             - Total banks: {}\n\
             - Code banks: #0-#{} (helpers in #{})\n\
             - Used banks: {}/{}\n\
             - Functions: {}\n\
             - Total used: {} bytes / {} bytes\n\
             - Utilization: {:.1}%",
            self.total_banks,
            self.code_banks - 1,
            self.helper_bank,
            self.used_banks,
            self.code_banks,
            self.total_functions,
            self.total_used_bytes,
            self.total_available_bytes,
            self.utilization
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{CallGraph, FunctionNode};

    #[test]
    fn test_single_bank_config() {
        let config = BankConfig::single_bank();
        assert_eq!(config.rom_bank_count, 1);
        assert_eq!(config.rom_bank_size, 32768);
    }
    
    #[test]
    fn test_multibank_config() {
        let config = BankConfig::multibank_512kb();
        assert_eq!(config.rom_bank_count, 32);
        assert_eq!(config.rom_bank_size, 16384);
        assert_eq!(config.helpers_bank, 31);
    }
    
    #[test]
    fn test_bank_info() {
        let mut bank = BankInfo::new(0);
        assert_eq!(bank.available_bytes(16384), 16384);
        
        bank.add_function("test".to_string(), 100);
        assert_eq!(bank.used_bytes, 100);
        assert_eq!(bank.available_bytes(16384), 16384 - 100);
    }
    
    #[test]
    fn test_allocator_simple() {
        let config = BankConfig::multibank_512kb();
        let mut graph = CallGraph::new();
        
        // Add a small function
        graph.add_node(FunctionNode {
            name: "main".to_string(),
            size_bytes: 100,
            is_critical: true,
        });
        
        let allocator = BankAllocator::new(config, graph);
        let assignments = allocator.assign_banks().unwrap();
        
        assert_eq!(assignments.len(), 1);
        assert_eq!(assignments["main"], 0); // Should go to bank #0
    }
    
    #[test]
    fn test_allocator_overflow() {
        let config = BankConfig::new(32768, 16384); // 2 banks total
        let mut graph = CallGraph::new();
        
        // Add function too large for single bank
        graph.add_node(FunctionNode {
            name: "huge".to_string(),
            size_bytes: 20000, // Larger than 16KB bank
            is_critical: false,
        });
        
        let allocator = BankAllocator::new(config, graph);
        let result = allocator.assign_banks();
        
        assert!(result.is_err());
    }
}
