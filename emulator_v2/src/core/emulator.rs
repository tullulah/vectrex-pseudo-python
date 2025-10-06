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
use std::rc::Rc; // UNSAFE FIX: UnsafeCell en lugar de RefCell
                 // ARCHITECTURE FIX: MemoryBus owned by CPU directly (no RefCell for MemoryBus itself)

// Helper macro para acceso unsafe más limpio
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
    // ARCHITECTURE FIX: CPU owns MemoryBus directly (no RefCell)
    // C++ Original: Cpu m_cpu; + MemoryBus m_memoryBus;
    cpu: Cpu6809,

    // C++ Original: Via m_via;
    via: Rc<UnsafeCell<Via6522>>,

    // C++ Original: Ram m_ram;
    ram: Rc<UnsafeCell<Ram>>,

    // C++ Original: BiosRom m_biosRom;
    bios_rom: Rc<UnsafeCell<BiosRom>>,

    // C++ Original: IllegalMemoryDevice m_illegal;
    illegal: Rc<UnsafeCell<IllegalMemoryDevice>>,

    // C++ Original: UnmappedMemoryDevice m_unmapped;
    unmapped: Rc<UnsafeCell<UnmappedMemoryDevice>>,

    // C++ Original: DevMemoryDevice m_dev;
    dev: Rc<UnsafeCell<DevMemoryDevice>>,

    // C++ Original: Cartridge m_cartridge;
    cartridge: Rc<UnsafeCell<Cartridge>>,
}

impl Emulator {
    pub fn new() -> Self {
        // ARCHITECTURE FIX: CPU takes ownership of MemoryBus
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
        }
    }

    // C++ Original: void Init(const char* biosRomFile)
    // NOTE: Made public for integration tests that need full emulator initialization
    pub fn init(&mut self, bios_rom_file: &str) {
        // C++ Original: const bool DeveloperMode = true;
        const DEVELOPER_MODE: bool = true;

        // ARCHITECTURE FIX: Access memory_bus through CPU
        // C++ Original: Now connect all devices to memory bus following exact C++ pattern
        // C++ Original: m_cpu.Init(m_memoryBus); - CPU init already handled in constructor

        // C++ Original: m_via.Init(m_memoryBus);
        Via6522::init_memory_bus(self.via.clone(), self.cpu.memory_bus_mut());

        // Note: SetSyncContext is NOT called here - it's called before each instruction execution
        // (see execute_instruction method)

        // C++ Original: m_ram.Init(m_memoryBus);
        Ram::init_memory_bus(self.ram.clone(), self.cpu.memory_bus_mut());

        // C++ Original: m_biosRom.Init(m_memoryBus);
        BiosRom::init_memory_bus(self.bios_rom.clone(), self.cpu.memory_bus_mut());

        // C++ Original: m_illegal.Init(m_memoryBus);
        IllegalMemoryDevice::init_memory_bus(self.illegal.clone(), self.cpu.memory_bus_mut());

        // C++ Original: if (DeveloperMode) { m_dev.Init(m_memoryBus); } else { m_unmapped.Init(m_memoryBus); }
        // NOTE: DevMemoryDevice disabled - requires printf callback infrastructure not needed for WASM
        if DEVELOPER_MODE {
            // Future: Could add WASM console logging here if needed
            UnmappedMemoryDevice::init_memory_bus(self.unmapped.clone(), self.cpu.memory_bus_mut());
        } else {
            UnmappedMemoryDevice::init_memory_bus(self.unmapped.clone(), self.cpu.memory_bus_mut());
        }

        // C++ Original: m_cartridge.Init(m_memoryBus);
        Cartridge::init_memory_bus(self.cartridge.clone(), self.cpu.memory_bus_mut());

        // C++ Original: LoadBios(biosRomFile);
        self.load_bios(bios_rom_file);
    }

    // C++ Original: void Reset()
    pub fn reset(&mut self) {
        // C++ Original: Some games rely on initial random state of memory (e.g. Mine Storm)
        // C++ Original: const unsigned int seed = std::random_device{}();
        // C++ Original: m_ram.Randomize(seed);
        let seed = rand::random::<u32>();
        unsafe_mut!(self.ram).randomize(seed);

        // C++ Original: m_cpu.Reset();
        self.cpu.reset();

        // C++ Original: m_via.Reset();
        unsafe_mut!(self.via).reset();
    }

    // C++ Original: bool LoadBios(const char* file)
    pub fn load_bios(&mut self, file: &str) -> bool {
        // C++ Original: return m_biosRom.LoadBiosRom(file);
        match std::fs::read(file) {
            Ok(data) => unsafe_mut!(self.bios_rom).load_bios_rom(&data),
            Err(_) => false,
        }
    }

    // Load BIOS from bytes (for WASM embedded BIOS)
    // C++ Original pattern: bool LoadBiosRom(const uint8_t* data, size_t size)
    pub fn load_bios_from_bytes(&mut self, data: &[u8]) -> bool {
        unsafe_mut!(self.bios_rom).load_bios_rom(data)
    }

    // C++ Original: bool LoadRom(const char* file)
    pub fn load_rom(&mut self, file: &str) -> bool {
        // C++ Original: return m_cartridge.LoadRom(file);
        unsafe_mut!(self.cartridge).load_rom(file)
    }

    // C++ Original: cycles_t ExecuteInstruction(const Input& input, RenderContext& renderContext, AudioContext& audioContext)
    pub fn execute_instruction(
        &mut self,
        input: &Input,
        render_context: &mut RenderContext,
        audio_context: &mut AudioContext,
    ) -> Result<Cycles, crate::core::cpu6809::CpuError> {
        // C++ Original: m_via.SetSyncContext(input, renderContext, audioContext);
        // 1:1 Port: Store pointers to contexts BEFORE executing instruction
        // VIA will use these when sync() is called during memory reads
        unsafe_mut!(self.via).set_sync_context(input, render_context, audio_context);

        // C++ Original: cycles_t cpuCycles = m_cpu.ExecuteInstruction(m_via.IrqEnabled(), m_via.FirqEnabled());
        let cpu_cycles = self.cpu.execute_instruction(
            unsafe_ref!(self.via).irq_enabled(),
            unsafe_ref!(self.via).firq_enabled(),
        )?;

        // C++ Original: m_memoryBus.Sync();
        // 1:1 Port: Sync without render_context (devices get it from their stored state)
        self.cpu.memory_bus_mut().sync();

        Ok(cpu_cycles)
    }

    // C++ Original: void FrameUpdate(double frameTime)
    pub fn frame_update(&mut self, frame_time: f64) {
        // C++ Original: m_via.FrameUpdate(frameTime);
        // C++ Original: Via FrameUpdate - ✅ IMPLEMENTED
        unsafe_mut!(self.via).frame_update(frame_time);
        let _ = frame_time; // suppress unused warning for now
    }

    // ARCHITECTURE FIX: Access memory_bus through CPU
    // C++ Original: MemoryBus& GetMemoryBus() { return m_memoryBus; }
    pub fn get_memory_bus(&mut self) -> &mut MemoryBus {
        self.cpu.memory_bus_mut()
    }

    // C++ Original: Cpu& GetCpu() { return m_cpu; }
    pub fn get_cpu(&mut self) -> &mut Cpu6809 {
        &mut self.cpu
    }

    // C++ Original: Ram& GetRam() { return m_ram; }
    pub fn get_ram(&self) -> Rc<UnsafeCell<Ram>> {
        self.ram.clone()
    }

    // C++ Original: Via& GetVia() { return m_via; }
    pub fn get_via(&self) -> Rc<UnsafeCell<Via6522>> {
        self.via.clone()
    }
}
