// Automatic RAM layout manager
// Ensures no collisions and compact memory usage

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RamVar {
    pub name: String,
    pub size: usize,  // Size in bytes
    pub comment: String,
}

pub struct RamLayout {
    base_address: u16,
    current_offset: usize,
    vars: Vec<RamVar>,
    offsets: HashMap<String, usize>,
    // Fixed-address variables (absolute address, not relative to base)
    fixed_vars: Vec<(String, u16, usize, String)>,
}

impl RamLayout {
    pub fn new(base_address: u16) -> Self {
        Self {
            base_address,
            current_offset: 0,
            vars: Vec::new(),
            offsets: HashMap::new(),
            fixed_vars: Vec::new(),
        }
    }
    
    /// Create RamLayout with first byte reserved (for multibank CURRENT_ROM_BANK)
    pub fn new_with_reserved_first_byte(base_address: u16) -> Self {
        Self {
            base_address,
            current_offset: 1, // Skip first byte
            vars: Vec::new(),
            offsets: HashMap::new(),
            fixed_vars: Vec::new(),
        }
    }
    
    /// Allocate a variable in RAM, returning its offset from base
    pub fn allocate(&mut self, name: impl Into<String>, size: usize, comment: impl Into<String>) -> usize {
        let name = name.into();
        let comment = comment.into();
        let offset = self.current_offset;
        
        self.vars.push(RamVar {
            name: name.clone(),
            size,
            comment,
        });
        self.offsets.insert(name, offset);
        self.current_offset += size;
        
        offset
    }
    
    /// Allocate a variable at a fixed absolute address (outside the compact region)
    /// Emits an EQU with the absolute address but does NOT reserve storage in ORG block.
    pub fn allocate_fixed(&mut self, name: impl Into<String>, address: u16, size: usize, comment: impl Into<String>) {
        let name = name.into();
        let comment = comment.into();
        self.fixed_vars.push((name, address, size, comment));
    }
    
    /// Get the offset of a variable (if already allocated)
    pub fn get_offset(&self, name: &str) -> Option<usize> {
        self.offsets.get(name).copied()
    }
    
    /// Get the absolute address of a variable
    pub fn get_address(&self, name: &str) -> Option<u16> {
        self.get_offset(name).map(|offset| self.base_address + offset as u16)
    }
    
    /// Total RAM used
    pub fn total_size(&self) -> usize {
        self.current_offset
    }
    
    /// Emit all EQU definitions in allocation order
    pub fn emit_equ_definitions(&self) -> String {
        let mut out = String::new();
        for var in &self.vars {
            let offset_hex = format!("${:02X}", self.offsets[&var.name]);
            out.push_str(&format!(
                "{:<20} EQU ${:04X}+{}   ; {} ({} bytes)\n",
                var.name,
                self.base_address,
                offset_hex,
                var.comment,
                var.size
            ));
        }
        // Emit fixed-address EQUs after compact region
        for (name, addr, size, comment) in &self.fixed_vars {
            // Don't append size if comment already contains it (avoid duplication)
            let formatted_comment = if comment.contains("byte") {
                comment.clone()
            } else {
                format!("{} ({} bytes)", comment, size)
            };
            out.push_str(&format!(
                "{:<20} EQU ${:04X}   ; {}\n",
                name,
                addr,
                formatted_comment
            ));
        }
        out
    }
    
    /// Emit all FDB/FCB storage allocations in allocation order
    pub fn emit_storage_allocations(&self) -> String {
        let mut out = String::new();
        for var in &self.vars {
            let directive = if var.size == 1 { "FCB" } else { "FDB" };
            out.push_str(&format!(
                "{}:    {} 0   ; {}\n",
                var.name,
                directive,
                var.comment
            ));
        }
        out
    }
    
    /// Get base address
    pub fn base_address(&self) -> u16 {
        self.base_address
    }
    
    /// Iterator over all variables (name, offset)
    pub fn iter_variables(&self) -> impl Iterator<Item = (&String, &usize)> {
        self.offsets.iter()
    }
    
    /// Iterator over fixed-address variables (name, address)
    pub fn iter_fixed_variables(&self) -> impl Iterator<Item = (&String, &u16)> + '_ {
        self.fixed_vars.iter().map(|(name, addr, _, _)| (name, addr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compact_allocation() {
        let mut layout = RamLayout::new(0xC880);
        
        // Allocate variables
        let result_offset = layout.allocate("RESULT", 2, "Main temporary");
        let tmpleft_offset = layout.allocate("TMPLEFT", 2, "Left operand");
        let tmpright_offset = layout.allocate("TMPRIGHT", 2, "Right operand");
        let flag_offset = layout.allocate("FLAG", 1, "Status flag");
        
        // Verify no gaps
        assert_eq!(result_offset, 0);
        assert_eq!(tmpleft_offset, 2);
        assert_eq!(tmpright_offset, 4);
        assert_eq!(flag_offset, 6);
        assert_eq!(layout.total_size(), 7);
        
        // Verify addresses
        assert_eq!(layout.get_address("RESULT"), Some(0xC880));
        assert_eq!(layout.get_address("TMPLEFT"), Some(0xC882));
        assert_eq!(layout.get_address("FLAG"), Some(0xC886));
    }
    
    #[test]
    fn test_no_collision() {
        let mut layout = RamLayout::new(0xC880);
        
        // Simulate PSG variables
        layout.allocate("PSG_MUSIC_PTR", 2, "Music pointer");
        layout.allocate("PSG_MUSIC_START", 2, "Music start");
        layout.allocate("PSG_IS_PLAYING", 1, "Playing flag");
        
        // Add NUM_STR - should not collide
        let num_str_offset = layout.allocate("NUM_STR", 2, "Number buffer");
        
        // NUM_STR should be at offset 5 (2+2+1)
        assert_eq!(num_str_offset, 5);
        assert_eq!(layout.get_address("NUM_STR"), Some(0xC885));
        
        // Verify no overlap
        assert_ne!(
            layout.get_address("PSG_MUSIC_START"),
            layout.get_address("NUM_STR")
        );
    }
}
