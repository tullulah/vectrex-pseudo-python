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
fn test_jmp_direct_0x0e() {
    let (mut emulator, memory) = setup_emulator();
    
    // Setup inicial
    emulator.cpu_mut().set_register_a(0x42);
    
    // JMP $C900 - opcode 0x0E, address 0xC900
    memory.write(RAM_START + 0x100, 0x0E).unwrap(); // JMP direct
    memory.write(RAM_START + 0x101, 0xC9).unwrap(); // High byte
    memory.write(RAM_START + 0x102, 0x00).unwrap(); // Low byte
    
    // Poner instrucción de destino para verificar salto
    memory.write(0xC900, 0x4C).unwrap(); // INCA en destino
    
    emulator.cpu_mut().set_program_counter(RAM_START + 0x100);
    emulator.step().unwrap(); // Ejecutar JMP
    
    // Verificar - PC debe saltar a 0xC900
    assert_eq!(emulator.cpu().program_counter(), 0xC900);
    assert_eq!(emulator.cpu().register_a(), 0x42); // No debe cambiar registros
}

#[test]
fn test_jmp_direct_to_same_page_0x0e() {
    let (mut emulator, memory) = setup_emulator();
    
    // JMP dentro del mismo bloque RAM
    memory.write(RAM_START + 0x100, 0x0E).unwrap(); // JMP direct
    memory.write(RAM_START + 0x101, 0xC8).unwrap(); // High byte
    memory.write(RAM_START + 0x102, 0x50).unwrap(); // Low byte = RAM_START + 0x50
    
    emulator.cpu_mut().set_program_counter(RAM_START + 0x100);
    emulator.step().unwrap();
    
    // Verificar salto a RAM_START + 0x50
    assert_eq!(emulator.cpu().program_counter(), RAM_START + 0x50);
}

#[test]
fn test_jmp_direct_forward_0x0e() {
    let (mut emulator, memory) = setup_emulator();
    
    // JMP hacia adelante
    memory.write(RAM_START + 0x100, 0x0E).unwrap(); // JMP direct
    memory.write(RAM_START + 0x101, 0xCA).unwrap(); // High byte
    memory.write(RAM_START + 0x102, 0x00).unwrap(); // Low byte = 0xCA00
    
    emulator.cpu_mut().set_program_counter(RAM_START + 0x100);
    emulator.step().unwrap();
    
    // Verificar salto a 0xCA00
    assert_eq!(emulator.cpu().program_counter(), 0xCA00);
}

#[test]
fn test_jmp_direct_timing_0x0e() {
    let (mut emulator, memory) = setup_emulator();
    
    // JMP direct debe tomar 3 ciclos
    memory.write(RAM_START + 0x100, 0x0E).unwrap();
    memory.write(RAM_START + 0x101, 0xC9).unwrap();
    memory.write(RAM_START + 0x102, 0x00).unwrap();
    
    emulator.cpu_mut().set_program_counter(RAM_START + 0x100);
    let initial_cycles = emulator.cpu().cycles();
    
    emulator.step().unwrap();
    
    let cycles_used = emulator.cpu().cycles() - initial_cycles;
    assert_eq!(cycles_used, 3); // JMP direct debe usar 3 ciclos
}