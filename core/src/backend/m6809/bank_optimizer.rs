/// Bank Optimizer - Automatic bank assignment using First-Fit Decreasing bin packing
/// 
/// This module implements the automatic bank assignment algorithm for multi-bank ROMs.
/// It uses a First-Fit Decreasing (FFD) strategy to pack functions into banks efficiently.

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
    
    /// Assign functions to banks using First-Fit Decreasing algorithm
    /// 
    /// Algorithm steps:
    /// 1. Identify critical functions (main, loop) → assign to fixed bank
    /// 2. Sort remaining functions by size (largest first)
    /// 3. For each function, find first bank with enough space
    /// 4. Validate all functions fit within available banks
    /// 
    /// Returns: HashMap<function_name, bank_id>
    pub fn assign_banks(&self) -> Result<HashMap<String, u8>, String> {
        let bank_size = self.config.rom_bank_size;
        let total_banks = self.config.num_banks();
        let fixed_bank = (total_banks - 1) as u8;
        
        // Initialize banks (bank 0 to bank N-1)
        let mut banks: Vec<BankInfo> = (0..total_banks)
            .map(|i| BankInfo::new(i as u8))
            .collect();
        
        let mut assignments: HashMap<String, u8> = HashMap::new();
        
        // Step 1: Assign critical functions to fixed bank
        let critical_functions: Vec<_> = self.graph.nodes.values()
            .filter(|n| n.is_critical)
            .collect();
        
        for func in &critical_functions {
            banks[fixed_bank as usize].add_function(func.name.clone(), func.size_bytes);
            assignments.insert(func.name.clone(), fixed_bank);
        }
        
        // Step 2: Sort non-critical functions by size (largest first)
        let mut remaining_functions: Vec<_> = self.graph.nodes.values()
            .filter(|n| !n.is_critical)
            .collect();
        remaining_functions.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
        
        // Step 3: First-Fit Decreasing assignment with load balancing
        // Use round-robin to distribute functions across banks instead of cramming into Bank #0
        let mut current_bank = 0usize;
        
        for func in remaining_functions {
            let mut assigned = false;
            let start_bank = current_bank;
            
            // Try to fit starting from current_bank, wrapping around
            loop {
                if current_bank >= (total_banks - 1) as usize {
                    current_bank = 0;
                }
                
                let bank = &mut banks[current_bank];
                if bank.can_fit(func.size_bytes, bank_size) {
                    bank.add_function(func.name.clone(), func.size_bytes);
                    assignments.insert(func.name.clone(), bank.id);
                    assigned = true;
                    current_bank = (current_bank + 1) % ((total_banks - 1) as usize); // Move to next bank
                    break;
                }
                
                current_bank = (current_bank + 1) % ((total_banks - 1) as usize);
                
                // If we've checked all banks, break
                if current_bank == start_bank {
                    break;
                }
            }
            
            // If no space in swappable banks, try fixed bank (as last resort)
            if !assigned {
                let fixed = &mut banks[fixed_bank as usize];
                if fixed.can_fit(func.size_bytes, bank_size) {
                    fixed.add_function(func.name.clone(), func.size_bytes);
                    assignments.insert(func.name.clone(), fixed_bank);
                    assigned = true;
                }
            }
            
            if !assigned {
                return Err(format!(
                    "Function '{}' ({} bytes) doesn't fit in any bank (bank size: {} bytes)",
                    func.name, func.size_bytes, bank_size
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
        
        // Calculate per-bank statistics
        let mut banks: Vec<BankInfo> = (0..total_banks)
            .map(|i| BankInfo::new(i as u8))
            .collect();
        
        for (func_name, bank_id) in assignments {
            if let Some(node) = self.graph.nodes.get(func_name) {
                banks[*bank_id as usize].add_function(func_name.clone(), node.size_bytes);
            }
        }
        
        let used_banks = banks.iter()
            .filter(|b| !b.functions.is_empty())
            .count();
        
        let total_used_bytes: usize = banks.iter()
            .map(|b| b.used_bytes)
            .sum();
        
        let total_available_bytes = bank_size * total_banks as usize;
        let utilization = (total_used_bytes as f64 / total_available_bytes as f64) * 100.0;
        
        BankStats {
            total_banks: total_banks as usize,
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
        eprintln!("   [Bank Assignment] Statistics:");
        eprintln!("     - Total banks: {}", self.total_banks);
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
    }
}
