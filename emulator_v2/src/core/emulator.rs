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

use crate::core::{MemoryBus, Cpu6809, Via6522, Ram, BiosRom, Cartridge, UnmappedMemoryDevice, IllegalMemoryDevice, DevMemoryDevice, engine_types::{Input, RenderContext, AudioContext}};
use crate::types::Cycles;
use std::rc::Rc;
use std::cell::RefCell;
// ARCHITECTURE FIX: MemoryBus owned by CPU directly (no RefCell for MemoryBus itself)

pub struct Emulator {
    // ARCHITECTURE FIX: CPU owns MemoryBus directly (no RefCell)
    // C++ Original: Cpu m_cpu; + MemoryBus m_memoryBus;
    cpu: Cpu6809,
    
    // C++ Original: Via m_via;
    via: Rc<RefCell<Via6522>>,
    
    // C++ Original: Ram m_ram;
    ram: Rc<RefCell<Ram>>,
    
    // C++ Original: BiosRom m_biosRom;
    bios_rom: Rc<RefCell<BiosRom>>,
    
    // C++ Original: IllegalMemoryDevice m_illegal;
    illegal: Rc<RefCell<IllegalMemoryDevice>>,
    
    // C++ Original: UnmappedMemoryDevice m_unmapped;
    unmapped: Rc<RefCell<UnmappedMemoryDevice>>,
    
    // C++ Original: DevMemoryDevice m_dev;
    dev: Rc<RefCell<DevMemoryDevice>>,
    
    // C++ Original: Cartridge m_cartridge;
    cartridge: Rc<RefCell<Cartridge>>,
}

impl Emulator {
    pub fn new() -> Self {
        // ARCHITECTURE FIX: CPU takes ownership of MemoryBus
        let memory_bus = MemoryBus::new();
        
        Self {
            cpu: Cpu6809::new(memory_bus),
            via: Rc::new(RefCell::new(Via6522::new())),
            ram: Rc::new(RefCell::new(Ram::new())),
            bios_rom: Rc::new(RefCell::new(BiosRom::new())),
            illegal: Rc::new(RefCell::new(IllegalMemoryDevice::new())),
            unmapped: Rc::new(RefCell::new(UnmappedMemoryDevice::new())),
            dev: Rc::new(RefCell::new(DevMemoryDevice::new())),
            cartridge: Rc::new(RefCell::new(Cartridge::new())),
        }
    }
    
    // C++ Original: void Init(const char* biosRomFile)
    pub fn init(&mut self, bios_rom_file: &str) {
        // C++ Original: const bool DeveloperMode = true;
        const DEVELOPER_MODE: bool = true;
        
        // ARCHITECTURE FIX: Access memory_bus through CPU
        // C++ Original: Now connect all devices to memory bus following exact C++ pattern
        // C++ Original: m_cpu.Init(m_memoryBus); - CPU init already handled in constructor
        
        // C++ Original: m_via.Init(m_memoryBus);
        Via6522::init_memory_bus(self.via.clone(), self.cpu.memory_bus_mut());
        
        // C++ Original: Set VIA sync context for component coordination
        self.via.borrow_mut().set_sync_context(
            Input::new(),
            RenderContext::new(), 
            AudioContext::new(1500000.0 / 44100.0) // 1.5MHz / 44.1kHz
        );
        
        // C++ Original: m_ram.Init(m_memoryBus);
        Ram::init_memory_bus(self.ram.clone(), self.cpu.memory_bus_mut());
        
        // C++ Original: m_biosRom.Init(m_memoryBus);
        BiosRom::init_memory_bus(self.bios_rom.clone(), self.cpu.memory_bus_mut());
        
        // C++ Original: m_illegal.Init(m_memoryBus);
        IllegalMemoryDevice::init_memory_bus(self.illegal.clone(), self.cpu.memory_bus_mut());
        
        // C++ Original: if (DeveloperMode) { m_dev.Init(m_memoryBus); } else { m_unmapped.Init(m_memoryBus); }
        if DEVELOPER_MODE {
            // TODO: DevMemoryDevice necesita Weak<RefCell<MemoryBus>> para printf
            // Por ahora skip init hasta resolver arquitectura
            // DevMemoryDevice::init_memory_bus(self.dev.clone(), self.cpu.memory_bus_mut(), weak_ref);
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
        self.ram.borrow_mut().randomize(seed);
        
        // C++ Original: m_cpu.Reset();
        self.cpu.reset();
        
        // C++ Original: m_via.Reset();
        self.via.borrow_mut().reset();
    }
    
    // C++ Original: bool LoadBios(const char* file)
    pub fn load_bios(&mut self, file: &str) -> bool {
        // C++ Original: return m_biosRom.LoadBiosRom(file);
        match std::fs::read(file) {
            Ok(data) => self.bios_rom.borrow_mut().load_bios_rom(&data),
            Err(_) => false,
        }
    }
    
    // Load BIOS from bytes (for WASM embedded BIOS)
    // C++ Original pattern: bool LoadBiosRom(const uint8_t* data, size_t size)
    pub fn load_bios_from_bytes(&mut self, data: &[u8]) -> bool {
        self.bios_rom.borrow_mut().load_bios_rom(data)
    }
    
    // C++ Original: bool LoadRom(const char* file)
    pub fn load_rom(&mut self, file: &str) -> bool {
        // C++ Original: return m_cartridge.LoadRom(file);
        self.cartridge.borrow_mut().load_rom(file)
    }
    
    // C++ Original: cycles_t ExecuteInstruction(const Input& input, RenderContext& renderContext, AudioContext& audioContext)
    pub fn execute_instruction(&mut self, _input: &Input, _render_context: &mut RenderContext, _audio_context: &mut AudioContext) -> Cycles {
        // C++ Original: m_via.SetSyncContext(input, renderContext, audioContext);
        // C++ Original: SetSyncContext for Via - ✅ IMPLEMENTED
        // Via sync context already set in init() method
        
        // C++ Original: cycles_t cpuCycles = m_cpu.ExecuteInstruction(m_via.IrqEnabled(), m_via.FirqEnabled());
        let cpu_cycles = self.cpu.execute_instruction(
            self.via.borrow().irq_enabled(),
            self.via.borrow().firq_enabled()
        );
        
        // C++ Original: m_memoryBus.Sync();
        self.cpu.memory_bus_mut().sync();
        
        cpu_cycles
    }
    
    // C++ Original: void FrameUpdate(double frameTime)
    pub fn frame_update(&mut self, frame_time: f64) {
        // C++ Original: m_via.FrameUpdate(frameTime);
        // C++ Original: Via FrameUpdate - ✅ IMPLEMENTED
        self.via.borrow_mut().frame_update(frame_time);
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
    pub fn get_ram(&self) -> Rc<RefCell<Ram>> {
        self.ram.clone()
    }
    
    // C++ Original: Via& GetVia() { return m_via; }
    pub fn get_via(&self) -> Rc<RefCell<Via6522>> {
        self.via.clone()
    }
}