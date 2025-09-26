//! Opcodes tests module
//! Comprehensive 1:1 tests for all 6809 CPU opcodes
//! Based on Vectrexy implementation

// LD (Load) family - COMPLETED
pub mod load;

// ST (Store) family - IN PROGRESS
pub mod store;
pub mod shift;

// Branch instructions - NEW
pub mod branch;

// Register operations - NEW  
pub mod register;

// Organized test modules by category (TEMPORARY - will be segregated)
pub mod comparison {
    //! Comparison instruction tests (CMP family)
    pub mod test_cmpa;
    pub mod test_cmpb;
    pub mod test_cmpx;
    pub mod test_cmpd;
    pub mod test_cmps;
    pub mod test_cmpu;
    pub mod test_cmpy;
}

pub mod arithmetic {
    //! Arithmetic instruction tests (ADD, SUB, etc.)
    pub mod test_arith_logic_direct_a_final;
}

pub mod logic;

pub mod memory {
    //! Memory operation tests (LD, ST, LEA, etc.)
    pub mod test_lea_opcodes;
}

pub mod data_transfer;

pub mod system;

pub mod test_all_new_opcodes;

pub mod stack {
    //! Stack and interrupt instruction tests
    pub mod test_interrupt_stack_compliance;
    pub mod test_interrupt_stack_compliance_simple;
    pub mod test_stack_compliance_comprehensive;
    pub mod test_swi_rti_stack_compliance;
}