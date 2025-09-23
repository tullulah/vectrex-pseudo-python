//! MC6809 CPU implementation
//! Port of vectrexy/libs/emulator/include/emulator/Cpu.h and src/Cpu.cpp

use crate::types::Cycles;
use crate::core::memory_bus::MemoryBus;
use std::cell::RefCell;
use std::rc::Rc;

// C++ Original: enum class AddressingMode
#[derive(Debug, Clone, Copy)]
pub enum AddressingMode {
    Inherent,
    Immediate,
    Direct,
    Indexed,
    Extended,
    Relative,
}

// C++ Original: using InterruptType = std::function<void()>;
pub type InterruptCallback = Box<dyn Fn()>;

/* C++ Original from Cpu.h:
struct CpuRegisters {
    union {
        struct {
            uint8_t A;
            uint8_t B;
        };
        uint16_t D;
    };

    uint16_t X = 0;
    uint16_t Y = 0;
    uint16_t U = 0;  // User stack pointer
    uint16_t S = 0;  // System stack pointer
    uint16_t PC = 0; // Program counter
    uint8_t DP = 0;  // Direct page register

    union ConditionCode {
        struct {
            uint8_t C : 1; // Carry
            uint8_t V : 1; // Overflow
            uint8_t Z : 1; // Zero
            uint8_t N : 1; // Negative
            uint8_t I : 1; // IRQ Mask
            uint8_t H : 1; // Half-carry
            uint8_t F : 1; // FIRQ Mask
            uint8_t E : 1; // Entire flag
        };
        uint8_t All;
    } CC;
};
*/

#[derive(Debug, Clone, Copy)]
pub struct ConditionCode {
    pub c: bool, // Carry
    pub v: bool, // Overflow  
    pub z: bool, // Zero
    pub n: bool, // Negative
    pub i: bool, // IRQ Mask
    pub h: bool, // Half-carry
    pub f: bool, // FIRQ Mask
    pub e: bool, // Entire flag
}

impl ConditionCode {
    pub fn new() -> Self {
        Self {
            c: false, v: false, z: false, n: false,
            i: false, h: false, f: false, e: false,
        }
    }

    // C++ Original: uint8_t All; getter/setter
    pub fn to_u8(&self) -> u8 {
        (self.c as u8) |
        ((self.v as u8) << 1) |
        ((self.z as u8) << 2) |
        ((self.n as u8) << 3) |
        ((self.i as u8) << 4) |
        ((self.h as u8) << 5) |
        ((self.f as u8) << 6) |
        ((self.e as u8) << 7)
    }

    pub fn from_u8(&mut self, value: u8) {
        self.c = (value & 0x01) != 0;
        self.v = (value & 0x02) != 0;
        self.z = (value & 0x04) != 0;
        self.n = (value & 0x08) != 0;
        self.i = (value & 0x10) != 0;
        self.h = (value & 0x20) != 0;
        self.f = (value & 0x40) != 0;
        self.e = (value & 0x80) != 0;
    }
}

#[derive(Debug)]
pub struct CpuRegisters {
    // C++ Original: union { struct { uint8_t A; uint8_t B; }; uint16_t D; };
    pub a: u8,
    pub b: u8,
    
    pub x: u16,
    pub y: u16,
    pub u: u16,  // User stack pointer
    pub s: u16,  // System stack pointer
    pub pc: u16, // Program counter
    pub dp: u8,  // Direct page register
    pub cc: ConditionCode,
}

impl CpuRegisters {
    pub fn new() -> Self {
        Self {
            a: 0, b: 0, x: 0, y: 0, u: 0, s: 0, pc: 0, dp: 0,
            cc: ConditionCode::new(),
        }
    }

    // C++ Original: uint16_t D; getter/setter for union
    pub fn d(&self) -> u16 {
        ((self.a as u16) << 8) | (self.b as u16)
    }

    pub fn set_d(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.b = value as u8;
    }
}

/* C++ Original from Cpu.cpp:
class CpuImpl : private CpuRegisters {
private:
    std::shared_ptr<MemoryBus> m_memoryBus;
    
    // Interrupt vectors
    static constexpr uint16_t RESET_VECTOR = 0xFFFE;
    static constexpr uint16_t NMI_VECTOR = 0xFFFC;
    static constexpr uint16_t SWI_VECTOR = 0xFFFA;
    static constexpr uint16_t IRQ_VECTOR = 0xFFF8;
    static constexpr uint16_t FIRQ_VECTOR = 0xFFF6;
    static constexpr uint16_t SWI2_VECTOR = 0xFFF4;
    static constexpr uint16_t SWI3_VECTOR = 0xFFF2;

    cycles_t m_cycles = 0;
    
    InterruptType m_nmiInterrupt;
    InterruptType m_irqInterrupt; 
    InterruptType m_firqInterrupt;
};
*/

pub struct Cpu6809 {
    pub registers: CpuRegisters,
    memory_bus: Rc<RefCell<MemoryBus>>,
    
    // C++ Original: Interrupt vectors as static constexpr
    cycles: Cycles,
    
    // C++ Original: InterruptType callbacks
    nmi_interrupt: Option<InterruptCallback>,
    irq_interrupt: Option<InterruptCallback>,
    firq_interrupt: Option<InterruptCallback>,
}

// C++ Original: Interrupt vector constants
const RESET_VECTOR: u16 = 0xFFFE;
const NMI_VECTOR: u16   = 0xFFFC;
const SWI_VECTOR: u16   = 0xFFFA;
const IRQ_VECTOR: u16   = 0xFFF8;
const FIRQ_VECTOR: u16  = 0xFFF6;
const SWI2_VECTOR: u16  = 0xFFF4;
const SWI3_VECTOR: u16  = 0xFFF2;

impl Cpu6809 {
    pub fn new(memory_bus: Rc<RefCell<MemoryBus>>) -> Self {
        Self {
            registers: CpuRegisters::new(),
            memory_bus,
            cycles: 0,
            nmi_interrupt: None,
            irq_interrupt: None,
            firq_interrupt: None,
        }
    }

    // Convenience methods for accessing registers
    pub fn registers(&self) -> &CpuRegisters {
        &self.registers
    }

    pub fn registers_mut(&mut self) -> &mut CpuRegisters {
        &mut self.registers
    }

    /* C++ Original:
    void Reset() {
        A = B = X = Y = U = S = DP = 0;
        CC.All = 0;
        PC = Read16(RESET_VECTOR);

        m_cycles = 0;

        AddCycles(1); // The reset itself takes 1 cycle
    }
    */
    pub fn reset(&mut self) {
        self.registers.a = 0;
        self.registers.b = 0;
        self.registers.x = 0;
        self.registers.y = 0;
        self.registers.u = 0;
        self.registers.s = 0;
        self.registers.dp = 0;
        self.registers.cc = ConditionCode::new();
        self.registers.pc = self.read16(RESET_VECTOR);

        self.cycles = 0;
        self.add_cycles(1); // The reset itself takes 1 cycle
    }

    /* C++ Original:
    void AddCycles(cycles_t cycles) {
        m_cycles += cycles;
        m_memoryBus->AddSyncCycles(cycles);
    }
    */
    pub fn add_cycles(&mut self, cycles: Cycles) {
        self.cycles += cycles;
        self.memory_bus.borrow().add_sync_cycles(cycles);
    }

    pub fn get_cycles(&self) -> Cycles {
        self.cycles
    }

    /* C++ Original:
    uint8_t Read8(uint16_t address) const {
        return m_memoryBus->Read(address);
    }
    */
    pub fn read8(&self, address: u16) -> u8 {
        self.memory_bus.borrow().read(address)
    }

    /* C++ Original:
    uint16_t Read16(uint16_t address) const {
        auto high = m_memoryBus->Read(address++);
        auto low = m_memoryBus->Read(address);
        return CombineToU16(high, low);
    }
    */
    pub fn read16(&self, address: u16) -> u16 {
        let high = self.memory_bus.borrow().read(address);
        let low = self.memory_bus.borrow().read(address.wrapping_add(1));
        self.combine_to_u16(high, low)
    }

    pub fn write8(&mut self, address: u16, value: u8) {
        self.memory_bus.borrow_mut().write(address, value);
    }

    /* C++ Original:
    uint8_t ReadPC8() { return Read8(PC++); }
    */
    pub fn read_pc8(&mut self) -> u8 {
        let value = self.read8(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        value
    }

    /* C++ Original:
    uint16_t ReadPC16() {
        uint16_t value = Read16(PC);
        PC += 2;
        return value;
    }
    */
    pub fn read_pc16(&mut self) -> u16 {
        let value = self.read16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);
        value
    }

    // C++ Original: CombineToU16 helper function
    fn combine_to_u16(&self, high: u8, low: u8) -> u16 {
        ((high as u16) << 8) | (low as u16)
    }

    // C++ Original: CombineToS16 helper function
    fn combine_to_s16(&self, high: u8, low: u8) -> i16 {
        self.combine_to_u16(high, low) as i16
    }

    // C++ Original: S16 helper function
    fn s16(&self, value: u8) -> i16 {
        value as i8 as i16
    }

    // C++ Original: U8 helper function  
    fn u8(&self, value: u16) -> u8 {
        value as u8
    }

    /* C++ Original:
    void Push8(uint16_t& stackPointer, uint8_t value) { 
        m_memoryBus->Write(--stackPointer, value); 
    }
    */
    fn push8(&mut self, stack_pointer: &mut u16, value: u8) {
        *stack_pointer = stack_pointer.wrapping_sub(1);
        self.write8(*stack_pointer, value);
    }

    /* C++ Original:
    uint8_t Pop8(uint16_t& stackPointer) {
        auto value = m_memoryBus->Read(stackPointer++);
        return value;
    }
    */
    fn pop8(&mut self, stack_pointer: &mut u16) -> u8 {
        let value = self.read8(*stack_pointer);
        *stack_pointer = stack_pointer.wrapping_add(1);
        value
    }

    /* C++ Original:
    void Push16(uint16_t& stackPointer, uint16_t value) {
        m_memoryBus->Write(--stackPointer, U8(value & 0xFF)); // Low
        m_memoryBus->Write(--stackPointer, U8(value >> 8));   // High
    }
    */
    fn push16(&mut self, stack_pointer: &mut u16, value: u16) {
        self.push8(stack_pointer, self.u8(value & 0xFF)); // Low
        self.push8(stack_pointer, self.u8(value >> 8));   // High
    }

    /* C++ Original:
    uint16_t Pop16(uint16_t& stackPointer) {
        auto high = m_memoryBus->Read(stackPointer++);
        auto low = m_memoryBus->Read(stackPointer++);
        return CombineToU16(high, low);
    }
    */
    fn pop16(&mut self, stack_pointer: &mut u16) -> u16 {
        let high = self.pop8(stack_pointer);
        let low = self.pop8(stack_pointer);
        self.combine_to_u16(high, low)
    }

    // Register interrupt callbacks
    pub fn register_nmi_interrupt(&mut self, callback: InterruptCallback) {
        self.nmi_interrupt = Some(callback);
    }

    pub fn register_irq_interrupt(&mut self, callback: InterruptCallback) {
        self.irq_interrupt = Some(callback);
    }

    pub fn register_firq_interrupt(&mut self, callback: InterruptCallback) {
        self.firq_interrupt = Some(callback);
    }

    /* C++ Original:
    uint16_t ReadDirectEA() {
        // EA = DP : (PC)
        uint16_t EA = CombineToU16(DP, ReadPC8());
        return EA;
    }
    */
    fn read_direct_ea(&mut self) -> u16 {
        // EA = DP : (PC)
        let dp = self.registers.dp;
        let pc_byte = self.read_pc8();
        let ea = self.combine_to_u16(dp, pc_byte);
        ea
    }

    /* C++ Original:
    uint16_t ReadExtendedEA() {
        // Contents of 2 bytes following opcode byte specify 16-bit effective address (always 3 byte
        // instruction) EA = (PC) : (PC + 1)
        auto msb = ReadPC8();
        auto lsb = ReadPC8();
        uint16_t EA = CombineToU16(msb, lsb);
        return EA;
    }
    */
    fn read_extended_ea(&mut self) -> u16 {
        // Contents of 2 bytes following opcode byte specify 16-bit effective address (always 3 byte instruction)
        // EA = (PC) : (PC + 1)
        let msb = self.read_pc8();
        let lsb = self.read_pc8();
        let ea = self.combine_to_u16(msb, lsb);
        ea
    }

    /* C++ Original:
    // Read CPU op's relative offset from next 8/16 bits
    int8_t ReadRelativeOffset8() { return static_cast<int8_t>(ReadPC8()); }
    int16_t ReadRelativeOffset16() { return static_cast<int16_t>(ReadPC16()); }
    */
    fn read_relative_offset8(&mut self) -> i8 {
        self.read_pc8() as i8
    }

    fn read_relative_offset16(&mut self) -> i16 {
        self.read_pc16() as i16
    }

    /* C++ Original:
    uint16_t ReadIndexedEA() {
        // In all indexed addressing one of the pointer registers (X, Y, U, S and sometimes PC) is
        // used in a calculation of the EA. The postbyte specifies type and variation of addressing
        // mode as well as pointer registers to be used.

        auto RegisterSelect = [this](uint8_t postbyte) -> uint16_t& {
            switch ((postbyte >> 5) & 0b11) {
            case 0b00: return X;
            case 0b01: return Y;
            case 0b10: return U;
            default: // 0b11: return S;
            }
        };
        // ... complex indexed addressing implementation
    }
    */
    fn read_indexed_ea(&mut self) -> u16 {
        // In all indexed addressing one of the pointer registers (X, Y, U, S and sometimes PC) is
        // used in a calculation of the EA. The postbyte specifies type and variation of addressing
        // mode as well as pointer registers to be used.
        
        let mut ea: u16;
        let postbyte = self.read_pc8();
        let mut supports_indirect = true;

        if (postbyte & 0x80) == 0 {
            // (+/- 4 bit offset),R
            // postbyte is a 5 bit two's complement number we convert to 8 bit.
            // So if bit 4 is set (sign bit), we extend the sign bit by turning on bits 6,7,8
            let mut offset = (postbyte & 0x1F) as i8;
            if (postbyte & 0x10) != 0 {
                offset |= 0xE0u8 as i8;
            }
            ea = self.register_select(postbyte).wrapping_add(offset as u16);
            supports_indirect = false;
            self.add_cycles(1);
        } else {
            match postbyte & 0x0F {
                0x00 => { // ,R+
                    let reg_ptr = self.register_select_mut(postbyte);
                    ea = *reg_ptr;
                    *reg_ptr = reg_ptr.wrapping_add(1);
                    supports_indirect = false;
                    self.add_cycles(2);
                }
                0x01 => { // ,R++
                    let reg_ptr = self.register_select_mut(postbyte);
                    ea = *reg_ptr;
                    *reg_ptr = reg_ptr.wrapping_add(2);
                    self.add_cycles(3);
                }
                0x02 => { // ,-R
                    let reg_ptr = self.register_select_mut(postbyte);
                    *reg_ptr = reg_ptr.wrapping_sub(1);
                    ea = *reg_ptr;
                    supports_indirect = false;
                    self.add_cycles(2);
                }
                0x03 => { // ,--R
                    let reg_ptr = self.register_select_mut(postbyte);
                    *reg_ptr = reg_ptr.wrapping_sub(2);
                    ea = *reg_ptr;
                    self.add_cycles(3);
                }
                0x04 => { // ,R
                    ea = self.register_select(postbyte);
                }
                0x05 => { // (+/- B),R
                    ea = self.register_select(postbyte).wrapping_add(self.s16(self.registers.b) as u16);
                    self.add_cycles(1);
                }
                0x06 => { // (+/- A),R
                    ea = self.register_select(postbyte).wrapping_add(self.s16(self.registers.a) as u16);
                    self.add_cycles(1);
                }
                0x07 => {
                    panic!("Illegal indexed instruction post-byte");
                }
                0x08 => { // (+/- 7 bit offset),R
                    let postbyte2 = self.read_pc8();
                    ea = self.register_select(postbyte).wrapping_add(self.s16(postbyte2) as u16);
                    self.add_cycles(1);
                }
                0x09 => { // (+/- 15 bit offset),R
                    let postbyte2 = self.read_pc8();
                    let postbyte3 = self.read_pc8();
                    ea = self.register_select(postbyte).wrapping_add(self.combine_to_s16(postbyte2, postbyte3) as u16);
                    self.add_cycles(4);
                }
                0x0A => {
                    panic!("Illegal indexed instruction post-byte");
                }
                0x0B => { // (+/- D),R
                    ea = self.register_select(postbyte).wrapping_add(self.s16(self.registers.d() as u8) as u16);
                    self.add_cycles(4);
                }
                0x0C => { // (+/- 7 bit offset),PC
                    let postbyte2 = self.read_pc8();
                    ea = self.registers.pc.wrapping_add(self.s16(postbyte2) as u16);
                    self.add_cycles(1);
                }
                0x0D => { // (+/- 15 bit offset),PC
                    let postbyte2 = self.read_pc8();
                    let postbyte3 = self.read_pc8();
                    ea = self.registers.pc.wrapping_add(self.combine_to_s16(postbyte2, postbyte3) as u16);
                    self.add_cycles(5);
                }
                0x0E => {
                    panic!("Illegal indexed instruction post-byte");
                }
                0x0F => { // [address] (Indirect-only)
                    let postbyte2 = self.read_pc8();
                    let postbyte3 = self.read_pc8();
                    ea = self.combine_to_s16(postbyte2, postbyte3) as u16;
                    self.add_cycles(2);
                }
                _ => {
                    panic!("Illegal indexed instruction post-byte");
                }
            }
        }

        if supports_indirect && (postbyte & 0x10) != 0 {
            let msb = self.read8(ea);
            let lsb = self.read8(ea + 1);
            ea = self.combine_to_u16(msb, lsb);
            self.add_cycles(3);
        }

        ea
    }

    // C++ Original: RegisterSelect lambda in ReadIndexedEA
    fn register_select(&self, postbyte: u8) -> u16 {
        match (postbyte >> 5) & 0x03 {
            0x00 => self.registers.x,
            0x01 => self.registers.y,
            0x02 => self.registers.u,
            _ => self.registers.s, // 0x03
        }
    }

    fn register_select_mut(&mut self, postbyte: u8) -> &mut u16 {
        match (postbyte >> 5) & 0x03 {
            0x00 => &mut self.registers.x,
            0x01 => &mut self.registers.y,
            0x02 => &mut self.registers.u,
            _ => &mut self.registers.s, // 0x03
        }
    }
}