//! Opcodes tests module
//! Comprehensive 1:1 tests for all 6809 CPU opcodes
//! Based on Vectrexy implementation

// LD (Load) family - COMPLETED
pub mod load;

// ST (Store) family - IN PROGRESS
pub mod store;
pub mod shift;

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

pub mod logic {
    //! Logic instruction tests (AND, EOR, OR, BIT, etc.)
    pub mod test_bit_opcodes;
    pub mod test_bit_simple;
}

pub mod memory {
    //! Memory operation tests (LD, ST, LEA, etc.)
    pub mod test_lea_opcodes;
}

pub mod branch {
    //! Branch and jump instruction tests
    pub mod test_branch_opcodes;
    pub mod test_jsr_bsr_opcodes;
    pub mod test_long_branch_final;
}

pub mod stack {
    //! Stack and interrupt instruction tests
    pub mod test_interrupt_stack_compliance;
    pub mod test_interrupt_stack_compliance_simple;
    pub mod test_stack_compliance_comprehensive;
    pub mod test_swi_rti_stack_compliance;
}