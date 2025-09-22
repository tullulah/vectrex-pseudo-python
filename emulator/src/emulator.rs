//! Nueva arquitectura del emulador Vectrex - Implementación gradual
//! 
//! Esta implementación mantiene compatibilidad con el CPU existente
//! mientras expone una interfaz limpia para WASM y futuras mejoras.

use crate::{CPU, BeamSegment};

/// Estado completo del emulador Vectrex (wrapper gradual)
pub struct Emulator {
    /// CPU 6809 (temporalmente mantiene todo embebido)
    pub cpu: CPU,
    
    /// Estadísticas y debugging
    pub stats: EmulatorStats,
    
    /// Configuración
    pub config: EmulatorConfig,
}

/// Estadísticas del emulador
#[derive(Default, Debug)]
pub struct EmulatorStats {
    pub total_cycles: u64,
    pub total_frames: u64,
    pub instructions_executed: u64,
    pub irq_count: u64,
    pub via_writes: u64,
    pub vector_segments_generated: u64,
}

/// Configuración del emulador
#[derive(Default, Debug)]
pub struct EmulatorConfig {
    pub auto_drain_integrator: bool,
    pub enable_sound: bool,
    pub trace_cpu: bool,
    pub trace_via: bool,
    pub max_cycles_per_frame: u32,
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Emulator {
    /// Crear nuevo emulador
    pub fn new() -> Self {
        let mut config = EmulatorConfig::default();
        config.max_cycles_per_frame = 10000; // Default reasonable limit
        
        Self {
            cpu: CPU::default(),
            stats: EmulatorStats::default(),
            config,
        }
    }
    
    /// Cargar BIOS
    pub fn load_bios(&mut self, data: &[u8]) -> bool {
        if data.len() != 4096 && data.len() != 8192 {
            return false;
        }
        self.cpu.load_bios(data);
        // Importante: hacer reset DESPUÉS de cargar la BIOS para configurar PC al vector de reset
        self.cpu.reset();
        true
    }
    
    /// Cargar cartucho
    pub fn load_cartridge(&mut self, data: &[u8]) {
        self.cpu.load_bin(data, 0x0000); // ROM starts at 0x0000
        self.cpu.cart_loaded = true;
    }
    
    /// Cargar cartucho en dirección específica (para compatibilidad API)
    pub fn load_cartridge_at(&mut self, base: u16, data: &[u8]) {
        self.cpu.load_bin(data, base);
        self.cpu.cart_loaded = true;
    }
    
    /// Reset completo del sistema
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.stats = EmulatorStats::default();
    }
    
    /// Resetear estadísticas únicamente
    pub fn reset_stats(&mut self) {
        self.cpu.reset_stats();
        self.stats = EmulatorStats::default();
    }
    
    /// Ejecutar una instrucción
    pub fn step(&mut self) -> bool {
        // Sync config to CPU
        self.cpu.trace = self.config.trace_cpu;
        
        // Execute instruction
        let result = self.cpu.step();
        
        if result {
            self.stats.instructions_executed += 1;
            self.stats.total_cycles = self.cpu.cycles;
            self.stats.irq_count = self.cpu.via_irq_count;
            
            // Extract frame count from CPU's cycle_frame
            self.stats.total_frames = self.cpu.cycle_frame;
        }
        
        result
    }
    
    /// Ejecutar múltiples instrucciones
    pub fn step_multiple(&mut self, count: u32) -> u32 {
        let mut executed = 0;
        for _ in 0..count {
            if !self.step() {
                break;
            }
            executed += 1;
        }
        executed
    }
    
    /// Ejecutar hasta completar un frame (o límite de ciclos)
    pub fn run_frame(&mut self) -> u32 {
        let mut instructions = 0;
        
        while instructions < self.config.max_cycles_per_frame {
            if !self.step() {
                break; // CPU halt
            }
            instructions += 1;
        }
        instructions
    }
    
    /// Obtener segmentos de vectores generados
    pub fn drain_vector_segments(&mut self) -> Vec<BeamSegment> {
        // Extract segments from CPU's integrator using available API
        let segments = self.cpu.integrator.take_segments();
        self.stats.vector_segments_generated += segments.len() as u64;
        segments
    }
    
    /// Obtener estado para debugging
    pub fn debug_state(&self) -> EmulatorDebugState {
        EmulatorDebugState {
            cpu_pc: self.cpu.pc,
            cpu_a: self.cpu.a,
            cpu_b: self.cpu.b,
            cpu_x: self.cpu.x,
            cpu_y: self.cpu.y,
            via_ifr: self.cpu.bus.via.raw_ifr(),
            via_ier: self.cpu.bus.via.raw_ier(),
            integrator_segments: self.cpu.integrator.segments.len(),
            total_cycles: self.stats.total_cycles,
            total_frames: self.stats.total_frames,
            bios_frame: self.cpu.bios_frame,
        }
    }
    
    /// Get last instruction cycles (for timing)
    pub fn last_instruction_cycles(&self) -> u32 {
        // CPU doesn't expose this directly, estimate based on instruction type
        // This is a temporary workaround until CPU is properly refactored
        2 // Most instructions are 2-4 cycles, use conservative estimate
    }
    
    /// Check if IRQ is pending
    pub fn irq_pending(&self) -> bool {
        self.cpu.irq_pending
    }
    
    /// Check if IRQ is masked
    pub fn irq_masked(&self) -> bool {
        self.cpu.cc_i
    }
    
    /// Trigger IRQ
    pub fn trigger_irq(&mut self) {
        self.cpu.irq_pending = true;
    }
}

/// Estado de debugging del emulador
#[derive(Debug, serde::Serialize)]
pub struct EmulatorDebugState {
    pub cpu_pc: u16,
    pub cpu_a: u8,
    pub cpu_b: u8,
    pub cpu_x: u16,
    pub cpu_y: u16,
    pub via_ifr: u8,
    pub via_ier: u8,
    pub integrator_segments: usize,
    pub total_cycles: u64,
    pub total_frames: u64,
    pub bios_frame: u64,
}