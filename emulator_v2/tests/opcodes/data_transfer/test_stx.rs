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
fn test_stx_direct_0x9f() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - X = 0x1234
    emulator.cpu_mut().set_register_x(0x1234);
    
    // STX $C850 - opcode 0x9F, address 0xC850
    memory.write(RAM_START + 0x100, 0x9F).unwrap(); // STX direct
    memory.write(RAM_START + 0x101, 0x50).unwrap(); // Direct page offset
    
    // Configurar direct page para apuntar a RAM_START
    emulator.cpu_mut().set_direct_page((RAM_START >> 8) as u8);
    
    emulator.cpu_mut().set_program_counter(RAM_START + 0x100);
    emulator.step().unwrap();
    
    // Verificar que X se guardó correctamente en memoria
    let stored_high = memory.read(RAM_START + 0x50).unwrap();
    let stored_low = memory.read(RAM_START + 0x51).unwrap();
    let stored_value = ((stored_high as u16) << 8) | (stored_low as u16);
    
    assert_eq!(stored_value, 0x1234);
    assert_eq!(emulator.cpu().register_x(), 0x1234); // X no debe cambiar
    assert_eq!(emulator.cpu().program_counter(), RAM_START + 0x102);
}

#[test]
fn test_stx_indexed_0xaf() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - X = 0xABCD, Y = 0xC850
    emulator.cpu_mut().set_register_x(0xABCD);
    emulator.cpu_mut().set_register_y(RAM_START + 0x50);
    
    // STX ,Y - opcode 0xAF, postbyte 0xA4 (indexed Y, no offset)
    memory.write(RAM_START + 0x100, 0xAF).unwrap(); // STX indexed
    memory.write(RAM_START + 0x101, 0xA4).unwrap(); // ,Y
    
    emulator.cpu_mut().set_program_counter(RAM_START + 0x100);
    emulator.step().unwrap();
    
    // Verificar que X se guardó en dirección apuntada por Y
    let stored_high = memory.read(RAM_START + 0x50).unwrap();
    let stored_low = memory.read(RAM_START + 0x51).unwrap();
    let stored_value = ((stored_high as u16) << 8) | (stored_low as u16);
    
    assert_eq!(stored_value, 0xABCD);
    assert_eq!(emulator.cpu().register_x(), 0xABCD); // X no debe cambiar
    assert_eq!(emulator.cpu().register_y(), RAM_START + 0x50); // Y no debe cambiar
}

#[test]
fn test_stx_extended_0xbf() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - X = 0x5678
    emulator.cpu_mut().set_register_x(0x5678);
    
    // STX $C900 - opcode 0xBF, address 0xC900
    memory.write(RAM_START + 0x100, 0xBF).unwrap(); // STX extended
    memory.write(RAM_START + 0x101, 0xC9).unwrap(); // High byte
    memory.write(RAM_START + 0x102, 0x00).unwrap(); // Low byte
    
    emulator.cpu_mut().set_program_counter(RAM_START + 0x100);
    emulator.step().unwrap();
    
    // Verificar que X se guardó en la dirección extendida
    let stored_high = memory.read(0xC900).unwrap();
    let stored_low = memory.read(0xC901).unwrap();
    let stored_value = ((stored_high as u16) << 8) | (stored_low as u16);
    
    assert_eq!(stored_value, 0x5678);
    assert_eq!(emulator.cpu().register_x(), 0x5678); // X no debe cambiar
    assert_eq!(emulator.cpu().program_counter(), RAM_START + 0x103);
}

#[test]
fn test_stx_zero_value_0x9f() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial - X = 0x0000
    emulator.cpu_mut().set_register_x(0x0000);
    
    // STX con valor cero
    memory.write(RAM_START + 0x100, 0x9F).unwrap();
    memory.write(RAM_START + 0x101, 0x60).unwrap();
    
    emulator.cpu_mut().set_direct_page((RAM_START >> 8) as u8);
    emulator.cpu_mut().set_program_counter(RAM_START + 0x100);
    emulator.step().unwrap();
    
    // Verificar que se guardó cero correctamente
    let stored_high = memory.read(RAM_START + 0x60).unwrap();
    let stored_low = memory.read(RAM_START + 0x61).unwrap();
    
    assert_eq!(stored_high, 0x00);
    assert_eq!(stored_low, 0x00);
}

#[test]
fn test_stx_condition_codes_0x9f() {
    let (mut emulator, memory) = setup_emulator();
    
    // STX debe afectar condition codes N y Z
    emulator.cpu_mut().set_register_x(0x8000); // Valor negativo
    
    memory.write(RAM_START + 0x100, 0x9F).unwrap();
    memory.write(RAM_START + 0x101, 0x70).unwrap();
    
    emulator.cpu_mut().set_direct_page((RAM_START >> 8) as u8);
    emulator.cpu_mut().set_program_counter(RAM_START + 0x100);
    emulator.step().unwrap();
    
    // Verificar flags - N=1 (negativo), Z=0 (no cero)
    assert_eq!(emulator.cpu().condition_codes().negative(), true);
    assert_eq!(emulator.cpu().condition_codes().zero(), false);
    
    // Test con valor cero
    emulator.cpu_mut().set_register_x(0x0000);
    emulator.cpu_mut().set_program_counter(RAM_START + 0x100);
    emulator.step().unwrap();
    
    // Verificar flags - N=0 (no negativo), Z=1 (cero)
    assert_eq!(emulator.cpu().condition_codes().negative(), false);
    assert_eq!(emulator.cpu().condition_codes().zero(), true);
}