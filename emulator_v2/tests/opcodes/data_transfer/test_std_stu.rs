use crate::emulator::Emulator;
use crate::memory::devices::RamDevice;
use crate::memory::MemoryDevice;

// Configuración estándar de memoria para tests
const RAM_START: u16 = 0xC800;
const STACK_START: u16 = 0xCFFF;

fn setup_emulator() -> (Emulator, Box<dyn MemoryDevice>) {
    let mut emulator = Emulator::new();
    let memory = Box::new(RamDevice::new());
    emulator.memory().add_device(RAM_START, memory.clone()).unwrap();
    emulator.cpu_mut().set_stack_pointer(STACK_START);
    (emulator, memory)
}

#[test]
fn test_std_direct_0xdd() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - D = 0x1234
    emulator.cpu_mut().set_register_d(0x1234);
    
    // STD $50 - opcode 0xDD, direct page offset 0x50
    memory.write(RAM_START + 0x100, 0xDD).unwrap(); // STD direct
    memory.write(RAM_START + 0x101, 0x50).unwrap(); // Direct page offset
    
    // Configurar direct page para apuntar a RAM_START
    emulator.cpu_mut().set_direct_page((RAM_START >> 8) as u8);
    
    emulator.cpu_mut().set_program_counter(RAM_START + 0x100);
    emulator.step().unwrap();
    
    // Verificar que D se guardó correctamente en memoria
    let stored_high = memory.read(RAM_START + 0x50).unwrap();
    let stored_low = memory.read(RAM_START + 0x51).unwrap();
    let stored_value = ((stored_high as u16) << 8) | (stored_low as u16);
    
    assert_eq!(stored_value, 0x1234);
    assert_eq!(emulator.cpu().register_d(), 0x1234); // D no debe cambiar
    assert_eq!(emulator.cpu().program_counter(), RAM_START + 0x102);
}

#[test]
fn test_std_indexed_0xed() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - D = 0xABCD, X = RAM_START + 0x60
    emulator.cpu_mut().set_register_d(0xABCD);
    emulator.cpu_mut().set_register_x(RAM_START + 0x60);
    
    // STD ,X - opcode 0xED, postbyte 0x84 (indexed X, no offset)
    memory.write(RAM_START + 0x100, 0xED).unwrap(); // STD indexed
    memory.write(RAM_START + 0x101, 0x84).unwrap(); // ,X
    
    emulator.cpu_mut().set_program_counter(RAM_START + 0x100);
    emulator.step().unwrap();
    
    // Verificar que D se guardó en dirección apuntada por X
    let stored_high = memory.read(RAM_START + 0x60).unwrap();
    let stored_low = memory.read(RAM_START + 0x61).unwrap();
    let stored_value = ((stored_high as u16) << 8) | (stored_low as u16);
    
    assert_eq!(stored_value, 0xABCD);
    assert_eq!(emulator.cpu().register_d(), 0xABCD); // D no debe cambiar
    assert_eq!(emulator.cpu().register_x(), RAM_START + 0x60); // X no debe cambiar
}

#[test]
fn test_std_extended_0xfd() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - D = 0x5678
    emulator.cpu_mut().set_register_d(0x5678);
    
    // STD $C900 - opcode 0xFD, address 0xC900
    memory.write(RAM_START + 0x100, 0xFD).unwrap(); // STD extended
    memory.write(RAM_START + 0x101, 0xC9).unwrap(); // High byte
    memory.write(RAM_START + 0x102, 0x00).unwrap(); // Low byte
    
    emulator.cpu_mut().set_program_counter(RAM_START + 0x100);
    emulator.step().unwrap();
    
    // Verificar que D se guardó en la dirección extendida
    let stored_high = memory.read(0xC900).unwrap();
    let stored_low = memory.read(0xC901).unwrap();
    let stored_value = ((stored_high as u16) << 8) | (stored_low as u16);
    
    assert_eq!(stored_value, 0x5678);
    assert_eq!(emulator.cpu().register_d(), 0x5678); // D no debe cambiar
    assert_eq!(emulator.cpu().program_counter(), RAM_START + 0x103);
}

#[test]
fn test_stu_direct_0xdf() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - U = 0x9ABC
    emulator.cpu_mut().set_user_stack_pointer(0x9ABC);
    
    // STU $70 - opcode 0xDF, direct page offset 0x70
    memory.write(RAM_START + 0x100, 0xDF).unwrap(); // STU direct
    memory.write(RAM_START + 0x101, 0x70).unwrap(); // Direct page offset
    
    emulator.cpu_mut().set_direct_page((RAM_START >> 8) as u8);
    emulator.cpu_mut().set_program_counter(RAM_START + 0x100);
    emulator.step().unwrap();
    
    // Verificar que U se guardó correctamente
    let stored_high = memory.read(RAM_START + 0x70).unwrap();
    let stored_low = memory.read(RAM_START + 0x71).unwrap();
    let stored_value = ((stored_high as u16) << 8) | (stored_low as u16);
    
    assert_eq!(stored_value, 0x9ABC);
    assert_eq!(emulator.cpu().user_stack_pointer(), 0x9ABC); // U no debe cambiar
}

#[test]
fn test_stu_indexed_0xef() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - U = 0xDEF0, Y = RAM_START + 0x80
    emulator.cpu_mut().set_user_stack_pointer(0xDEF0);
    emulator.cpu_mut().set_register_y(RAM_START + 0x80);
    
    // STU ,Y - opcode 0xEF, postbyte 0xA4 (indexed Y, no offset)
    memory.write(RAM_START + 0x100, 0xEF).unwrap(); // STU indexed
    memory.write(RAM_START + 0x101, 0xA4).unwrap(); // ,Y
    
    emulator.cpu_mut().set_program_counter(RAM_START + 0x100);
    emulator.step().unwrap();
    
    // Verificar que U se guardó en dirección apuntada por Y
    let stored_high = memory.read(RAM_START + 0x80).unwrap();
    let stored_low = memory.read(RAM_START + 0x81).unwrap();
    let stored_value = ((stored_high as u16) << 8) | (stored_low as u16);
    
    assert_eq!(stored_value, 0xDEF0);
    assert_eq!(emulator.cpu().user_stack_pointer(), 0xDEF0); // U no debe cambiar
    assert_eq!(emulator.cpu().register_y(), RAM_START + 0x80); // Y no debe cambiar
}

#[test]
fn test_stu_extended_0xff() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - U = 0x1111
    emulator.cpu_mut().set_user_stack_pointer(0x1111);
    
    // STU $CA00 - opcode 0xFF, address 0xCA00
    memory.write(RAM_START + 0x100, 0xFF).unwrap(); // STU extended
    memory.write(RAM_START + 0x101, 0xCA).unwrap(); // High byte
    memory.write(RAM_START + 0x102, 0x00).unwrap(); // Low byte
    
    emulator.cpu_mut().set_program_counter(RAM_START + 0x100);
    emulator.step().unwrap();
    
    // Verificar que U se guardó en la dirección extendida
    let stored_high = memory.read(0xCA00).unwrap();
    let stored_low = memory.read(0xCA01).unwrap();
    let stored_value = ((stored_high as u16) << 8) | (stored_low as u16);
    
    assert_eq!(stored_value, 0x1111);
    assert_eq!(emulator.cpu().user_stack_pointer(), 0x1111); // U no debe cambiar
    assert_eq!(emulator.cpu().program_counter(), RAM_START + 0x103);
}

#[test]
fn test_std_condition_codes_0xdd() {
    let (mut emulator, memory) = setup_emulator();
    
    // STD debe afectar condition codes N y Z
    emulator.cpu_mut().set_register_d(0x8000); // Valor negativo
    
    memory.write(RAM_START + 0x100, 0xDD).unwrap();
    memory.write(RAM_START + 0x101, 0x90).unwrap();
    
    emulator.cpu_mut().set_direct_page((RAM_START >> 8) as u8);
    emulator.cpu_mut().set_program_counter(RAM_START + 0x100);
    emulator.step().unwrap();
    
    // Verificar flags - N=1 (negativo), Z=0 (no cero)
    assert_eq!(emulator.cpu().condition_codes().negative(), true);
    assert_eq!(emulator.cpu().condition_codes().zero(), false);
    
    // Test con valor cero
    emulator.cpu_mut().set_register_d(0x0000);
    emulator.cpu_mut().set_program_counter(RAM_START + 0x100);
    emulator.step().unwrap();
    
    // Verificar flags - N=0 (no negativo), Z=1 (cero)
    assert_eq!(emulator.cpu().condition_codes().negative(), false);
    assert_eq!(emulator.cpu().condition_codes().zero(), true);
}