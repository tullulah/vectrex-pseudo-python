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

use crate::core::{MemoryBus, Cpu6809, Via6522, Ram, BiosRom};
use crate::types::{Cycles, Input, RenderContext, AudioContext};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Emulator {
    // C++ Original: MemoryBus m_memoryBus;
    memory_bus: Rc<RefCell<MemoryBus>>,
    
    // C++ Original: Cpu m_cpu;
    cpu: Cpu6809,
    
    // C++ Original: Via m_via;
    via: Via6522,
    
    // C++ Original: Ram m_ram;
    ram: Ram,
    
    // C++ Original: BiosRom m_biosRom;
    bios_rom: BiosRom,
    
    // TODO: Implement missing devices following 1:1 verification
    // C++ Original: IllegalMemoryDevice m_illegal;
    // illegal: IllegalMemoryDevice,
    
    // C++ Original: UnmappedMemoryDevice m_unmapped;
    // unmapped: UnmappedMemoryDevice,
    
    // C++ Original: DevMemoryDevice m_dev;
    // dev: DevMemoryDevice,
    
    // C++ Original: Cartridge m_cartridge;
    // cartridge: Cartridge,
}

impl Emulator {
    pub fn new() -> Self {
        let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
        
        Self {
            cpu: Cpu6809::new(memory_bus.clone()),
            via: Via6522::new(),
            ram: Ram::new(),
            bios_rom: BiosRom::new(),
            memory_bus,
        }
    }
    
    // C++ Original: void Init(const char* biosRomFile)
    pub fn init(&mut self, bios_rom_file: &str) {
        // C++ Original: const bool DeveloperMode = true;
        const DEVELOPER_MODE: bool = true;
        
        // C++ Original: m_cpu.Init(m_memoryBus); - Already done in constructor
        // C++ Original: m_via.Init(m_memoryBus); - TODO: implement Init for Via
        // C++ Original: m_ram.Init(m_memoryBus); - TODO: implement Init for Ram  
        // C++ Original: m_biosRom.Init(m_memoryBus); - TODO: implement Init for BiosRom
        // C++ Original: m_illegal.Init(m_memoryBus); - TODO: implement IllegalMemoryDevice
        
        // C++ Original: if (DeveloperMode) { m_dev.Init(m_memoryBus); } else { m_unmapped.Init(m_memoryBus); }
        // TODO: implement DevMemoryDevice and UnmappedMemoryDevice
        
        // C++ Original: m_cartridge.Init(m_memoryBus); - TODO: implement Cartridge
        
        // C++ Original: LoadBios(biosRomFile);
        self.load_bios(bios_rom_file);
    }
    
    // C++ Original: void Reset()
    pub fn reset(&mut self) {
        // C++ Original: Some games rely on initial random state of memory (e.g. Mine Storm)
        // C++ Original: const unsigned int seed = std::random_device{}();
        // C++ Original: m_ram.Randomize(seed);
        let seed = rand::random::<u32>();
        self.ram.randomize(seed);
        
        // C++ Original: m_cpu.Reset();
        self.cpu.reset();
        
        // C++ Original: m_via.Reset();
        // TODO: implement reset for Via
    }
    
    // C++ Original: bool LoadBios(const char* file)
    pub fn load_bios(&mut self, file: &str) -> bool {
        // C++ Original: return m_biosRom.LoadBiosRom(file);
        match std::fs::read(file) {
            Ok(data) => self.bios_rom.load_bios_rom(&data),
            Err(_) => false,
        }
    }
    
    // C++ Original: bool LoadRom(const char* file)
    pub fn load_rom(&mut self, _file: &str) -> bool {
        // C++ Original: return m_cartridge.LoadRom(file);
        // TODO: implement Cartridge
        false
    }
    
    // C++ Original: cycles_t ExecuteInstruction(const Input& input, RenderContext& renderContext, AudioContext& audioContext)
    pub fn execute_instruction(&mut self, _input: &Input, _render_context: &mut RenderContext, _audio_context: &mut AudioContext) -> Cycles {
        // C++ Original: m_via.SetSyncContext(input, renderContext, audioContext);
        // TODO: implement SetSyncContext for Via
        
        // C++ Original: cycles_t cpuCycles = m_cpu.ExecuteInstruction(m_via.IrqEnabled(), m_via.FirqEnabled());
        // TODO: implement ExecuteInstruction for CPU and IRQ/FIRQ methods for Via
        let cpu_cycles = 1; // Stub
        
        // C++ Original: m_memoryBus.Sync();
        // TODO: implement Sync for MemoryBus
        
        cpu_cycles
    }
    
    // C++ Original: void FrameUpdate(double frameTime)
    pub fn frame_update(&mut self, frame_time: f64) {
        // C++ Original: m_via.FrameUpdate(frameTime);
        self.via.frame_update(frame_time);
    }
    
    // C++ Original: MemoryBus& GetMemoryBus() { return m_memoryBus; }
    pub fn get_memory_bus(&self) -> Rc<RefCell<MemoryBus>> {
        self.memory_bus.clone()
    }
    
    // C++ Original: Cpu& GetCpu() { return m_cpu; }
    pub fn get_cpu(&mut self) -> &mut Cpu6809 {
        &mut self.cpu
    }
    
    // C++ Original: Ram& GetRam() { return m_ram; }
    pub fn get_ram(&mut self) -> &mut Ram {
        &mut self.ram
    }
    
    // C++ Original: Via& GetVia() { return m_via; }
    pub fn get_via(&mut self) -> &mut Via6522 {
        &mut self.via
    }
}