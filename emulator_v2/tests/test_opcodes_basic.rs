// Test integrado simple para BRA usando CPU directamente
// Verificando implementación de emulator_v2

#[cfg(test)]
mod tests {
    use vectrex_emulator_v2::core::cpu6809::Cpu6809;
    use vectrex_emulator_v2::core::memory_bus::MemoryBus;
    use vectrex_emulator_v2::core::ram::Ram;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn create_test_cpu() -> Cpu6809 {
        let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
        let ram = Rc::new(RefCell::new(Ram::new()));
        Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
        Cpu6809::new(memory_bus)
    }

    #[test]
    fn test_bra_opcode_works() {
        let mut cpu = create_test_cpu();
        
        // Setup inicial - PC en área de RAM
        cpu.registers.pc = 0xC800;
        cpu.registers.s = 0xCFFF;
        
        // Escribir BRA +5 en memoria
        cpu.memory_bus().borrow_mut().write(0xC800, 0x20); // BRA opcode
        cpu.memory_bus().borrow_mut().write(0xC801, 0x05); // offset +5
        
        // Ejecutar instrucción BRA
        let cycles = cpu.execute_instruction(false, false);
        
        // Verificar que PC saltó correctamente
        // PC esperado = PC original + 2 (tamaño BRA) + 5 (offset) = 0xC800 + 2 + 5 = 0xC807
        assert_eq!(cpu.registers.pc, 0xC807, "BRA should jump to PC + 2 + offset");
        assert_eq!(cycles, 3, "BRA should take 3 cycles");
        
        println!("✅ Test BRA básico funciona: PC={:04X}, cycles={}", cpu.registers.pc, cycles);
    }

    #[test]
    fn test_inca_opcode_works() {
        let mut cpu = create_test_cpu();
        
        // Setup inicial
        cpu.registers.pc = 0xC800;
        cpu.registers.a = 0x42;
        
        // Escribir INCA en memoria  
        cpu.memory_bus().borrow_mut().write(0xC800, 0x4C); // INCA opcode
        
        // Ejecutar instrucción INCA
        let cycles = cpu.execute_instruction(false, false);
        
        // Verificar que A se incrementó
        assert_eq!(cpu.registers.a, 0x43, "INCA should increment A register");
        assert_eq!(cpu.registers.pc, 0xC801, "PC should advance by 1");
        assert_eq!(cycles, 2, "INCA should take 2 cycles");
        
        println!("✅ Test INCA básico funciona: A={:02X}, cycles={}", cpu.registers.a, cycles);
    }

    #[test]
    fn test_inca_zero_flag() {
        let mut cpu = create_test_cpu();
        
        // Setup inicial - A=0xFF para que se convierta en 0x00
        cpu.registers.pc = 0xC800;
        cpu.registers.a = 0xFF;
        
        cpu.memory_bus().borrow_mut().write(0xC800, 0x4C); // INCA
        let cycles = cpu.execute_instruction(false, false);
        
        // Verificar overflow a cero y flag
        assert_eq!(cpu.registers.a, 0x00, "INCA 0xFF should wrap to 0x00");
        assert!(cpu.registers.cc.z, "Zero flag should be set");
        assert!(!cpu.registers.cc.n, "Negative flag should be clear");
        assert_eq!(cycles, 2);
        
        println!("✅ Test INCA zero flag funciona: A={:02X}, Z={}", cpu.registers.a, cpu.registers.cc.z);
    }

    #[test]
    fn test_inca_overflow_flag() {
        let mut cpu = create_test_cpu();
        
        // Setup inicial - A=0x7F (máximo positivo)
        cpu.registers.pc = 0xC800;
        cpu.registers.a = 0x7F;
        
        cpu.memory_bus().borrow_mut().write(0xC800, 0x4C); // INCA
        let cycles = cpu.execute_instruction(false, false);
        
        // Verificar overflow de positivo a negativo
        assert_eq!(cpu.registers.a, 0x80, "INCA 0x7F should become 0x80");
        assert!(cpu.registers.cc.v, "Overflow flag should be set");  
        assert!(cpu.registers.cc.n, "Negative flag should be set");
        assert!(!cpu.registers.cc.z, "Zero flag should be clear");
        assert_eq!(cycles, 2);
        
        println!("✅ Test INCA overflow funciona: A={:02X}, V={}, N={}", 
                cpu.registers.a, cpu.registers.cc.v, cpu.registers.cc.n);
    }

    #[test]
    fn test_bra_backward_jump() {
        let mut cpu = create_test_cpu();
        
        // Setup inicial - PC más adelante para poder saltar hacia atrás
        cpu.registers.pc = 0xC900;
        
        // Escribir BRA -50 (0xCE en complemento a 2)
        cpu.memory_bus().borrow_mut().write(0xC900, 0x20); // BRA opcode
        cpu.memory_bus().borrow_mut().write(0xC901, 0xCE); // offset -50
        
        // Ejecutar instrucción BRA
        let cycles = cpu.execute_instruction(false, false);
        
        // Verificar salto hacia atrás: PC + 2 - 50 = 0xC902 - 50 = 0xC8D0
        let expected_pc = 0xC900 + 2 - 50;
        assert_eq!(cpu.registers.pc, expected_pc, "BRA should jump backward");
        assert_eq!(cycles, 3, "BRA should take 3 cycles");
        
        println!("✅ Test BRA backward funciona: PC={:04X}, cycles={}", cpu.registers.pc, cycles);
    }

    #[test]
    fn test_beq_taken() {
        let mut cpu = create_test_cpu();
        
        // Setup inicial - configurar zero flag activo
        cpu.registers.pc = 0xC800;
        cpu.registers.cc.z = true; // Condición para que BEQ salte
        
        // Escribir BEQ +10
        cpu.memory_bus().borrow_mut().write(0xC800, 0x27); // BEQ opcode
        cpu.memory_bus().borrow_mut().write(0xC801, 0x0A); // offset +10
        
        // Ejecutar instrucción BEQ
        let cycles = cpu.execute_instruction(false, false);
        
        // Verificar que saltó: PC + 2 + 10 = 0xC80C
        let expected_pc = 0xC800 + 2 + 10;
        assert_eq!(cpu.registers.pc, expected_pc, "BEQ should branch when zero flag is set");
        assert_eq!(cycles, 3, "BEQ taken should take 3 cycles");
        
        println!("✅ Test BEQ taken funciona: PC={:04X}, cycles={}", cpu.registers.pc, cycles);
    }

    #[test]
    fn test_beq_not_taken() {
        let mut cpu = create_test_cpu();
        
        // Setup inicial - zero flag inactivo
        cpu.registers.pc = 0xC800;
        cpu.registers.cc.z = false; // Condición para que BEQ NO salte
        
        // Escribir BEQ +10
        cpu.memory_bus().borrow_mut().write(0xC800, 0x27); // BEQ opcode
        cpu.memory_bus().borrow_mut().write(0xC801, 0x0A); // offset +10
        
        // Ejecutar instrucción BEQ
        let cycles = cpu.execute_instruction(false, false);
        
        // Verificar que NO saltó: PC + 2 = 0xC802
        let expected_pc = 0xC800 + 2;
        assert_eq!(cpu.registers.pc, expected_pc, "BEQ should not branch when zero flag is clear");
        // Nota: BEQ always takes 3 cycles in this implementation (consistent with branch opcodes)
        assert_eq!(cycles, 3, "BEQ should take 3 cycles");
        
        println!("✅ Test BEQ not taken funciona: PC={:04X}, cycles={}", cpu.registers.pc, cycles);
    }

    #[test]
    fn test_lda_immediate() {
        let mut cpu = create_test_cpu();
        
        // Setup inicial
        cpu.registers.pc = 0xC800;
        cpu.registers.a = 0x00; // Valor inicial
        
        // Escribir LDA #$42 (immediate mode)
        cpu.memory_bus().borrow_mut().write(0xC800, 0x86); // LDA immediate opcode
        cpu.memory_bus().borrow_mut().write(0xC801, 0x42); // value to load
        
        // Ejecutar instrucción LDA
        let cycles = cpu.execute_instruction(false, false);
        
        // Verificar que se cargó el valor
        assert_eq!(cpu.registers.a, 0x42, "LDA should load immediate value");
        assert_eq!(cpu.registers.pc, 0xC802, "PC should advance by 2");
        assert_eq!(cycles, 2, "LDA immediate should take 2 cycles");
        assert!(!cpu.registers.cc.z, "Zero flag should be clear");
        assert!(!cpu.registers.cc.n, "Negative flag should be clear");
        
        println!("✅ Test LDA immediate funciona: A={:02X}, cycles={}", cpu.registers.a, cycles);
    }

    #[test]
    fn test_nop_opcode() {
        let mut cpu = create_test_cpu();
        
        // Setup inicial - guardar estado para verificar que no cambia
        cpu.registers.pc = 0xC800;
        cpu.registers.a = 0x55;
        cpu.registers.b = 0xAA;
        let initial_flags = cpu.registers.cc.clone();
        
        // Escribir NOP
        cpu.memory_bus().borrow_mut().write(0xC800, 0x12); // NOP opcode
        
        // Ejecutar instrucción NOP
        let cycles = cpu.execute_instruction(false, false);
        
        // Verificar que solo PC cambió
        assert_eq!(cpu.registers.pc, 0xC801, "PC should advance by 1");
        assert_eq!(cpu.registers.a, 0x55, "Register A should be unchanged");
        assert_eq!(cpu.registers.b, 0xAA, "Register B should be unchanged");
        assert_eq!(cpu.registers.cc.z, initial_flags.z, "Flags should be unchanged");
        assert_eq!(cycles, 2, "NOP should take 2 cycles");
        
        println!("✅ Test NOP funciona: PC={:04X}, cycles={}", cpu.registers.pc, cycles);
    }
}