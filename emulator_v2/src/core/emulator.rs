// C++ Original: vectrexy/libs/emulator/include/emulator/Emulator.h + Emulator.cpp
// #pragma once
// #include "core/Base.h"
// #include "emulator/BiosRom.h"
// #include "emulator/Cartridge.h"
// #include "emulator/Cpu.h"
// #include "emulator/DevMemoryDevice.h"
// #include "emulator/IllegalMemoryDevice.h"
// #include "emulator/Ram.h"
// #include "emulator/UnmappedMemoryDevice.h"
// #include "emulator/Via.h"
//
// class Emulator {
// public:
//     void Init(const char* biosRomFile);
//     void Reset();
//     bool LoadBios(const char* file);
//     bool LoadRom(const char* file);
//     cycles_t ExecuteInstruction(const Input& input, RenderContext& renderContext, AudioContext& audioContext);
//     void FrameUpdate(double frameTime);
//
//     MemoryBus& GetMemoryBus() { return m_memoryBus; }
//     Cpu& GetCpu() { return m_cpu; }
//     Ram& GetRam() { return m_ram; }
//     Via& GetVia() { return m_via; }
//
// private:
//     MemoryBus m_memoryBus;
//     Cpu m_cpu;
//     Via m_via;
//     Ram m_ram;
//     BiosRom m_biosRom;
//     IllegalMemoryDevice m_illegal;
//     UnmappedMemoryDevice m_unmapped;
//     DevMemoryDevice m_dev;
//     Cartridge m_cartridge;
// };

use crate::core::{
    engine_types::{AudioContext, Input, RenderContext},
    BiosRom, Cartridge, Cpu6809, DevMemoryDevice, IllegalMemoryDevice, MemoryBus, Ram,
    UnmappedMemoryDevice, Via6522,
};
use crate::types::Cycles;
use std::cell::UnsafeCell;
use std::collections::HashSet;
use std::rc::Rc;

macro_rules! unsafe_ref {
    ($cell:expr) => {
        unsafe { &*$cell.get() }
    };
}

macro_rules! unsafe_mut {
    ($cell:expr) => {
        unsafe { &mut *$cell.get() }
    };
}

pub struct Emulator {
    cpu: Cpu6809,
    via: Rc<UnsafeCell<Via6522>>,
    ram: Rc<UnsafeCell<Ram>>,
    bios_rom: Rc<UnsafeCell<BiosRom>>,
    illegal: Rc<UnsafeCell<IllegalMemoryDevice>>,
    unmapped: Rc<UnsafeCell<UnmappedMemoryDevice>>,
    dev: Rc<UnsafeCell<DevMemoryDevice>>,
    cartridge: Rc<UnsafeCell<Cartridge>>,
    
    // Breakpoint support
    breakpoints: HashSet<u16>,
    paused_by_breakpoint: bool,
}

impl Emulator {
    pub fn new() -> Self {
        let memory_bus = MemoryBus::new();

        Self {
            cpu: Cpu6809::new(memory_bus),
            via: Rc::new(UnsafeCell::new(Via6522::new())),
            ram: Rc::new(UnsafeCell::new(Ram::new())),
            bios_rom: Rc::new(UnsafeCell::new(BiosRom::new())),
            illegal: Rc::new(UnsafeCell::new(IllegalMemoryDevice::new())),
            unmapped: Rc::new(UnsafeCell::new(UnmappedMemoryDevice::new())),
            dev: Rc::new(UnsafeCell::new(DevMemoryDevice::new())),
            cartridge: Rc::new(UnsafeCell::new(Cartridge::new())),
            breakpoints: HashSet::new(),
            paused_by_breakpoint: false,
        }
    }

    pub fn init(&mut self, bios_rom_file: &str) {
        const DEVELOPER_MODE: bool = true;

        Via6522::init_memory_bus(self.via.clone(), self.cpu.memory_bus_mut());
        Ram::init_memory_bus(self.ram.clone(), self.cpu.memory_bus_mut());
        BiosRom::init_memory_bus(self.bios_rom.clone(), self.cpu.memory_bus_mut());
        IllegalMemoryDevice::init_memory_bus(self.illegal.clone(), self.cpu.memory_bus_mut());

        if DEVELOPER_MODE {
             // In the original C++, DevMemoryDevice requires a weak_ptr to the memory bus
             // which is complex to replicate safely here. For now, we'll use Unmapped.
            UnmappedMemoryDevice::init_memory_bus(self.unmapped.clone(), self.cpu.memory_bus_mut());
        } else {
            UnmappedMemoryDevice::init_memory_bus(self.unmapped.clone(), self.cpu.memory_bus_mut());
        }

        Cartridge::init_memory_bus(self.cartridge.clone(), self.cpu.memory_bus_mut());

        self.load_bios(bios_rom_file);
    }

    pub fn reset(&mut self) {
        let seed = rand::random::<u32>();
        unsafe_mut!(self.ram).randomize(seed);
        self.cpu.reset();
        unsafe_mut!(self.via).reset();
    }

    pub fn load_bios(&mut self, file: &str) -> bool {
        match std::fs::read(file) {
            Ok(data) => unsafe_mut!(self.bios_rom).load_bios_rom(&data),
            Err(_) => false,
        }
    }

    pub fn load_bios_from_bytes(&mut self, data: &[u8]) -> bool {
        unsafe_mut!(self.bios_rom).load_bios_rom(data)
    }

    pub fn load_rom(&mut self, file: &str) -> bool {
        unsafe_mut!(self.cartridge).load_rom(file)
    }

    pub fn execute_instruction(
        &mut self,
        input: &Input,
        render_context: &mut RenderContext,
        audio_context: &mut AudioContext,
    ) -> Result<Cycles, crate::core::cpu6809::CpuError> {
        unsafe_mut!(self.via).set_sync_context(input, render_context, audio_context);

        let cpu_cycles = self.cpu.execute_instruction(
            unsafe_ref!(self.via).irq_enabled(),
            unsafe_ref!(self.via).firq_enabled(),
        )?;

        self.cpu.memory_bus_mut().sync();
        
        // CRITICAL: Check breakpoints after instruction execution
        let pc = self.cpu.registers.pc;
        if self.breakpoints.contains(&pc) {
            self.paused_by_breakpoint = true;
        }

        Ok(cpu_cycles)
    }

    pub fn frame_update(&mut self, frame_time: f64) {
        unsafe_mut!(self.via).frame_update(frame_time);
    }

    pub fn get_memory_bus(&mut self) -> &mut MemoryBus {
        self.cpu.memory_bus_mut()
    }

    pub fn get_cpu(&mut self) -> &mut Cpu6809 {
        &mut self.cpu
    }

    pub fn get_ram(&self) -> Rc<UnsafeCell<Ram>> {
        self.ram.clone()
    }

    pub fn get_via(&self) -> Rc<UnsafeCell<Via6522>> {
        self.via.clone()
    }
    
    // Breakpoint management
    pub fn add_breakpoint(&mut self, address: u16) {
        self.breakpoints.insert(address);
    }
    
    pub fn remove_breakpoint(&mut self, address: u16) {
        self.breakpoints.remove(&address);
    }
    
    pub fn clear_breakpoints(&mut self) {
        self.breakpoints.clear();
    }
    
    pub fn is_paused_by_breakpoint(&self) -> bool {
        self.paused_by_breakpoint
    }
    
    pub fn resume_from_breakpoint(&mut self) {
        self.paused_by_breakpoint = false;
    }
}
