#[cfg(test)]
mod tests {
    use crate::core::via6522::{IF_CA1, IF_CA2, IF_CB1, IF_CB2, IF_SHIFT, IF_TIMER1, IF_TIMER2, IF_IRQ_ENABLED,
                               IE_CA1, IE_CA2, IE_CB1, IE_CB2, IE_SHIFT, IE_TIMER1, IE_TIMER2, IE_SET_CLEAR_CONTROL};

    #[test]
    fn test_via_interrupt_flag_constants() {
        // Test that all VIA interrupt flag constants are properly defined
        // C++ Original: These constants match the bit positions in Vectrexy Via.cpp
        
        assert_eq!(IF_CA2, 0x01);   // BITS(0)
        assert_eq!(IF_CA1, 0x02);   // BITS(1) 
        assert_eq!(IF_SHIFT, 0x04); // BITS(2)
        assert_eq!(IF_CB2, 0x08);   // BITS(3)
        assert_eq!(IF_CB1, 0x10);   // BITS(4)
        assert_eq!(IF_TIMER2, 0x20); // BITS(5)
        assert_eq!(IF_TIMER1, 0x40); // BITS(6)
        assert_eq!(IF_IRQ_ENABLED, 0x80); // BITS(7)
        
        // Test that constants can be combined (typical usage pattern)
        let combined_flags = IF_TIMER1 | IF_TIMER2;
        assert_eq!(combined_flags, 0x60);
        
        // Test individual bit testing
        assert!((combined_flags & IF_TIMER1) != 0);
        assert!((combined_flags & IF_TIMER2) != 0);
        assert!((combined_flags & IF_CA1) == 0);
    }

    #[test] 
    fn test_via_interrupt_enable_constants() {
        // Test that all VIA interrupt enable constants are properly defined
        // C++ Original: These constants match the bit positions in Vectrexy Via.cpp
        
        assert_eq!(IE_CA2, 0x01);   // BITS(0)
        assert_eq!(IE_CA1, 0x02);   // BITS(1)
        assert_eq!(IE_SHIFT, 0x04); // BITS(2)
        assert_eq!(IE_CB2, 0x08);   // BITS(3)
        assert_eq!(IE_CB1, 0x10);   // BITS(4)
        assert_eq!(IE_TIMER2, 0x20); // BITS(5)
        assert_eq!(IE_TIMER1, 0x40); // BITS(6)
        assert_eq!(IE_SET_CLEAR_CONTROL, 0x80); // BITS(7)
        
        // Test that constants can be combined (typical usage pattern)
        let combined_enables = IE_TIMER1 | IE_TIMER2;
        assert_eq!(combined_enables, 0x60);
        
        // Test individual bit testing
        assert!((combined_enables & IE_TIMER1) != 0);
        assert!((combined_enables & IE_TIMER2) != 0);
        assert!((combined_enables & IE_CA1) == 0);
    }

    #[test]
    fn test_via_bit_manipulation() {
        // Test that VIA constants work correctly for bit manipulation
        // This simulates typical VIA register operations
        
        let mut ifr_register: u8 = 0;
        
        // Set some interrupt flags
        ifr_register |= IF_TIMER1;
        ifr_register |= IF_TIMER2;
        
        // Verify flags are set
        assert!((ifr_register & IF_TIMER1) != 0);
        assert!((ifr_register & IF_TIMER2) != 0);
        assert!((ifr_register & IF_CA1) == 0);
        
        // Clear specific flag
        ifr_register &= !IF_TIMER1;
        assert!((ifr_register & IF_TIMER1) == 0);
        assert!((ifr_register & IF_TIMER2) != 0);
        
        // Test IER operations
        let mut ier_register: u8 = 0;
        ier_register |= IE_SET_CLEAR_CONTROL; // Set control bit
        ier_register |= IE_TIMER1;
        ier_register |= IE_TIMER2;
        
        assert!((ier_register & IE_SET_CLEAR_CONTROL) != 0);
        assert!((ier_register & IE_TIMER1) != 0);
        assert!((ier_register & IE_TIMER2) != 0);
    }
}