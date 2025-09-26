#[cfg(test)]
mod tests {
    use crate::core::cpu6809::{NMI_VECTOR, IRQ_VECTOR, FIRQ_VECTOR, SWI_VECTOR};

    #[test]
    fn test_interrupt_vector_constants() {
        // Test that all interrupt vector constants are properly defined
        // C++ Original: These constants match InterruptVector enum in Vectrexy Cpu.cpp
        
        assert_eq!(NMI_VECTOR, 0xFFFC);  // C++ Original: Nmi = 0xFFFC
        assert_eq!(SWI_VECTOR, 0xFFFA);  // C++ Original: Swi = 0xFFFA
        assert_eq!(IRQ_VECTOR, 0xFFF8);  // C++ Original: Irq = 0xFFF8
        assert_eq!(FIRQ_VECTOR, 0xFFF6); // C++ Original: Firq = 0xFFF6
        
        // Test that vectors are in correct order (descending)
        assert!(NMI_VECTOR > SWI_VECTOR);
        assert!(SWI_VECTOR > IRQ_VECTOR);
        assert!(IRQ_VECTOR > FIRQ_VECTOR);
        
        // Test high memory range (all vectors are in 0xFF00+ range)
        assert!(NMI_VECTOR >= 0xFF00);
        assert!(SWI_VECTOR >= 0xFF00);
        assert!(IRQ_VECTOR >= 0xFF00);
        assert!(FIRQ_VECTOR >= 0xFF00);
    }
}