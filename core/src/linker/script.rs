// Linker Script Parser (.ld files)
//
// Parses linker scripts that define memory layout and section assignment rules.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Linker script configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkerScript {
    pub memory_regions: Vec<MemoryRegion>,
    pub section_rules: Vec<SectionRule>,
    pub entry_point: Option<String>,
    pub bank_register: Option<u16>,
}

/// Memory region definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRegion {
    pub name: String,
    pub start: u16,
    pub size: usize,
    pub region_type: RegionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegionType {
    Banked,    // Switchable bank
    Fixed,     // Always visible (Bank #31)
    Ram,       // RAM variables
}

/// Section assignment rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionRule {
    pub pattern: String,       // Glob pattern (e.g., ".text.main", ".text.level1*")
    pub region: String,        // Target memory region name
}

impl LinkerScript {
    /// Create default Vectrex linker script
    pub fn default_vectrex() -> Self {
        let mut regions = Vec::new();
        
        // Create 31 switchable banks (0-30)
        for i in 0..31 {
            regions.push(MemoryRegion {
                name: format!("BANK{}", i),
                start: 0x0000,
                size: 16384, // 16KB
                region_type: RegionType::Banked,
            });
        }
        
        // Fixed bank (Bank #31)
        regions.push(MemoryRegion {
            name: "BANK31".to_string(),
            start: 0x4000,
            size: 8192, // 8KB
            region_type: RegionType::Fixed,
        });
        
        // RAM
        regions.push(MemoryRegion {
            name: "RAM".to_string(),
            start: 0xC880,
            size: 896,
            region_type: RegionType::Ram,
        });
        
        // Default section rules
        let section_rules = vec![
            SectionRule {
                pattern: ".text.main".to_string(),
                region: "BANK31".to_string(),
            },
            SectionRule {
                pattern: ".text.loop".to_string(),
                region: "BANK31".to_string(),
            },
            SectionRule {
                pattern: ".text.*_WRAPPER".to_string(),
                region: "BANK31".to_string(),
            },
            SectionRule {
                pattern: ".rodata*".to_string(),
                region: "BANK31".to_string(),
            },
            SectionRule {
                pattern: ".bss*".to_string(),
                region: "RAM".to_string(),
            },
        ];
        
        Self {
            memory_regions: regions,
            section_rules,
            entry_point: Some("main".to_string()),
            bank_register: Some(0x4000),
        }
    }
}
