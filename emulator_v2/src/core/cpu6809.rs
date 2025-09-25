// MC6809 CPU implementation
// Port of vectrexy/libs/emulator/include/emulator/Cpu.h and src/Cpu.cpp

impl PartialEq<u8> for ConditionCode {
    fn eq(&self, other: &u8) -> bool {
        self.to_u8() == *other
    }
}

use crate::types::Cycles;
use crate::core::memory_bus::MemoryBus;
use crate::core::cpu_helpers::{combine_to_u16, combine_to_s16, u8, s16_from_u8};
use crate::core::cpu_op_codes::{lookup_cpu_op_runtime, is_opcode_page1, is_opcode_page2, AddressingMode};
use std::cell::RefCell;
use std::rc::Rc;

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
    static constexpr uint16_t SWI2_VECTOR = 0xFFF2;
    static constexpr uint16_t SWI3_VECTOR = 0xFFF4;

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
    
    // C++ Original: bool m_waitingForInterrupts;
    pub waiting_for_interrupts: bool,
    
    // C++ Original: InterruptType callbacks
    nmi_interrupt: Option<InterruptCallback>,
    irq_interrupt: Option<InterruptCallback>,
    firq_interrupt: Option<InterruptCallback>,
}

// C++ Original: Interrupt vector constants from Vectrexy Cpu.cpp
const RESET_VECTOR: u16 = 0xFFFE; // Used in reset()
const NMI_VECTOR: u16   = 0xFFFC; // TODO: Non-maskable interrupt - not implemented
const SWI_VECTOR: u16   = 0xFFFA; // TODO: Will be used when SWI opcode is implemented
const IRQ_VECTOR: u16   = 0xFFF8; // TODO: Will be used when interrupt handling is implemented
const FIRQ_VECTOR: u16  = 0xFFF6; // TODO: Fast interrupt - not implemented
const SWI2_VECTOR: u16  = 0xFFF2; // C++ Original: InterruptVector::Swi2 = 0xFFF2
const SWI3_VECTOR: u16  = 0xFFF4; // C++ Original: InterruptVector::Swi3 = 0xFFF4

impl Cpu6809 {
    pub fn new(memory_bus: Rc<RefCell<MemoryBus>>) -> Self {
        Self {
            registers: CpuRegisters::new(),
            memory_bus,
            cycles: 0,
            waiting_for_interrupts: false,
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

    // C++ Original: Init(MemoryBus& memoryBus) - for testing access
    pub fn memory_bus(&self) -> &Rc<RefCell<MemoryBus>> {
        &self.memory_bus
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
        self.waiting_for_interrupts = false; // C++ Original: m_waitingForInterrupts = false
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

    // C++ Original: cycles_t ExecuteInstruction(bool irqEnabled, bool firqEnabled)
    pub fn execute_instruction(&mut self, irq_enabled: bool, firq_enabled: bool) -> Cycles {
        self.cycles = 0;
        self.do_execute_instruction(irq_enabled, firq_enabled);
        self.cycles
    }

    // C++ Original: DoExecuteInstruction
    fn do_execute_instruction(&mut self, irq_enabled: bool, firq_enabled: bool) {
        // Check if CPU is waiting for interrupts (SYNC instruction)
        if self.waiting_for_interrupts {
            // First, check if the next instruction is RESET* (0x3E) - it should execute even during SYNC
            let next_opcode = self.read8(self.registers.pc);
            if next_opcode == 0x3E {
                // RESET* always executes, even during SYNC - continue with normal execution
                self.waiting_for_interrupts = false;
            } else if irq_enabled || firq_enabled {
                // If an interrupt occurs, clear the waiting state
                self.waiting_for_interrupts = false;
            } else {
                // Still waiting for interrupt - add minimal cycles
                self.add_cycles(1);
                return;
            }
        }

        // C++ Original: Read op code byte and page
        let mut cpu_op_page = 0;
        let mut opcode_byte = self.read_pc8();
        
        if is_opcode_page1(opcode_byte) {
            cpu_op_page = 1;
            opcode_byte = self.read_pc8();
        } else if is_opcode_page2(opcode_byte) {
            cpu_op_page = 2; 
            opcode_byte = self.read_pc8();
        }

        // C++ Original: const CpuOp& cpuOp = LookupCpuOpRuntime(cpuOpPage, opCodeByte);
        let cpu_op = lookup_cpu_op_runtime(cpu_op_page, opcode_byte);
        
        // C++ Original: AddCycles(cpuOp.cycles);
        self.add_cycles(cpu_op.cycles as Cycles);

        if cpu_op.addr_mode == AddressingMode::Illegal {
            panic!("Illegal instruction at PC={:04X}, opcode: {:02X}, page: {}", 
                   self.registers.pc.wrapping_sub(1), opcode_byte, cpu_op_page);
        }

        // C++ Original: switch (cpuOpPage)
        match cpu_op_page {
            0 => {
                // C++ Original: switch (cpuOp.opCode) - Page 0 instructions
                match opcode_byte {
                    // C++ Original: OpNEG<0, 0x00>(); - NEG direct
                    // C++ Original: uint8_t value = m_memoryBus->Read(EA); value = SubtractImpl(0, value, 0, CC); m_memoryBus->Write(EA, value);
                    0x00 => {
                        self.op_neg_memory(opcode_byte);
                    },

                    // C++ Original: OpCOM<0, 0x03>(); - COM direct  
                    // C++ Original: value = ~value; CC.Carry = 1; CC.Overflow = 0;
                    0x03 => {
                        self.op_com_memory(opcode_byte);
                    },

                    // C++ Original: OpLSR<0, 0x04>();
                    0x04 => {
                        self.op_lsr_memory(opcode_byte);
                    },

                    // C++ Original: OpROR<0, 0x06>();
                    0x06 => {
                        self.op_ror_memory(opcode_byte);
                    },

                    // C++ Original: OpASR<0, 0x07>();
                    0x07 => {
                        self.op_asr_memory(opcode_byte);
                    },

                    // C++ Original: OpASL<0, 0x08>();
                    0x08 => {
                        self.op_asl_memory(opcode_byte);
                    },

                    // C++ Original: OpROL<0, 0x09>();
                    0x09 => {
                        self.op_rol_memory(opcode_byte);
                    },

                    // C++ Original: OpDEC<0, 0x0A>(); - DEC direct
                    // C++ Original: uint8_t origValue = value; --value; CC.Overflow = (origValue == 0x80) ? 1 : 0;
                    0x0A => {
                        self.op_dec_memory(opcode_byte);
                    },

                    // C++ Original: OpINC<0, 0x0C>(); - INC direct
                    // C++ Original: uint8_t origValue = value; ++value; CC.Overflow = (origValue == 0x7F) ? 1 : 0;
                    0x0C => {
                        self.op_inc_memory(opcode_byte);
                    },

                    // C++ Original: OpTST<0, 0x0D>(); - TST direct
                    // C++ Original: CC.Negative = CalcNegative(value); CC.Zero = CalcZero(value); CC.Overflow = 0; CC.Carry = 0;
                    0x0D => {
                        self.op_tst_memory(opcode_byte);
                    },

                    // C++ Original: OpCLR<0, 0x0F>(); - CLR direct  
                    // C++ Original: value = 0; CC.Negative = 0; CC.Zero = 1; CC.Overflow = 0; CC.Carry = 0;
                    0x0F => {
                        self.op_clr_memory(opcode_byte);
                    },

                    // NOP
                    0x12 => {
                        // No operation - cycles already added
                    },

                    // SYNC - C++ Original: Synchronize with interrupt
                    0x13 => {
                        self.op_sync();
                    },

                    // C++ Original: OpDAA(); - DAA (Decimal Adjust A)
                    // C++ Original: uint8_t cfLsn = ((CC.HalfCarry == 1) || (lsn > 9)) ? 6 : 0; uint8_t cfMsn = ((CC.Carry == 1) || (msn > 9) || (msn > 8 && lsn > 9)) ? 6 : 0;
                    0x19 => {
                        self.op_daa();
                    },
                    
                    // 8-bit LD instructions - C++ Original: OpLD<0, opCode>(register)
                    0x86 => { // LDA #immediate
                        let value = self.op_ld_8(opcode_byte);
                        self.registers.a = value;
                    },
                    0x96 => { // LDA direct
                        let value = self.op_ld_8(opcode_byte);
                        self.registers.a = value;
                    },
                    0xA6 => { // LDA indexed
                        let value = self.op_ld_8(opcode_byte);
                        self.registers.a = value;
                    },
                    0xB6 => { // LDA extended
                        let value = self.op_ld_8(opcode_byte);
                        self.registers.a = value;
                    },
                    
                    0xC6 => { // LDB #immediate
                        let value = self.op_ld_8(opcode_byte);
                        self.registers.b = value;
                    },
                    0xD6 => { // LDB direct
                        let value = self.op_ld_8(opcode_byte);
                        self.registers.b = value;
                    },
                    0xE6 => { // LDB indexed
                        let value = self.op_ld_8(opcode_byte);
                        self.registers.b = value;
                    },
                    0xF6 => { // LDB extended
                        let value = self.op_ld_8(opcode_byte);
                        self.registers.b = value;
                    },

                    // 16-bit LD instructions
                    0x8E => { // LDX #immediate
                        let value = self.op_ld_16(opcode_byte);
                        self.registers.x = value;
                    },
                    0x9E => { // LDX direct
                        let value = self.op_ld_16(opcode_byte);
                        self.registers.x = value;
                    },
                    0xAE => { // LDX indexed
                        let value = self.op_ld_16(opcode_byte);
                        self.registers.x = value;
                    },
                    0xBE => { // LDX extended
                        let value = self.op_ld_16(opcode_byte);
                        self.registers.x = value;
                    },
                    
                    0xCE => { // LDU #immediate
                        let value = self.op_ld_16(opcode_byte);
                        self.registers.u = value;
                    },
                    0xDE => { // LDU direct
                        let value = self.op_ld_16(opcode_byte);
                        self.registers.u = value;
                    },
                    0xEE => { // LDU indexed
                        let value = self.op_ld_16(opcode_byte);
                        self.registers.u = value;
                    },
                    0xFE => { // LDU extended
                        let value = self.op_ld_16(opcode_byte);
                        self.registers.u = value;
                    },

                    // C++ Original: 8-bit ST instructions - OpST<0, opCode>(register)
                    0x97 => { // STA direct
                        self.op_st_8(self.registers.a, opcode_byte);
                    },
                    0xA7 => { // STA indexed
                        self.op_st_8(self.registers.a, opcode_byte);
                    },
                    0xB7 => { // STA extended
                        self.op_st_8(self.registers.a, opcode_byte);
                    },

                    0xD7 => { // STB direct
                        self.op_st_8(self.registers.b, opcode_byte);
                    },
                    0xE7 => { // STB indexed
                        self.op_st_8(self.registers.b, opcode_byte);
                    },
                    0xF7 => { // STB extended
                        self.op_st_8(self.registers.b, opcode_byte);
                    },

                    // C++ Original: 16-bit ST instructions - OpST<0, opCode>(register)
                    0x9F => { // STX direct
                        self.op_st_16(self.registers.x, opcode_byte);
                    },
                    0xAF => { // STX indexed
                        self.op_st_16(self.registers.x, opcode_byte);
                    },
                    0xBF => { // STX extended
                        self.op_st_16(self.registers.x, opcode_byte);
                    },

                    0xDD => { // STD direct
                        self.op_st_16(self.registers.d(), opcode_byte);
                    },
                    0xED => { // STD indexed
                        self.op_st_16(self.registers.d(), opcode_byte);
                    },
                    0xFD => { // STD extended
                        self.op_st_16(self.registers.d(), opcode_byte);
                    },

                    0xDF => { // STU direct
                        self.op_st_16(self.registers.u, opcode_byte);
                    },
                    0xEF => { // STU indexed
                        self.op_st_16(self.registers.u, opcode_byte);
                    },
                    0xFF => { // STU extended
                        self.op_st_16(self.registers.u, opcode_byte);
                    },

                    // C++ Original: OpSUB<0, 0x80>(A); - SUBA immediate
                    // C++ Original for immediate mode: ReadOperandValue8<AddressingMode::Immediate>() { return ReadPC8(); }
                    0x80 => {
                        let operand = self.read_pc8();
                        self.registers.a = self.subtract_impl_u8(self.registers.a, operand, 0);
                    },

                    // C++ Original: OpADD<0, 0x8B>(A); - ADDA immediate  
                    // C++ Original for immediate mode: ReadOperandValue8<AddressingMode::Immediate>() { return ReadPC8(); }
                    0x8B => {
                        let operand = self.read_pc8();
                        self.registers.a = self.add_impl_u8(self.registers.a, operand, 0);
                    },

                    // C++ Original: OpSUB<0, 0xC0>(B); - SUBB immediate
                    // C++ Original for immediate mode: ReadOperandValue8<AddressingMode::Immediate>() { return ReadPC8(); }
                    0xC0 => {
                        let operand = self.read_pc8();
                        self.registers.b = self.subtract_impl_u8(self.registers.b, operand, 0);
                    },

                    // C++ Original: OpADD<0, 0xCB>(B); - ADDB immediate
                    // C++ Original for immediate mode: ReadOperandValue8<AddressingMode::Immediate>() { return ReadPC8(); }
                    0xCB => {
                        let operand = self.read_pc8();
                        self.registers.b = self.add_impl_u8(self.registers.b, operand, 0);
                    },

                    // C++ Original: OpAND<0, 0x84>(A); - ANDA immediate
                    // C++ Original: reg = reg & value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0x84 => {
                        let operand = self.read_pc8();
                        self.registers.a = self.registers.a & operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                    },

                    // C++ Original: OpBIT<0, 0x85>(A); - BITA immediate
                    // C++ Original: uint8_t andResult = reg & ReadOperandValue8<cpuOp, opCode>(); CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;
                    0x85 => {
                        self.op_bita_immediate();
                    },

                    // C++ Original: OpEOR<0, 0x88>(A); - EORA immediate  
                    // C++ Original: reg ^= value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0x88 => {
                        let operand = self.read_pc8();
                        self.registers.a = self.registers.a ^ operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                    },

                    // C++ Original: OpADC<0, 0x89>(A); - ADCA immediate
                    // C++ Original: reg = AddImpl(reg, value, CC.Carry); CC.Carry = carry; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = overflow;
                    0x89 => {
                        let operand = self.read_pc8();
                        self.registers.a = self.add_impl_u8(self.registers.a, operand, if self.registers.cc.c { 1 } else { 0 });
                    },

                    // C++ Original: OpOR<0, 0x8A>(A); - ORA immediate
                    // C++ Original: reg = reg | value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0x8A => {
                        let operand = self.read_pc8();
                        self.registers.a = self.registers.a | operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                    },

                    // C++ Original: OpAND<0, 0xC4>(B); - ANDB immediate
                    // C++ Original: reg = reg & value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0xC4 => {
                        let operand = self.read_pc8();
                        self.registers.b = self.registers.b & operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },

                    // C++ Original: OpBIT<0, 0xC5>(B); - BITB immediate
                    // C++ Original: uint8_t andResult = reg & ReadOperandValue8<cpuOp, opCode>(); CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;
                    0xC5 => {
                        self.op_bitb_immediate();
                    },

                    // C++ Original: OpEOR<0, 0xC8>(B); - EORB immediate  
                    // C++ Original: reg ^= value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0xC8 => {
                        let operand = self.read_pc8();
                        self.registers.b = self.registers.b ^ operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },

                    // C++ Original: OpADC<0, 0xC9>(B); - ADCB immediate
                    // C++ Original: reg = AddImpl(reg, ReadOperandValue8<...>(), carry, CC);
                    0xC9 => {
                        let operand = self.read_pc8();
                        let carry = if self.registers.cc.c { 1 } else { 0 };
                        self.registers.b = self.add_impl_u8(self.registers.b, operand, carry);
                    },

                    // C++ Original: OpOR<0, 0xCA>(B); - ORB immediate
                    // C++ Original: reg = reg | value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0xCA => {
                        let operand = self.read_pc8();
                        self.registers.b = self.registers.b | operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },

                    // C++ Original: OpLD<0, 0xCC>(D); - LDD immediate
                    // C++ Original: CC.Negative = CalcNegative(value); CC.Zero = CalcZero(value); CC.Overflow = 0; targetReg = value;
                    0xCC => {
                        let operand = self.read_pc16();
                        self.registers.cc.n = Self::calc_negative_u16(operand);
                        self.registers.cc.z = Self::calc_zero_u16(operand);
                        self.registers.cc.v = false;
                        self.registers.set_d(operand);
                    },

                    // C++ Original: OpOR<0, 0xDA>(B); - ORB direct
                    // C++ Original: reg = reg | value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0xDA => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.registers.b | operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },

                    // C++ Original: OpLD<0, 0xDC>(D); - LDD direct
                    // C++ Original: CC.Negative = CalcNegative(value); CC.Zero = CalcZero(value); CC.Overflow = 0; targetReg = value;
                    0xDC => {
                        let ea = self.read_direct_ea();
                        let operand = self.read16(ea);
                        self.registers.cc.n = Self::calc_negative_u16(operand);
                        self.registers.cc.z = Self::calc_zero_u16(operand);
                        self.registers.cc.v = false;
                        self.registers.set_d(operand);
                    },

                    // C++ Original: OpADD<0, 0xDB>(A); - ADDA direct
                    // C++ Original: reg = AddImpl(reg, ReadOperandValue8<...>(), 0, CC);
                    0xDB => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.add_impl_u8(self.registers.a, operand, 0);
                    },

                    // C++ Original: OpAND<0, 0xD4>(B); - ANDB direct
                    // C++ Original: reg = reg & value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0xD4 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.registers.b & operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },

                    // C++ Original: OpBIT<0, 0xD5>(B); - BITB direct
                    // C++ Original: uint8_t andResult = reg & ReadOperandValue8<cpuOp, opCode>(); CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;
                    0xD5 => {
                        self.op_bitb_direct();
                    },

                    // C++ Original: OpEOR<0, 0xD8>(B); - EORB direct
                    // C++ Original: reg ^= value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0xD8 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.registers.b ^ operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },

                    // C++ Original: OpADC<0, 0xD9>(B); - ADCB direct
                    // C++ Original: reg = AddImpl(reg, ReadOperandValue8<...>(), carry, CC);
                    0xD9 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        let carry = if self.registers.cc.c { 1 } else { 0 };
                        self.registers.b = self.add_impl_u8(self.registers.b, operand, carry);
                    },

                    // C++ Original: OpAND<0, 0xF4>(B); - ANDB extended
                    // C++ Original: reg = reg & value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0xF4 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.registers.b & operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },

                    // C++ Original: OpBIT<0, 0xF5>(B); - BITB extended
                    // C++ Original: uint8_t andResult = reg & ReadOperandValue8<cpuOp, opCode>(); CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;
                    0xF5 => {
                        self.op_bitb_extended();
                    },

                    // C++ Original: OpEOR<0, 0xF8>(B); - EORB extended
                    // C++ Original: reg ^= value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0xF8 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.registers.b ^ operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },

                    // C++ Original: OpADC<0, 0xF9>(B); - ADCB extended
                    // C++ Original: reg = AddImpl(reg, ReadOperandValue8<...>(), carry, CC);
                    0xF9 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        let carry = if self.registers.cc.c { 1 } else { 0 };
                        self.registers.b = self.add_impl_u8(self.registers.b, operand, carry);
                    },

                    // C++ Original: OpADD<0, 0xFB>(A); - ADDA extended
                    // C++ Original: reg = AddImpl(reg, ReadOperandValue8<...>(), 0, CC);
                    0xFB => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.add_impl_u8(self.registers.a, operand, 0);
                    },

                    // C++ Original: OpLD<0, 0xFC>(D); - LDD extended
                    // C++ Original: CC.Negative = CalcNegative(value); CC.Zero = CalcZero(value); CC.Overflow = 0; targetReg = value;
                    0xFC => {
                        let ea = self.read_extended_ea();
                        let operand = self.read16(ea);
                        self.registers.cc.n = Self::calc_negative_u16(operand);
                        self.registers.cc.z = Self::calc_zero_u16(operand);
                        self.registers.cc.v = false;
                        self.registers.set_d(operand);
                    },

                    // C++ Original: OpOR<0, 0xFA>(B); - ORB extended
                    // C++ Original: reg = reg | value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0xFA => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.registers.b | operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },

                    // ======== COMPARE OPERATIONS ========

                    // C++ Original: OpCMP<0, 0x81>(A); - CMPA immediate
                    // C++ Original: uint8_t discard = SubtractImpl(reg, ReadOperandValue8<...>(), 0, CC); (void)discard;
                    0x81 => {
                        let operand = self.read_pc8();
                        let _discard = self.subtract_impl_u8(self.registers.a, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpSBC<0, 0x82>(A); - SBCA immediate  
                    // C++ Original: reg = SubtractImpl(reg, ReadOperandValue8<...>(), CC.Carry, CC);
                    0x82 => {
                        let operand = self.read_pc8();
                        self.registers.a = self.subtract_impl_u8(self.registers.a, operand, self.registers.cc.c as u8);
                    },

                    // C++ Original: OpCMP<0, 0x91>(A); - CMPA direct
                    // C++ Original: uint8_t discard = SubtractImpl(reg, ReadOperandValue8<...>(), 0, CC); (void)discard;
                    0x91 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        let _discard = self.subtract_impl_u8(self.registers.a, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpSBC<0, 0x92>(A); - SBCA direct
                    // C++ Original: reg = SubtractImpl(reg, value, CC.Carry); CC.Carry = carry; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = overflow;
                    0x92 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.subtract_impl_u8(self.registers.a, operand, if self.registers.cc.c { 1 } else { 0 });
                    },

                    // C++ Original: OpSUB<0, 0xA0>(A); - SUBA indexed
                    0xA0 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.subtract_impl_u8(self.registers.a, operand, 0);
                    },

                    // C++ Original: OpCMP<0, 0xA1>(A); - CMPA indexed
                    // C++ Original: uint8_t discard = SubtractImpl(reg, ReadOperandValue8<...>(), 0, CC); (void)discard;
                    0xA1 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        let _discard = self.subtract_impl_u8(self.registers.a, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpSBC<0, 0xA2>(A); - SBCA indexed
                    0xA2 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.subtract_impl_u8(self.registers.a, operand, if self.registers.cc.c { 1 } else { 0 });
                    },

                    // C++ Original: OpAND<0, 0xA4>(A); - ANDA indexed
                    0xA4 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.registers.a & operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                    },

                    // C++ Original: OpBIT<0, 0xA5>(A); - BITA indexed
                    // C++ Original: uint8_t andResult = reg & ReadOperandValue8<cpuOp, opCode>(); CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;
                    0xA5 => {
                        self.op_bita_indexed();
                    },

                    // C++ Original: OpADC<0, 0xA9>(A); - ADCA indexed
                    0xA9 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.add_impl_u8(self.registers.a, operand, if self.registers.cc.c { 1 } else { 0 });
                    },

                    // C++ Original: OpADD<0, 0xAB>(A); - ADDA indexed
                    0xAB => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.add_impl_u8(self.registers.a, operand, 0);
                    },

                    // C++ Original: OpCMP<0, 0xB1>(A); - CMPA extended  
                    // C++ Original: uint8_t discard = SubtractImpl(reg, ReadOperandValue8<...>(), 0, CC); (void)discard;
                    0xB1 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        let _discard = self.subtract_impl_u8(self.registers.a, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpSBC<0, 0xB2>(A); - SBCA extended
                    // C++ Original: reg = SubtractImpl(reg, ReadOperandValue8<...>(), borrow, CC);
                    0xB2 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        let borrow = if self.registers.cc.c { 1 } else { 0 };
                        self.registers.a = self.subtract_impl_u8(self.registers.a, operand, borrow);
                    },

                    // C++ Original: OpCMP<0, 0xC1>(B); - CMPB immediate
                    // C++ Original: uint8_t discard = SubtractImpl(reg, ReadOperandValue8<...>(), 0, CC); (void)discard;
                    0xC1 => {
                        let operand = self.read_pc8();
                        let _discard = self.subtract_impl_u8(self.registers.b, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpSBC<0, 0xC2>(B); - SBCB immediate
                    // C++ Original: reg = SubtractImpl(reg, ReadOperandValue8<...>(), borrow, CC);
                    0xC2 => {
                        let operand = self.read_pc8();
                        let borrow = if self.registers.cc.c { 1 } else { 0 };
                        self.registers.b = self.subtract_impl_u8(self.registers.b, operand, borrow);
                    },

                    // C++ Original: OpADD<1, 0xC3>(D); - ADDD immediate
                    // C++ Original: reg = AddImpl(reg, ReadOperandValue16<...>(), 0, CC);
                    0xC3 => {
                        let operand = self.read_pc16();
                        let current_d = self.registers.d();
                        let result = self.add_impl_u16(current_d, operand, 0);
                        self.registers.set_d(result);
                    },

                    // C++ Original: OpSUB<0, 0xD0>(B); - SUBB direct
                    // C++ Original: reg = SubtractImpl(reg, ReadOperandValue8<cpuOp, opCode>(), 0, CC);
                    0xD0 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.subtract_impl_u8(self.registers.b, operand, 0);
                    },

                    // C++ Original: OpCMP<0, 0xD1>(B); - CMPB direct
                    // C++ Original: uint8_t discard = SubtractImpl(reg, ReadOperandValue8<...>(), 0, CC); (void)discard;
                    0xD1 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        let _discard = self.subtract_impl_u8(self.registers.b, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpSBC<0, 0xD2>(B); - SBCB direct
                    // C++ Original: reg = SubtractImpl(reg, ReadOperandValue8<...>(), borrow, CC);
                    0xD2 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        let borrow = if self.registers.cc.c { 1 } else { 0 };
                        self.registers.b = self.subtract_impl_u8(self.registers.b, operand, borrow);
                    },

                    // C++ Original: OpADD<1, 0xD3>(D); - ADDD direct
                    // C++ Original: reg = AddImpl(reg, ReadOperandValue16<...>(), 0, CC);
                    0xD3 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read16(ea);
                        let current_d = self.registers.d();
                        let result = self.add_impl_u16(current_d, operand, 0);
                        self.registers.set_d(result);
                    },

                    // C++ Original: OpSUB<0, 0xE0>(B); - SUBB indexed
                    // C++ Original: reg = SubtractImpl(reg, ReadOperandValue8<cpuOp, opCode>(), 0, CC);
                    0xE0 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.subtract_impl_u8(self.registers.b, operand, 0);
                    },

                    // C++ Original: OpCMP<0, 0xE1>(B); - CMPB indexed
                    // C++ Original: uint8_t discard = SubtractImpl(reg, ReadOperandValue8<...>(), 0, CC); (void)discard;
                    0xE1 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        let _discard = self.subtract_impl_u8(self.registers.b, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpSBC<0, 0xE2>(B); - SBCB indexed
                    // C++ Original: reg = SubtractImpl(reg, ReadOperandValue8<...>(), borrow, CC);
                    0xE2 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        let borrow = if self.registers.cc.c { 1 } else { 0 };
                        self.registers.b = self.subtract_impl_u8(self.registers.b, operand, borrow);
                    },

                    // C++ Original: OpADD<1, 0xE3>(D); - ADDD indexed
                    // C++ Original: reg = AddImpl(reg, ReadOperandValue16<...>(), 0, CC);
                    0xE3 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read16(ea);
                        let current_d = self.registers.d();
                        let result = self.add_impl_u16(current_d, operand, 0);
                        self.registers.set_d(result);
                    },

                    // C++ Original: OpAND<0, 0xE4>(B); - ANDB indexed
                    // C++ Original: reg = reg & value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0xE4 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.registers.b & operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },

                    // C++ Original: OpBIT<0, 0xE5>(B); - BITB indexed
                    // C++ Original: uint8_t andResult = reg & ReadOperandValue8<cpuOp, opCode>(); CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;
                    0xE5 => {
                        self.op_bitb_indexed();
                    },

                    // C++ Original: OpEOR<0, 0xE8>(B); - EORB indexed
                    // C++ Original: reg ^= value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0xE8 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.registers.b ^ operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },

                    // C++ Original: OpADC<0, 0xE9>(B); - ADCB indexed
                    // C++ Original: reg = AddImpl(reg, ReadOperandValue8<...>(), carry, CC);
                    0xE9 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        let carry = if self.registers.cc.c { 1 } else { 0 };
                        self.registers.b = self.add_impl_u8(self.registers.b, operand, carry);
                    },

                    // C++ Original: OpOR<0, 0xEA>(B); - ORB indexed
                    // C++ Original: reg = reg | value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0xEA => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.registers.b | operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },

                    // C++ Original: OpLD<0, 0xEC>(D); - LDD indexed
                    // C++ Original: CC.Negative = CalcNegative(value); CC.Zero = CalcZero(value); CC.Overflow = 0; targetReg = value;
                    0xEC => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read16(ea);
                        self.registers.cc.n = Self::calc_negative_u16(operand);
                        self.registers.cc.z = Self::calc_zero_u16(operand);
                        self.registers.cc.v = false;
                        self.registers.set_d(operand);
                    },

                    // C++ Original: OpADD<0, 0xEB>(A); - ADDA indexed
                    // C++ Original: reg = AddImpl(reg, ReadOperandValue8<...>(), 0, CC);
                    0xEB => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.add_impl_u8(self.registers.a, operand, 0);
                    },

                    // C++ Original: OpSUB<0, 0xF0>(B); - SUBB extended
                    // C++ Original: reg = SubtractImpl(reg, ReadOperandValue8<cpuOp, opCode>(), 0, CC);
                    0xF0 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.subtract_impl_u8(self.registers.b, operand, 0);
                    },

                    // C++ Original: OpCMP<0, 0xF1>(B); - CMPB extended  
                    // C++ Original: uint8_t discard = SubtractImpl(reg, ReadOperandValue8<...>(), 0, CC); (void)discard;
                    0xF1 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        let _discard = self.subtract_impl_u8(self.registers.b, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpSBC<0, 0xF2>(B); - SBCB extended
                    // C++ Original: reg = SubtractImpl(reg, ReadOperandValue8<...>(), borrow, CC);
                    0xF2 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        let borrow = if self.registers.cc.c { 1 } else { 0 };
                        self.registers.b = self.subtract_impl_u8(self.registers.b, operand, borrow);
                    },

                    // C++ Original: OpADD<1, 0xF3>(D); - ADDD extended
                    // C++ Original: reg = AddImpl(reg, ReadOperandValue16<...>(), 0, CC);
                    0xF3 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read16(ea);
                        let current_d = self.registers.d();
                        let result = self.add_impl_u16(current_d, operand, 0);
                        self.registers.set_d(result);
                    },

                    // ======== SINGLE REGISTER OPERATIONS ========

                    // C++ Original: void OpNEG(uint8_t& value) { value = SubtractImpl(0, value, 0, CC); }
                    0x40 => {
                        self.registers.a = self.subtract_impl_u8(0, self.registers.a, 0);
                    },                    // C++ Original: void OpCOM(uint8_t& value) { value = ~value; CC.Negative = CalcNegative(value); CC.Zero = CalcZero(value); CC.Overflow = 0; CC.Carry = 1; }
                    0x43 => {
                        self.registers.a = !self.registers.a;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                        self.registers.cc.c = true;
                    },

                    // C++ Original: OpLSR<0, 0x44>(A);
                    0x44 => {
                        let orig_value = self.registers.a;
                        self.registers.a = self.registers.a >> 1;
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.n = false; // Bit 7 always shifted out
                        self.registers.cc.c = (orig_value & 0b0000_0001) != 0;
                    },

                    // C++ Original: OpROR<0, 0x46>(A);
                    0x46 => {
                        let result = ((self.registers.cc.c as u8) << 7) | (self.registers.a >> 1);
                        self.registers.cc.c = (self.registers.a & 0b0000_0001) != 0;
                        self.registers.cc.n = Self::calc_negative_u8(result);
                        self.registers.cc.z = Self::calc_zero_u8(result);
                        self.registers.a = result;
                    },

                    // C++ Original: OpASR<0, 0x47>(A);
                    0x47 => {
                        let orig_value = self.registers.a;
                        self.registers.a = (orig_value & 0b1000_0000) | (self.registers.a >> 1);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.c = (orig_value & 0b0000_0001) != 0;
                    },

                    // C++ Original: OpASL<0, 0x48>(A);
                    0x48 => {
                        self.registers.a = self.add_impl_u8(self.registers.a, self.registers.a, 0);
                    },

                    // C++ Original: OpROL<0, 0x49>(A);
                    0x49 => {
                        let result = (self.registers.a << 1) | (self.registers.cc.c as u8);
                        self.registers.cc.c = (self.registers.a & 0b1000_0000) != 0;
                        self.registers.cc.v = ((self.registers.a & 0b1000_0000) ^ ((self.registers.a & 0b0100_0000) << 1)) != 0;
                        self.registers.cc.n = Self::calc_negative_u8(result);
                        self.registers.cc.z = Self::calc_zero_u8(result);
                        self.registers.a = result;
                    },

                    // C++ Original: void OpDEC(uint8_t& value) { uint8_t origValue = value; --value; CC.Overflow = origValue == 0b1000'0000; CC.Zero = CalcZero(value); CC.Negative = CalcNegative(value); }
                    0x4A => {
                        let orig_value = self.registers.a;
                        self.registers.a = self.registers.a.wrapping_sub(1);
                        self.registers.cc.v = orig_value == 0b1000_0000;
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        // Note: DEC does NOT modify Carry flag in 6809
                    },

                    // C++ Original: void OpINC(uint8_t& value) { uint8_t origValue = value; ++value; CC.Overflow = origValue == 0b0111'1111; CC.Zero = CalcZero(value); CC.Negative = CalcNegative(value); }
                    0x4C => {
                        let orig_value = self.registers.a;
                        self.registers.a = self.registers.a.wrapping_add(1);
                        self.registers.cc.v = orig_value == 0b0111_1111;
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        // Note: INC does NOT modify Carry flag in 6809
                    },

                    // C++ Original: void OpTST(const uint8_t& value) { CC.Negative = CalcNegative(value); CC.Zero = CalcZero(value); CC.Overflow = 0; }
                    0x4D => {
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                        // Note: TST does NOT modify Carry flag in 6809
                    },

                    // C++ Original: void OpCLR(uint8_t& reg) { reg = 0; CC.Negative = 0; CC.Zero = 1; CC.Overflow = 0; CC.Carry = 0; }
                    0x4F => {
                        self.registers.a = 0;
                        self.registers.cc.n = false;
                        self.registers.cc.z = true;
                        self.registers.cc.v = false;
                        self.registers.cc.c = false;
                    },

                    // C++ Original: OpNEG<0, 0x50>(B);
                    0x50 => {
                        self.registers.b = self.subtract_impl_u8(0, self.registers.b, 0);
                    },

                    // C++ Original: OpCOM<0, 0x53>(B);
                    0x53 => {
                        self.registers.b = !self.registers.b;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                        self.registers.cc.c = true;
                    },

                    // C++ Original: OpLSR<0, 0x54>(B);
                    0x54 => {
                        let orig_value = self.registers.b;
                        self.registers.b = self.registers.b >> 1;
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.n = false; // Bit 7 always shifted out
                        self.registers.cc.c = (orig_value & 0b0000_0001) != 0;
                    },

                    // C++ Original: OpROR<0, 0x56>(B);
                    0x56 => {
                        let result = ((self.registers.cc.c as u8) << 7) | (self.registers.b >> 1);
                        self.registers.cc.c = (self.registers.b & 0b0000_0001) != 0;
                        self.registers.cc.n = Self::calc_negative_u8(result);
                        self.registers.cc.z = Self::calc_zero_u8(result);
                        self.registers.b = result;
                    },

                    // C++ Original: OpASR<0, 0x57>(B);
                    0x57 => {
                        let orig_value = self.registers.b;
                        self.registers.b = (orig_value & 0b1000_0000) | (self.registers.b >> 1);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.c = (orig_value & 0b0000_0001) != 0;
                    },

                    // C++ Original: OpASL<0, 0x58>(B);
                    0x58 => {
                        self.registers.b = self.add_impl_u8(self.registers.b, self.registers.b, 0);
                    },

                    // C++ Original: OpROL<0, 0x59>(B);
                    0x59 => {
                        let result = (self.registers.b << 1) | (self.registers.cc.c as u8);
                        self.registers.cc.c = (self.registers.b & 0b1000_0000) != 0;
                        self.registers.cc.v = ((self.registers.b & 0b1000_0000) ^ ((self.registers.b & 0b0100_0000) << 1)) != 0;
                        self.registers.cc.n = Self::calc_negative_u8(result);
                        self.registers.cc.z = Self::calc_zero_u8(result);
                        self.registers.b = result;
                    },

                    // C++ Original: OpDEC<0, 0x5A>(B);
                    0x5A => {
                        let orig_value = self.registers.b;
                        self.registers.b = self.registers.b.wrapping_sub(1);
                        self.registers.cc.v = orig_value == 0b1000_0000;
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        // Note: DEC does NOT modify Carry flag in 6809
                    },

                    // C++ Original: OpINC<0, 0x5C>(B);
                    0x5C => {
                        let orig_value = self.registers.b;
                        self.registers.b = self.registers.b.wrapping_add(1);
                        self.registers.cc.v = orig_value == 0b0111_1111;
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        // Note: INC does NOT modify Carry flag in 6809
                    },

                    // C++ Original: OpTST<0, 0x5D>(B);
                    0x5D => {
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                        // Note: TST does NOT modify Carry flag in 6809
                    },

                    // C++ Original: void OpCLR(uint8_t& reg) { reg = 0; CC.Negative = 0; CC.Zero = 1; CC.Overflow = 0; CC.Carry = 0; }
                    0x5F => {
                        self.registers.b = 0;
                        self.registers.cc.n = false;
                        self.registers.cc.z = true;
                        self.registers.cc.v = false;
                        self.registers.cc.c = false;
                    },

                    // ======== INDEXED ADDRESSING MODE OPERATIONS ========

                    // C++ Original: OpNEG<0, 0x60>(); - NEG indexed
                    0x60 => {
                        self.op_neg_memory(opcode_byte);
                    },

                    // C++ Original: OpCOM<0, 0x63>(); - COM indexed
                    0x63 => {
                        self.op_com_memory(opcode_byte);
                    },

                    // C++ Original: OpLSR<0, 0x64>();
                    0x64 => {
                        self.op_lsr_memory(opcode_byte);
                    },

                    // C++ Original: OpROR<0, 0x66>();
                    0x66 => {
                        self.op_ror_memory(opcode_byte);
                    },

                    // C++ Original: OpASR<0, 0x67>();
                    0x67 => {
                        self.op_asr_memory(opcode_byte);
                    },

                    // C++ Original: OpASL<0, 0x68>();
                    0x68 => {
                        self.op_asl_memory(opcode_byte);
                    },

                    // C++ Original: OpROL<0, 0x69>();
                    0x69 => {
                        self.op_rol_memory(opcode_byte);
                    },

                    // C++ Original: OpDEC<0, 0x6A>(); - DEC indexed
                    0x6A => {
                        self.op_dec_memory(opcode_byte);
                    },

                    // C++ Original: OpINC<0, 0x6C>(); - INC indexed
                    0x6C => {
                        self.op_inc_memory(opcode_byte);
                    },

                    // C++ Original: OpTST<0, 0x6D>(); - TST indexed
                    0x6D => {
                        self.op_tst_memory(opcode_byte);
                    },

                    // C++ Original: OpCLR<0, 0x6F>(); - CLR indexed
                    0x6F => {
                        self.op_clr_memory(opcode_byte);
                    },

                    // ======== EXTENDED ADDRESSING MODE OPERATIONS ========

                    // C++ Original: OpNEG<0, 0x70>(); - NEG extended
                    0x70 => {
                        self.op_neg_memory(opcode_byte);
                    },

                    // C++ Original: OpCOM<0, 0x73>(); - COM extended
                    0x73 => {
                        self.op_com_memory(opcode_byte);
                    },

                    // C++ Original: OpLSR<0, 0x74>();
                    0x74 => {
                        self.op_lsr_memory(opcode_byte);
                    },

                    // C++ Original: OpROR<0, 0x76>();
                    0x76 => {
                        self.op_ror_memory(opcode_byte);
                    },

                    // C++ Original: OpASR<0, 0x77>();
                    0x77 => {
                        self.op_asr_memory(opcode_byte);
                    },

                    // C++ Original: OpASL<0, 0x78>();
                    0x78 => {
                        self.op_asl_memory(opcode_byte);
                    },

                    // C++ Original: OpROL<0, 0x79>();
                    0x79 => {
                        self.op_rol_memory(opcode_byte);
                    },

                    // C++ Original: OpDEC<0, 0x7A>(); - DEC extended
                    0x7A => {
                        self.op_dec_memory(opcode_byte);
                    },

                    // C++ Original: OpINC<0, 0x7C>(); - INC extended
                    0x7C => {
                        self.op_inc_memory(opcode_byte);
                    },

                    // C++ Original: OpTST<0, 0x7D>(); - TST extended
                    0x7D => {
                        self.op_tst_memory(opcode_byte);
                    },

                    // C++ Original: OpCLR<0, 0x7F>(); - CLR extended
                    0x7F => {
                        self.op_clr_memory(opcode_byte);
                    },

                    // C++ Original: OpCMP<0, 0x8C>(X); - CMPX immediate
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0x8C => {
                        let operand = self.read_pc16();
                        let _discard = self.subtract_impl_u16(self.registers.x, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpSUB<0, 0x90>(A); - SUBA direct
                    // C++ Original: reg = SubtractImpl(reg, ReadOperandValue8<...>(), 0, CC);
                    0x90 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.subtract_impl_u8(self.registers.a, operand, 0);
                    },

                    // C++ Original: OpAND<0, 0x94>(A); - ANDA direct
                    // C++ Original: reg = reg & value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0x94 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.registers.a & operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                    },

                    // C++ Original: OpBIT<0, 0x95>(A); - BITA direct
                    // C++ Original: uint8_t andResult = reg & ReadOperandValue8<cpuOp, opCode>(); CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;
                    0x95 => {
                        self.op_bita_direct();
                    },

                    // C++ Original: OpEOR<0, 0x98>(A); - EORA direct
                    // C++ Original: reg ^= value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0x98 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.registers.a ^ operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                    },

                    // C++ Original: OpADC<0, 0x99>(A); - ADCA direct
                    // C++ Original: reg = AddImpl(reg, value, CC.Carry); CC.Carry = carry; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = overflow;
                    0x99 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.add_impl_u8(self.registers.a, operand, if self.registers.cc.c { 1 } else { 0 });
                    },

                    // C++ Original: OpOR<0, 0x9A>(A); - ORA direct
                    // C++ Original: reg = reg | value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0x9A => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.registers.a | operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                    },

                    // C++ Original: OpADD<0, 0x9B>(A); - ADDA direct
                    // C++ Original: reg = AddImpl(reg, value, 0, CC);
                    0x9B => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.add_impl_u8(self.registers.a, operand, 0);
                    },

                    // C++ Original: OpCMP<0, 0x9C>(X); - CMPX direct
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0x9C => {
                        let ea = self.read_direct_ea();
                        let operand = self.read16(ea);
                        let _discard = self.subtract_impl_u16(self.registers.x, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpEOR<0, 0xA8>(A); - EORA indexed
                    // C++ Original: reg ^= value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0xA8 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.a ^= operand;
                        self.registers.cc.n = (self.registers.a & 0x80) != 0;
                        self.registers.cc.z = self.registers.a == 0;
                        self.registers.cc.v = false; // Always cleared for logical operations
                    },

                    // C++ Original: OpOR<0, 0xAA>(A); - ORA indexed
                    // C++ Original: reg = reg | value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0xAA => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.a |= operand;
                        self.registers.cc.n = (self.registers.a & 0x80) != 0;
                        self.registers.cc.z = self.registers.a == 0;
                        self.registers.cc.v = false; // Always cleared for logical operations
                    },

                    // C++ Original: OpCMP<0, 0xAC>(X); - CMPX indexed
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0xAC => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read16(ea);
                        let _discard = self.subtract_impl_u16(self.registers.x, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpCMP<0, 0xBC>(X); - CMPX extended
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0xBC => {
                        let ea = self.read_extended_ea();
                        let operand = self.read16(ea);
                        let _discard = self.subtract_impl_u16(self.registers.x, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpSUB<0, 0xB0>(A); - SUBA extended
                    // C++ Original: reg = SubtractImpl(reg, ReadOperandValue8<...>(), 0, CC);
                    0xB0 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea); 
                        self.registers.a = self.subtract_impl_u8(self.registers.a, operand, 0);
                    },

                    // C++ Original: OpAND<0, 0xB4>(A); - ANDA extended  
                    // C++ Original: reg = reg & value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0xB4 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.a &= operand;
                        self.registers.cc.n = (self.registers.a & 0x80) != 0;
                        self.registers.cc.z = self.registers.a == 0;
                        self.registers.cc.v = false; // Always cleared for logical operations
                    },

                    // C++ Original: OpBIT<0, 0xB5>(A); - BITA extended
                    // C++ Original: uint8_t andResult = reg & ReadOperandValue8<cpuOp, opCode>(); CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;
                    0xB5 => {
                        self.op_bita_extended();
                    },

                    // C++ Original: OpEOR<0, 0xB8>(A); - EORA extended
                    // C++ Original: reg ^= value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0xB8 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.a ^= operand;
                        self.registers.cc.n = (self.registers.a & 0x80) != 0;
                        self.registers.cc.z = self.registers.a == 0;
                        self.registers.cc.v = false; // Always cleared for logical operations
                    },

                    // C++ Original: OpADC<0, 0xB9>(A); - ADCA extended
                    // C++ Original: reg = AddImpl(reg, ReadOperandValue8<...>(), carry, CC);
                    0xB9 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        let carry = if self.registers.cc.c { 1 } else { 0 };
                        self.registers.a = self.add_impl_u8(self.registers.a, operand, carry);
                    },

                    // C++ Original: OpOR<0, 0xBA>(A); - ORA extended
                    // C++ Original: reg = reg | value; CC.Negative = CalcNegative(reg); CC.Zero = CalcZero(reg); CC.Overflow = 0;
                    0xBA => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.a |= operand;
                        self.registers.cc.n = (self.registers.a & 0x80) != 0;
                        self.registers.cc.z = self.registers.a == 0;
                        self.registers.cc.v = false; // Always cleared for logical operations
                    },

                    // C++ Original: OpADD<0, 0xBB>(A); - ADDA extended
                    // C++ Original: reg = AddImpl(reg, value, 0, CC);
                    0xBB => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.add_impl_u8(self.registers.a, operand, 0);
                    },

                    // C++ Original: OpLEA<0, 0x30>(X); - LEAX indexed
                    // C++ Original: reg = EA; if (&reg == &X || &reg == &Y) { CC.Zero = (reg == 0); }
                    0x30 => {
                        let ea = self.read_indexed_ea();
                        self.registers.x = ea;
                        // Z flag affected by LEAX/LEAY only
                        self.registers.cc.z = ea == 0;
                    },

                    // C++ Original: OpLEA<0, 0x31>(Y); - LEAY indexed  
                    // C++ Original: reg = EA; if (&reg == &X || &reg == &Y) { CC.Zero = (reg == 0); }
                    0x31 => {
                        let ea = self.read_indexed_ea();
                        self.registers.y = ea;
                        // Z flag affected by LEAX/LEAY only
                        self.registers.cc.z = ea == 0;
                    },

                    // C++ Original: OpLEA<0, 0x32>(S); - LEAS indexed
                    // C++ Original: reg = EA; Zero flag not affected by LEAU/LEAS
                    0x32 => {
                        let ea = self.read_indexed_ea();
                        self.registers.s = ea;
                        // Z flag NOT affected by LEAS/LEAU
                    },

                    // C++ Original: OpLEA<0, 0x33>(U); - LEAU indexed
                    // C++ Original: reg = EA; Zero flag not affected by LEAU/LEAS  
                    0x33 => {
                        let ea = self.read_indexed_ea();
                        self.registers.u = ea;
                        // Z flag NOT affected by LEAS/LEAU
                    },

                    // Branch opcodes - C++ Original: case 0x20-0x2F in CPU switch statement
                    // C++ Original: OpBranch(condFunc) - Branch opcodes with 8-bit relative offset

                    // C++ Original: case 0x20: OpBranch([] { return true; }); - BRA (branch always)
                    0x20 => {
                        let offset = self.read_relative_offset8();
                        // Always branch
                        self.registers.pc = ((self.registers.pc as i32) + (offset as i32)) as u16;
                    },

                    // C++ Original: case 0x21: OpBranch([] { return false; }); - BRN (branch never)
                    0x21 => {
                        let _offset = self.read_relative_offset8();
                        // Never branch - just consume the offset byte
                    },

                    // C++ Original: case 0x22: OpBranch([this] { return (CC.Carry | CC.Zero) == 0; }); - BHI (branch if higher)
                    0x22 => {
                        let offset = self.read_relative_offset8();
                        if !self.registers.cc.c && !self.registers.cc.z {
                            self.registers.pc = ((self.registers.pc as i32) + (offset as i32)) as u16;
                        }
                    },

                    // C++ Original: case 0x23: OpBranch([this] { return (CC.Carry | CC.Zero) != 0; }); - BLS (branch if lower or same)  
                    0x23 => {
                        let offset = self.read_relative_offset8();
                        if self.registers.cc.c || self.registers.cc.z {
                            self.registers.pc = ((self.registers.pc as i32) + (offset as i32)) as u16;
                        }
                    },

                    // C++ Original: case 0x24: OpBranch([this] { return CC.Carry == 0; }); - BCC (branch if carry clear)
                    0x24 => {
                        let offset = self.read_relative_offset8();
                        if !self.registers.cc.c {
                            self.registers.pc = ((self.registers.pc as i32) + (offset as i32)) as u16;
                        }
                    },

                    // C++ Original: case 0x25: OpBranch([this] { return CC.Carry != 0; }); - BCS (branch if carry set)
                    0x25 => {
                        let offset = self.read_relative_offset8();
                        if self.registers.cc.c {
                            self.registers.pc = ((self.registers.pc as i32) + (offset as i32)) as u16;
                        }
                    },

                    // C++ Original: case 0x26: OpBranch([this] { return CC.Zero == 0; }); - BNE (branch if not equal)
                    0x26 => {
                        let offset = self.read_relative_offset8();
                        if !self.registers.cc.z {
                            self.registers.pc = ((self.registers.pc as i32) + (offset as i32)) as u16;
                        }
                    },

                    // C++ Original: case 0x27: OpBranch([this] { return CC.Zero != 0; }); - BEQ (branch if equal)
                    0x27 => {
                        let offset = self.read_relative_offset8();
                        if self.registers.cc.z {
                            self.registers.pc = ((self.registers.pc as i32) + (offset as i32)) as u16;
                        }
                    },

                    // C++ Original: case 0x28: OpBranch([this] { return CC.Overflow == 0; }); - BVC (branch if overflow clear)
                    0x28 => {
                        let offset = self.read_relative_offset8();
                        if !self.registers.cc.v {
                            self.registers.pc = ((self.registers.pc as i32) + (offset as i32)) as u16;
                        }
                    },

                    // C++ Original: case 0x29: OpBranch([this] { return CC.Overflow != 0; }); - BVS (branch if overflow set)
                    0x29 => {
                        let offset = self.read_relative_offset8();
                        if self.registers.cc.v {
                            self.registers.pc = ((self.registers.pc as i32) + (offset as i32)) as u16;
                        }
                    },

                    // C++ Original: case 0x2A: OpBranch([this] { return CC.Negative == 0; }); - BPL (branch if plus)
                    0x2A => {
                        let offset = self.read_relative_offset8();
                        if !self.registers.cc.n {
                            self.registers.pc = ((self.registers.pc as i32) + (offset as i32)) as u16;
                        }
                    },

                    // C++ Original: case 0x2B: OpBranch([this] { return CC.Negative != 0; }); - BMI (branch if minus)
                    0x2B => {
                        let offset = self.read_relative_offset8();
                        if self.registers.cc.n {
                            self.registers.pc = ((self.registers.pc as i32) + (offset as i32)) as u16;
                        }
                    },

                    // C++ Original: case 0x2C: OpBranch([this] { return (CC.Negative ^ CC.Overflow) == 0; }); - BGE (branch if greater or equal)
                    0x2C => {
                        let offset = self.read_relative_offset8();
                        if self.registers.cc.n == self.registers.cc.v {
                            self.registers.pc = ((self.registers.pc as i32) + (offset as i32)) as u16;
                        }
                    },

                    // C++ Original: case 0x2D: OpBranch([this] { return (CC.Negative ^ CC.Overflow) != 0; }); - BLT (branch if less than)
                    0x2D => {
                        let offset = self.read_relative_offset8();
                        if self.registers.cc.n != self.registers.cc.v {
                            self.registers.pc = ((self.registers.pc as i32) + (offset as i32)) as u16;
                        }
                    },

                    // C++ Original: case 0x2E: OpBranch([this] { return (CC.Zero | (CC.Negative ^ CC.Overflow)) == 0; }); - BGT (branch if greater)
                    0x2E => {
                        let offset = self.read_relative_offset8();
                        if !self.registers.cc.z && (self.registers.cc.n == self.registers.cc.v) {
                            self.registers.pc = ((self.registers.pc as i32) + (offset as i32)) as u16;
                        }
                    },

                    // C++ Original: case 0x2F: OpBranch([this] { return (CC.Zero | (CC.Negative ^ CC.Overflow)) != 0; }); - BLE (branch if less or equal)
                    0x2F => {
                        let offset = self.read_relative_offset8();
                        if self.registers.cc.z || (self.registers.cc.n != self.registers.cc.v) {
                            self.registers.pc = ((self.registers.pc as i32) + (offset as i32)) as u16;
                        }
                    },

                    // Stack operations - C++ Original: case 0x34-0x37 in CPU switch statement
                    
                    // C++ Original: case 0x34: OpPSH<0, 0x34>(S); - PSHS (Push to system stack)
                    // C++ Original: OpPSH template pushes registers based on bit mask to given stack
                    0x34 => {
                        let mask = self.read_pc8();
                        self.op_pshs(mask);
                    },

                    // C++ Original: case 0x35: OpPUL<0, 0x35>(S); - PULS (Pull from system stack)  
                    // C++ Original: OpPUL template pulls registers in reverse order from given stack
                    0x35 => {
                        let mask = self.read_pc8();
                        self.op_puls(mask);
                    },

                    // C++ Original: case 0x36: OpPSH<0, 0x36>(U); - PSHU (Push to user stack)
                    // C++ Original: OpPSH template pushes registers based on bit mask to given stack
                    0x36 => {
                        let mask = self.read_pc8();
                        self.op_pshu(mask);
                    },

                    // C++ Original: case 0x37: OpPUL<0, 0x37>(U); - PULU (Pull from user stack)
                    // C++ Original: OpPUL template pulls registers in reverse order from given stack  
                    0x37 => {
                        let mask = self.read_pc8();
                        self.op_pulu(mask);
                    },

                    // C++ Original: case 0x39: OpRTS(); - RTS (Return from Subroutine)
                    // C++ Original: PC = Pop16(S);
                    0x39 => {
                        self.op_rts();
                    },

                    // Jump/Subroutine operations - C++ Original: case 0x8D/0x9D/0xAD/0xBD/0x17 in CPU switch statement

                    // C++ Original: case 0x16: OpLBRA(); - LBRA (Long Branch Always)
                    // C++ Original: int16_t offset = ReadRelativeOffset16(); PC += offset;
                    0x16 => {
                        self.op_lbra();
                    },

                    // C++ Original: case 0x17: OpLBSR(); - LBSR (Long Branch to Subroutine)
                    // C++ Original: int16_t offset = ReadRelativeOffset16(); Push16(S, PC); PC += offset;
                    0x17 => {
                        self.op_lbsr();
                    },

                    // C++ Original: case 0x8D: OpBSR(); - BSR (Branch to Subroutine)  
                    // C++ Original: int8_t offset = ReadRelativeOffset8(); Push16(S, PC); PC += offset;
                    0x8D => {
                        self.op_bsr();
                    },

                    // C++ Original: case 0x9D: OpJSR<0, 0x9D>(); - JSR Direct
                    // C++ Original: uint16_t EA = ReadEA16<AddressingMode::Direct>(); Push16(S, PC); PC = EA;
                    0x9D => {
                        self.op_jsr(opcode_byte);
                    },

                    // C++ Original: case 0xAD: OpJSR<0, 0xAD>(); - JSR Indexed
                    // C++ Original: uint16_t EA = ReadEA16<AddressingMode::Indexed>(); Push16(S, PC); PC = EA;
                    0xAD => {
                        self.op_jsr(opcode_byte);
                    },

                    // C++ Original: case 0xBD: OpJSR<0, 0xBD>(); - JSR Extended
                    // C++ Original: uint16_t EA = ReadEA16<AddressingMode::Extended>(); Push16(S, PC); PC = EA;
                    0xBD => {
                        self.op_jsr(opcode_byte);
                    },

                    // Transfer/Exchange operations - C++ Original: case 0x1E/0x1F in CPU switch statement

                    // C++ Original: case 0x1A: OpOR<0, 0x1A>(CC.Value); - ORCC (OR with Condition Codes)
                    // C++ Original: OR immediate value with CC register
                    0x1A => {
                        self.op_orcc();
                        // Cycles already counted by lookup table
                    },

                    // C++ Original: case 0x1C: OpAND<0, 0x1C>(CC.Value); - ANDCC (AND with Condition Codes)
                    // C++ Original: AND immediate value with CC register
                    0x1C => {
                        self.op_andcc();
                        // Cycles already counted by lookup table
                    },

                    // C++ Original: case 0x1D: OpSEX(); - SEX (Sign Extend)
                    // C++ Original: A = TestBits(B, BITS(7)) ? 0xFF : 0; CC.Negative = CalcNegative(D); CC.Zero = CalcZero(D);
                    0x1D => {
                        self.op_sex();
                        // Cycles already counted by lookup table
                    },

                    // C++ Original: case 0x1E: OpEXG(); - EXG (Exchange registers)
                    // C++ Original: ExchangeOrTransfer(true) - swap register contents
                    0x1E => {
                        self.op_exg();
                        // Cycles already counted by lookup table
                    },

                    // C++ Original: case 0x1F: OpTFR(); - TFR (Transfer registers)  
                    // C++ Original: ExchangeOrTransfer(false) - copy src to dst register
                    0x1F => {
                        self.op_tfr();
                        // Cycles already counted by lookup table
                    },

                    // Jump operations - C++ Original: OpJMP<0, opCode>()
                    
                    // C++ Original: case 0x0E: OpJMP<0, 0x0E>(); - JMP Direct
                    // C++ Original: uint16_t EA = ReadEA16<AddressingMode::Direct>(); PC = EA;
                    0x0E => {
                        self.op_jmp(opcode_byte);
                    },

                    // C++ Original: case 0x6E: OpJMP<0, 0x6E>(); - JMP Indexed  
                    // C++ Original: uint16_t EA = ReadEA16<AddressingMode::Indexed>(); PC = EA;
                    0x6E => {
                        self.op_jmp(opcode_byte);
                    },

                    // C++ Original: case 0x7E: OpJMP<0, 0x7E>(); - JMP Extended
                    // C++ Original: uint16_t EA = ReadEA16<AddressingMode::Extended>(); PC = EA;
                    0x7E => {
                        self.op_jmp(opcode_byte);
                    },

                    // System operations
                    
                    // C++ Original: case 0x3A: OpABX(); - ABX (Add B to X)
                    // C++ Original: X += B; - Simple unsigned addition, no flags affected
                    0x3A => {
                        self.op_abx();
                        // Cycles already counted by lookup table
                    },

                    // C++ Original: case 0x3C: OpCWAI<0, 0x3C>(); - CWAI (Clear and Wait for Interrupt)
                    // C++ Original: CC.Value = CC.Value & value; PushCCState(true); m_waitingForInterrupts = true;
                    0x3C => {
                        self.op_cwai();
                        // Cycles already counted by lookup table (20 cycles)
                    },

                    // C++ Original: case 0x3D: OpMUL<0, 0x3D>(); - MUL (Multiply A * B -> D)
                    // C++ Original: uint16_t result = A * B; CC.Zero = CalcZero(result); CC.Carry = TestBits01(result, BITS(7)); D = result;
                    0x3D => {
                        self.op_mul();
                    },

                    // RESET* - System reset instruction (undocumented)
                    0x3E => {
                        self.op_reset();
                    },

                    // C++ Original: case 0x3F: OpSWI(InterruptVector::Swi); - SWI (Software Interrupt)
                    // C++ Original: PushCCState(true); CC.InterruptMask = 1; CC.FastInterruptMask = 1; PC = Read16(InterruptVector::Swi);
                    0x3F => {
                        self.op_swi(); // Cycles already counted by lookup table
                    },

                    // C++ Original: case 0x3B: OpRTI<0, 0x3B>(); - RTI (Return from Interrupt)
                    // C++ Original: bool poppedEntire{}; PopCCState(poppedEntire); AddCycles(poppedEntire ? 15 : 6);
                    0x3B => {
                        self.op_rti();
                    },

                    _ => {
                        panic!("Unhandled opcode: {:02X} on page {}", opcode_byte, cpu_op_page);
                    }
                }
            },
            1 => {
                // Page 1 instructions (0x10xx)
                // C++ Original: switch (cpuOp.opCode) - Page 1 instructions
                match opcode_byte {
                    // C++ Original: OpCMP<1, 0x83>(D); - CMPD immediate
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0x83 => {
                        let operand = self.read_pc16();
                        let d_reg = combine_to_u16(self.registers.a, self.registers.b);
                        let _discard = self.subtract_impl_u16(d_reg, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpCMP<1, 0x93>(D); - CMPD direct
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0x93 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read16(ea);
                        let d_reg = combine_to_u16(self.registers.a, self.registers.b);
                        let _discard = self.subtract_impl_u16(d_reg, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpCMP<1, 0xA3>(D); - CMPD indexed
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0xA3 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read16(ea);
                        let d_reg = combine_to_u16(self.registers.a, self.registers.b);
                        let _discard = self.subtract_impl_u16(d_reg, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpCMP<1, 0xB3>(D); - CMPD extended  
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0xB3 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read16(ea);
                        let d_reg = combine_to_u16(self.registers.a, self.registers.b);
                        let _discard = self.subtract_impl_u16(d_reg, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpCMP<1, 0x8C>(Y); - CMPY immediate  
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0x8C => {
                        let operand = self.read_pc16();
                        let _discard = self.subtract_impl_u16(self.registers.y, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpCMP<1, 0x9C>(Y); - CMPY direct
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0x9C => {
                        let ea = self.read_direct_ea();
                        let operand = self.read16(ea);
                        let _discard = self.subtract_impl_u16(self.registers.y, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpCMP<1, 0xAC>(Y); - CMPY indexed
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0xAC => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read16(ea);
                        let _discard = self.subtract_impl_u16(self.registers.y, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpCMP<1, 0xBC>(Y); - CMPY extended
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0xBC => {
                        let ea = self.read_extended_ea();
                        let operand = self.read16(ea);
                        let _discard = self.subtract_impl_u16(self.registers.y, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: case 0x3F: OpSWI(InterruptVector::Swi2); - SWI2 (Software Interrupt 2)
                    // C++ Original: PushCCState(true); PC = Read16(InterruptVector::Swi2); (no interrupt mask change)
                    0x3F => {
                        self.op_swi2(); // Cycles already counted by lookup table
                    },

                    // Long Branch Operations - C++ Original: OpLongBranch(condFunc)
                    // C++ Original: case 0x21: OpLongBranch([] { return false; }); - LBRN (Long Branch Never)
                    0x21 => {
                        self.op_long_branch_never();
                    },

                    // C++ Original: case 0x22: OpLongBranch([this] { return (CC.Carry | CC.Zero) == 0; }); - LBHI (Long Branch if Higher)
                    0x22 => {
                        self.op_long_branch_if_higher();
                    },

                    // C++ Original: case 0x23: OpLongBranch([this] { return (CC.Carry | CC.Zero) != 0; }); - LBLS (Long Branch if Lower or Same)
                    0x23 => {
                        self.op_long_branch_if_lower_or_same();
                    },

                    // C++ Original: case 0x24: OpLongBranch([this] { return CC.Carry == 0; }); - LBCC (Long Branch if Carry Clear)
                    0x24 => {
                        self.op_long_branch_if_carry_clear();
                    },

                    // C++ Original: case 0x25: OpLongBranch([this] { return CC.Carry != 0; }); - LBCS (Long Branch if Carry Set)
                    0x25 => {
                        self.op_long_branch_if_carry_set();
                    },

                    // C++ Original: case 0x26: OpLongBranch([this] { return CC.Zero == 0; }); - LBNE (Long Branch if Not Equal)
                    0x26 => {
                        self.op_long_branch_if_not_equal();
                    },

                    // C++ Original: case 0x27: OpLongBranch([this] { return CC.Zero != 0; }); - LBEQ (Long Branch if Equal)
                    0x27 => {
                        self.op_long_branch_if_equal();
                    },

                    // C++ Original: case 0x28: OpLongBranch([this] { return CC.Overflow == 0; }); - LBVC (Long Branch if Overflow Clear)
                    0x28 => {
                        self.op_long_branch_if_overflow_clear();
                    },

                    // C++ Original: case 0x29: OpLongBranch([this] { return CC.Overflow != 0; }); - LBVS (Long Branch if Overflow Set)
                    0x29 => {
                        self.op_long_branch_if_overflow_set();
                    },

                    // C++ Original: case 0x2A: OpLongBranch([this] { return CC.Negative == 0; }); - LBPL (Long Branch if Plus)
                    0x2A => {
                        self.op_long_branch_if_plus();
                    },

                    // C++ Original: case 0x2B: OpLongBranch([this] { return CC.Negative != 0; }); - LBMI (Long Branch if Minus)
                    0x2B => {
                        self.op_long_branch_if_minus();
                    },

                    // C++ Original: case 0x2C: OpLongBranch([this] { return (CC.Negative ^ CC.Overflow) == 0; }); - LBGE (Long Branch if Greater or Equal)
                    0x2C => {
                        self.op_long_branch_if_greater_or_equal();
                    },

                    // C++ Original: case 0x2D: OpLongBranch([this] { return (CC.Negative ^ CC.Overflow) != 0; }); - LBLT (Long Branch if Less Than)
                    0x2D => {
                        self.op_long_branch_if_less_than();
                    },

                    // C++ Original: case 0x2E: OpLongBranch([this] { return (CC.Zero | (CC.Negative ^ CC.Overflow)) == 0; }); - LBGT (Long Branch if Greater)
                    0x2E => {
                        self.op_long_branch_if_greater();
                    },

                    // C++ Original: case 0x2F: OpLongBranch([this] { return (CC.Zero | (CC.Negative ^ CC.Overflow)) != 0; }); - LBLE (Long Branch if Less or Equal)
                    0x2F => {
                        self.op_long_branch_if_less_or_equal();
                    },

                    _ => {
                        panic!("Unhandled Page 1 opcode: {:02X}", opcode_byte);
                    }
                }
            },
            2 => {
                // Page 2 instructions (0x11xx)
                // C++ Original: switch (cpuOp.opCode) - Page 2 instructions
                match opcode_byte {
                    // C++ Original: OpCMP<2, 0x8C>(S); - CMPS immediate
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0x8C => {
                        let operand = self.read_pc16();
                        let _discard = self.subtract_impl_u16(self.registers.s, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpCMP<2, 0x9C>(S); - CMPS direct
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0x9C => {
                        let ea = self.read_direct_ea();
                        let operand = self.read16(ea);
                        let _discard = self.subtract_impl_u16(self.registers.s, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpCMP<2, 0xAC>(S); - CMPS indexed
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0xAC => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read16(ea);
                        let _discard = self.subtract_impl_u16(self.registers.s, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpCMP<2, 0xBC>(S); - CMPS extended
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0xBC => {
                        let ea = self.read_extended_ea();
                        let operand = self.read16(ea);
                        let _discard = self.subtract_impl_u16(self.registers.s, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpCMP<2, 0x83>(U); - CMPU immediate
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0x83 => {
                        let operand = self.read_pc16();
                        let _discard = self.subtract_impl_u16(self.registers.u, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpCMP<2, 0x93>(U); - CMPU direct
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0x93 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read16(ea);
                        let _discard = self.subtract_impl_u16(self.registers.u, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpCMP<2, 0xA3>(U); - CMPU indexed
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0xA3 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read16(ea);
                        let _discard = self.subtract_impl_u16(self.registers.u, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: OpCMP<2, 0xB3>(U); - CMPU extended
                    // C++ Original: uint16_t discard = SubtractImpl(reg, ReadOperandValue16<...>(), 0, CC); (void)discard;
                    0xB3 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read16(ea);
                        let _discard = self.subtract_impl_u16(self.registers.u, operand, 0);
                        // Note: CMP only updates flags, result is discarded
                    },

                    // C++ Original: case 0x3F: OpSWI(InterruptVector::Swi3); - SWI3 (Software Interrupt 3)
                    // C++ Original: PushCCState(true); PC = Read16(InterruptVector::Swi3); (no interrupt mask change)
                    0x3F => {
                        self.op_swi3(); // Cycles already counted by lookup table
                    },

                    _ => {
                        panic!("Unhandled Page 2 opcode: {:02X}", opcode_byte);
                    }
                }
            },
            _ => panic!("Invalid CPU op page: {}", cpu_op_page)
        }
    }

    // C++ Original: template <int page, uint8_t opCode> void OpLD(uint8_t& targetReg)
    fn op_ld_8(&mut self, opcode: u8) -> u8 {
        let value = self.read_operand_value8(opcode);
        self.registers.cc.n = self.calc_negative_8(value);
        self.registers.cc.z = self.calc_zero_8(value);
        self.registers.cc.v = false;
        value
    }

    // C++ Original: template <int page, uint8_t opCode> void OpLD(uint16_t& targetReg)
    fn op_ld_16(&mut self, opcode: u8) -> u16 {
        let value = self.read_operand_value16(opcode);
        self.registers.cc.n = self.calc_negative_16(value);
        self.registers.cc.z = self.calc_zero_16(value);
        self.registers.cc.v = false;
        value
    }

    // C++ Original: template <int page, uint8_t opCode> void OpST(const uint8_t& sourceReg)
    fn op_st_8(&mut self, source_value: u8, opcode: u8) {
        let ea = self.read_effective_address(opcode);
        self.write8(ea, source_value);
        self.registers.cc.n = self.calc_negative_8(source_value);
        self.registers.cc.z = self.calc_zero_8(source_value);
        self.registers.cc.v = false;
    }

    // C++ Original: template<int regIdx, OpCode opCode> void OpST(uint16_t sourceValue)
    fn op_st_16(&mut self, source_value: u16, opcode: u8) {
        let ea = self.read_effective_address(opcode);
        self.write16(ea, source_value);
        
        // Set condition codes - C++ Original: CalcNegative, CalcZero
        self.registers.cc.n = self.calc_negative_16(source_value);
        self.registers.cc.z = self.calc_zero_16(source_value);
        self.registers.cc.v = false;
    }

    // Helper function to read effective address for store operations
    fn read_effective_address(&mut self, opcode: u8) -> u16 {
        let cpu_op = lookup_cpu_op_runtime(0, opcode);
        match cpu_op.addr_mode {
            AddressingMode::Direct => self.read_direct_ea(),
            AddressingMode::Indexed => self.read_indexed_ea(),
            AddressingMode::Extended => self.read_extended_ea(),
            AddressingMode::Immediate => panic!("Store instructions don't use immediate addressing"),
            AddressingMode::Inherent => panic!("Store instructions don't use inherent addressing"),
            AddressingMode::Relative => panic!("Store instructions don't use relative addressing"),
            AddressingMode::Illegal => panic!("Illegal addressing mode for store instruction"),
            AddressingMode::Variant => panic!("Variant addressing mode not applicable for EA calculation"),
        }
    }

    // Special case for LDD (Load D register)
    fn op_ld_16_d(&mut self, opcode: u8) {
        let value = self.read_operand_value16(opcode);
        self.registers.cc.n = self.calc_negative_16(value);
        self.registers.cc.z = self.calc_zero_16(value);
        self.registers.cc.v = false;
        self.registers.set_d(value);
    }
    
    // Helper functions for reading operand values based on addressing mode
    fn read_operand_value8(&mut self, opcode: u8) -> u8 {
        let addressing_mode = self.get_addressing_mode_for_opcode(opcode);
        match addressing_mode {
            AddressingMode::Immediate => self.read_pc8(),
            AddressingMode::Direct => {
                let ea = self.read_direct_ea();
                self.read8(ea)
            },
            AddressingMode::Indexed => {
                let ea = self.read_indexed_ea();
                self.read8(ea)
            },
            AddressingMode::Extended => {
                let ea = self.read_extended_ea();
                self.read8(ea)
            },
            _ => panic!("Invalid addressing mode for 8-bit read: {:?}", addressing_mode)
        }
    }

    fn read_operand_value16(&mut self, opcode: u8) -> u16 {
        let addressing_mode = self.get_addressing_mode_for_opcode(opcode);
        match addressing_mode {
            AddressingMode::Immediate => self.read_pc16(),
            AddressingMode::Direct => {
                let ea = self.read_direct_ea();
                self.read16(ea)
            },
            AddressingMode::Indexed => {
                let ea = self.read_indexed_ea();
                self.read16(ea)
            },
            AddressingMode::Extended => {
                let ea = self.read_extended_ea();
                self.read16(ea)
            },
            _ => panic!("Invalid addressing mode for 16-bit read: {:?}", addressing_mode)
        }
    }

    // Helper functions for condition codes - C++ Original: CalcNegative, CalcZero
    fn calc_negative_8(&self, value: u8) -> bool {
        (value & 0x80) != 0
    }

    fn calc_negative_16(&self, value: u16) -> bool {
        (value & 0x8000) != 0
    }

    fn calc_zero_8(&self, value: u8) -> bool {
        value == 0
    }

    fn calc_zero_16(&self, value: u16) -> bool {
        value == 0
    }

    // Helper to get addressing mode for opcode (simplified version)
    fn get_addressing_mode_for_opcode(&self, opcode: u8) -> AddressingMode {
        match opcode {
            // Immediate addressing
            0x86 | 0xC6 | 0x8E | 0xCC | 0xCE => AddressingMode::Immediate,
            // Direct addressing  
            0x96 | 0xD6 | 0x9E | 0xDC | 0xDE | 0x97 | 0xD7 | 0x9F | 0xDD | 0xDF => AddressingMode::Direct,
            // Indexed addressing
            0xA6 | 0xE6 | 0xAE | 0xEC | 0xEE | 0xA7 | 0xE7 | 0xAF | 0xED | 0xEF => AddressingMode::Indexed,
            // Extended addressing
            0xB6 | 0xF6 | 0xBE | 0xFC | 0xFE | 0xB7 | 0xF7 | 0xBF | 0xFD | 0xFF => AddressingMode::Extended,
            _ => panic!("Unknown addressing mode for opcode: {:02X}", opcode)
        }
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
        combine_to_u16(high, low)
    }

    pub fn write8(&mut self, address: u16, value: u8) {
        self.memory_bus.borrow_mut().write(address, value);
    }

    pub fn write16(&mut self, address: u16, value: u16) {
        let high = (value >> 8) as u8;
        let low = value as u8;
        self.memory_bus.borrow_mut().write(address, high);
        self.memory_bus.borrow_mut().write(address.wrapping_add(1), low);
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

    /* C++ Original:
    void Push8(uint16_t& stackPointer, uint8_t value) { 
        m_memoryBus->Write(--stackPointer, value); 
    }
    TODO: Will be used in PSHS/PSHU opcodes, SWI/SWI2/SWI3 interrupts
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
    TODO: Will be used in PULS/PULU opcodes, interrupt handling
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
    TODO: Will be used in PSHS/PSHU opcodes, SWI/SWI2/SWI3 interrupts
    */
    fn push16(&mut self, stack_pointer: &mut u16, value: u16) {
        self.push8(stack_pointer, u8(value & 0xFF)); // Low
        self.push8(stack_pointer, u8(value >> 8));   // High
    }

    /* C++ Original:
    uint16_t Pop16(uint16_t& stackPointer) {
        auto high = m_memoryBus->Read(stackPointer++);
        auto low = m_memoryBus->Read(stackPointer++);
        return CombineToU16(high, low);
    }
    TODO: Will be used in PULS/PULU opcodes, interrupt handling
    */
    fn pop16(&mut self, stack_pointer: &mut u16) -> u16 {
        let high = self.pop8(stack_pointer);
        let low = self.pop8(stack_pointer);
        combine_to_u16(high, low)
    }

    /* C++ Original: case 0x34: OpPSH<0, 0x34>(S); - PSHS (Push to system stack)
    Push registers in order: PC, U, Y, X, DP, B, A, CC based on bit mask
    */
    /* C++ Original: template <int page, uint8_t opCode> void OpPSH(uint16_t& stackReg)
    Push16: Write(--stackPointer, Low); Write(--stackPointer, High) - Low first, High second
    Push order: PC(bit7), U(bit6), Y(bit5), X(bit4), DP(bit3), B(bit2), A(bit1), CC(bit0)
    */
    fn op_pshs(&mut self, mask: u8) {
        // C++ Original 1:1: if (value & BITS(7)) Push16(stackReg, PC);
        if mask & 0x80 != 0 { // Bit 7: PC
            let pc_val = self.registers.pc;
            // C++ Push16: Write(--stackPointer, Low); Write(--stackPointer, High);
            self.registers.s = self.registers.s.wrapping_sub(1);
            self.write8(self.registers.s, (pc_val & 0xFF) as u8);
            self.registers.s = self.registers.s.wrapping_sub(1);
            self.write8(self.registers.s, (pc_val >> 8) as u8);
        }
        // C++ Original 1:1: if (value & BITS(6)) Push16(stackReg, otherStackReg);
        if mask & 0x40 != 0 { // Bit 6: U (other stack register)
            let u_val = self.registers.u;
            // C++ Push16: Write(--stackPointer, Low); Write(--stackPointer, High);
            self.registers.s = self.registers.s.wrapping_sub(1);
            self.write8(self.registers.s, (u_val & 0xFF) as u8);
            self.registers.s = self.registers.s.wrapping_sub(1);
            self.write8(self.registers.s, (u_val >> 8) as u8);
        }
        // C++ Original 1:1: if (value & BITS(5)) Push16(stackReg, Y);
        if mask & 0x20 != 0 { // Bit 5: Y
            let y_val = self.registers.y;
            // C++ Push16: Write(--stackPointer, Low); Write(--stackPointer, High);
            self.registers.s = self.registers.s.wrapping_sub(1);
            self.write8(self.registers.s, (y_val & 0xFF) as u8);
            self.registers.s = self.registers.s.wrapping_sub(1);
            self.write8(self.registers.s, (y_val >> 8) as u8);
        }
        // C++ Original 1:1: if (value & BITS(4)) Push16(stackReg, X);
        if mask & 0x10 != 0 { // Bit 4: X
            let x_val = self.registers.x;
            // C++ Push16: Write(--stackPointer, Low); Write(--stackPointer, High);
            self.registers.s = self.registers.s.wrapping_sub(1);
            self.write8(self.registers.s, (x_val & 0xFF) as u8);
            self.registers.s = self.registers.s.wrapping_sub(1);
            self.write8(self.registers.s, (x_val >> 8) as u8);
        }
        // C++ Original 1:1: if (value & BITS(3)) Push8(stackReg, DP);
        if mask & 0x08 != 0 { // Bit 3: DP
            let dp_val = self.registers.dp;
            // C++ Push8: Write(--stackPointer, value);
            self.registers.s = self.registers.s.wrapping_sub(1);
            self.write8(self.registers.s, dp_val);
        }
        // C++ Original 1:1: if (value & BITS(2)) Push8(stackReg, B);
        if mask & 0x04 != 0 { // Bit 2: B
            let b_val = self.registers.b;
            // C++ Push8: Write(--stackPointer, value);
            self.registers.s = self.registers.s.wrapping_sub(1);
            self.write8(self.registers.s, b_val);
        }
        // C++ Original 1:1: if (value & BITS(1)) Push8(stackReg, A);
        if mask & 0x02 != 0 { // Bit 1: A
            let a_val = self.registers.a;
            // C++ Push8: Write(--stackPointer, value);
            self.registers.s = self.registers.s.wrapping_sub(1);
            self.write8(self.registers.s, a_val);
        }
        // C++ Original 1:1: if (value & BITS(0)) Push8(stackReg, CC.Value);
        if mask & 0x01 != 0 { // Bit 0: CC
            let cc_val = self.registers.cc.to_u8();
            // C++ Push8: Write(--stackPointer, value);
            self.registers.s = self.registers.s.wrapping_sub(1);
            self.write8(self.registers.s, cc_val);
        }
    }

    /* C++ Original: template <int page, uint8_t opCode> void OpPUL(uint16_t& stackReg)
    Pop16: high = Read(stackPointer++); low = Read(stackPointer++); return (high<<8)|low
    Pull order: CC(bit0), A(bit1), B(bit2), DP(bit3), X(bit4), Y(bit5), U(bit6), PC(bit7)
    */
    fn op_puls(&mut self, mask: u8) {
        // C++ Original 1:1: if (value & BITS(0)) CC.Value = Pop8(stackReg);
        if mask & 0x01 != 0 { // Bit 0: CC
            // C++ Pop8: value = Read(stackPointer++);
            let cc_val = self.read8(self.registers.s);
            self.registers.s = self.registers.s.wrapping_add(1);
            self.registers.cc.from_u8(cc_val);
        }
        // C++ Original 1:1: if (value & BITS(1)) A = Pop8(stackReg);
        if mask & 0x02 != 0 { // Bit 1: A
            self.registers.a = self.read8(self.registers.s);
            self.registers.s = self.registers.s.wrapping_add(1);
        }
        // C++ Original 1:1: if (value & BITS(2)) B = Pop8(stackReg);
        if mask & 0x04 != 0 { // Bit 2: B
            self.registers.b = self.read8(self.registers.s);
            self.registers.s = self.registers.s.wrapping_add(1);
        }
        // C++ Original 1:1: if (value & BITS(3)) DP = Pop8(stackReg);
        if mask & 0x08 != 0 { // Bit 3: DP
            self.registers.dp = self.read8(self.registers.s);
            self.registers.s = self.registers.s.wrapping_add(1);
        }
        // C++ Original 1:1: if (value & BITS(4)) X = Pop16(stackReg);
        if mask & 0x10 != 0 { // Bit 4: X
            // C++ Pop16: high = Read(stackPointer++); low = Read(stackPointer++);
            let high = self.read8(self.registers.s);
            self.registers.s = self.registers.s.wrapping_add(1);
            let low = self.read8(self.registers.s);
            self.registers.s = self.registers.s.wrapping_add(1);
            self.registers.x = ((high as u16) << 8) | (low as u16);
        }
        // C++ Original 1:1: if (value & BITS(5)) Y = Pop16(stackReg);
        if mask & 0x20 != 0 { // Bit 5: Y
            // C++ Pop16: high = Read(stackPointer++); low = Read(stackPointer++);
            let high = self.read8(self.registers.s);
            self.registers.s = self.registers.s.wrapping_add(1);
            let low = self.read8(self.registers.s);
            self.registers.s = self.registers.s.wrapping_add(1);
            self.registers.y = ((high as u16) << 8) | (low as u16);
        }
        // C++ Original 1:1: if (value & BITS(6)) otherStackReg = Pop16(stackReg);
        if mask & 0x40 != 0 { // Bit 6: U (other stack register)
            // C++ Pop16: high = Read(stackPointer++); low = Read(stackPointer++);
            let high = self.read8(self.registers.s);
            self.registers.s = self.registers.s.wrapping_add(1);
            let low = self.read8(self.registers.s);
            self.registers.s = self.registers.s.wrapping_add(1);
            self.registers.u = ((high as u16) << 8) | (low as u16);
        }
        // C++ Original 1:1: if (value & BITS(7)) PC = Pop16(stackReg);
        if mask & 0x80 != 0 { // Bit 7: PC
            // C++ Pop16: high = Read(stackPointer++); low = Read(stackPointer++);
            let high = self.read8(self.registers.s);
            self.registers.s = self.registers.s.wrapping_add(1);
            let low = self.read8(self.registers.s);
            self.registers.s = self.registers.s.wrapping_add(1);
            self.registers.pc = ((high as u16) << 8) | (low as u16);
        }
    }

    /* C++ Original: template <int page, uint8_t opCode> void OpPSH(uint16_t& stackReg) - U stack version
    Push16: Write(--stackPointer, Low); Write(--stackPointer, High) - Low first, High second
    Push order: PC(bit7), S(bit6), Y(bit5), X(bit4), DP(bit3), B(bit2), A(bit1), CC(bit0)
    */
    fn op_pshu(&mut self, mask: u8) {
        // C++ Original 1:1: if (value & BITS(7)) Push16(stackReg, PC);
        if mask & 0x80 != 0 { // Bit 7: PC
            let pc_val = self.registers.pc;
            // C++ Push16: Write(--stackPointer, Low); Write(--stackPointer, High);
            self.registers.u = self.registers.u.wrapping_sub(1);
            self.write8(self.registers.u, (pc_val & 0xFF) as u8);
            self.registers.u = self.registers.u.wrapping_sub(1);
            self.write8(self.registers.u, (pc_val >> 8) as u8);
        }
        // C++ Original 1:1: if (value & BITS(6)) Push16(stackReg, otherStackReg); (S for PSHU)  
        if mask & 0x40 != 0 { // Bit 6: S (other stack register)
            let s_val = self.registers.s;
            // C++ Push16: Write(--stackPointer, Low); Write(--stackPointer, High);
            self.registers.u = self.registers.u.wrapping_sub(1);
            self.write8(self.registers.u, (s_val & 0xFF) as u8);
            self.registers.u = self.registers.u.wrapping_sub(1);
            self.write8(self.registers.u, (s_val >> 8) as u8);
        }
        // C++ Original 1:1: if (value & BITS(5)) Push16(stackReg, Y);
        if mask & 0x20 != 0 { // Bit 5: Y
            let y_val = self.registers.y;
            // C++ Push16: Write(--stackPointer, Low); Write(--stackPointer, High);
            self.registers.u = self.registers.u.wrapping_sub(1);
            self.write8(self.registers.u, (y_val & 0xFF) as u8);
            self.registers.u = self.registers.u.wrapping_sub(1);
            self.write8(self.registers.u, (y_val >> 8) as u8);
        }
        // C++ Original 1:1: if (value & BITS(4)) Push16(stackReg, X);
        if mask & 0x10 != 0 { // Bit 4: X
            let x_val = self.registers.x;
            // C++ Push16: Write(--stackPointer, Low); Write(--stackPointer, High);
            self.registers.u = self.registers.u.wrapping_sub(1);
            self.write8(self.registers.u, (x_val & 0xFF) as u8);
            self.registers.u = self.registers.u.wrapping_sub(1);
            self.write8(self.registers.u, (x_val >> 8) as u8);
        }
        // C++ Original 1:1: if (value & BITS(3)) Push8(stackReg, DP);
        if mask & 0x08 != 0 { // Bit 3: DP
            let dp_val = self.registers.dp;
            // C++ Push8: Write(--stackPointer, value);
            self.registers.u = self.registers.u.wrapping_sub(1);
            self.write8(self.registers.u, dp_val);
        }
        // C++ Original 1:1: if (value & BITS(2)) Push8(stackReg, B);
        if mask & 0x04 != 0 { // Bit 2: B
            let b_val = self.registers.b;
            // C++ Push8: Write(--stackPointer, value);
            self.registers.u = self.registers.u.wrapping_sub(1);
            self.write8(self.registers.u, b_val);
        }
        // C++ Original 1:1: if (value & BITS(1)) Push8(stackReg, A);
        if mask & 0x02 != 0 { // Bit 1: A
            let a_val = self.registers.a;
            // C++ Push8: Write(--stackPointer, value);
            self.registers.u = self.registers.u.wrapping_sub(1);
            self.write8(self.registers.u, a_val);
        }
        // C++ Original 1:1: if (value & BITS(0)) Push8(stackReg, CC.Value);
        if mask & 0x01 != 0 { // Bit 0: CC
            let cc_val = self.registers.cc.to_u8();
            // C++ Push8: Write(--stackPointer, value);
            self.registers.u = self.registers.u.wrapping_sub(1);
            self.write8(self.registers.u, cc_val);
        }
    }

    /* C++ Original: template <int page, uint8_t opCode> void OpPUL(uint16_t& stackReg) - U stack version
    Pop16: high = Read(stackPointer++); low = Read(stackPointer++); return (high<<8)|low
    Pull order: CC(bit0), A(bit1), B(bit2), DP(bit3), X(bit4), Y(bit5), S(bit6), PC(bit7)
    */
    fn op_pulu(&mut self, mask: u8) {
        // C++ Original 1:1: if (value & BITS(0)) CC.Value = Pop8(stackReg);
        if mask & 0x01 != 0 { // Bit 0: CC
            // C++ Pop8: value = Read(stackPointer++);
            let cc_val = self.read8(self.registers.u);
            self.registers.u = self.registers.u.wrapping_add(1);
            self.registers.cc.from_u8(cc_val);
        }
        // C++ Original 1:1: if (value & BITS(1)) A = Pop8(stackReg);
        if mask & 0x02 != 0 { // Bit 1: A
            self.registers.a = self.read8(self.registers.u);
            self.registers.u = self.registers.u.wrapping_add(1);
        }
        // C++ Original 1:1: if (value & BITS(2)) B = Pop8(stackReg);
        if mask & 0x04 != 0 { // Bit 2: B
            self.registers.b = self.read8(self.registers.u);
            self.registers.u = self.registers.u.wrapping_add(1);
        }
        // C++ Original 1:1: if (value & BITS(3)) DP = Pop8(stackReg);
        if mask & 0x08 != 0 { // Bit 3: DP
            self.registers.dp = self.read8(self.registers.u);
            self.registers.u = self.registers.u.wrapping_add(1);
        }
        // C++ Original 1:1: if (value & BITS(4)) X = Pop16(stackReg);
        if mask & 0x10 != 0 { // Bit 4: X
            // C++ Pop16: high = Read(stackPointer++); low = Read(stackPointer++);
            let high = self.read8(self.registers.u);
            self.registers.u = self.registers.u.wrapping_add(1);
            let low = self.read8(self.registers.u);
            self.registers.u = self.registers.u.wrapping_add(1);
            self.registers.x = ((high as u16) << 8) | (low as u16);
        }
        // C++ Original 1:1: if (value & BITS(5)) Y = Pop16(stackReg);
        if mask & 0x20 != 0 { // Bit 5: Y
            // C++ Pop16: high = Read(stackPointer++); low = Read(stackPointer++);
            let high = self.read8(self.registers.u);
            self.registers.u = self.registers.u.wrapping_add(1);
            let low = self.read8(self.registers.u);
            self.registers.u = self.registers.u.wrapping_add(1);
            self.registers.y = ((high as u16) << 8) | (low as u16);
        }
        // C++ Original 1:1: if (value & BITS(6)) otherStackReg = Pop16(stackReg); (S for PULU)
        if mask & 0x40 != 0 { // Bit 6: S (other stack register)
            // C++ Pop16: high = Read(stackPointer++); low = Read(stackPointer++);
            let high = self.read8(self.registers.u);
            self.registers.u = self.registers.u.wrapping_add(1);
            let low = self.read8(self.registers.u);
            self.registers.u = self.registers.u.wrapping_add(1);
            self.registers.s = ((high as u16) << 8) | (low as u16);
        }
        // C++ Original 1:1: if (value & BITS(7)) PC = Pop16(stackReg);
        if mask & 0x80 != 0 { // Bit 7: PC
            // C++ Pop16: high = Read(stackPointer++); low = Read(stackPointer++);
            let high = self.read8(self.registers.u);
            self.registers.u = self.registers.u.wrapping_add(1);
            let low = self.read8(self.registers.u);
            self.registers.u = self.registers.u.wrapping_add(1);
            self.registers.pc = ((high as u16) << 8) | (low as u16);
        }
    }

    // C++ Original: void ExchangeOrTransfer(bool exchange)
    fn exchange_or_transfer(&mut self, exchange: bool) {
        let postbyte = self.read_pc8();
        
        // C++ Original: ASSERT(!!(postbyte & BITS(3)) == !!(postbyte & BITS(7)));
        // 8-bit to 8-bit or 16-bit to 16-bit only
        let bit3_set = (postbyte & 0x08) != 0;
        let bit7_set = (postbyte & 0x80) != 0;
        assert_eq!(bit3_set, bit7_set, "TFR/EXG: 8-bit to 8-bit or 16-bit to 16-bit only");

        let src = (postbyte >> 4) & 0b111;
        let dst = postbyte & 0b111;

        if postbyte & 0x08 != 0 {
            // 8-bit transfer
            // C++ Original: ASSERT(src < 4 && dst < 4); // Only first 4 are valid 8-bit register indices
            assert!(src < 4 && dst < 4, "TFR/EXG: Invalid 8-bit register index");
            
            // C++ Original: uint8_t* const reg[]{&A, &B, &CC.Value, &DP};
            let src_val = match src {
                0 => self.registers.a,
                1 => self.registers.b,
                2 => self.registers.cc.to_u8(),
                3 => self.registers.dp,
                _ => unreachable!()
            };
            
            let dst_val = match dst {
                0 => self.registers.a,
                1 => self.registers.b,
                2 => self.registers.cc.to_u8(),
                3 => self.registers.dp,
                _ => unreachable!()
            };

            if exchange {
                // Swap values
                match src {
                    0 => self.registers.a = dst_val,
                    1 => self.registers.b = dst_val,
                    2 => self.registers.cc.from_u8(dst_val),
                    3 => self.registers.dp = dst_val,
                    _ => unreachable!()
                }
            }

            // Set destination
            match dst {
                0 => self.registers.a = src_val,
                1 => self.registers.b = src_val,
                2 => self.registers.cc.from_u8(src_val),
                3 => self.registers.dp = src_val,
                _ => unreachable!()
            }

        } else {
            // 16-bit transfer
            // C++ Original: ASSERT(src < 6 && dst < 6); // Only first 6 are valid 16-bit register indices
            assert!(src < 6 && dst < 6, "TFR/EXG: Invalid 16-bit register index");
            
            // C++ Original: uint16_t* const reg[]{&D, &X, &Y, &U, &S, &PC};
            let src_val = match src {
                0 => self.registers.d(), // D = A:B
                1 => self.registers.x,
                2 => self.registers.y,
                3 => self.registers.u,
                4 => self.registers.s,
                5 => self.registers.pc,
                _ => unreachable!()
            };
            
            let dst_val = match dst {
                0 => self.registers.d(), // D = A:B
                1 => self.registers.x,
                2 => self.registers.y,
                3 => self.registers.u,
                4 => self.registers.s,
                5 => self.registers.pc,
                _ => unreachable!()
            };

            if exchange {
                // Swap values
                match src {
                    0 => self.registers.set_d(dst_val), // D = A:B
                    1 => self.registers.x = dst_val,
                    2 => self.registers.y = dst_val,
                    3 => self.registers.u = dst_val,
                    4 => self.registers.s = dst_val,
                    5 => self.registers.pc = dst_val,
                    _ => unreachable!()
                }
            }

            // Set destination
            match dst {
                0 => self.registers.set_d(src_val), // D = A:B
                1 => self.registers.x = src_val,
                2 => self.registers.y = src_val,
                3 => self.registers.u = src_val,
                4 => self.registers.s = src_val,
                5 => self.registers.pc = src_val,
                _ => unreachable!()
            }
        }
    }

    // C++ Original: void OpEXG() { ExchangeOrTransfer(true); }
    fn op_exg(&mut self) {
        self.exchange_or_transfer(true);
    }

    // C++ Original: void OpTFR() { ExchangeOrTransfer(false); }
    fn op_tfr(&mut self) {
        self.exchange_or_transfer(false);
    }

    // C++ Original: void OpSEX() { A = TestBits(B, BITS(7)) ? 0xFF : 0; CC.Negative = CalcNegative(D); CC.Zero = CalcZero(D); }
    fn op_sex(&mut self) {
        // C++ Original: A = TestBits(B, BITS(7)) ? 0xFF : 0;
        self.registers.a = if (self.registers.b & 0x80) != 0 { 0xFF } else { 0x00 };
        
        // C++ Original: CC.Negative = CalcNegative(D); CC.Zero = CalcZero(D);
        let d_value = self.registers.d();
        self.registers.cc.n = Self::calc_negative_u16(d_value);
        self.registers.cc.z = Self::calc_zero_u16(d_value);
    }

    // C++ Original: void OpABX() { X += B; }
    fn op_abx(&mut self) {
        // C++ Original: X += B; - Simple unsigned addition, no flags affected
        self.registers.x = self.registers.x.wrapping_add(self.registers.b as u16);
    }

    // C++ Original: template <int page, uint8_t opCode> void OpJSR()
    // C++ Original: uint16_t EA = ReadEA16<LookupCpuOp(page, opCode).addrMode>(); Push16(S, PC); PC = EA;
    fn op_jsr(&mut self, opcode_byte: u8) {
        let ea = self.read_effective_address(opcode_byte);
        let pc_value = self.registers.pc;
        // Push return address to system stack - inline Push16(S, PC)
        // C++ Original: m_memoryBus->Write(--stackPointer, U8(value & 0xFF)); // Low
        // C++ Original: m_memoryBus->Write(--stackPointer, U8(value >> 8));   // High
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, (pc_value & 0xFF) as u8); // Low byte first
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, (pc_value >> 8) as u8);   // High byte second
        // Jump to effective address
        self.registers.pc = ea;
    }

    // C++ Original: void OpBSR()
    // C++ Original: int8_t offset = ReadRelativeOffset8(); Push16(S, PC); PC += offset;
    fn op_bsr(&mut self) {
        let offset = self.read_relative_offset8();
        let pc_value = self.registers.pc;
        // Push return address to system stack - inline Push16(S, PC)
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, (pc_value & 0xFF) as u8); // Low byte first
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, (pc_value >> 8) as u8);   // High byte second
        // Branch by adding offset to PC
        self.registers.pc = ((self.registers.pc as i32) + (offset as i32)) as u16;
    }

    // C++ Original: void OpLBSR()
    // C++ Original: int16_t offset = ReadRelativeOffset16(); Push16(S, PC); PC += offset;
    fn op_lbsr(&mut self) {
        let offset = self.read_relative_offset16();
        let pc_value = self.registers.pc;
        // Push return address to system stack - inline Push16(S, PC)
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, (pc_value & 0xFF) as u8); // Low byte first
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, (pc_value >> 8) as u8);   // High byte second
        // Branch by adding 16-bit offset to PC
        self.registers.pc = ((self.registers.pc as i32) + (offset as i32)) as u16;
    }

    // C++ Original: void OpRTS()
    // C++ Original: PC = Pop16(S);
    fn op_rts(&mut self) {
        // Pop return address from system stack - inline Pop16(S)
        let high_byte = self.read8(self.registers.s) as u16;
        self.registers.s = self.registers.s.wrapping_add(1);
        let low_byte = self.read8(self.registers.s) as u16;
        self.registers.s = self.registers.s.wrapping_add(1);
        // Combine bytes and set PC
        self.registers.pc = (high_byte << 8) | low_byte;
    }

    // C++ Original: static uint8_t AddImpl(uint8_t a, uint8_t b, uint8_t carry, ConditionCode& CC)
    fn add_impl_u8(&mut self, a: u8, b: u8, carry: u8) -> u8 {
        let r16 = (a as u16) + (b as u16) + (carry as u16);
        
        // CC.HalfCarry = CalcHalfCarryFromAdd(a, b, carry);
        self.registers.cc.h = Self::calc_half_carry_from_add(a, b, carry);
        
        // CC.Carry = CalcCarry(r16);
        self.registers.cc.c = Self::calc_carry_u16(r16);
        
        // CC.Overflow = CalcOverflow(a, b, r16);
        self.registers.cc.v = Self::calc_overflow_u8(a, b, r16);
        
        let r8 = r16 as u8;
        
        // CC.Zero = CalcZero(r8);
        self.registers.cc.z = Self::calc_zero_u8(r8);
        
        // CC.Negative = CalcNegative(r8);
        self.registers.cc.n = Self::calc_negative_u8(r8);
        
        r8
    }

    // C++ Original: static uint16_t AddImpl(uint16_t a, uint16_t b, uint16_t carry, ConditionCode& CC)
    fn add_impl_u16(&mut self, a: u16, b: u16, carry: u16) -> u16 {
        let r32 = (a as u32) + (b as u32) + (carry as u32);
        
        // No HalfCarry for 16-bit operations in Vectrexy
        
        // CC.Carry = CalcCarry(r32);
        self.registers.cc.c = Self::calc_carry_u32(r32);
        
        // CC.Overflow = CalcOverflow(a, b, r32);
        self.registers.cc.v = Self::calc_overflow_u16(a, b, r32);
        
        let r16 = r32 as u16;
        
        // CC.Zero = CalcZero(r16);
        self.registers.cc.z = Self::calc_zero_u16(r16);
        
        // CC.Negative = CalcNegative(r16);
        self.registers.cc.n = Self::calc_negative_u16(r16);
        
        r16
    }

    // C++ Original: static uint8_t SubtractImpl(uint8_t a, uint8_t b, uint8_t carry, ConditionCode& CC)
    fn subtract_impl_u8(&mut self, a: u8, b: u8, carry: u8) -> u8 {
        let result = self.add_impl_u8(a, !b, 1 - carry);
        // CC.Carry = !CC.Carry; // Carry is set if no borrow occurs
        self.registers.cc.c = !self.registers.cc.c;
        result
    }

    // C++ Original: static uint16_t SubtractImpl(uint16_t a, uint16_t b, uint16_t carry, ConditionCode& CC)
    fn subtract_impl_u16(&mut self, a: u16, b: u16, carry: u16) -> u16 {
        let result = self.add_impl_u16(a, !b, 1 - carry);
        // CC.Carry = !CC.Carry; // Carry is set if no borrow occurs
        self.registers.cc.c = !self.registers.cc.c;
        result
    }

    // C++ Original: constexpr uint8_t CalcHalfCarryFromAdd(uint8_t a, uint8_t b, uint8_t carry) { return (((a & 0x0F) + (b & 0x0F) + carry) & 0x10) != 0; }
    fn calc_half_carry_from_add(a: u8, b: u8, carry: u8) -> bool {
        (((a & 0x0F) + (b & 0x0F) + carry) & 0x10) != 0
    }

    // C++ Original: constexpr uint8_t CalcCarry(uint16_t r) { return (r & 0xFF00) != 0; }
    fn calc_carry_u16(value: u16) -> bool {
        (value & 0xFF00) != 0
    }

    // C++ Original: constexpr uint8_t CalcCarry(uint32_t r) { return (r & 0xFFFF'0000) != 0; }
    fn calc_carry_u32(value: u32) -> bool {
        (value & 0xFFFF0000) != 0
    }

    // C++ Original: constexpr uint8_t CalcOverflow(uint8_t a, uint8_t b, uint16_t r) { return (((uint16_t)a ^ r) & ((uint16_t)b ^ r) & BITS(7)) != 0; }
    fn calc_overflow_u8(a: u8, b: u8, result: u16) -> bool {
        (((a as u16) ^ result) & ((b as u16) ^ result) & 0x80) != 0
    }

    // C++ Original: constexpr uint8_t CalcOverflow(uint16_t a, uint16_t b, uint32_t r) { return (((uint32_t)a ^ r) & ((uint32_t)b ^ r) & BITS(15)) != 0; }
    fn calc_overflow_u16(a: u16, b: u16, result: u32) -> bool {
        (((a as u32) ^ result) & ((b as u32) ^ result) & 0x8000) != 0
    }

    fn calc_zero_u8(value: u8) -> bool {
        value == 0
    }

    fn calc_zero_u16(value: u16) -> bool {
        value == 0
    }

    fn calc_negative_u8(value: u8) -> bool {
        (value & 0x80) != 0
    }

    fn calc_negative_u16(value: u16) -> bool {
        (value & 0x8000) != 0
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
        let ea = combine_to_u16(dp, pc_byte);
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
        let ea = combine_to_u16(msb, lsb);
        ea
    }

    /* C++ Original:
    // Read CPU op's relative offset from next 8/16 bits
    int8_t ReadRelativeOffset8() { return static_cast<int8_t>(ReadPC8()); }
    int16_t ReadRelativeOffset16() { return static_cast<int16_t>(ReadPC16()); }
    */
    // C++ Original: int8_t ReadRelativeOffset8() - TODO: Used in branch opcodes (BEQ, BNE, BSR, etc.)
    fn read_relative_offset8(&mut self) -> i8 {
        self.read_pc8() as i8
    }

    // C++ Original: int16_t ReadRelativeOffset16() - TODO: Used in long branch opcodes (LBEQ, LBSR, etc.)
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
                    ea = self.register_select(postbyte).wrapping_add(s16_from_u8(self.registers.b) as u16);
                    self.add_cycles(1);
                }
                0x06 => { // (+/- A),R
                    ea = self.register_select(postbyte).wrapping_add(s16_from_u8(self.registers.a) as u16);
                    self.add_cycles(1);
                }
                0x07 => {
                    panic!("Illegal indexed instruction post-byte");
                }
                0x08 => { // (+/- 7 bit offset),R
                    let postbyte2 = self.read_pc8();
                    ea = self.register_select(postbyte).wrapping_add(s16_from_u8(postbyte2) as u16);
                    self.add_cycles(1);
                }
                0x09 => { // (+/- 15 bit offset),R
                    let postbyte2 = self.read_pc8();
                    let postbyte3 = self.read_pc8();
                    ea = self.register_select(postbyte).wrapping_add(combine_to_s16(postbyte2, postbyte3) as u16);
                    self.add_cycles(4);
                }
                0x0A => {
                    panic!("Illegal indexed instruction post-byte");
                }
                0x0B => { // (+/- D),R
                    ea = self.register_select(postbyte).wrapping_add(s16_from_u8(self.registers.d() as u8) as u16);
                    self.add_cycles(4);
                }
                0x0C => { // (+/- 7 bit offset),PC
                    let postbyte2 = self.read_pc8();
                    ea = self.registers.pc.wrapping_add(s16_from_u8(postbyte2) as u16);
                    self.add_cycles(1);
                }
                0x0D => { // (+/- 15 bit offset),PC
                    let postbyte2 = self.read_pc8();
                    let postbyte3 = self.read_pc8();
                    ea = self.registers.pc.wrapping_add(combine_to_s16(postbyte2, postbyte3) as u16);
                    self.add_cycles(5);
                }
                0x0E => {
                    panic!("Illegal indexed instruction post-byte");
                }
                0x0F => { // [address] (Indirect-only)
                    let postbyte2 = self.read_pc8();
                    let postbyte3 = self.read_pc8();
                    ea = combine_to_s16(postbyte2, postbyte3) as u16;
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
            ea = combine_to_u16(msb, lsb);
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



    // C++ Original: template <int page, uint8_t opCode> void OpLSR() { uint16_t EA = ReadEA16<LookupCpuOp(page, opCode).addrMode>(); uint8_t value = m_memoryBus->Read(EA); OpLSR<page, opCode>(value); m_memoryBus->Write(EA, value); }
    fn op_lsr_memory(&mut self, opcode_byte: u8) {
        let ea = self.read_effective_address(opcode_byte);
        let mut value = self.read8(ea);
        // C++ Original: OpLSR - inline implementation
        let orig_value = value;
        value = value >> 1;
        self.registers.cc.z = Self::calc_zero_u8(value);
        self.registers.cc.n = false; // Bit 7 always shifted out
        self.registers.cc.c = (orig_value & 0b0000_0001) != 0;
        self.write8(ea, value);
    }

    // C++ Original: template <int page, uint8_t opCode> void OpASR() { uint16_t EA = ReadEA16<LookupCpuOp(page, opCode).addrMode>(); uint8_t value = m_memoryBus->Read(EA); OpASR<page, opCode>(value); m_memoryBus->Write(EA, value); }
    fn op_asr_memory(&mut self, opcode_byte: u8) {
        let ea = self.read_effective_address(opcode_byte);
        let mut value = self.read8(ea);
        // C++ Original: OpASR - inline implementation
        let orig_value = value;
        value = (orig_value & 0b1000_0000) | (value >> 1);
        self.registers.cc.z = Self::calc_zero_u8(value);
        self.registers.cc.n = Self::calc_negative_u8(value);
        self.registers.cc.c = (orig_value & 0b0000_0001) != 0;
        self.write8(ea, value);
    }

    // C++ Original: template <int page, uint8_t opCode> void OpROR() { uint16_t EA = ReadEA16<LookupCpuOp(page, opCode).addrMode>(); uint8_t value = m_memoryBus->Read(EA); OpROR<page, opCode>(value); m_memoryBus->Write(EA, value); }
    fn op_ror_memory(&mut self, opcode_byte: u8) {
        let ea = self.read_effective_address(opcode_byte);
        let mut value = self.read8(ea);
        // C++ Original: OpROR - inline implementation
        let result = ((self.registers.cc.c as u8) << 7) | (value >> 1);
        self.registers.cc.c = (value & 0b0000_0001) != 0;
        self.registers.cc.n = Self::calc_negative_u8(result);
        self.registers.cc.z = Self::calc_zero_u8(result);
        value = result;
        self.write8(ea, value);
    }

    // C++ Original: template <int page, uint8_t opCode> void OpASL() { uint16_t EA = ReadEA16<LookupCpuOp(page, opCode).addrMode>(); uint8_t value = m_memoryBus->Read(EA); OpASL<page, opCode>(value); m_memoryBus->Write(EA, value); }
    fn op_asl_memory(&mut self, opcode_byte: u8) {
        let ea = self.read_effective_address(opcode_byte);
        let mut value = self.read8(ea);
        // C++ Original: OpASL - inline implementation (Shifting left is same as adding value + value)
        value = self.add_impl_u8(value, value, 0);
        self.write8(ea, value);
    }

    // C++ Original: template <int page, uint8_t opCode> void OpROL() { uint16_t EA = ReadEA16<LookupCpuOp(page, opCode).addrMode>(); uint8_t value = m_memoryBus->Read(EA); OpROL<page, opCode>(value); m_memoryBus->Write(EA, value); }
    fn op_rol_memory(&mut self, opcode_byte: u8) {
        let ea = self.read_effective_address(opcode_byte);
        let mut value = self.read8(ea);
        // C++ Original: OpROL - inline implementation
        let result = (value << 1) | (self.registers.cc.c as u8);
        self.registers.cc.c = (value & 0b1000_0000) != 0;
        self.registers.cc.v = ((value & 0b1000_0000) ^ ((value & 0b0100_0000) << 1)) != 0;
        self.registers.cc.n = Self::calc_negative_u8(result);
        self.registers.cc.z = Self::calc_zero_u8(result);
        value = result;
        self.write8(ea, value);
    }

    // ======== IMMEDIATE PRIORITY OPCODES IMPLEMENTATION ========

    // C++ Original: template <int page, uint8_t opCode> void OpJMP() { uint16_t EA = ReadEA16<LookupCpuOp(page, opCode).addrMode>(); PC = EA; }
    fn op_jmp(&mut self, opcode_byte: u8) -> Cycles {
        let ea = self.read_effective_address(opcode_byte);
        // C++ Original: PC = EA;
        self.registers.pc = ea;
        // JMP doesn't return cycles - they're already added by CPU infrastructure
        0
    }

    // C++ Original: void OpMUL() { uint16_t result = A * B; CC.Zero = CalcZero(result); CC.Carry = TestBits01(result, BITS(7)); D = result; }
    fn op_mul(&mut self) -> Cycles {
        // C++ Original: uint16_t result = A * B;
        let result = (self.registers.a as u16) * (self.registers.b as u16);
        
        // C++ Original: CC.Zero = CalcZero(result);
        self.registers.cc.z = Self::calc_zero_u16(result);
        
        // C++ Original: CC.Carry = TestBits01(result, BITS(7)); // Test bit 7 of low byte (B register)
        self.registers.cc.c = (result & 0x0080) != 0; // Test bit 7 of low byte (B position in result)
        
        // C++ Original: D = result;
        self.registers.set_d(result);
        
        11 // MUL takes exactly 11 cycles
    }



    // C++ Original: ANDCC is implemented as OpOR/OpAND template
    fn op_andcc(&mut self) {
        // C++ Original: uint8_t value = ReadOperandValue8<AddressingMode::Immediate>();
        let value = self.read_pc8();
        
        // C++ Original: CC.Value = CC.Value & value; (For ANDCC, we don't update CC itself)
        let current_cc = self.registers.cc.to_u8();
        let new_cc = current_cc & value;
        self.registers.cc.from_u8(new_cc);
        
        // Cycles already counted by lookup table
    }

    // C++ Original: ORCC is implemented as OpOR/OpAND template
    fn op_orcc(&mut self) {
        // C++ Original: uint8_t value = ReadOperandValue8<AddressingMode::Immediate>();
        let value = self.read_pc8();
        
        // C++ Original: CC.Value = CC.Value | value; (For ORCC, we don't update CC itself)
        let current_cc = self.registers.cc.to_u8();
        let new_cc = current_cc | value;
        self.registers.cc.from_u8(new_cc);
        
        // Cycles already counted by lookup table
    }

    // C++ Original: void OpCWAI() { uint8_t value = ReadOperandValue8<...>(); CC.Value = CC.Value & value; PushCCState(true); m_waitingForInterrupts = true; }
    fn op_cwai(&mut self) {
        // C++ Original: uint8_t value = ReadOperandValue8<LookupCpuOp(page, opCode).addrMode>();
        let value = self.read_pc8(); // CWAI uses immediate addressing
        
        // C++ Original: CC.Value = CC.Value & value;
        let current_cc = self.registers.cc.to_u8();
        let masked_cc = current_cc & value;
        self.registers.cc.from_u8(masked_cc);
        
        // C++ Original: PushCCState(true);
        // NOTE: push_cc_state modifies CC.Entire, so we must preserve the masked value
        let final_masked_cc = masked_cc | 0x80; // Set Entire bit for push 
        self.push_cc_state_with_value(final_masked_cc);
        
        // C++ Original: ASSERT(!m_waitingForInterrupts); m_waitingForInterrupts = true;
        // TODO: Implement interrupt waiting state - for now just do the CC and stack operations
        
        // Cycles (20) already counted by lookup table
    }

    // C++ Original: void OpSWI(InterruptVector::Type type) - for SWI (0x3F)
    fn op_swi(&mut self) {
        // C++ Original: PushCCState(true);
        self.push_cc_state(true);
        
        // C++ Original: CC.InterruptMask = 1; CC.FastInterruptMask = 1;
        self.registers.cc.i = true;  // Interrupt mask
        self.registers.cc.f = true;  // Fast interrupt mask
        
        // C++ Original: PC = Read16(InterruptVector::Swi); (0xFFFA)
        self.registers.pc = self.read16(0xFFFA);
        
        // Cycles (19) already counted by lookup table
    }

    // C++ Original: void OpSWI(InterruptVector::Type type) - for SWI2 (0x103F)  
    fn op_swi2(&mut self) {
        // C++ Original: PushCCState(true);
        self.push_cc_state(true);
        
        // C++ Original: PC = Read16(InterruptVector::Swi2); (0xFFF2) - SWI2 uses 0xFFF2 vector
        self.registers.pc = self.read16(SWI2_VECTOR);
        
        // Cycles (20) already counted by lookup table
    }

    // C++ Original: void OpSWI(InterruptVector::Type type) - for SWI3 (0x113F)
    fn op_swi3(&mut self) {
        // C++ Original: PushCCState(true);
        self.push_cc_state(true);
        
        // C++ Original: PC = Read16(InterruptVector::Swi3); (0xFFF4) - SWI3 uses 0xFFF4 vector
        self.registers.pc = self.read16(SWI3_VECTOR);
        
        // Cycles (20) already counted by lookup table
    }

    // SYNC - Synchronize with interrupt (Opcode 0x13)
    // 6809 Original: The SYNC instruction causes the CPU to wait for an interrupt to occur
    fn op_sync(&mut self) {
        // C++ Original: SYNC instruction waits for interrupts
        // Set flag to indicate CPU is waiting for interrupts
        self.waiting_for_interrupts = true;
        // Cycles (2) already counted by lookup table
    }

    // RESET* - System reset instruction (Opcode 0x3E, undocumented)
    // 6809 Original: Forces a system reset (undocumented behavior)
    fn op_reset(&mut self) {
        // C++ Original: Reset instruction causes system reset
        // Reset CPU state but don't add the cycle that reset() normally adds
        // since the instruction itself takes 0 cycles according to the table
        self.registers.a = 0;
        self.registers.b = 0;
        self.registers.x = 0;
        self.registers.y = 0;
        self.registers.u = 0;
        self.registers.s = 0;
        self.registers.dp = 0;
        self.registers.cc = ConditionCode::new();
        self.registers.pc = self.read16(RESET_VECTOR);
        self.waiting_for_interrupts = false;
        // Don't add extra cycles - the lookup table already specifies 0 cycles
    }

    // C++ Original: void OpRTI() { bool poppedEntire{}; PopCCState(poppedEntire); AddCycles(poppedEntire ? 15 : 6); }
    fn op_rti(&mut self) {
        // C++ Original: bool poppedEntire{}; PopCCState(poppedEntire);
        let popped_entire = self.pop_cc_state();
        
        // C++ Original: AddCycles(poppedEntire ? 15 : 6);
        let cycles = if popped_entire { 15 } else { 6 };
        self.add_cycles(cycles);
    }

    // C++ Original: void PushCCState(bool entire)
    fn push_cc_state(&mut self, entire: bool) {
        // C++ Original: CC.Entire = entire ? 1 : 0;
        self.registers.cc.e = entire;
        let cc_value = self.registers.cc.to_u8();
        self.push_cc_state_with_value(cc_value);
    }

    fn push_cc_state_with_value(&mut self, cc_value: u8) {
        // C++ Original stack push order: PC, U, Y, X, DP, B, A, CC
        // C++ Original: Push16(S, PC); Push16(S, U); Push16(S, Y); Push16(S, X); Push8(S, DP); Push8(S, B); Push8(S, A); Push8(S, CC.Value);
        
        // Inline stack push operations to avoid borrow checker issues
        // Push PC (16-bit) - C++ Original: Push16(S, PC) writes low byte first, then high byte
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, (self.registers.pc & 0xFF) as u8); // Low byte first
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, (self.registers.pc >> 8) as u8);   // High byte second
        
        // Push U (16-bit) - C++ Original: Push16(S, U) writes low byte first, then high byte
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, (self.registers.u & 0xFF) as u8); // Low byte first
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, (self.registers.u >> 8) as u8);   // High byte second
        
        // Push Y (16-bit) - C++ Original: Push16(S, Y) writes low byte first, then high byte
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, (self.registers.y & 0xFF) as u8); // Low byte first
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, (self.registers.y >> 8) as u8);   // High byte second
        
        // Push X (16-bit) - C++ Original: Push16(S, X) writes low byte first, then high byte
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, (self.registers.x & 0xFF) as u8); // Low byte first
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, (self.registers.x >> 8) as u8);   // High byte second
        
        // Push DP (8-bit)
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, self.registers.dp);
        
        // Push B (8-bit)
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, self.registers.b);
        
        // Push A (8-bit)
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, self.registers.a);
        
        // Push CC (8-bit) - use provided value instead of reading from register
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, cc_value);
    }

    // C++ Original: void PopCCState(bool& poppedEntire)
    fn pop_cc_state(&mut self) -> bool {
        // C++ Original: CC.Value = Pop8(S); poppedEntire = CC.Entire != 0;
        let cc_value = self.read8(self.registers.s);
        self.registers.s = self.registers.s.wrapping_add(1);
        self.registers.cc.from_u8(cc_value);
        
        let popped_entire = self.registers.cc.e;
        
        if popped_entire {
            // C++ Original: if (CC.Entire) { A = Pop8(S); B = Pop8(S); ... }
            // Only pop all registers if Entire flag is set
            
            // Pop A (8-bit)
            self.registers.a = self.read8(self.registers.s);
            self.registers.s = self.registers.s.wrapping_add(1);
            
            // Pop B (8-bit)
            self.registers.b = self.read8(self.registers.s);
            self.registers.s = self.registers.s.wrapping_add(1);
            
            // Pop DP (8-bit)
            self.registers.dp = self.read8(self.registers.s);
            self.registers.s = self.registers.s.wrapping_add(1);
            
            // Pop X (16-bit) - C++ Original: Pop16(S) reads high byte first, then low byte
            let x_high = self.read8(self.registers.s) as u16;
            self.registers.s = self.registers.s.wrapping_add(1);
            let x_low = self.read8(self.registers.s) as u16;
            self.registers.s = self.registers.s.wrapping_add(1);
            self.registers.x = (x_high << 8) | x_low;
            
            // Pop Y (16-bit) - C++ Original: Pop16(S) reads high byte first, then low byte
            let y_high = self.read8(self.registers.s) as u16;
            self.registers.s = self.registers.s.wrapping_add(1);
            let y_low = self.read8(self.registers.s) as u16;
            self.registers.s = self.registers.s.wrapping_add(1);
            self.registers.y = (y_high << 8) | y_low;
            
            // Pop U (16-bit) - C++ Original: Pop16(S) reads high byte first, then low byte
            let u_high = self.read8(self.registers.s) as u16;
            self.registers.s = self.registers.s.wrapping_add(1);
            let u_low = self.read8(self.registers.s) as u16;
            self.registers.s = self.registers.s.wrapping_add(1);
            self.registers.u = (u_high << 8) | u_low;
            
            // Pop PC (16-bit) - C++ Original: Pop16(S) reads high byte first, then low byte
            let pc_high = self.read8(self.registers.s) as u16;
            self.registers.s = self.registers.s.wrapping_add(1);
            let pc_low = self.read8(self.registers.s) as u16;
            self.registers.s = self.registers.s.wrapping_add(1);
            self.registers.pc = (pc_high << 8) | pc_low;
        } else {
            // C++ Original: } else { PC = Pop16(S); }
            // Fast interrupt context: only pop PC - C++ Original: Pop16(S) reads high byte first
            let pc_high = self.read8(self.registers.s) as u16;
            self.registers.s = self.registers.s.wrapping_add(1);
            let pc_low = self.read8(self.registers.s) as u16;
            self.registers.s = self.registers.s.wrapping_add(1);
            self.registers.pc = (pc_high << 8) | pc_low;
        }
        
        popped_entire
    }

    // ======== LONG BRANCH OPERATIONS IMPLEMENTATION ========

    // C++ Original: void OpLBRA() { int16_t offset = ReadRelativeOffset16(); PC += offset; }
    fn op_lbra(&mut self) {
        let offset = self.read_relative_offset16();
        // C++ Original: PC += offset;
        self.registers.pc = ((self.registers.pc as i32) + (offset as i32)) as u16;
        // LBRA always takes exactly 5 cycles (no extra cycle like conditional branches)
    }

    // Helper function for long branch operations
    // C++ Original: void OpLongBranch(CondFunc condFunc) { int16_t offset = ReadRelativeOffset16(); if (condFunc()) { PC += offset; AddCycles(1); } }
    fn op_long_branch(&mut self, condition: bool) {
        let offset = self.read_relative_offset16();
        if condition {
            // C++ Original: PC += offset; AddCycles(1);
            self.registers.pc = ((self.registers.pc as i32) + (offset as i32)) as u16;
            self.add_cycles(1); // Extra cycle if branch is taken
        }
    }

    // C++ Original: case 0x21: OpLongBranch([] { return false; }); - LBRN (Long Branch Never)
    fn op_long_branch_never(&mut self) {
        self.op_long_branch(false) // Always false - never branch
    }

    // C++ Original: case 0x22: OpLongBranch([this] { return (CC.Carry | CC.Zero) == 0; }); - LBHI (Long Branch if Higher)
    fn op_long_branch_if_higher(&mut self) {
        let condition = !self.registers.cc.c && !self.registers.cc.z;
        self.op_long_branch(condition)
    }

    // C++ Original: case 0x23: OpLongBranch([this] { return (CC.Carry | CC.Zero) != 0; }); - LBLS (Long Branch if Lower or Same)
    fn op_long_branch_if_lower_or_same(&mut self) {
        let condition = self.registers.cc.c || self.registers.cc.z;
        self.op_long_branch(condition)
    }

    // C++ Original: case 0x24: OpLongBranch([this] { return CC.Carry == 0; }); - LBCC (Long Branch if Carry Clear)
    fn op_long_branch_if_carry_clear(&mut self) {
        let condition = !self.registers.cc.c;
        self.op_long_branch(condition)
    }

    // C++ Original: case 0x25: OpLongBranch([this] { return CC.Carry != 0; }); - LBCS (Long Branch if Carry Set)
    fn op_long_branch_if_carry_set(&mut self) {
        let condition = self.registers.cc.c;
        self.op_long_branch(condition)
    }

    // C++ Original: case 0x26: OpLongBranch([this] { return CC.Zero == 0; }); - LBNE (Long Branch if Not Equal)
    fn op_long_branch_if_not_equal(&mut self) {
        let condition = !self.registers.cc.z;
        self.op_long_branch(condition)
    }

    // C++ Original: case 0x27: OpLongBranch([this] { return CC.Zero != 0; }); - LBEQ (Long Branch if Equal)
    fn op_long_branch_if_equal(&mut self) {
        let condition = self.registers.cc.z;
        self.op_long_branch(condition)
    }

    // C++ Original: case 0x28: OpLongBranch([this] { return CC.Overflow == 0; }); - LBVC (Long Branch if Overflow Clear)
    fn op_long_branch_if_overflow_clear(&mut self) {
        let condition = !self.registers.cc.v;
        self.op_long_branch(condition)
    }

    // C++ Original: case 0x29: OpLongBranch([this] { return CC.Overflow != 0; }); - LBVS (Long Branch if Overflow Set)
    fn op_long_branch_if_overflow_set(&mut self) {
        let condition = self.registers.cc.v;
        self.op_long_branch(condition)
    }

    // C++ Original: case 0x2A: OpLongBranch([this] { return CC.Negative == 0; }); - LBPL (Long Branch if Plus)
    fn op_long_branch_if_plus(&mut self) {
        let condition = !self.registers.cc.n;
        self.op_long_branch(condition)
    }

    // C++ Original: case 0x2B: OpLongBranch([this] { return CC.Negative != 0; }); - LBMI (Long Branch if Minus)
    fn op_long_branch_if_minus(&mut self) {
        let condition = self.registers.cc.n;
        self.op_long_branch(condition)
    }

    // C++ Original: case 0x2C: OpLongBranch([this] { return (CC.Negative ^ CC.Overflow) == 0; }); - LBGE (Long Branch if Greater or Equal)
    fn op_long_branch_if_greater_or_equal(&mut self) {
        let condition = self.registers.cc.n == self.registers.cc.v;
        self.op_long_branch(condition)
    }

    // C++ Original: case 0x2D: OpLongBranch([this] { return (CC.Negative ^ CC.Overflow) != 0; }); - LBLT (Long Branch if Less Than)
    fn op_long_branch_if_less_than(&mut self) {
        let condition = self.registers.cc.n != self.registers.cc.v;
        self.op_long_branch(condition)
    }

    // C++ Original: case 0x2E: OpLongBranch([this] { return (CC.Zero | (CC.Negative ^ CC.Overflow)) == 0; }); - LBGT (Long Branch if Greater)
    fn op_long_branch_if_greater(&mut self) {
        let condition = !self.registers.cc.z && (self.registers.cc.n == self.registers.cc.v);
        self.op_long_branch(condition)
    }

    // C++ Original: case 0x2F: OpLongBranch([this] { return (CC.Zero | (CC.Negative ^ CC.Overflow)) != 0; }); - LBLE (Long Branch if Less or Equal)
    fn op_long_branch_if_less_or_equal(&mut self) {
        let condition = self.registers.cc.z || (self.registers.cc.n != self.registers.cc.v);
        self.op_long_branch(condition)
    }

    // ======== MEMORY OPERATION HELPER FUNCTIONS ========

    // C++ Original: void OpNEG(uint8_t& value) { value = SubtractImpl(0, value, 0, CC); }
    fn op_neg_memory(&mut self, opcode_byte: u8) {
        let ea = self.read_effective_address(opcode_byte);
        let value = self.read8(ea);
        // C++ Original: Negating is 0 - value
        let result = self.subtract_impl_u8(0, value, 0);
        self.write8(ea, result);
    }

    // C++ Original: void OpCOM(uint8_t& value) { value = ~value; CC.Negative = CalcNegative(value); CC.Zero = CalcZero(value); CC.Overflow = 0; CC.Carry = 1; }
    fn op_com_memory(&mut self, opcode_byte: u8) {
        let ea = self.read_effective_address(opcode_byte);
        let value = self.read8(ea);
        // C++ Original: value = ~value;
        let result = !value;
        // C++ Original: CC flags
        self.registers.cc.n = Self::calc_negative_u8(result);
        self.registers.cc.z = Self::calc_zero_u8(result);
        self.registers.cc.v = false;
        self.registers.cc.c = true;
        self.write8(ea, result);
    }

    // C++ Original: void OpINC(uint8_t& value) { uint8_t origValue = value; ++value; CC.Overflow = origValue == 0b0111'1111; CC.Zero = CalcZero(value); CC.Negative = CalcNegative(value); }
    fn op_inc_memory(&mut self, opcode_byte: u8) {
        let ea = self.read_effective_address(opcode_byte);
        let orig_value = self.read8(ea);
        let result = orig_value.wrapping_add(1);
        // C++ Original: CC.Overflow = origValue == 0b0111'1111;
        self.registers.cc.v = orig_value == 0b0111_1111;
        self.registers.cc.z = Self::calc_zero_u8(result);
        self.registers.cc.n = Self::calc_negative_u8(result);
        self.write8(ea, result);
    }

    // C++ Original: void OpDEC(uint8_t& value) { uint8_t origValue = value; --value; CC.Overflow = origValue == 0b1000'0000; CC.Zero = CalcZero(value); CC.Negative = CalcNegative(value); }
    fn op_dec_memory(&mut self, opcode_byte: u8) {
        let ea = self.read_effective_address(opcode_byte);
        let orig_value = self.read8(ea);
        let result = orig_value.wrapping_sub(1);
        // C++ Original: CC.Overflow = origValue == 0b1000'0000;
        self.registers.cc.v = orig_value == 0b1000_0000;
        self.registers.cc.z = Self::calc_zero_u8(result);
        self.registers.cc.n = Self::calc_negative_u8(result);
        self.write8(ea, result);
    }

    // C++ Original: void OpTST(const uint8_t& value) { CC.Negative = CalcNegative(value); CC.Zero = CalcZero(value); CC.Overflow = 0; }
    fn op_tst_memory(&mut self, opcode_byte: u8) {
        let ea = self.read_effective_address(opcode_byte);
        let value = self.read8(ea);
        // C++ Original: CC flags only, no value change
        self.registers.cc.n = Self::calc_negative_u8(value);
        self.registers.cc.z = Self::calc_zero_u8(value);
        self.registers.cc.v = false;
    }

    // C++ Original: void OpCLR() { uint16_t EA = ReadEA16<...>(); m_memoryBus->Write(EA, 0); CC.Negative = 0; CC.Zero = 1; CC.Overflow = 0; CC.Carry = 0; }
    fn op_clr_memory(&mut self, opcode_byte: u8) {
        let ea = self.read_effective_address(opcode_byte);
        // C++ Original: Write 0 and set CC flags
        self.write8(ea, 0);
        self.registers.cc.n = false;
        self.registers.cc.z = true;
        self.registers.cc.v = false;
        self.registers.cc.c = false;
    }

    // C++ Original: void OpDAA() { ... complex BCD adjust implementation }
    fn op_daa(&mut self) {
        // C++ Original: Extract least and most significant nibbles
        let lsn = self.registers.a & 0b0000_1111;
        let msn = (self.registers.a & 0b1111_0000) >> 4;

        // C++ Original: Compute correction factors
        let cf_lsn = if self.registers.cc.h || lsn > 9 { 6 } else { 0 };
        let cf_msn = if self.registers.cc.c || msn > 9 || (msn > 8 && lsn > 9) { 6 } else { 0 };
        let adjust = (cf_msn << 4) | cf_lsn;
        let r16 = (self.registers.a as u16) + (adjust as u16);
        self.registers.a = r16 as u8;
        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
        self.registers.cc.c = self.registers.cc.c || Self::calc_carry_u16(r16);
    }

    // ======== BIT OPERATION HELPER FUNCTIONS ========

    // C++ Original: template<int cpuOp, uint8_t opCode, typename RegOrAccType> void OpBIT(RegOrAccType& reg)
    // C++ Original: uint8_t andResult = reg & ReadOperandValue8<cpuOp, opCode>(); CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;
    fn bit_impl_u8(&mut self, reg: u8, operand: u8) {
        // C++ Original: uint8_t andResult = reg & ReadOperandValue8<cpuOp, opCode>();
        let and_result = reg & operand;
        
        // C++ Original: CC.Zero = CalcZero(andResult); CC.Negative = CalcNegative(andResult); CC.Overflow = 0;
        self.registers.cc.z = Self::calc_zero_u8(and_result);
        self.registers.cc.n = Self::calc_negative_u8(and_result);
        self.registers.cc.v = false;
    }

    // C++ Original: OpBIT<0, 0x85>(A); - BITA immediate
    fn op_bita_immediate(&mut self) {
        let operand = self.read_pc8();
        self.bit_impl_u8(self.registers.a, operand);
    }

    // C++ Original: OpBIT<0, 0x95>(A); - BITA direct
    fn op_bita_direct(&mut self) {
        let ea = self.read_direct_ea();
        let operand = self.read8(ea);
        self.bit_impl_u8(self.registers.a, operand);
    }

    // C++ Original: OpBIT<0, 0xA5>(A); - BITA indexed
    fn op_bita_indexed(&mut self) {
        let ea = self.read_indexed_ea();
        let operand = self.read8(ea);
        self.bit_impl_u8(self.registers.a, operand);
    }

    // C++ Original: OpBIT<0, 0xB5>(A); - BITA extended
    fn op_bita_extended(&mut self) {
        let ea = self.read_extended_ea();
        let operand = self.read8(ea);
        self.bit_impl_u8(self.registers.a, operand);
    }

    // C++ Original: OpBIT<0, 0xC5>(B); - BITB immediate
    fn op_bitb_immediate(&mut self) {
        let operand = self.read_pc8();
        self.bit_impl_u8(self.registers.b, operand);
    }

    // C++ Original: OpBIT<0, 0xD5>(B); - BITB direct
    fn op_bitb_direct(&mut self) {
        let ea = self.read_direct_ea();
        let operand = self.read8(ea);
        self.bit_impl_u8(self.registers.b, operand);
    }

    // C++ Original: OpBIT<0, 0xE5>(B); - BITB indexed
    fn op_bitb_indexed(&mut self) {
        let ea = self.read_indexed_ea();
        let operand = self.read8(ea);
        self.bit_impl_u8(self.registers.b, operand);
    }

    // C++ Original: OpBIT<0, 0xF5>(B); - BITB extended
    fn op_bitb_extended(&mut self) {
        let ea = self.read_extended_ea();
        let operand = self.read8(ea);
        self.bit_impl_u8(self.registers.b, operand);
    }
}