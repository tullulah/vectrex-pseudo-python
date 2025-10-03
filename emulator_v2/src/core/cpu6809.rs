//! MC6809 CPU implementation
//! Port of vectrexy/libs/emulator/include/emulator/Cpu.h and src/Cpu.cpp

use crate::types::Cycles;
use crate::core::memory_bus::MemoryBus;
use crate::core::cpu_helpers::{combine_to_u16, combine_to_s16, s16_from_u8};
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
#[allow(dead_code)]
const NMI_VECTOR: u16   = 0xFFFC;
const SWI_VECTOR: u16   = 0xFFFA;
#[allow(dead_code)]
const IRQ_VECTOR: u16   = 0xFFF8;
#[allow(dead_code)]
const FIRQ_VECTOR: u16  = 0xFFF6;
#[allow(dead_code)]
const SWI2_VECTOR: u16  = 0xFFF4;
#[allow(dead_code)]
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
    fn do_execute_instruction(&mut self, _irq_enabled: bool, _firq_enabled: bool) {
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
                // C++ Original: switch (cpuOp.opCode) - Page 0 instructions
                match opcode_byte {
                    // Implemented opcode
                    0x00 => {
                        self.op_neg_memory(AddressingMode::Direct);
                    },
                    // Implemented opcode
                    0x01 => {
                        // TODO: Implement or mark as illegal
                        panic!("Unimplemented opcode 0x01 (Reserved)");
                    },
                    // Implemented opcode
                    0x02 => {
                        // TODO: Implement or mark as illegal
                        panic!("Unimplemented opcode 0x02 (Reserved)");
                    },
                    // Implemented opcode
                    0x03 => {
                        self.op_com_memory(AddressingMode::Direct);
                    },
                    // Implemented opcode
                    0x04 => {
                        self.op_lsr_memory(AddressingMode::Direct);
                    },
                    // Implemented opcode
                    0x05 => {
                        // TODO: Implement or mark as illegal
                        panic!("Unimplemented opcode 0x05 (Reserved)");
                    },
                    // Implemented opcode
                    0x06 => {
                        self.op_ror_memory(AddressingMode::Direct);
                    },
                    // Implemented opcode
                    0x07 => {
                        self.op_asr_memory(AddressingMode::Direct);
                    },
                    // Implemented opcode
                    0x08 => {
                        self.op_asl_memory(AddressingMode::Direct);
                    },
                    // Implemented opcode
                    0x09 => {
                        self.op_rol_memory(AddressingMode::Direct);
                    },
                    // Implemented opcode
                    0x0A => {
                        self.op_dec_memory(AddressingMode::Direct);
                    },
                    // Implemented opcode
                    0x0B => {
                        // TODO: Implement or mark as illegal
                        panic!("Unimplemented opcode 0x0B (Reserved)");
                    },
                    // Implemented opcode
                    0x0C => {
                        self.op_inc_memory(AddressingMode::Direct);
                    },
                    // Implemented opcode
                    0x0D => {
                        self.op_tst_memory(AddressingMode::Direct);
                    },
                    // Implemented opcode
                    0x0E => {
                        let ea = self.read_direct_ea();
                        self.registers.pc = ea;
                    },
                    // Implemented opcode
                    0x0F => {
                        self.op_clr_memory(AddressingMode::Direct);
                    },

                    // Implemented opcode
                    0x10 => {
                        panic!("Page 1 prefix should not reach here");
                    },
                    // Implemented opcode
                    0x11 => {
                        panic!("Page 2 prefix should not reach here");
                    },
                    // Implemented opcode
                    0x12 => {
                        // No operation - cycles already added
                    },
                    // Implemented opcode
                    0x13 => {
                        // TODO: Implement SYNC instruction
                        panic!("Unimplemented opcode 0x13 (SYNC)");
                    },
                    // Implemented opcode
                    0x14 => {
                        // TODO: Implement or mark as illegal
                        panic!("Unimplemented opcode 0x14 (Reserved)");
                    },
                    // Implemented opcode
                    0x15 => {
                        // TODO: Implement or mark as illegal
                        panic!("Unimplemented opcode 0x15 (Reserved)");
                    },
                    // Implemented opcode: LBRA (Long Branch Always)
                    // C++ Original: case 0x16: OpLBranch<true>(); break;
                    0x16 => {
                        let offset = self.read_relative_offset16();
                        self.registers.pc = self.registers.pc.wrapping_add(offset as u16);
                    },
                    // Implemented opcode: LBSR (Long Branch to Subroutine)
                    // C++ Original: case 0x17: OpLBSR(); break;
                    0x17 => {
                        let offset = self.read_relative_offset16();
                        // Push return address onto stack
                        self.registers.s = self.registers.s.wrapping_sub(1);
                        self.write8(self.registers.s, (self.registers.pc & 0xFF) as u8); // Low
                        self.registers.s = self.registers.s.wrapping_sub(1);
                        self.write8(self.registers.s, (self.registers.pc >> 8) as u8);   // High
                        // Branch to target
                        self.registers.pc = self.registers.pc.wrapping_add(offset as u16);
                    },
                    // Implemented opcode
                    0x18 => {
                        // TODO: Implement or mark as illegal
                        panic!("Unimplemented opcode 0x18 (Reserved)");
                    },
                    // Implemented opcode
                    0x19 => {
                        self.op_daa();
                    },
                    // Implemented opcode
                    0x1A => {
                        let operand = self.read_pc8();
                        let current_cc = self.registers.cc.to_u8();
                        self.registers.cc.from_u8(current_cc | operand);
                    },
                    // Implemented opcode
                    0x1B => {
                        // TODO: Implement or mark as illegal
                        panic!("Unimplemented opcode 0x1B (Reserved)");
                    },
                    // Implemented opcode
                    0x1C => {
                        let operand = self.read_pc8();
                        let current_cc = self.registers.cc.to_u8();
                        self.registers.cc.from_u8(current_cc & operand);
                    },
                    // Implemented opcode
                    0x1D => {
                        if (self.registers.b & 0x80) != 0 {
                            self.registers.a = 0xFF;
                        } else {
                            self.registers.a = 0x00;
                        }
                        self.registers.cc.n = (self.registers.b & 0x80) != 0;
                        self.registers.cc.z = self.registers.b == 0;
                    },
                    // Implemented opcode
                    0x1E => {
                        self.op_exg();
                    },
                    // Implemented opcode
                    0x1F => {
                        self.op_tfr();
                    },

                    // Implemented opcode
                    0x20 => {
                        self.op_branch(|| true);
                    },
                    // Implemented opcode
                    0x21 => {
                        self.op_branch(|| false);
                    },
                    // Implemented opcode
                    0x22 => {
                        let c = self.registers.cc.c;
                        let z = self.registers.cc.z;
                        self.op_branch(move || (c as u8 | z as u8) == 0);
                    },
                    // Implemented opcode
                    0x23 => {
                        let c = self.registers.cc.c;
                        let z = self.registers.cc.z;
                        self.op_branch(move || (c as u8 | z as u8) != 0);
                    },
                    // Implemented opcode
                    0x24 => {
                        let c = self.registers.cc.c;
                        self.op_branch(move || !c);
                    },
                    // Implemented opcode
                    0x25 => {
                        let c = self.registers.cc.c;
                        self.op_branch(move || c);
                    },
                    // Implemented opcode
                    0x26 => {
                        let z = self.registers.cc.z;
                        self.op_branch(move || !z);
                    },
                    // Implemented opcode
                    0x27 => {
                        let z = self.registers.cc.z;
                        self.op_branch(move || z);
                    },
                    // Implemented opcode
                    0x28 => {
                        let v = self.registers.cc.v;
                        self.op_branch(move || !v);
                    },
                    // Implemented opcode
                    0x29 => {
                        let v = self.registers.cc.v;
                        self.op_branch(move || v);
                    },
                    // Implemented opcode
                    0x2A => {
                        let n = self.registers.cc.n;
                        self.op_branch(move || !n);
                    },
                    // Implemented opcode
                    0x2B => {
                        let n = self.registers.cc.n;
                        self.op_branch(move || n);
                    },
                    // Implemented opcode
                    0x2C => {
                        let n = self.registers.cc.n;
                        let v = self.registers.cc.v;
                        self.op_branch(move || (n as u8 ^ v as u8) == 0);
                    },
                    // Implemented opcode
                    0x2D => {
                        let n = self.registers.cc.n;
                        let v = self.registers.cc.v;
                        self.op_branch(move || (n as u8 ^ v as u8) != 0);
                    },
                    // Implemented opcode
                    0x2E => {
                        let z = self.registers.cc.z;
                        let n = self.registers.cc.n;
                        let v = self.registers.cc.v;
                        self.op_branch(move || (z as u8 | (n as u8 ^ v as u8)) == 0);
                    },
                    // Implemented opcode
                    0x2F => {
                        let z = self.registers.cc.z;
                        let n = self.registers.cc.n;
                        let v = self.registers.cc.v;
                        self.op_branch(move || (z as u8 | (n as u8 ^ v as u8)) != 0);
                    },

                    // Implemented opcode
                    0x30 => {
                        let ea = self.calc_indexed_ea();
                        self.registers.x = ea;
                        self.registers.cc.z = Self::calc_zero_u16(ea);
                        // LEA only affects Zero flag, not N, V, or C
                    },
                    // Implemented opcode
                    0x31 => {
                        let ea = self.calc_indexed_ea();
                        self.registers.y = ea;
                        self.registers.cc.z = Self::calc_zero_u16(ea);
                        // LEA only affects Zero flag, not N, V, or C
                    },
                    // Implemented opcode
                    0x32 => {
                        let ea = self.calc_indexed_ea();
                        self.registers.s = ea;
                        // LEAS does NOT affect any condition code flags
                    },
                    // Implemented opcode
                    0x33 => {
                        let ea = self.calc_indexed_ea();
                        self.registers.u = ea;
                        // LEAU does NOT affect any condition code flags
                    },
                    // Implemented opcode
                    0x34 => {
                        self.op_psh(true); // true = S stack
                    },
                    // Implemented opcode
                    0x35 => {
                        self.op_pul(true); // true = S stack
                    },
                    // Implemented opcode
                    0x36 => {
                        self.op_psh(false); // false = U stack
                    },
                    // Implemented opcode
                    0x37 => {
                        self.op_pul(false); // false = U stack
                    },
                    // ILLEGAL OPCODE: Reserved
                    0x38 => {
                        panic!("Illegal opcode 0x38: Reserved opcode on MC6809");
                    },
                    // Implemented opcode
                    0x39 => {
                        self.op_rts();
                    },
                    // Implemented opcode
                    0x3A => {
                        self.op_abx();
                    },
                    // Implemented opcode
                    0x3B => {
                        // RTI - Return from Interrupt
                        // MC6809 Spec: Pop entire state from stack (reverse order of interrupt entry)
                        // C++ Original (Cpu.cpp lines 880-891): Uses Pop8/Pop16 helpers
                        
                        // Pop CC first to check E bit (Vectrexy pops CC first)
                        let mut sp = self.registers.s;
                        let cc_value = self.pop8(&mut sp);
                        let entire_state = (cc_value & 0x80) != 0; // E bit
                        
                        if entire_state {
                            // Pop entire state (matches Vectrexy order)
                            self.registers.a = self.pop8(&mut sp);
                            self.registers.b = self.pop8(&mut sp);
                            self.registers.dp = self.pop8(&mut sp);
                            self.registers.x = self.pop16(&mut sp);
                            self.registers.y = self.pop16(&mut sp);
                            self.registers.u = self.pop16(&mut sp);
                            self.registers.pc = self.pop16(&mut sp);
                        } else {
                            // FIRQ-style: only PC+CC
                            self.registers.pc = self.pop16(&mut sp);
                        }
                        self.registers.s = sp;
                        
                        // Restore CC (already popped at start)
                        self.registers.cc.from_u8(cc_value);
                        
                        // Cycles: 6 for FIRQ-style (PC+CC), 15 for entire state
                        self.add_cycles(if entire_state { 15 } else { 6 });
                    }
                    // Implemented opcode
                    0x3C => {
                        // CWAI - Clear and Wait for Interrupt
                        // MC6809 Spec: AND CC with immediate, push entire state, wait for interrupt
                        // C++ Original: Similar to SWI but clears CC bits first
                        
                        // Read immediate mask operand
                        let mask = self.read8(self.registers.pc);
                        self.registers.pc = self.registers.pc.wrapping_add(1);
                        
                        // Clear specified bits in CC (AND operation)
                        let new_cc = self.registers.cc.to_u8() & mask;
                        self.registers.cc.from_u8(new_cc);
                        
                        // Set E bit in CC before pushing (indicates entire state on stack)
                        self.registers.cc.e = true; // E bit = 1
                        
                        // Push entire state using helpers (same as SWI)
                        let mut sp = self.registers.s;
                        self.push8(&mut sp, self.registers.cc.to_u8());
                        self.push8(&mut sp, self.registers.a);
                        self.push8(&mut sp, self.registers.b);
                        self.push8(&mut sp, self.registers.dp);
                        self.push16(&mut sp, self.registers.x);
                        self.push16(&mut sp, self.registers.y);
                        self.push16(&mut sp, self.registers.u);
                        self.push16(&mut sp, self.registers.pc);
                        self.registers.s = sp;
                        
                        // Set waiting state (emulator halts until interrupt)
                        // TODO: Implement actual wait mechanism when interrupt system is ready
                        
                        // 20 cycles
                        self.add_cycles(20);
                    }
                    // Implemented opcode
                    0x3D => {
                        let result = (self.registers.a as u16) * (self.registers.b as u16);
                        self.registers.cc.z = result == 0;
                        self.registers.cc.c = (result & 0x80) != 0; // Carry = bit 7 of 16-bit result (BITS(7) in Vectrexy)
                        self.registers.set_d(result);
                    },
                    // ILLEGAL OPCODE: Reserved
                    0x3E => {
                        panic!("Illegal opcode 0x3E: Reserved opcode on MC6809");
                    },
                    // Implemented opcode
                    0x3F => {
                        // SWI - Software Interrupt
                        // MC6809 Spec: Push entire state, set E and I bits, jump to SWI vector (0xFFFA)
                        // C++ Original (Cpu.cpp lines 869-877): Uses Push8/Push16 helpers
                        
                        // Set E bit in CC before pushing (indicates entire state on stack)
                        self.registers.cc.e = true; // E bit = 1
                        
                        // Push entire state using helpers (matches Vectrexy order)
                        let mut sp = self.registers.s;
                        self.push8(&mut sp, self.registers.cc.to_u8());
                        self.push8(&mut sp, self.registers.a);
                        self.push8(&mut sp, self.registers.b);
                        self.push8(&mut sp, self.registers.dp);
                        self.push16(&mut sp, self.registers.x);
                        self.push16(&mut sp, self.registers.y);
                        self.push16(&mut sp, self.registers.u);
                        self.push16(&mut sp, self.registers.pc);
                        self.registers.s = sp;
                        
                        // Set I bit (mask normal interrupts)
                        self.registers.cc.i = true; // I bit = 1
                        
                        // Jump to SWI vector (0xFFFA) - big endian
                        self.registers.pc = self.read16(SWI_VECTOR);
                        
                        // 19 cycles
                        self.add_cycles(19);
                    }

                    // Implemented opcode
                    0x40 => {
                        self.registers.a = self.subtract_impl_u8(0, self.registers.a, 0);
                    },
                    // ILLEGAL OPCODE: Cannot use indexed addressing for comparison/test operations
                    0x41 => {
                        panic!("Illegal opcode 0x41: Invalid indexed addressing for CMP/TST");
                    },
                    // ILLEGAL OPCODE: Cannot use indexed addressing for SBC operations
                    0x42 => {
                        panic!("Illegal opcode 0x42: Invalid indexed addressing for SBC");
                    },
                    // Implemented opcode
                    0x43 => {
                        self.registers.a = !self.registers.a;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                        self.registers.cc.c = true;
                    },
                    // Implemented opcode
                    0x44 => {
                        let orig_value = self.registers.a;
                        self.registers.a = self.registers.a >> 1;
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.n = false; // Bit 7 always shifted out
                        self.registers.cc.c = (orig_value & 0b0000_0001) != 0;
                    },
                    // ILLEGAL OPCODE: Cannot use indexed addressing for BIT test operations
                    0x45 => {
                        panic!("Illegal opcode 0x45: Invalid indexed addressing for BIT");
                    },
                    // Implemented opcode
                    0x46 => {
                        let result = ((self.registers.cc.c as u8) << 7) | (self.registers.a >> 1);
                        self.registers.cc.c = (self.registers.a & 0b0000_0001) != 0;
                        self.registers.cc.n = Self::calc_negative_u8(result);
                        self.registers.cc.z = Self::calc_zero_u8(result);
                        self.registers.a = result;
                    },
                    // Implemented opcode
                    0x47 => {
                        let orig_value = self.registers.a;
                        self.registers.a = (orig_value & 0b1000_0000) | (self.registers.a >> 1);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.c = (orig_value & 0b0000_0001) != 0;
                    },
                    // Implemented opcode
                    0x48 => {
                        self.registers.a = self.add_impl_u8(self.registers.a, self.registers.a, 0);
                    },
                    // Implemented opcode
                    0x49 => {
                        let result = (self.registers.a << 1) | (self.registers.cc.c as u8);
                        self.registers.cc.c = (self.registers.a & 0b1000_0000) != 0;
                        self.registers.cc.v = ((self.registers.a & 0b1000_0000) ^ ((self.registers.a & 0b0100_0000) << 1)) != 0;
                        self.registers.cc.n = Self::calc_negative_u8(result);
                        self.registers.cc.z = Self::calc_zero_u8(result);
                        self.registers.a = result;
                    },
                    // Implemented opcode
                    0x4A => {
                        let orig_value = self.registers.a;
                        self.registers.a = self.registers.a.wrapping_sub(1);
                        self.registers.cc.v = orig_value == 0b1000_0000;
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        // Note: DEC does NOT modify Carry flag in 6809
                    },
                    // ILLEGAL OPCODE: Cannot use indexed addressing for ADD operations  
                    0x4B => {
                        panic!("Illegal opcode 0x4B: Invalid indexed addressing for ADD");
                    },
                    // Implemented opcode
                    0x4C => {
                        let orig_value = self.registers.a;
                        self.registers.a = self.registers.a.wrapping_add(1);
                        self.registers.cc.v = orig_value == 0b0111_1111;
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        // Note: INC does NOT modify Carry flag in 6809
                    },
                    // Implemented opcode
                    0x4D => {
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                        // Note: TST does NOT modify Carry flag in 6809
                    },
                    // ILLEGAL OPCODE: Invalid postbyte for JMP indexed
                    0x4E => {
                        panic!("Illegal opcode 0x4E: Invalid indexed addressing postbyte for JMP");
                    },
                    // Implemented opcode
                    0x4F => {
                        self.registers.a = 0;
                        self.registers.cc.n = false;
                        self.registers.cc.z = true;
                        self.registers.cc.v = false;
                        self.registers.cc.c = false;
                    },

                    // Implemented opcode
                    0x50 => {
                        self.registers.b = self.subtract_impl_u8(0, self.registers.b, 0);
                    },
                    // ILLEGAL OPCODE: Cannot use indexed addressing for CMPB
                    0x51 => {
                        panic!("Illegal opcode 0x51: Invalid indexed addressing for CMPB");
                    },
                    // ILLEGAL OPCODE: Cannot use indexed addressing for SBCB
                    0x52 => {
                        panic!("Illegal opcode 0x52: Invalid indexed addressing for SBCB");
                    },
                    // Implemented opcode
                    0x53 => {
                        self.registers.b = !self.registers.b;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                        self.registers.cc.c = true;
                    },
                    // Implemented opcode
                    0x54 => {
                        let orig_value = self.registers.b;
                        self.registers.b = self.registers.b >> 1;
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.n = false; // Bit 7 always shifted out
                        self.registers.cc.c = (orig_value & 0b0000_0001) != 0;
                    },
                    // ILLEGAL OPCODE: Cannot use indexed addressing for BITB
                    0x55 => {
                        panic!("Illegal opcode 0x55: Invalid indexed addressing for BITB");
                    },
                    // Implemented opcode
                    0x56 => {
                        let result = ((self.registers.cc.c as u8) << 7) | (self.registers.b >> 1);
                        self.registers.cc.c = (self.registers.b & 0b0000_0001) != 0;
                        self.registers.cc.n = Self::calc_negative_u8(result);
                        self.registers.cc.z = Self::calc_zero_u8(result);
                        self.registers.b = result;
                    },
                    // Implemented opcode
                    0x57 => {
                        let orig_value = self.registers.b;
                        self.registers.b = (orig_value & 0b1000_0000) | (self.registers.b >> 1);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.c = (orig_value & 0b0000_0001) != 0;
                    },
                    // Implemented opcode
                    0x58 => {
                        self.registers.b = self.add_impl_u8(self.registers.b, self.registers.b, 0);
                    },
                    // Implemented opcode
                    0x59 => {
                        let result = (self.registers.b << 1) | (self.registers.cc.c as u8);
                        self.registers.cc.c = (self.registers.b & 0b1000_0000) != 0;
                        self.registers.cc.v = ((self.registers.b & 0b1000_0000) ^ ((self.registers.b & 0b0100_0000) << 1)) != 0;
                        self.registers.cc.n = Self::calc_negative_u8(result);
                        self.registers.cc.z = Self::calc_zero_u8(result);
                        self.registers.b = result;
                    },
                    // Implemented opcode
                    0x5A => {
                        let orig_value = self.registers.b;
                        self.registers.b = self.registers.b.wrapping_sub(1);
                        self.registers.cc.v = orig_value == 0b1000_0000;
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        // Note: DEC does NOT modify Carry flag in 6809
                    },
                    // ILLEGAL OPCODE: Cannot use indexed addressing for ADDB
                    0x5B => {
                        panic!("Illegal opcode 0x5B: Invalid indexed addressing for ADDB");
                    },
                    // Implemented opcode
                    0x5C => {
                        let orig_value = self.registers.b;
                        self.registers.b = self.registers.b.wrapping_add(1);
                        self.registers.cc.v = orig_value == 0b0111_1111;
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        // Note: INC does NOT modify Carry flag in 6809
                    },
                    // Implemented opcode
                    0x5D => {
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                        // Note: TST does NOT modify Carry flag in 6809
                    },
                    // ILLEGAL OPCODE: Invalid postbyte for JMP indexed
                    0x5E => {
                        panic!("Illegal opcode 0x5E: Invalid indexed addressing postbyte for JMP");
                    },
                    // Implemented opcode
                    0x5F => {
                        self.registers.b = 0;
                        self.registers.cc.n = false;
                        self.registers.cc.z = true;
                        self.registers.cc.v = false;
                        self.registers.cc.c = false;
                    },

                    // Implemented opcode
                    0x60 => {
                        self.op_neg_memory(AddressingMode::Indexed);
                    },
                    // ILLEGAL OPCODE: Cannot use indexed addressing for CMP
                    0x61 => {
                        panic!("Illegal opcode 0x61: Invalid indexed addressing for CMP");
                    },
                    // ILLEGAL OPCODE: Cannot use indexed addressing for SBC
                    0x62 => {
                        panic!("Illegal opcode 0x62: Invalid indexed addressing for SBC");
                    },
                    // Implemented opcode
                    0x63 => {
                        self.op_com_memory(AddressingMode::Indexed);
                    },
                    // Implemented opcode
                    0x64 => {
                        self.op_lsr_memory(AddressingMode::Indexed);
                    },
                    // ILLEGAL OPCODE: Cannot use indexed addressing for BIT
                    0x65 => {
                        panic!("Illegal opcode 0x65: Invalid indexed addressing for BIT");
                    },
                    // Implemented opcode
                    0x66 => {
                        self.op_ror_memory(AddressingMode::Indexed);
                    },
                    // Implemented opcode
                    0x67 => {
                        self.op_asr_memory(AddressingMode::Indexed);
                    },
                    // Implemented opcode
                    0x68 => {
                        self.op_asl_memory(AddressingMode::Indexed);
                    },
                    // Implemented opcode
                    0x69 => {
                        self.op_rol_memory(AddressingMode::Indexed);
                    },
                    // Implemented opcode
                    0x6A => {
                        self.op_dec_memory(AddressingMode::Indexed);
                    },
                    // ILLEGAL OPCODE: Cannot use indexed addressing for ADD
                    0x6B => {
                        panic!("Illegal opcode 0x6B: Invalid indexed addressing for ADD");
                    },
                    // Implemented opcode
                    0x6C => {
                        self.op_inc_memory(AddressingMode::Indexed);
                    },
                    // Implemented opcode
                    0x6D => {
                        self.op_tst_memory(AddressingMode::Indexed);
                    },
                    // Implemented opcode 0x6E - JMP indexed
                    // C++ Original: JMP indexed - Sets PC to effective address
                    0x6E => {
                        let ea = self.read_indexed_ea();
                        self.registers.pc = ea;
                    },
                    // Implemented opcode
                    0x6F => {
                        self.op_clr_memory(AddressingMode::Indexed);
                    },

                    // Implemented opcode
                    0x70 => {
                        self.op_neg_memory(AddressingMode::Extended);
                    },
                    // ILLEGAL OPCODE: Cannot use extended addressing for CMP
                    0x71 => {
                        panic!("Illegal opcode 0x71: Invalid extended addressing for CMP");
                    },
                    // ILLEGAL OPCODE: Cannot use extended addressing for SBC
                    0x72 => {
                        panic!("Illegal opcode 0x72: Invalid extended addressing for SBC");
                    },
                    // Implemented opcode
                    0x73 => {
                        self.op_com_memory(AddressingMode::Extended);
                    },
                    // Implemented opcode
                    0x74 => {
                        self.op_lsr_memory(AddressingMode::Extended);
                    },
                    // ILLEGAL OPCODE: Cannot use extended addressing for BIT
                    0x75 => {
                        panic!("Illegal opcode 0x75: Invalid extended addressing for BIT");
                    },
                    // Implemented opcode
                    0x76 => {
                        self.op_ror_memory(AddressingMode::Extended);
                    },
                    // Implemented opcode
                    0x77 => {
                        self.op_asr_memory(AddressingMode::Extended);
                    },
                    // Implemented opcode
                    0x78 => {
                        self.op_asl_memory(AddressingMode::Extended);
                    },
                    // Implemented opcode
                    0x79 => {
                        self.op_rol_memory(AddressingMode::Extended);
                    },
                    // Implemented opcode
                    0x7A => {
                        self.op_dec_memory(AddressingMode::Extended);
                    },
                    // ILLEGAL OPCODE: Cannot use extended addressing for ADD
                    0x7B => {
                        panic!("Illegal opcode 0x7B: Invalid extended addressing for ADD");
                    },
                    // Implemented opcode
                    0x7C => {
                        self.op_inc_memory(AddressingMode::Extended);
                    },
                    // Implemented opcode
                    0x7D => {
                        self.op_tst_memory(AddressingMode::Extended);
                    },
                    // Implemented opcode 0x7E - JMP extended
                    // C++ Original: JMP extended - Sets PC to effective address
                    0x7E => {
                        let ea = self.read_extended_ea();
                        self.registers.pc = ea;
                    },
                    // Implemented opcode
                    0x7F => {
                        self.op_clr_memory(AddressingMode::Extended);
                    },

                    // Implemented opcode
                    0x80 => {
                        let operand = self.read_pc8();
                        self.registers.a = self.subtract_impl_u8(self.registers.a, operand, 0);
                    },
                    // Implemented opcode
                    0x81 => {
                        let operand = self.read_pc8();
                        self.subtract_impl_u8(self.registers.a, operand, 0);
                    },
                    // Implemented opcode
                    0x82 => {
                        let operand = self.read_pc8();
                        self.registers.a = self.subtract_impl_u8(self.registers.a, operand, self.registers.cc.c as u8);
                    },
                    // Implemented opcode
                    0x83 => {
                        let operand = self.read_pc16();
                        let d_value = self.registers.d();
                        let result = self.subtract_impl_u16(d_value, operand, 0);
                        self.registers.set_d(result);
                    },
                    // Implemented opcode
                    0x84 => {
                        let operand = self.read_pc8();
                        self.registers.a = self.registers.a & operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0x85 => {
                        let operand = self.read_pc8();
                        let result = self.registers.a & operand;
                        self.registers.cc.n = Self::calc_negative_u8(result);
                        self.registers.cc.z = Self::calc_zero_u8(result);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0x86 => {
                        // LDA #immediate
                        let value = self.op_ld_8(opcode_byte);
                        self.registers.a = value;
                    },
                    // ILLEGAL OPCODE: Cannot store to immediate value (STA immediate)
                    0x87 => {
                        panic!("Illegal opcode 0x87: STA immediate - cannot store A register to literal value");
                    },
                    // Implemented opcode
                    0x88 => {
                        let operand = self.read_pc8();
                        self.registers.a = self.registers.a ^ operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0x89 => {
                        let operand = self.read_pc8();
                        self.registers.a = self.add_impl_u8(self.registers.a, operand, self.registers.cc.c as u8);
                    },
                    // Implemented opcode
                    0x8A => {
                        let operand = self.read_pc8();
                        self.registers.a = self.registers.a | operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0x8B => {
                        let operand = self.read_pc8();
                        self.registers.a = self.add_impl_u8(self.registers.a, operand, 0);
                    },
                    // Implemented opcode
                    0x8C => {
                        let operand = self.read_pc16();
                        self.subtract_impl_u16(self.registers.x, operand, 0);
                    },
                    // Implemented opcode
                    0x8D => {
                        self.op_jsr(AddressingMode::Direct);
                    },
                    // Implemented opcode
                    0x8E => {
                        // LDX #immediate
                        let value = self.op_ld_16(opcode_byte);
                        self.registers.x = value;
                    },
                    // ILLEGAL OPCODE: Cannot store to immediate value (STX immediate)
                    0x8F => {
                        panic!("Illegal opcode 0x8F: STX immediate - cannot store X register to literal value");
                    },

                    // Implemented opcode
                    0x90 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.subtract_impl_u8(self.registers.a, operand, 0);
                    },
                    // Implemented opcode
                    0x91 => {
                        let operand = self.read_operand_value8(opcode_byte);
                        self.subtract_impl_u8(self.registers.a, operand, 0);
                    },
                    // Implemented opcode
                    0x92 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.subtract_impl_u8(self.registers.a, operand, self.registers.cc.c as u8);
                    },
                    // Implemented opcode
                    0x93 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read16(ea);
                        let d_value = self.registers.d();
                        let result = self.subtract_impl_u16(d_value, operand, 0);
                        self.registers.set_d(result);
                    },
                    // Implemented opcode
                    0x94 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.registers.a & operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0x95 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        let result = self.registers.a & operand;
                        self.registers.cc.n = Self::calc_negative_u8(result);
                        self.registers.cc.z = Self::calc_zero_u8(result);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0x96 => {
                        // LDA direct
                        let value = self.op_ld_8(opcode_byte);
                        self.registers.a = value;
                    },
                    // Implemented opcode
                    0x97 => {
                        // STA direct
                        self.op_st_8(self.registers.a, opcode_byte);
                    },
                    // Implemented opcode
                    0x98 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.registers.a ^ operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0x99 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.add_impl_u8(self.registers.a, operand, self.registers.cc.c as u8);
                    },
                    // Implemented opcode
                    0x9A => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.registers.a | operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0x9B => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.add_impl_u8(self.registers.a, operand, 0);
                    },
                    // Implemented opcode
                    0x9C => {
                        let operand = self.read_operand_value16(opcode_byte);
                        self.subtract_impl_u16(self.registers.x, operand, 0);
                    },
                    // Implemented opcode
                    0x9D => {
                        // JSR direct - fixed: was incorrectly using AddressingMode::Indexed
                        self.op_jsr(AddressingMode::Direct);
                    },
                    // Implemented opcode
                    0x9E => {
                        // LDX direct
                        let value = self.op_ld_16(opcode_byte);
                        self.registers.x = value;
                    },
                    // Implemented opcode
                    0x9F => {
                        // STX direct
                        self.op_st_16(self.registers.x, opcode_byte);
                    },

                    // Implemented opcode
                    0xA0 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.subtract_impl_u8(self.registers.a, operand, 0);
                    },
                    // Implemented opcode
                    0xA1 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.subtract_impl_u8(self.registers.a, operand, 0);
                    },
                    // Implemented opcode
                    0xA2 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.subtract_impl_u8(self.registers.a, operand, self.registers.cc.c as u8);
                    },
                    // Implemented opcode
                    0xA3 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read16(ea);
                        let d_value = self.registers.d();
                        let result = self.subtract_impl_u16(d_value, operand, 0);
                        self.registers.set_d(result);
                    },
                    // Implemented opcode
                    0xA4 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.registers.a & operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xA5 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        let result = self.registers.a & operand;
                        self.registers.cc.n = Self::calc_negative_u8(result);
                        self.registers.cc.z = Self::calc_zero_u8(result);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xA6 => {
                        // LDA indexed
                        let value = self.op_ld_8(opcode_byte);
                        self.registers.a = value;
                    },
                    // Implemented opcode
                    0xA7 => {
                        // STA indexed
                        self.op_st_8(self.registers.a, opcode_byte);
                    },
                    // Implemented opcode
                    0xA8 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.registers.a ^ operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xA9 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.add_impl_u8(self.registers.a, operand, self.registers.cc.c as u8);
                    },
                    // Implemented opcode
                    0xAA => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.registers.a | operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xAB => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.add_impl_u8(self.registers.a, operand, 0);
                    },
                    // Implemented opcode
                    0xAC => {
                        let operand = self.read_operand_value16(opcode_byte);
                        self.subtract_impl_u16(self.registers.x, operand, 0);
                    },
                    // Implemented opcode
                    0xAD => {
                        // JSR indexed - Jump to Subroutine using indexed addressing
                        // Push return address (PC) onto system stack, then jump to EA
                        self.op_jsr(AddressingMode::Indexed);
                    },
                    // Implemented opcode
                    0xAE => {
                        // LDX indexed
                        let value = self.op_ld_16(opcode_byte);
                        self.registers.x = value;
                    },
                    // Implemented opcode
                    0xAF => {
                        // STX indexed
                        self.op_st_16(self.registers.x, opcode_byte);
                    },

                    // Implemented opcode
                    0xB0 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.subtract_impl_u8(self.registers.a, operand, 0);
                    },
                    // Implemented opcode
                    0xB1 => {
                        let operand = self.read_operand_value8(opcode_byte);
                        self.subtract_impl_u8(self.registers.a, operand, 0);
                    },
                    // Implemented opcode
                    0xB2 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.subtract_impl_u8(self.registers.a, operand, self.registers.cc.c as u8);
                    },
                    // Implemented opcode
                    0xB3 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read16(ea);
                        let d_value = self.registers.d();
                        let result = self.subtract_impl_u16(d_value, operand, 0);
                        self.registers.set_d(result);
                    },
                    // Implemented opcode
                    0xB4 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.registers.a & operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xB5 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        let result = self.registers.a & operand;
                        self.registers.cc.n = Self::calc_negative_u8(result);
                        self.registers.cc.z = Self::calc_zero_u8(result);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xB6 => {
                        // LDA extended
                        let value = self.op_ld_8(opcode_byte);
                        self.registers.a = value;
                    },
                    // Implemented opcode
                    0xB7 => {
                        // STA extended
                        self.op_st_8(self.registers.a, opcode_byte);
                    },
                    // Implemented opcode
                    0xB8 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.registers.a ^ operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xB9 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.add_impl_u8(self.registers.a, operand, self.registers.cc.c as u8);
                    },
                    // Implemented opcode
                    0xBA => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.registers.a | operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xBB => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.a = self.add_impl_u8(self.registers.a, operand, 0);
                    },
                    // Implemented opcode
                    0xBC => {
                        let operand = self.read_operand_value16(opcode_byte);
                        self.subtract_impl_u16(self.registers.x, operand, 0);
                    },
                    // Implemented opcode
                    0xBD => {
                        self.op_jsr(AddressingMode::Extended);
                    },
                    // Implemented opcode
                    0xBE => {
                        // LDX extended
                        let value = self.op_ld_16(opcode_byte);
                        self.registers.x = value;
                    },
                    // Implemented opcode
                    0xBF => {
                        // STX extended
                        self.op_st_16(self.registers.x, opcode_byte);
                    },

                    // Implemented opcode
                    0xC0 => {
                        let operand = self.read_pc8();
                        self.registers.b = self.subtract_impl_u8(self.registers.b, operand, 0);
                    },
                    // Implemented opcode
                    0xC1 => {
                        let operand = self.read_pc8();
                        self.subtract_impl_u8(self.registers.b, operand, 0);
                    },
                    // Implemented opcode
                    0xC2 => {
                        let operand = self.read_pc8();
                        self.registers.b = self.subtract_impl_u8(self.registers.b, operand, self.registers.cc.c as u8);
                    },
                    // Implemented opcode
                    0xC3 => {
                        let operand = self.read_pc16();
                        let d_value = self.registers.d();
                        let result = self.add_impl_u16(d_value, operand, 0);
                        self.registers.set_d(result);
                    },
                    // Implemented opcode
                    0xC4 => {
                        let operand = self.read_pc8();
                        self.registers.b = self.registers.b & operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xC5 => {
                        let operand = self.read_pc8();
                        let result = self.registers.b & operand;
                        self.registers.cc.n = Self::calc_negative_u8(result);
                        self.registers.cc.z = Self::calc_zero_u8(result);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xC6 => {
                        // LDB #immediate
                        let value = self.op_ld_8(opcode_byte);
                        self.registers.b = value;
                    },
                    // ILLEGAL OPCODE: Cannot store to immediate value (STB immediate)
                    0xC7 => {
                        panic!("Illegal opcode 0xC7: STB immediate - cannot store B register to literal value");
                    },
                    // Implemented opcode
                    0xC8 => {
                        let operand = self.read_pc8();
                        self.registers.b = self.registers.b ^ operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xC9 => {
                        let operand = self.read_pc8();
                        self.registers.b = self.add_impl_u8(self.registers.b, operand, self.registers.cc.c as u8);
                    },
                    // Implemented opcode
                    0xCA => {
                        let operand = self.read_pc8();
                        self.registers.b = self.registers.b | operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xCB => {
                        let operand = self.read_pc8();
                        self.registers.b = self.add_impl_u8(self.registers.b, operand, 0);
                    },
                    // Implemented opcode
                    0xCC => {
                        self.op_ld_16_d(opcode_byte); // LDD #immediate
                    },
                    // ILLEGAL OPCODE: Cannot store to immediate value (STD immediate)
                    0xCD => {
                        panic!("Illegal opcode 0xCD: STD immediate - cannot store D register to literal value");
                    },
                    // Implemented opcode
                    0xCE => {
                        // LDU #immediate
                        let value = self.op_ld_16(opcode_byte);
                        self.registers.u = value;
                    },
                    // ILLEGAL OPCODE: Cannot store to immediate value (STU immediate)
                    0xCF => {
                        panic!("Illegal opcode 0xCF: STU immediate - cannot store U register to literal value");
                    },

                    // Implemented opcode
                    0xD0 => {
                        // SUBB direct - Subtract from B using direct addressing
                        let operand = self.read_operand_value8(opcode_byte);
                        self.registers.b = self.subtract_impl_u8(self.registers.b, operand, 0);
                    },
                    // Implemented opcode
                    0xD1 => {
                        let operand = self.read_operand_value8(opcode_byte);
                        self.subtract_impl_u8(self.registers.b, operand, 0);
                    },
                    // Implemented opcode
                    0xD2 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.subtract_impl_u8(self.registers.b, operand, self.registers.cc.c as u8);
                    },
                    // Implemented opcode
                    0xD3 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read16(ea);
                        let d_value = self.registers.d();
                        let result = self.add_impl_u16(d_value, operand, 0);
                        self.registers.set_d(result);
                    },
                    // Implemented opcode
                    0xD4 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.registers.b & operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xD5 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        let result = self.registers.b & operand;
                        self.registers.cc.n = Self::calc_negative_u8(result);
                        self.registers.cc.z = Self::calc_zero_u8(result);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xD6 => {
                        // LDB direct
                        let value = self.op_ld_8(opcode_byte);
                        self.registers.b = value;
                    },
                    // Implemented opcode
                    0xD7 => {
                        // STB direct
                        self.op_st_8(self.registers.b, opcode_byte);
                    },
                    // Implemented opcode
                    0xD8 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.registers.b ^ operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xD9 => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.add_impl_u8(self.registers.b, operand, self.registers.cc.c as u8);
                    },
                    // Implemented opcode
                    0xDA => {
                        let ea = self.read_direct_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.registers.b | operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xDB => {
                        // ADDB direct - Add to B using direct addressing
                        let operand = self.read_operand_value8(opcode_byte);
                        self.registers.b = self.add_impl_u8(self.registers.b, operand, 0);
                    },
                    // Implemented opcode
                    0xDC => {
                        self.op_ld_16_d(opcode_byte); // LDD direct
                    },
                    // Implemented opcode
                    0xDD => {
                        // STD direct
                        self.op_st_16(self.registers.d(), opcode_byte);
                    },
                    // Implemented opcode
                    0xDE => {
                        // LDU direct
                        let value = self.op_ld_16(opcode_byte);
                        self.registers.u = value;
                    },
                    // Implemented opcode
                    0xDF => {
                        // STU direct
                        self.op_st_16(self.registers.u, opcode_byte);
                    },

                    // Implemented opcode
                    0xE0 => {
                        // SUBB indexed - Subtract from B using indexed addressing
                        let operand = self.read_operand_value8(opcode_byte);
                        self.registers.b = self.subtract_impl_u8(self.registers.b, operand, 0);
                    },
                    // Implemented opcode
                    0xE1 => {
                        let operand = self.read_operand_value8(opcode_byte);
                        self.subtract_impl_u8(self.registers.b, operand, 0);
                    },
                    // Implemented opcode
                    0xE2 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.subtract_impl_u8(self.registers.b, operand, self.registers.cc.c as u8);
                    },
                    // Implemented opcode
                    0xE3 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read16(ea);
                        let d_value = self.registers.d();
                        let result = self.add_impl_u16(d_value, operand, 0);
                        self.registers.set_d(result);
                    },
                    // Implemented opcode
                    0xE4 => {
                        // ANDB indexed - AND B with memory using indexed addressing
                        let operand = self.read_operand_value8(opcode_byte);
                        self.registers.b = self.registers.b & operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xE5 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        let result = self.registers.b & operand;
                        self.registers.cc.n = Self::calc_negative_u8(result);
                        self.registers.cc.z = Self::calc_zero_u8(result);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xE6 => {
                        // LDB indexed
                        let value = self.op_ld_8(opcode_byte);
                        self.registers.b = value;
                    },
                    // Implemented opcode
                    0xE7 => {
                        // STB indexed
                        self.op_st_8(self.registers.b, opcode_byte);
                    },
                    // Implemented opcode
                    0xE8 => {
                        // EORB indexed - Exclusive OR B with memory using indexed addressing
                        let operand = self.read_operand_value8(opcode_byte);
                        self.registers.b = self.registers.b ^ operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xE9 => {
                        let ea = self.read_indexed_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.add_impl_u8(self.registers.b, operand, self.registers.cc.c as u8);
                    },
                    // Implemented opcode
                    0xEA => {
                        // ORAB indexed - OR B with memory using indexed addressing
                        let operand = self.read_operand_value8(opcode_byte);
                        self.registers.b = self.registers.b | operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xEB => {
                        // ADDB indexed - Add to B using indexed addressing
                        let operand = self.read_operand_value8(opcode_byte);
                        self.registers.b = self.add_impl_u8(self.registers.b, operand, 0);
                    },
                    // Implemented opcode
                    0xEC => {
                        self.op_ld_16_d(opcode_byte); // LDD indexed
                    },
                    // Implemented opcode
                    0xED => {
                        // STD indexed
                        self.op_st_16(self.registers.d(), opcode_byte);
                    },
                    // Implemented opcode
                    0xEE => {
                        // LDU indexed
                        let value = self.op_ld_16(opcode_byte);
                        self.registers.u = value;
                    },
                    // Implemented opcode
                    0xEF => {
                        // STU indexed
                        self.op_st_16(self.registers.u, opcode_byte);
                    },

                    // Implemented opcode
                    0xF0 => {
                        // SUBB extended - Subtract from B using extended addressing
                        let operand = self.read_operand_value8(opcode_byte);
                        self.registers.b = self.subtract_impl_u8(self.registers.b, operand, 0);
                    },
                    // Implemented opcode
                    0xF1 => {
                        let operand = self.read_operand_value8(opcode_byte);
                        self.subtract_impl_u8(self.registers.b, operand, 0);
                    },
                    // Implemented opcode
                    0xF2 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.subtract_impl_u8(self.registers.b, operand, self.registers.cc.c as u8);
                    },
                    // Implemented opcode
                    0xF3 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read16(ea);
                        let d_value = self.registers.d();
                        let result = self.add_impl_u16(d_value, operand, 0);
                        self.registers.set_d(result);
                    },
                    // Implemented opcode
                    0xF4 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.registers.b & operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xF5 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        let result = self.registers.b & operand;
                        self.registers.cc.n = Self::calc_negative_u8(result);
                        self.registers.cc.z = Self::calc_zero_u8(result);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xF6 => {
                        // LDB extended
                        let value = self.op_ld_8(opcode_byte);
                        self.registers.b = value;
                    },
                    // Implemented opcode
                    0xF7 => {
                        // STB extended
                        self.op_st_8(self.registers.b, opcode_byte);
                    },
                    // Implemented opcode
                    0xF8 => {
                        // EORB extended - Exclusive OR B with memory using extended addressing
                        let operand = self.read_operand_value8(opcode_byte);
                        self.registers.b = self.registers.b ^ operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xF9 => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.add_impl_u8(self.registers.b, operand, self.registers.cc.c as u8);
                    },
                    // Implemented opcode
                    0xFA => {
                        let ea = self.read_extended_ea();
                        let operand = self.read8(ea);
                        self.registers.b = self.registers.b | operand;
                        self.registers.cc.n = Self::calc_negative_u8(self.registers.b);
                        self.registers.cc.z = Self::calc_zero_u8(self.registers.b);
                        self.registers.cc.v = false;
                    },
                    // Implemented opcode
                    0xFB => {
                        // ADDB extended - Add to B using extended addressing
                        let operand = self.read_operand_value8(opcode_byte);
                        self.registers.b = self.add_impl_u8(self.registers.b, operand, 0);
                    },
                    // Implemented opcode
                    0xFC => {
                        self.op_ld_16_d(opcode_byte); // LDD extended
                    },
                    // Implemented opcode
                    0xFD => {
                        // STD extended
                        self.op_st_16(self.registers.d(), opcode_byte);
                    },
                    // Implemented opcode
                    0xFE => {
                        // LDU extended
                        let value = self.op_ld_16(opcode_byte);
                        self.registers.u = value;
                    },
                    // Implemented opcode
                    0xFF => {
                        // STU extended
                        self.op_st_16(self.registers.u, opcode_byte);
                    },
                }
            },
            1 => {
                // Page 1 instructions (0x10xx)
                match opcode_byte {
                    // C++ Original: OpCMP<1, 0x83>(D); - CMPD immediate
                    // Compare D register (16-bit combination of A and B)
                    0x83 => {
                        let operand = self.read_pc16();
                        let d_value = ((self.registers.a as u16) << 8) | (self.registers.b as u16);
                        self.subtract_impl_u16(d_value, operand, 0);
                    },

                    // C++ Original: OpCMP<1, 0x93>(D); - CMPD direct
                    0x93 => {
                        let operand = self.read_operand_value16_page1(opcode_byte);
                        let d_value = ((self.registers.a as u16) << 8) | (self.registers.b as u16);
                        self.subtract_impl_u16(d_value, operand, 0);
                    },

                    // C++ Original: OpCMP<1, 0xA3>(D); - CMPD indexed
                    0xA3 => {
                        let operand = self.read_operand_value16_page1(opcode_byte);
                        let d_value = ((self.registers.a as u16) << 8) | (self.registers.b as u16);
                        self.subtract_impl_u16(d_value, operand, 0);
                    },

                    // C++ Original: OpCMP<1, 0xB3>(D); - CMPD extended
                    0xB3 => {
                        let operand = self.read_operand_value16_page1(opcode_byte);
                        let d_value = ((self.registers.a as u16) << 8) | (self.registers.b as u16);
                        self.subtract_impl_u16(d_value, operand, 0);
                    },

                    // C++ Original: OpCMP<1, 0x8C>(Y); - CMPY immediate  
                    0x8C => {
                        let operand = self.read_pc16();
                        self.subtract_impl_u16(self.registers.y, operand, 0);
                    },

                    // C++ Original: OpCMP<1, 0x9C>(Y); - CMPY direct
                    0x9C => {
                        let operand = self.read_operand_value16_page1(opcode_byte);
                        self.subtract_impl_u16(self.registers.y, operand, 0);
                    },

                    // C++ Original: OpCMP<1, 0xAC>(Y); - CMPY indexed
                    0xAC => {
                        let operand = self.read_operand_value16_page1(opcode_byte);
                        self.subtract_impl_u16(self.registers.y, operand, 0);
                    },

                    // C++ Original: OpCMP<1, 0xBC>(Y); - CMPY extended
                    0xBC => {
                        let operand = self.read_operand_value16_page1(opcode_byte);
                        self.subtract_impl_u16(self.registers.y, operand, 0);
                    },

                    _ => {
                        panic!("Unhandled page 1 opcode: {:02X}", opcode_byte);
                    }
                }
            },
            2 => {
                // Page 2 instructions (0x11xx)
                match opcode_byte {
                    // C++ Original: OpCMP<2, 0x83>(U); - CMPU immediate
                    // Compare U register (16-bit user stack pointer)
                    0x83 => {
                        let operand = self.read_pc16();
                        self.subtract_impl_u16(self.registers.u, operand, 0);
                    },

                    // C++ Original: OpCMP<2, 0x93>(U); - CMPU direct
                    0x93 => {
                        let operand = self.read_operand_value16_page2(opcode_byte);
                        self.subtract_impl_u16(self.registers.u, operand, 0);
                    },

                    // C++ Original: OpCMP<2, 0xA3>(U); - CMPU indexed
                    0xA3 => {
                        let operand = self.read_operand_value16_page2(opcode_byte);
                        self.subtract_impl_u16(self.registers.u, operand, 0);
                    },

                    // C++ Original: OpCMP<2, 0xB3>(U); - CMPU extended
                    0xB3 => {
                        let operand = self.read_operand_value16_page2(opcode_byte);
                        self.subtract_impl_u16(self.registers.u, operand, 0);
                    },

                    // C++ Original: OpCMP<2, 0x8C>(S); - CMPS immediate
                    // Compare S register (16-bit system stack pointer)
                    0x8C => {
                        let operand = self.read_pc16();
                        self.subtract_impl_u16(self.registers.s, operand, 0);
                    },

                    // C++ Original: OpCMP<2, 0x9C>(S); - CMPS direct
                    0x9C => {
                        let operand = self.read_operand_value16_page2(opcode_byte);
                        self.subtract_impl_u16(self.registers.s, operand, 0);
                    },

                    // C++ Original: OpCMP<2, 0xAC>(S); - CMPS indexed
                    0xAC => {
                        let operand = self.read_operand_value16_page2(opcode_byte);
                        self.subtract_impl_u16(self.registers.s, operand, 0);
                    },

                    // C++ Original: OpCMP<2, 0xBC>(S); - CMPS extended
                    0xBC => {
                        let operand = self.read_operand_value16_page2(opcode_byte);
                        self.subtract_impl_u16(self.registers.s, operand, 0);
                    },

                    _ => {
                        panic!("Unhandled page 2 opcode: {:02X}", opcode_byte);
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

    fn read_operand_value16_page1(&mut self, opcode: u8) -> u16 {
        let addressing_mode = self.get_addressing_mode_for_opcode_page1(opcode);
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
            _ => panic!("Invalid addressing mode for 16-bit page 1 read: {:?}", addressing_mode)
        }
    }

    fn read_operand_value16_page2(&mut self, opcode: u8) -> u16 {
        let addressing_mode = self.get_addressing_mode_for_opcode_page2(opcode);
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
            _ => panic!("Invalid addressing mode for 16-bit page 2 read: {:?}", addressing_mode)
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
            // Immediate addressing - includes SBCA, BITA, ADCA immediate for A register and SBCB, BITB, ADCB immediate for B register, ADDD immediate, SUBD immediate, ORCC, ANDCC
            0x86 | 0xC6 | 0x8E | 0xCC | 0xCE | 0x81 | 0xC1 | 0x8C | 0x8A | 0x34 | 0x35 | 0x36 | 0x37 | 0x1E | 0x1F | 0x82 | 0x85 | 0x89 | 0xC2 | 0xC5 | 0xC9 | 0xC3 | 0x83 | 0x1A | 0x1C => AddressingMode::Immediate,
            // Direct addressing - includes SBCA, BITA, ADCA direct for A register and SBCB, BITB, ADCB direct for B register, ADDD direct, SUBD direct, SUBB direct, ANDB direct, EORB direct, ORAB direct, ADDB direct
            0x96 | 0xD6 | 0x9E | 0xDC | 0xDE | 0x97 | 0xD7 | 0x9F | 0xDD | 0xDF | 0x91 | 0xD1 | 0x9C | 0x90 | 0x94 | 0x98 | 0x9A | 0x9B | 0x0F | 0x00 | 0x03 | 0x0A | 0x0C | 0x0D | 0x07 | 0x04 | 0x09 | 0x06 | 0x08 | 0x92 | 0x95 | 0x99 | 0xD2 | 0xD5 | 0xD9 | 0xD3 | 0x93 | 0xD0 | 0xD4 | 0xD8 | 0xDA | 0xDB => AddressingMode::Direct,
            // Indexed addressing - includes SBCA, BITA, ADCA indexed for A register and SBCB, BITB, ADCB indexed for B register, ADDD indexed, SUBD indexed, SUBB indexed, ANDB indexed, EORB indexed, ORAB indexed, ADDB indexed
            0xA6 | 0xE6 | 0xAE | 0xEC | 0xEE | 0xA7 | 0xE7 | 0xAF | 0xED | 0xEF | 0xA1 | 0xE1 | 0xAC | 0xA0 | 0xA4 | 0xA8 | 0xAA | 0xAB | 0x30 | 0x31 | 0x32 | 0x33 | 0x6F | 0x60 | 0x63 | 0x6A | 0x6C | 0x6D | 0x67 | 0x64 | 0x69 | 0x66 | 0x68 | 0xA2 | 0xA5 | 0xA9 | 0xE2 | 0xE5 | 0xE9 | 0xE3 | 0xA3 | 0xE0 | 0xE4 | 0xE8 | 0xEA | 0xEB => AddressingMode::Indexed,
            // Extended addressing - includes SBCA, BITA, ADCA extended for A register and SBCB, BITB, ADCB extended for B register, ADDD extended, SUBD extended, SUBB extended, ANDB extended, EORB extended, ORAB extended, ADDB extended
            0xB6 | 0xF6 | 0xBE | 0xFC | 0xFE | 0xB7 | 0xF7 | 0xBF | 0xFD | 0xFF | 0xB1 | 0xF1 | 0xBC | 0xB0 | 0xB4 | 0xB8 | 0xBA | 0xBB | 0x7F | 0x70 | 0x73 | 0x7A | 0x7C | 0x7D | 0x77 | 0x74 | 0x79 | 0x76 | 0x78 | 0xB2 | 0xB5 | 0xB9 | 0xF2 | 0xF5 | 0xF9 | 0xF3 | 0xB3 | 0xF0 | 0xF4 | 0xF8 | 0xFA | 0xFB => AddressingMode::Extended,
            // Inherent addressing (no operand)
            0x4F | 0x5F | 0x3A | 0x19 | 0x39 | 0x40 | 0x43 | 0x4A | 0x4C | 0x4D | 0x50 | 0x53 | 0x5A | 0x5C | 0x5D | 0x47 | 0x44 | 0x49 | 0x46 | 0x48 | 0x57 | 0x54 | 0x59 | 0x56 | 0x58 | 0x3D | 0x1D => AddressingMode::Inherent,
            _ => panic!("Unknown addressing mode for opcode: {:02X}", opcode)
        }
    }

    // Helper to get addressing mode for page 1 opcodes
    fn get_addressing_mode_for_opcode_page1(&self, opcode: u8) -> AddressingMode {
        match opcode {
            // Immediate addressing - CMPD immediate, CMPY immediate
            0x83 | 0x8C => AddressingMode::Immediate,
            // Direct addressing - CMPD direct, CMPY direct
            0x93 | 0x9C => AddressingMode::Direct,
            // Indexed addressing - CMPD indexed, CMPY indexed
            0xA3 | 0xAC => AddressingMode::Indexed,
            // Extended addressing - CMPD extended, CMPY extended
            0xB3 | 0xBC => AddressingMode::Extended,
            _ => panic!("Unknown addressing mode for page 1 opcode: {:02X}", opcode)
        }
    }

    // Helper to get addressing mode for page 2 opcodes
    fn get_addressing_mode_for_opcode_page2(&self, opcode: u8) -> AddressingMode {
        match opcode {
            // Immediate addressing - CMPU immediate, CMPS immediate
            0x83 | 0x8C => AddressingMode::Immediate,
            // Direct addressing - CMPU direct, CMPS direct
            0x93 | 0x9C => AddressingMode::Direct,
            // Indexed addressing - CMPU indexed, CMPS indexed
            0xA3 | 0xAC => AddressingMode::Indexed,
            // Extended addressing - CMPU extended, CMPS extended
            0xB3 | 0xBC => AddressingMode::Extended,
            _ => panic!("Unknown addressing mode for page 2 opcode: {:02X}", opcode)
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
        self.push8(stack_pointer, (value & 0xFF) as u8); // Low
        self.push8(stack_pointer, (value >> 8) as u8);   // High
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
        combine_to_u16(high, low)
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

    // C++ Original: ReadIndexedEA but without adding cycles (for LEA instructions)
    // LEA instructions only need the effective address calculation, not the cycles
    // since they're already counted in the opcode lookup table
    fn calc_indexed_ea(&mut self) -> u16 {
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
            // Note: NO self.add_cycles(1) for LEA instructions
        } else {
            match postbyte & 0x0F {
                0x00 => { // ,R+
                    let reg_ptr = self.register_select_mut(postbyte);
                    ea = *reg_ptr;
                    *reg_ptr = reg_ptr.wrapping_add(1);
                    supports_indirect = false;
                    // Note: NO self.add_cycles(2) for LEA instructions
                }
                0x01 => { // ,R++
                    let reg_ptr = self.register_select_mut(postbyte);
                    ea = *reg_ptr;
                    *reg_ptr = reg_ptr.wrapping_add(2);
                    // Note: NO self.add_cycles(3) for LEA instructions
                }
                0x02 => { // ,-R
                    let reg_ptr = self.register_select_mut(postbyte);
                    *reg_ptr = reg_ptr.wrapping_sub(1);
                    ea = *reg_ptr;
                    supports_indirect = false;
                    // Note: NO self.add_cycles(2) for LEA instructions
                }
                0x03 => { // ,--R
                    let reg_ptr = self.register_select_mut(postbyte);
                    *reg_ptr = reg_ptr.wrapping_sub(2);
                    ea = *reg_ptr;
                    // Note: NO self.add_cycles(3) for LEA instructions
                }
                0x04 => { // ,R (zero offset)
                    ea = self.register_select(postbyte);
                    supports_indirect = false;
                    // Note: NO self.add_cycles(0) for LEA instructions
                }
                0x05 => { // (signed B),R
                    ea = self.register_select(postbyte).wrapping_add(self.registers.b as i8 as u16);
                    // Note: NO self.add_cycles(1) for LEA instructions
                }
                0x06 => { // (signed A),R
                    ea = self.register_select(postbyte).wrapping_add(self.registers.a as i8 as u16);
                    // Note: NO self.add_cycles(1) for LEA instructions
                }
                0x07 => {
                    panic!("Illegal indexed instruction post-byte");
                }
                0x08 => { // (signed 8-bit offset),R
                    let offset = self.read_pc8() as i8;
                    ea = self.register_select(postbyte).wrapping_add(offset as u16);
                    // Note: NO self.add_cycles(1) for LEA instructions
                }
                0x09 => { // (signed 16-bit offset),R
                    let msb = self.read_pc8();
                    let lsb = self.read_pc8();
                    let offset = combine_to_s16(msb, lsb);
                    ea = self.register_select(postbyte).wrapping_add(offset as u16);
                    // Note: NO self.add_cycles(4) for LEA instructions
                }
                0x0A => { // unused
                    panic!("Illegal indexed instruction post-byte");
                }
                0x0B => { // (signed D),R
                    ea = self.register_select(postbyte).wrapping_add(self.registers.d());
                    // Note: NO self.add_cycles(4) for LEA instructions
                }
                0x0C => { // (signed 8-bit offset),PC
                    let offset = self.read_pc8() as i8;
                    ea = self.registers.pc.wrapping_add(offset as u16);
                    // Note: NO self.add_cycles(1) for LEA instructions
                }
                0x0D => { // (signed 16-bit offset),PC
                    let postbyte2 = self.read_pc8();
                    let postbyte3 = self.read_pc8();
                    ea = self.registers.pc.wrapping_add(combine_to_s16(postbyte2, postbyte3) as u16);
                    // Note: NO self.add_cycles(5) for LEA instructions
                }
                0x0E => {
                    panic!("Illegal indexed instruction post-byte");
                }
                0x0F => { // [address] (Indirect-only)
                    let postbyte2 = self.read_pc8();
                    let postbyte3 = self.read_pc8();
                    ea = combine_to_s16(postbyte2, postbyte3) as u16;
                    // Note: NO self.add_cycles(2) for LEA instructions
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
            // Note: NO self.add_cycles(3) for LEA instructions
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

    // C++ Original: template <typename CondFunc> void OpBranch(CondFunc condFunc)
    fn op_branch<F>(&mut self, condition_func: F) 
    where 
        F: FnOnce() -> bool,
    {
        let offset = self.read_relative_offset8();
        if condition_func() {
            self.registers.pc = self.registers.pc.wrapping_add(offset as u16);
        }
    }

    // C++ Original: void OpRTS() { PC = Pop16(S); }
    fn op_rts(&mut self) {
        // C++ Original: PC = Pop16(S); inline implementation to avoid borrowing conflicts
        let high = self.read8(self.registers.s);
        self.registers.s = self.registers.s.wrapping_add(1);
        let low = self.read8(self.registers.s);
        self.registers.s = self.registers.s.wrapping_add(1);
        self.registers.pc = combine_to_u16(high, low);
    }

    // C++ Original: template <int page, uint8_t opCode> void OpJSR()
    fn op_jsr(&mut self, addressing_mode: AddressingMode) {
        let ea = match addressing_mode {
            AddressingMode::Direct => self.read_direct_ea(),
            AddressingMode::Indexed => self.read_indexed_ea(),
            AddressingMode::Extended => self.read_extended_ea(),
            _ => panic!("Invalid addressing mode for JSR: {:?}", addressing_mode),
        };
        // C++ Original: Push16(S, PC); inline implementation to avoid borrowing conflicts
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, (self.registers.pc & 0xFF) as u8); // Low
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.write8(self.registers.s, (self.registers.pc >> 8) as u8);   // High
        self.registers.pc = ea;
    }

    // C++ Original: template <typename T> size_t NumBitsSet(T value)
    fn num_bits_set(value: u8) -> usize {
        let mut count = 0;
        let mut v = value;
        while v != 0 {
            if (v & 0x1) != 0 {
                count += 1;
            }
            v >>= 1;
        }
        count
    }

    // C++ Original: template <int page, uint8_t opCode> void OpPSH(uint16_t& stackReg)
    fn op_psh(&mut self, stack_reg: bool) { // true = S stack, false = U stack
        let value = self.read_operand_value8(if stack_reg { 0x34 } else { 0x36 });
        
        // C++ Original: if (value & BITS(7)) Push16(stackReg, PC);
        if (value & 0x80) != 0 { // bit 7
            if stack_reg {
                // inline push16 for S stack and PC
                self.registers.s = self.registers.s.wrapping_sub(1);
                self.write8(self.registers.s, (self.registers.pc & 0xFF) as u8); // Low
                self.registers.s = self.registers.s.wrapping_sub(1);
                self.write8(self.registers.s, (self.registers.pc >> 8) as u8);   // High
            } else {
                // inline push16 for U stack and PC
                self.registers.u = self.registers.u.wrapping_sub(1);
                self.write8(self.registers.u, (self.registers.pc & 0xFF) as u8); // Low
                self.registers.u = self.registers.u.wrapping_sub(1);
                self.write8(self.registers.u, (self.registers.pc >> 8) as u8);   // High
            }
        }
        
        // C++ Original: if (value & BITS(6)) { auto otherStackReg = &stackReg == &S ? U : S; Push16(stackReg, otherStackReg); }
        if (value & 0x40) != 0 { // bit 6
            let other_stack = if stack_reg { self.registers.u } else { self.registers.s };
            if stack_reg {
                // inline push16 for S stack and other_stack
                self.registers.s = self.registers.s.wrapping_sub(1);
                self.write8(self.registers.s, (other_stack & 0xFF) as u8); // Low
                self.registers.s = self.registers.s.wrapping_sub(1);
                self.write8(self.registers.s, (other_stack >> 8) as u8);   // High
            } else {
                // inline push16 for U stack and other_stack
                self.registers.u = self.registers.u.wrapping_sub(1);
                self.write8(self.registers.u, (other_stack & 0xFF) as u8); // Low
                self.registers.u = self.registers.u.wrapping_sub(1);
                self.write8(self.registers.u, (other_stack >> 8) as u8);   // High
            }
        }
        
        // C++ Original: if (value & BITS(5)) Push16(stackReg, Y);
        if (value & 0x20) != 0 { // bit 5
            if stack_reg {
                // inline push16 for S stack and Y
                self.registers.s = self.registers.s.wrapping_sub(1);
                self.write8(self.registers.s, (self.registers.y & 0xFF) as u8); // Low
                self.registers.s = self.registers.s.wrapping_sub(1);
                self.write8(self.registers.s, (self.registers.y >> 8) as u8);   // High
            } else {
                // inline push16 for U stack and Y
                self.registers.u = self.registers.u.wrapping_sub(1);
                self.write8(self.registers.u, (self.registers.y & 0xFF) as u8); // Low
                self.registers.u = self.registers.u.wrapping_sub(1);
                self.write8(self.registers.u, (self.registers.y >> 8) as u8);   // High
            }
        }
        
        // C++ Original: if (value & BITS(4)) Push16(stackReg, X);
        if (value & 0x10) != 0 { // bit 4
            if stack_reg {
                // inline push16 for S stack and X
                self.registers.s = self.registers.s.wrapping_sub(1);
                self.write8(self.registers.s, (self.registers.x & 0xFF) as u8); // Low
                self.registers.s = self.registers.s.wrapping_sub(1);
                self.write8(self.registers.s, (self.registers.x >> 8) as u8);   // High
            } else {
                // inline push16 for U stack and X
                self.registers.u = self.registers.u.wrapping_sub(1);
                self.write8(self.registers.u, (self.registers.x & 0xFF) as u8); // Low
                self.registers.u = self.registers.u.wrapping_sub(1);
                self.write8(self.registers.u, (self.registers.x >> 8) as u8);   // High
            }
        }
        
        // C++ Original: if (value & BITS(3)) Push8(stackReg, DP);
        if (value & 0x08) != 0 { // bit 3
            if stack_reg {
                // inline push8 for S stack and DP
                self.registers.s = self.registers.s.wrapping_sub(1);
                self.write8(self.registers.s, self.registers.dp);
            } else {
                // inline push8 for U stack and DP
                self.registers.u = self.registers.u.wrapping_sub(1);
                self.write8(self.registers.u, self.registers.dp);
            }
        }
        
        // C++ Original: if (value & BITS(2)) Push8(stackReg, B);
        if (value & 0x04) != 0 { // bit 2
            if stack_reg {
                // inline push8 for S stack and B
                self.registers.s = self.registers.s.wrapping_sub(1);
                self.write8(self.registers.s, self.registers.b);
            } else {
                // inline push8 for U stack and B
                self.registers.u = self.registers.u.wrapping_sub(1);
                self.write8(self.registers.u, self.registers.b);
            }
        }
        
        // C++ Original: if (value & BITS(1)) Push8(stackReg, A);
        if (value & 0x02) != 0 { // bit 1
            if stack_reg {
                // inline push8 for S stack and A
                self.registers.s = self.registers.s.wrapping_sub(1);
                self.write8(self.registers.s, self.registers.a);
            } else {
                // inline push8 for U stack and A
                self.registers.u = self.registers.u.wrapping_sub(1);
                self.write8(self.registers.u, self.registers.a);
            }
        }
        
        // C++ Original: if (value & BITS(0)) Push8(stackReg, CC.Value);
        if (value & 0x01) != 0 { // bit 0
            let cc_value = self.registers.cc.to_u8();
            if stack_reg {
                // inline push8 for S stack and CC
                self.registers.s = self.registers.s.wrapping_sub(1);
                self.write8(self.registers.s, cc_value);
            } else {
                // inline push8 for U stack and CC
                self.registers.u = self.registers.u.wrapping_sub(1);
                self.write8(self.registers.u, cc_value);
            }
        }

        // C++ Original: // 1 cycle per byte pushed
        // C++ Original: AddCycles(NumBitsSet(ReadBits(value, BITS(0, 1, 2, 3))));
        // C++ Original: AddCycles(NumBitsSet(ReadBits(value, BITS(4, 5, 6, 7))) * 2);
        let low_bits = value & 0x0F; // bits 0-3 (8-bit registers)
        let high_bits = value & 0xF0; // bits 4-7 (16-bit registers)
        let cycles = Self::num_bits_set(low_bits) + (Self::num_bits_set(high_bits) * 2);
        self.add_cycles(cycles as u64);
    }

    // C++ Original: template <int page, uint8_t opCode> void OpPUL(uint16_t& stackReg)
    fn op_pul(&mut self, stack_reg: bool) { // true = S stack, false = U stack
        let value = self.read_operand_value8(if stack_reg { 0x35 } else { 0x37 });
        
        // C++ Original: if (value & BITS(0)) CC.Value = Pop8(stackReg);
        if (value & 0x01) != 0 { // bit 0
            let cc_value = if stack_reg {
                // inline pop8 for S stack
                let val = self.read8(self.registers.s);
                self.registers.s = self.registers.s.wrapping_add(1);
                val
            } else {
                // inline pop8 for U stack
                let val = self.read8(self.registers.u);
                self.registers.u = self.registers.u.wrapping_add(1);
                val
            };
            self.registers.cc.from_u8(cc_value);
        }
        
        // C++ Original: if (value & BITS(1)) A = Pop8(stackReg);
        if (value & 0x02) != 0 { // bit 1
            self.registers.a = if stack_reg {
                // inline pop8 for S stack
                let val = self.read8(self.registers.s);
                self.registers.s = self.registers.s.wrapping_add(1);
                val
            } else {
                // inline pop8 for U stack
                let val = self.read8(self.registers.u);
                self.registers.u = self.registers.u.wrapping_add(1);
                val
            };
        }
        
        // C++ Original: if (value & BITS(2)) B = Pop8(stackReg);
        if (value & 0x04) != 0 { // bit 2
            self.registers.b = if stack_reg {
                // inline pop8 for S stack
                let val = self.read8(self.registers.s);
                self.registers.s = self.registers.s.wrapping_add(1);
                val
            } else {
                // inline pop8 for U stack
                let val = self.read8(self.registers.u);
                self.registers.u = self.registers.u.wrapping_add(1);
                val
            };
        }
        
        // C++ Original: if (value & BITS(3)) DP = Pop8(stackReg);
        if (value & 0x08) != 0 { // bit 3
            self.registers.dp = if stack_reg {
                // inline pop8 for S stack
                let val = self.read8(self.registers.s);
                self.registers.s = self.registers.s.wrapping_add(1);
                val
            } else {
                // inline pop8 for U stack
                let val = self.read8(self.registers.u);
                self.registers.u = self.registers.u.wrapping_add(1);
                val
            };
        }
        
        // C++ Original: if (value & BITS(4)) X = Pop16(stackReg);
        if (value & 0x10) != 0 { // bit 4
            self.registers.x = if stack_reg {
                // inline pop16 for S stack
                let high = self.read8(self.registers.s);
                self.registers.s = self.registers.s.wrapping_add(1);
                let low = self.read8(self.registers.s);
                self.registers.s = self.registers.s.wrapping_add(1);
                combine_to_u16(high, low)
            } else {
                // inline pop16 for U stack
                let high = self.read8(self.registers.u);
                self.registers.u = self.registers.u.wrapping_add(1);
                let low = self.read8(self.registers.u);
                self.registers.u = self.registers.u.wrapping_add(1);
                combine_to_u16(high, low)
            };
        }
        
        // C++ Original: if (value & BITS(5)) Y = Pop16(stackReg);
        if (value & 0x20) != 0 { // bit 5
            self.registers.y = if stack_reg {
                // inline pop16 for S stack
                let high = self.read8(self.registers.s);
                self.registers.s = self.registers.s.wrapping_add(1);
                let low = self.read8(self.registers.s);
                self.registers.s = self.registers.s.wrapping_add(1);
                combine_to_u16(high, low)
            } else {
                // inline pop16 for U stack
                let high = self.read8(self.registers.u);
                self.registers.u = self.registers.u.wrapping_add(1);
                let low = self.read8(self.registers.u);
                self.registers.u = self.registers.u.wrapping_add(1);
                combine_to_u16(high, low)
            };
        }
        
        // C++ Original: if (value & BITS(6)) { auto& otherStackReg = &stackReg == &S ? U : S; otherStackReg = Pop16(stackReg); }
        if (value & 0x40) != 0 { // bit 6
            let other_stack = if stack_reg {
                // inline pop16 for S stack
                let high = self.read8(self.registers.s);
                self.registers.s = self.registers.s.wrapping_add(1);
                let low = self.read8(self.registers.s);
                self.registers.s = self.registers.s.wrapping_add(1);
                combine_to_u16(high, low)
            } else {
                // inline pop16 for U stack
                let high = self.read8(self.registers.u);
                self.registers.u = self.registers.u.wrapping_add(1);
                let low = self.read8(self.registers.u);
                self.registers.u = self.registers.u.wrapping_add(1);
                combine_to_u16(high, low)
            };
            if stack_reg {
                self.registers.u = other_stack;
            } else {
                self.registers.s = other_stack;
            }
        }
        
        // C++ Original: if (value & BITS(7)) PC = Pop16(stackReg);
        if (value & 0x80) != 0 { // bit 7
            self.registers.pc = if stack_reg {
                // inline pop16 for S stack
                let high = self.read8(self.registers.s);
                self.registers.s = self.registers.s.wrapping_add(1);
                let low = self.read8(self.registers.s);
                self.registers.s = self.registers.s.wrapping_add(1);
                combine_to_u16(high, low)
            } else {
                // inline pop16 for U stack
                let high = self.read8(self.registers.u);
                self.registers.u = self.registers.u.wrapping_add(1);
                let low = self.read8(self.registers.u);
                self.registers.u = self.registers.u.wrapping_add(1);
                combine_to_u16(high, low)
            };
        }

        // C++ Original: // 1 cycle per byte pulled
        // C++ Original: AddCycles(NumBitsSet(ReadBits(value, BITS(0, 1, 2, 3))));
        // C++ Original: AddCycles(NumBitsSet(ReadBits(value, BITS(4, 5, 6, 7))) * 2);
        let low_bits = value & 0x0F; // bits 0-3 (8-bit registers)
        let high_bits = value & 0xF0; // bits 4-7 (16-bit registers)
        let cycles = Self::num_bits_set(low_bits) + (Self::num_bits_set(high_bits) * 2);
        self.add_cycles(cycles as u64);
    }

    // C++ Original: template <int page, uint8_t opCode> void OpCLR() { uint16_t EA = ReadEA16<>; m_memoryBus->Write(EA, 0); CC... }
    fn op_clr_memory(&mut self, addressing_mode: AddressingMode) {
        let ea = match addressing_mode {
            AddressingMode::Direct => self.read_direct_ea(),
            AddressingMode::Indexed => self.read_indexed_ea(),
            AddressingMode::Extended => self.read_extended_ea(),
            _ => panic!("Invalid addressing mode for CLR: {:?}", addressing_mode),
        };
        self.write8(ea, 0);
        self.registers.cc.n = false;  // CC.Negative = 0
        self.registers.cc.z = true;   // CC.Zero = 1
        self.registers.cc.v = false;  // CC.Overflow = 0
        self.registers.cc.c = false;  // CC.Carry = 0
    }

    // C++ Original: template <int page, uint8_t opCode> void OpNEG() { uint16_t EA = ReadEA16<>; uint8_t value = m_memoryBus->Read(EA); OpNEG<>(value); m_memoryBus->Write(EA, value); }
    fn op_neg_memory(&mut self, addressing_mode: AddressingMode) {
        let ea = match addressing_mode {
            AddressingMode::Direct => self.read_direct_ea(),
            AddressingMode::Indexed => self.read_indexed_ea(),
            AddressingMode::Extended => self.read_extended_ea(),
            _ => panic!("Invalid addressing mode for NEG: {:?}", addressing_mode),
        };
        let mut value = self.read8(ea);
        value = self.subtract_impl_u8(0, value, 0);
        self.write8(ea, value);
    }

    // C++ Original: template <int page, uint8_t opCode> void OpCOM() { uint16_t EA = ReadEA16<>; uint8_t value = m_memoryBus->Read(EA); OpCOM<>(value); m_memoryBus->Write(EA, value); }
    fn op_com_memory(&mut self, addressing_mode: AddressingMode) {
        let ea = match addressing_mode {
            AddressingMode::Direct => self.read_direct_ea(),
            AddressingMode::Indexed => self.read_indexed_ea(),
            AddressingMode::Extended => self.read_extended_ea(),
            _ => panic!("Invalid addressing mode for COM: {:?}", addressing_mode),
        };
        let mut value = self.read8(ea);
        value = !value;
        self.registers.cc.n = Self::calc_negative_u8(value);
        self.registers.cc.z = Self::calc_zero_u8(value);
        self.registers.cc.v = false;
        self.registers.cc.c = true;
        self.write8(ea, value);
    }

    // C++ Original: template <int page, uint8_t opCode> void OpDEC() { uint16_t EA = ReadEA16<>; uint8_t value = m_memoryBus->Read(EA); OpDEC<>(value); m_memoryBus->Write(EA, value); }
    fn op_dec_memory(&mut self, addressing_mode: AddressingMode) {
        let ea = match addressing_mode {
            AddressingMode::Direct => self.read_direct_ea(),
            AddressingMode::Indexed => self.read_indexed_ea(),
            AddressingMode::Extended => self.read_extended_ea(),
            _ => panic!("Invalid addressing mode for DEC: {:?}", addressing_mode),
        };
        let value = self.read8(ea);
        let orig_value = value;
        let new_value = value.wrapping_sub(1);
        self.registers.cc.v = orig_value == 0b1000_0000;
        self.registers.cc.z = Self::calc_zero_u8(new_value);
        self.registers.cc.n = Self::calc_negative_u8(new_value);
        self.write8(ea, new_value);
    }

    // C++ Original: template <int page, uint8_t opCode> void OpINC() { uint16_t EA = ReadEA16<>; uint8_t value = m_memoryBus->Read(EA); OpINC<>(value); m_memoryBus->Write(EA, value); }
    fn op_inc_memory(&mut self, addressing_mode: AddressingMode) {
        let ea = match addressing_mode {
            AddressingMode::Direct => self.read_direct_ea(),
            AddressingMode::Indexed => self.read_indexed_ea(),
            AddressingMode::Extended => self.read_extended_ea(),
            _ => panic!("Invalid addressing mode for INC: {:?}", addressing_mode),
        };
        let value = self.read8(ea);
        let orig_value = value;
        let new_value = value.wrapping_add(1);
        self.registers.cc.v = orig_value == 0b0111_1111;
        self.registers.cc.z = Self::calc_zero_u8(new_value);
        self.registers.cc.n = Self::calc_negative_u8(new_value);
        self.write8(ea, new_value);
    }

    // C++ Original: template <int page, uint8_t opCode> void OpTST() { OpTST<>(ReadOperandValue8<>()); }
    fn op_tst_memory(&mut self, addressing_mode: AddressingMode) {
        let ea = match addressing_mode {
            AddressingMode::Direct => self.read_direct_ea(),
            AddressingMode::Indexed => self.read_indexed_ea(),
            AddressingMode::Extended => self.read_extended_ea(),
            _ => panic!("Invalid addressing mode for TST: {:?}", addressing_mode),
        };
        let value = self.read8(ea);
        self.registers.cc.n = Self::calc_negative_u8(value);
        self.registers.cc.z = Self::calc_zero_u8(value);
        self.registers.cc.v = false;
        // Note: TST does NOT modify Carry flag in 6809
    }

    // C++ Original: template <int page, uint8_t opCode> void OpASR() { uint16_t EA = ReadEA16<>; uint8_t value = m_memoryBus->Read(EA); OpASR<>(value); m_memoryBus->Write(EA, value); }
    fn op_asr_memory(&mut self, addressing_mode: AddressingMode) {
        let ea = match addressing_mode {
            AddressingMode::Direct => self.read_direct_ea(),
            AddressingMode::Indexed => self.read_indexed_ea(),
            AddressingMode::Extended => self.read_extended_ea(),
            _ => panic!("Invalid addressing mode for ASR: {:?}", addressing_mode),
        };
        let value = self.read8(ea);
        let orig_value = value;
        let new_value = (orig_value & 0b1000_0000) | (value >> 1);
        self.registers.cc.z = Self::calc_zero_u8(new_value);
        self.registers.cc.n = Self::calc_negative_u8(new_value);
        self.registers.cc.c = (orig_value & 0b0000_0001) != 0;
        self.write8(ea, new_value);
    }

    // C++ Original: template <int page, uint8_t opCode> void OpLSR() { uint16_t EA = ReadEA16<>; uint8_t value = m_memoryBus->Read(EA); OpLSR<>(value); m_memoryBus->Write(EA, value); }
    fn op_lsr_memory(&mut self, addressing_mode: AddressingMode) {
        let ea = match addressing_mode {
            AddressingMode::Direct => self.read_direct_ea(),
            AddressingMode::Indexed => self.read_indexed_ea(),
            AddressingMode::Extended => self.read_extended_ea(),
            _ => panic!("Invalid addressing mode for LSR: {:?}", addressing_mode),
        };
        let value = self.read8(ea);
        let orig_value = value;
        let new_value = value >> 1;
        self.registers.cc.z = Self::calc_zero_u8(new_value);
        self.registers.cc.n = false; // Bit 7 always shifted out
        self.registers.cc.c = (orig_value & 0b0000_0001) != 0;
        self.write8(ea, new_value);
    }

    // C++ Original: template <int page, uint8_t opCode> void OpROL() { uint16_t EA = ReadEA16<>; uint8_t value = m_memoryBus->Read(EA); OpROL<>(value); m_memoryBus->Write(EA, value); }
    fn op_rol_memory(&mut self, addressing_mode: AddressingMode) {
        let ea = match addressing_mode {
            AddressingMode::Direct => self.read_direct_ea(),
            AddressingMode::Indexed => self.read_indexed_ea(),
            AddressingMode::Extended => self.read_extended_ea(),
            _ => panic!("Invalid addressing mode for ROL: {:?}", addressing_mode),
        };
        let value = self.read8(ea);
        let result = (value << 1) | (self.registers.cc.c as u8);
        self.registers.cc.c = (value & 0b1000_0000) != 0;
        self.registers.cc.v = ((value & 0b1000_0000) ^ ((value & 0b0100_0000) << 1)) != 0;
        self.registers.cc.n = Self::calc_negative_u8(result);
        self.registers.cc.z = Self::calc_zero_u8(result);
        self.write8(ea, result);
    }

    // C++ Original: template <int page, uint8_t opCode> void OpROR() { uint16_t EA = ReadEA16<>; uint8_t value = m_memoryBus->Read(EA); OpROR<>(value); m_memoryBus->Write(EA, value); }
    fn op_ror_memory(&mut self, addressing_mode: AddressingMode) {
        let ea = match addressing_mode {
            AddressingMode::Direct => self.read_direct_ea(),
            AddressingMode::Indexed => self.read_indexed_ea(),
            AddressingMode::Extended => self.read_extended_ea(),
            _ => panic!("Invalid addressing mode for ROR: {:?}", addressing_mode),
        };
        let value = self.read8(ea);
        let result = ((self.registers.cc.c as u8) << 7) | (value >> 1);
        self.registers.cc.c = (value & 0b0000_0001) != 0;
        self.registers.cc.n = Self::calc_negative_u8(result);
        self.registers.cc.z = Self::calc_zero_u8(result);
        self.write8(ea, result);
    }

    // C++ Original: template <int page, uint8_t opCode> void OpASL() { uint16_t EA = ReadEA16<>; uint8_t value = m_memoryBus->Read(EA); OpASL<>(value); m_memoryBus->Write(EA, value); }
    fn op_asl_memory(&mut self, addressing_mode: AddressingMode) {
        let ea = match addressing_mode {
            AddressingMode::Direct => self.read_direct_ea(),
            AddressingMode::Indexed => self.read_indexed_ea(),
            AddressingMode::Extended => self.read_extended_ea(),
            _ => panic!("Invalid addressing mode for ASL: {:?}", addressing_mode),
        };
        let value = self.read8(ea);
        // C++ Original: Shifting left is same as adding value + value (aka value * 2)
        let new_value = self.add_impl_u8_temp(value, value, 0);
        self.write8(ea, new_value);
    }

    // Helper function para ASL que no modifica self.registers - necesaria para op_asl_memory
    fn add_impl_u8_temp(&mut self, a: u8, b: u8, carry: u8) -> u8 {
        let r16 = (a as u16) + (b as u16) + (carry as u16);
        let result = r16 as u8;
        
        self.registers.cc.h = Self::calc_half_carry_from_add(a, b, carry);
        self.registers.cc.c = Self::calc_carry_u16(r16);
        self.registers.cc.v = Self::calc_overflow_u8(a, b, r16);
        self.registers.cc.n = Self::calc_negative_u8(result);
        self.registers.cc.z = Self::calc_zero_u8(result);
        
        result
    }

    // C++ Original: void OpABX() { X += B; }
    fn op_abx(&mut self) {
        self.registers.x = self.registers.x.wrapping_add(self.registers.b as u16);
    }

    // C++ Original: void OpDAA() - Decimal Adjust A register
    fn op_daa(&mut self) {
        // C++ Original: Extract least and most significant nibbles
        let lsn = self.registers.a & 0x0F;
        let msn = (self.registers.a & 0xF0) >> 4;

        // C++ Original: Compute correction factors
        let cf_lsn = if self.registers.cc.h || (lsn > 9) { 6 } else { 0 };
        let cf_msn = if self.registers.cc.c || (msn > 9) || (msn > 8 && lsn > 9) { 6 } else { 0 };
        let adjust = (cf_msn << 4) | cf_lsn;
        let r16 = (self.registers.a as u16) + (adjust as u16);
        self.registers.a = r16 as u8;
        self.registers.cc.n = Self::calc_negative_u8(self.registers.a);
        self.registers.cc.z = Self::calc_zero_u8(self.registers.a);
        self.registers.cc.c = self.registers.cc.c || Self::calc_carry_u16(r16);
    }

    // C++ Original: void OpEXG() { ExchangeOrTransfer(true); }
    fn op_exg(&mut self) {
        self.exchange_or_transfer(true);
    }

    // C++ Original: void OpTFR() { ExchangeOrTransfer(false); }
    fn op_tfr(&mut self) {
        self.exchange_or_transfer(false);
    }

    // C++ Original: void ExchangeOrTransfer(bool exchange)
    fn exchange_or_transfer(&mut self, exchange: bool) {
        let postbyte = self.read_pc8();
        // C++ Original: ASSERT(!!(postbyte & BITS(3)) == !!(postbyte & BITS(7))); // 8-bit to 8-bit or 16-bit to 16-bit only
        assert_eq!((postbyte & 0x08) != 0, (postbyte & 0x80) != 0, "8-bit to 8-bit or 16-bit to 16-bit only");

        let src = (postbyte >> 4) & 0b111;  // C++ Original: (postbyte >> 4) & 0b111
        let dst = postbyte & 0b111;         // C++ Original: postbyte & 0b111

        if (postbyte & 0x08) != 0 { // 8-bit registers
            // C++ Original: ASSERT(src < 4 && dst < 4); uint8_t* const reg[]{&A, &B, &CC.Value, &DP};
            assert!(src < 4 && dst < 4, "Only first 4 are valid 8-bit register indices");
            let src_val = match src {
                0 => self.registers.a,
                1 => self.registers.b, 
                2 => self.registers.cc.to_u8(),
                3 => self.registers.dp,
                _ => unreachable!(),
            };
            let dst_val = match dst {
                0 => self.registers.a,
                1 => self.registers.b,
                2 => self.registers.cc.to_u8(), 
                3 => self.registers.dp,
                _ => unreachable!(),
            };

            if exchange {
                // C++ Original: std::swap(*reg[dst], *reg[src]);
                match dst {
                    0 => self.registers.a = src_val,
                    1 => self.registers.b = src_val,
                    2 => self.registers.cc.from_u8(src_val),
                    3 => self.registers.dp = src_val,
                    _ => unreachable!(),
                }
                match src {
                    0 => self.registers.a = dst_val,
                    1 => self.registers.b = dst_val,
                    2 => self.registers.cc.from_u8(dst_val),
                    3 => self.registers.dp = dst_val,
                    _ => unreachable!(),
                }
            } else {
                // C++ Original: *reg[dst] = *reg[src];
                match dst {
                    0 => self.registers.a = src_val,
                    1 => self.registers.b = src_val,
                    2 => self.registers.cc.from_u8(src_val),
                    3 => self.registers.dp = src_val,
                    _ => unreachable!(),
                }
            }
        } else { // 16-bit registers
            // C++ Original: ASSERT(src < 6 && dst < 6); uint16_t* const reg[]{&D, &X, &Y, &U, &S, &PC};
            assert!(src < 6 && dst < 6, "Only first 6 are valid 16-bit register indices");
            let src_val = match src {
                0 => combine_to_u16(self.registers.a, self.registers.b), // D register
                1 => self.registers.x,
                2 => self.registers.y,
                3 => self.registers.u,
                4 => self.registers.s,
                5 => self.registers.pc,
                _ => unreachable!(),
            };
            let dst_val = match dst {
                0 => combine_to_u16(self.registers.a, self.registers.b), // D register
                1 => self.registers.x,
                2 => self.registers.y,
                3 => self.registers.u,
                4 => self.registers.s,
                5 => self.registers.pc,
                _ => unreachable!(),
            };

            if exchange {
                // C++ Original: std::swap(*reg[dst], *reg[src]);
                match dst {
                    0 => { // D register
                        self.registers.a = (src_val >> 8) as u8;
                        self.registers.b = (src_val & 0xFF) as u8;
                    },
                    1 => self.registers.x = src_val,
                    2 => self.registers.y = src_val,
                    3 => self.registers.u = src_val,
                    4 => self.registers.s = src_val,
                    5 => self.registers.pc = src_val,
                    _ => unreachable!(),
                }
                match src {
                    0 => { // D register
                        self.registers.a = (dst_val >> 8) as u8;
                        self.registers.b = (dst_val & 0xFF) as u8;
                    },
                    1 => self.registers.x = dst_val,
                    2 => self.registers.y = dst_val,
                    3 => self.registers.u = dst_val,
                    4 => self.registers.s = dst_val,
                    5 => self.registers.pc = dst_val,
                    _ => unreachable!(),
                }
            } else {
                // C++ Original: *reg[dst] = *reg[src];
                match dst {
                    0 => { // D register
                        self.registers.a = (src_val >> 8) as u8;
                        self.registers.b = (src_val & 0xFF) as u8;
                    },
                    1 => self.registers.x = src_val,
                    2 => self.registers.y = src_val,
                    3 => self.registers.u = src_val,
                    4 => self.registers.s = src_val,
                    5 => self.registers.pc = src_val,
                    _ => unreachable!(),
                }
            }
        }
    }
}