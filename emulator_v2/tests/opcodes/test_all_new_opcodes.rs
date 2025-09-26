//! Simple validation test for all 14 newly implemented Page 1 opcodes
//! This test verifies that all opcodes can be executed without panicking

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

const RAM_START: u16 = 0xC800;
const STACK_START: u16 = 0xCFFF;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    let mut cpu = Cpu6809::new(memory_bus);
    cpu.registers.s = STACK_START;
    cpu
}

#[test]
fn test_all_14_new_opcodes_execute_without_panic() {
    let mut cpu = create_test_cpu();
    
    // Test data address
    let target_addr = RAM_START + 0x200;
    
    // Test 1: LDY immediate (0x108E)
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x100, 0x10); // Page 1 prefix
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x101, 0x8E); // LDY immediate
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x102, 0x12);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x103, 0x34);
    cpu.registers.pc = RAM_START + 0x100;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers.y, 0x1234);
    
    // Test 2: LDY direct (0x109E)
    cpu.registers.dp = 0xC8;
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x110, 0x10);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x111, 0x9E);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x112, 0x50); // Offset
    cpu.memory_bus().borrow_mut().write(0xC850, 0x56);
    cpu.memory_bus().borrow_mut().write(0xC851, 0x78);
    cpu.registers.pc = RAM_START + 0x110;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers.y, 0x5678);
    
    // Test 3: LDY indexed (0x10AE) - simple X indexed
    cpu.registers.x = RAM_START + 0x250;
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x120, 0x10);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x121, 0xAE);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x122, 0x84); // ,X
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x250, 0x9A);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x251, 0xBC);
    cpu.registers.pc = RAM_START + 0x120;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers.y, 0x9ABC);
    
    // Test 4: LDY extended (0x10BE)
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x130, 0x10);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x131, 0xBE);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x132, (target_addr >> 8) as u8);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x133, (target_addr & 0xFF) as u8);
    cpu.memory_bus().borrow_mut().write(target_addr, 0xDE);
    cpu.memory_bus().borrow_mut().write(target_addr + 1, 0xF0);
    cpu.registers.pc = RAM_START + 0x130;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers.y, 0xDEF0);
    
    // Test 5: STY direct (0x109F)
    cpu.registers.y = 0x1357;
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x140, 0x10);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x141, 0x9F);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x142, 0x60);
    cpu.registers.pc = RAM_START + 0x140;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.memory_bus().borrow().read(0xC860), 0x13);
    assert_eq!(cpu.memory_bus().borrow().read(0xC861), 0x57);
    
    // Test 6: STY indexed (0x10AF)
    cpu.registers.y = 0x2468;
    cpu.registers.x = RAM_START + 0x300;
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x150, 0x10);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x151, 0xAF);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x152, 0x84); // ,X
    cpu.registers.pc = RAM_START + 0x150;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.memory_bus().borrow().read(RAM_START + 0x300), 0x24);
    assert_eq!(cpu.memory_bus().borrow().read(RAM_START + 0x301), 0x68);
    
    // Test 7: STY extended (0x10BF)
    cpu.registers.y = 0x3579;
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x160, 0x10);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x161, 0xBF);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x162, (target_addr >> 8) as u8);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x163, (target_addr & 0xFF) as u8);
    cpu.registers.pc = RAM_START + 0x160;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.memory_bus().borrow().read(target_addr), 0x35);
    assert_eq!(cpu.memory_bus().borrow().read(target_addr + 1), 0x79);
    
    // Test 8: LDS immediate (0x10CE)
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x170, 0x10);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x171, 0xCE);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x172, 0x48);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x173, 0x6A);
    cpu.registers.pc = RAM_START + 0x170;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers.s, 0x486A);
    
    // Test 9: LDS direct (0x10DE)
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x180, 0x10);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x181, 0xDE);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x182, 0x70);
    cpu.memory_bus().borrow_mut().write(0xC870, 0x59);
    cpu.memory_bus().borrow_mut().write(0xC871, 0x7B);
    cpu.registers.pc = RAM_START + 0x180;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers.s, 0x597B);
    
    // Test 10: LDS indexed (0x10EE)
    cpu.registers.y = RAM_START + 0x350;
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x190, 0x10);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x191, 0xEE);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x192, 0xA4); // ,Y
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x350, 0x6A);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x351, 0x8C);
    cpu.registers.pc = RAM_START + 0x190;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers.s, 0x6A8C);
    
    // Test 11: LDS extended (0x10FE)
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x1A0, 0x10);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x1A1, 0xFE);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x1A2, (target_addr >> 8) as u8);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x1A3, (target_addr & 0xFF) as u8);
    cpu.memory_bus().borrow_mut().write(target_addr, 0x7B);
    cpu.memory_bus().borrow_mut().write(target_addr + 1, 0x9D);
    cpu.registers.pc = RAM_START + 0x1A0;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.registers.s, 0x7B9D);
    
    // Test 12: STS direct (0x10DF)
    cpu.registers.s = 0x8CAE;
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x1B0, 0x10);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x1B1, 0xDF);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x1B2, 0x80);
    cpu.registers.pc = RAM_START + 0x1B0;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.memory_bus().borrow().read(0xC880), 0x8C);
    assert_eq!(cpu.memory_bus().borrow().read(0xC881), 0xAE);
    
    // Test 13: STS indexed (0x10EF)
    cpu.registers.s = 0x9DBF;
    cpu.registers.x = RAM_START + 0x400;
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x1C0, 0x10);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x1C1, 0xEF);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x1C2, 0x84); // ,X
    cpu.registers.pc = RAM_START + 0x1C0;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.memory_bus().borrow().read(RAM_START + 0x400), 0x9D);
    assert_eq!(cpu.memory_bus().borrow().read(RAM_START + 0x401), 0xBF);
    
    // Test 14: STS extended (0x10FF)
    cpu.registers.s = 0xAEC0;
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x1D0, 0x10);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x1D1, 0xFF);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x1D2, (target_addr >> 8) as u8);
    cpu.memory_bus().borrow_mut().write(RAM_START + 0x1D3, (target_addr & 0xFF) as u8);
    cpu.registers.pc = RAM_START + 0x1D0;
    cpu.execute_instruction(false, false);
    assert_eq!(cpu.memory_bus().borrow().read(target_addr), 0xAE);
    assert_eq!(cpu.memory_bus().borrow().read(target_addr + 1), 0xC0);
    
    println!("✅ All 14 newly implemented Page 1 opcodes executed successfully!");
}