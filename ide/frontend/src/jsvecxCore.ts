import { MetricsSnapshot, RegistersSnapshot, Segment, IEmulatorCore } from './emulatorCore';

export class JsVecxEmulatorCore implements IEmulatorCore {
  private mod: any = null;
  private inst: any = null;
  private biosOk: boolean = false;
  private frameCounter: number = 0;
  private lastFrameSegments: Segment[] = [];
  private fcInitCached: number | null = null;
  private memScratch: Uint8Array | null = null;
  private vecxEmuPatched: boolean = false;

  async init(){
    if (this.mod) return;
    try {
      // Usar la instancia global de JSVecX creada en index.html (igual que test_jsvecx_exact.html)
      console.log('[JsVecxCore] Using global vecx instance...');
      
      // Verificar que las clases globales existen
      const VecX = (window as any).VecX;
      const vecx = (window as any).vecx;
      const Globals = (window as any).Globals;
      
      if (!VecX) {
        console.error('[JsVecxCore] VecX class not found in global scope - JSVecX scripts not loaded?');
        return;
      }
      
      if (!vecx) {
        console.error('[JsVecxCore] Global vecx instance not found - creation failed?');
        return;
      }
      
      console.log('[JsVecxCore] Found global VecX class and vecx instance');
      this.mod = { VecX, Globals };
      this.inst = vecx;
      
      // Verificar que la instancia tiene los componentes necesarios
      if (!this.inst.rom) {
        console.warn('[JsVecxCore] vecx.rom not initialized - this may cause issues');
      }
      if (!this.inst.cart) {
        console.warn('[JsVecxCore] vecx.cart not initialized - this may cause issues');  
      }
      
      console.log('[JsVecxCore] JSVecX core initialized successfully');
      
      // INICIO DE BLOQUE TRY PARA INICIALIZACIÓN JSVECX
      try {
        // ¡DEJAR QUE JSVECX HAGA SU INICIALIZACIÓN NATIVA!
        // El constructor VecX ya inicializa todo correctamente
        console.log('[JsVecxCore] Using jsvecx native initialization...');
        
        // Solo verificar que los componentes principales existen
        if (!this.inst.rom) {
          console.error('[JsVecxCore] jsvecx failed to initialize ROM array');
          return;
        }
        if (!this.inst.cart) {
          console.error('[JsVecxCore] jsvecx failed to initialize cart array');
          return;
        }
        if (!this.inst.e6809) {
          console.error('[JsVecxCore] jsvecx failed to initialize CPU');
          return;
        }
        
        console.log('[JsVecxCore] jsvecx native initialization verified - rom, cart, cpu ready');
        
        // CONFIGURAR FUNCIONES DE MEMORIA QUE JSVECX NECESITA
        console.log('[JsVecxCore] Setting up memory bus functions...');
        
        // Función read8: lee de los arrays apropiados según la dirección
        this.inst.read8 = (address: number): number => {
          address = address & 0xFFFF; // Asegurar 16-bit
          
          if (address < 0x8000) {
            // Cartridge space 0x0000-0x7FFF
            return this.inst!.cart[address] || 0;
          } else if (address >= 0xC800 && address < 0xD000) {
            // RAM 0xC800-0xCFFF (1K mirrored)
            const ramAddr = (address - 0xC800) & 0x3FF;
            return this.inst!.ram![ramAddr] || 0;
          } else if (address >= 0xD000 && address < 0xE000) {
            // VIA registers 0xD000-0xDFFF (simplified)
            return 0; // TODO: Implementar VIA si es necesario
          } else if (address >= 0xE000) {
            // ROM/BIOS 0xE000-0xFFFF
            const romAddr = address - 0xE000;
            return this.inst!.rom[romAddr] || 0;
          }
          
          return 0xFF; // Unmapped memory
        };
        
        // Función write8: escribe en los arrays apropiados
        this.inst.write8 = (address: number, value: number): void => {
          address = address & 0xFFFF;
          value = value & 0xFF;
          
          if (address < 0x8000) {
            // Cartridge space - normalmente read-only, pero permitimos por ahora
            this.inst!.cart[address] = value;
          } else if (address >= 0xC800 && address < 0xD000) {
            // RAM 0xC800-0xCFFF
            const ramAddr = (address - 0xC800) & 0x3FF;
            if (this.inst!.ram) this.inst!.ram[ramAddr] = value;
          } else if (address >= 0xD000 && address < 0xE000) {
            // VIA registers - ignorar por ahora
            // TODO: Implementar escritura VIA si es necesario
          }
          // ROM es read-only, ignorar escrituras
        };
        
        console.log('[JsVecxCore] Memory bus functions configured');
        
        // Asignar funciones de memoria a TODOS los contextos posibles donde jsvecx las busque
        this.assignMemoryFunctionsToAllContexts();
        
        // CONFIGURAR PC CORRECTAMENTE PARA ARRANQUE DE BIOS (sin hacks de cartridge)
        console.log('[JsVecxCore] Setting up proper BIOS startup...');
        
        console.log('[JsVecxCore] Arrays initialized, attempting vecx_reset...');
        try {
          this.inst.vecx_reset();
          console.log('[JsVecxCore] vecx_reset completed successfully');
        } catch (resetError) {
          console.warn('[JsVecxCore] vecx_reset failed, doing manual initialization:', resetError);
          // Reset manual básico
          this.inst.vector_draw_cnt = 0;
          this.inst.fcycles = 0;
        }
        
        // NOTA: PC se configurará al reset vector DESPUÉS de cargar la BIOS en loadBios()
        console.log('[JsVecxCore] PC will be set to reset vector after BIOS loading');
        
        // FINAL DE INICIALIZACIÓN - RECREAR FUNCIONES DE MEMORIA
        console.log('[JsVecxCore] Recreating memory bus functions after initialization...');
        this.recreateMemoryFunctions();
        this.assignMemoryFunctionsToAllContexts();
        
      } catch (jsvecxError) {
        console.warn('[JsVecxCore] jsvecx initialization failed', jsvecxError);
      }
    } catch (e) {
      console.warn('[JsVecxCore] init failed', e);
    }
  }

  private recreateMemoryFunctions() {
    if (!this.inst) return;
    
    this.inst.read8 = (address: number): number => {
      address = address & 0xFFFF;
      if (address < 0x8000) {
        return this.inst!.cart[address] || 0;
      } else if (address >= 0xC800 && address < 0xD000) {
        const ramAddr = (address - 0xC800) & 0x3FF;
        return this.inst!.ram![ramAddr] || 0;
      } else if (address >= 0xE000) {
        const romAddr = address - 0xE000;
        return this.inst!.rom[romAddr] || 0;
      }
      return 0xFF;
    };
    
    this.inst.write8 = (address: number, value: number): void => {
      address = address & 0xFFFF;
      value = value & 0xFF;
      if (address < 0x8000) {
        this.inst!.cart[address] = value;
      } else if (address >= 0xC800 && address < 0xD000) {
        const ramAddr = (address - 0xC800) & 0x3FF;
        if (this.inst!.ram) this.inst!.ram[ramAddr] = value;
      }
    };
  }

  private assignMemoryFunctionsToAllContexts() {
    if (!this.inst || !this.inst.read8 || !this.inst.write8) return;
    
    console.log('[JsVecxCore] Assigning memory functions to all possible contexts...');
    
    // CRÍTICO: jsvecx parece buscar las funciones en el contexto global/this de e6809_sstep
    // Vamos a asignar a ABSOLUTAMENTE TODOS los contextos posibles
    
    // 1. Contexto global (window)
    (window as any).read8 = this.inst.read8;
    (window as any).write8 = this.inst.write8;
    
    // 2. Contexto del módulo completo
    if (this.mod) {
      (this.mod as any).read8 = this.inst.read8;
      (this.mod as any).write8 = this.inst.write8;
      
      // 3. Todos los contextos internos del módulo
      if ((this.mod as any).Globals) {
        (this.mod.Globals as any).read8 = this.inst.read8;
        (this.mod.Globals as any).write8 = this.inst.write8;
      }
      
      if ((this.mod as any).HEAP8) {
        (this.mod.HEAP8 as any).read8 = this.inst.read8;
        (this.mod.HEAP8 as any).write8 = this.inst.write8;
      }
      
      // 4. Asignar al propio constructor VecX
      if (this.mod.VecX) {
        (this.mod.VecX as any).read8 = this.inst.read8;
        (this.mod.VecX as any).write8 = this.inst.write8;
        (this.mod.VecX.prototype as any).read8 = this.inst.read8;
        (this.mod.VecX.prototype as any).write8 = this.inst.write8;
      }
    }
    
    // 5. Instancia principal
    this.inst.read8 = this.inst.read8; // Redundante pero asegurar
    this.inst.write8 = this.inst.write8;
    
    // 6. Asignar al CPU y TODOS sus contextos
    if (this.inst.e6809) {
      console.log('[JsVecxCore] Assigning memory functions to CPU...');
      (this.inst.e6809 as any).read8 = this.inst.read8;
      (this.inst.e6809 as any).write8 = this.inst.write8;
      
      // También asignar al prototipo del CPU si existe
      if (this.inst.e6809.constructor) {
        (this.inst.e6809.constructor as any).read8 = this.inst.read8;
        (this.inst.e6809.constructor as any).write8 = this.inst.write8;
        if (this.inst.e6809.constructor.prototype) {
          (this.inst.e6809.constructor.prototype as any).read8 = this.inst.read8;
          (this.inst.e6809.constructor.prototype as any).write8 = this.inst.write8;
        }
      }
    }
    
    // 7. CRÍTICO: Asignar a 'this' del contexto actual de la función
    (this as any).read8 = this.inst.read8;
    (this as any).write8 = this.inst.write8;
    
    // 8. SÚPER CRÍTICO: En JavaScript, a veces las funciones buscan en el objeto global
    try {
      (globalThis as any).read8 = this.inst.read8;
      (globalThis as any).write8 = this.inst.write8;
    } catch {}
    
    // 9. NUEVO ENFOQUE: Interceptar e6809_sstep para asignar funciones JUSTo antes de ejecutar
    if (this.inst.e6809 && (this.inst.e6809 as any).e6809_sstep && !(this.inst.e6809 as any)._originalSstep) {
      const originalSstep = (this.inst.e6809 as any).e6809_sstep;
      // CRÍTICO: Guardar referencia para evitar recursión infinita
      (this.inst.e6809 as any)._originalSstep = originalSstep;
      
      (this.inst.e6809 as any).e6809_sstep = (...args: any[]) => {
        // ESTRATEGIA AGRESIVA: Inyectar funciones en el scope global Y local
        const memoryFunctions = {
          read8: this.inst.read8,
          write8: this.inst.write8
        };
        
        // Asignar a TODOS los posibles contextos que e6809_sstep podría usar
        (this.inst.e6809 as any).read8 = memoryFunctions.read8;
        (this.inst.e6809 as any).write8 = memoryFunctions.write8;
        (this.inst as any).read8 = memoryFunctions.read8;
        (this.inst as any).write8 = memoryFunctions.write8;
        (window as any).read8 = memoryFunctions.read8;
        (window as any).write8 = memoryFunctions.write8;
        (globalThis as any).read8 = memoryFunctions.read8;
        (globalThis as any).write8 = memoryFunctions.write8;
        
        // USAR LA FUNCIÓN ORIGINAL GUARDADA para evitar recursión
        try {
          return (this.inst.e6809 as any)._originalSstep.call(this.inst.e6809, ...args);
        } catch (e) {
          console.warn('[JsVecxCore] e6809_sstep with call failed, trying direct:', e);
          return (this.inst.e6809 as any)._originalSstep.apply(this.inst.e6809, args);
        }
      };
      console.log('[JsVecxCore] Intercepted e6809_sstep to inject memory functions before each CPU step');
    }
    
    // 10. ENFOQUE EXTREMO: Bind las funciones al contexto del CPU
    if (this.inst.e6809) {
      try {
        (this.inst.e6809 as any).read8 = this.inst.read8.bind(this.inst);
        (this.inst.e6809 as any).write8 = this.inst.write8.bind(this.inst);
        console.log('[JsVecxCore] Bound memory functions to CPU context');
      } catch (e) {
        console.log('[JsVecxCore] Could not bind memory functions:', e);
      }
    }
    
    // 11. ENFOQUE NUCLEAR: Definir funciones en el scope global de JavaScript
    try {
      const globalCode = `
        if (typeof read8 === 'undefined') {
          window.read8 = ${this.inst.read8.toString()};
          window.write8 = ${this.inst.write8.toString()};
          var read8 = window.read8;
          var write8 = window.write8;
        }
      `;
      // Usar setTimeout para ejecutar en el siguiente tick
      setTimeout(() => {
        try {
          (window as any).eval(globalCode);
          console.log('[JsVecxCore] Global functions defined via eval');
        } catch (evalError) {
          console.warn('[JsVecxCore] Eval failed:', evalError);
        }
      }, 0);
    } catch (e) {
      console.log('[JsVecxCore] Could not set global functions:', e);
    }
    
    console.log('[JsVecxCore] Memory functions assigned to ALL contexts (inst, e6809, Globals, module, HEAP8, window, VecX, globalThis, this)');
    
    // Debug final de función assignments
    console.log('[JsVecxCore] Function assignment verification:');
    console.log('  inst.read8 =', typeof this.inst.read8);
    console.log('  e6809.read8 =', typeof (this.inst.e6809 as any)?.read8);
    console.log('  Globals.read8 =', typeof (this.mod?.Globals as any)?.read8);
    console.log('  module.read8 =', typeof (this.mod as any)?.read8);
    console.log('  window.read8 =', typeof (window as any).read8);
    console.log('  globalThis.read8 =', typeof (globalThis as any).read8);
    console.log('  this.read8 =', typeof (this as any).read8);
  }
  
  loadBios(bytes: Uint8Array){
    // jsvecx espera 8K ROM en this.rom; copiamos mínimo (clamp a 0x2000)
    if (!this.inst) return; 
    
    // Asegurar que el array ROM existe
    if (!this.inst.rom) {
      this.inst.rom = new Array(0x2000).fill(0);
    }
    
    const maxLen = Math.min(bytes.length, 0x2000);
    for (let i = 0; i < maxLen; i++) {
      this.inst.rom[i] = bytes[i];
    }
    this.biosOk = true;
    console.log(`[JsVecxCore] BIOS loaded: ${maxLen} bytes copied to ROM`);
    
    // AHORA QUE LA BIOS ESTÁ CARGADA, CONFIGURAR PC AL RESET VECTOR
    if (this.inst.rom.length >= 0x2000 && this.inst.e6809) {
      // Reset vector está en 0xFFFE-0xFFFF (últimos 2 bytes de ROM)
      const resetVectorLow = this.inst.rom[0x1FFE];   // 0xFFFE - 0xE000 = 0x1FFE 
      const resetVectorHigh = this.inst.rom[0x1FFF];  // 0xFFFF - 0xE000 = 0x1FFF
      const resetVector = (resetVectorHigh << 8) | resetVectorLow;
      
      console.log(`[JsVecxCore] Reset vector bytes: High=0x${resetVectorHigh.toString(16).toUpperCase()}, Low=0x${resetVectorLow.toString(16).toUpperCase()}`);
      console.log(`[JsVecxCore] Calculated reset vector: 0x${resetVector.toString(16).toUpperCase()}`);
      
      // VERIFICAR si la BIOS está realmente cargada
      const biosCheck = this.inst.rom.slice(0x1FF0, 0x2000);
      console.log(`[JsVecxCore] BIOS end bytes (0xFFF0-0xFFFF):`, biosCheck.map((b: number) => b.toString(16).padStart(2, '0')).join(' '));
      
      // Configurar PC directamente al reset vector de la BIOS
      if (resetVector !== 0 && resetVector >= 0x1000) {
        this.inst.e6809.reg_pc = resetVector;
        console.log(`[JsVecxCore] PC set to BIOS reset vector: 0x${resetVector.toString(16).toUpperCase()}`);
      } else {
        console.warn(`[JsVecxCore] Invalid reset vector 0x${resetVector.toString(16)}, using fallback 0xF000`);
        this.inst.e6809.reg_pc = 0xF000;
        console.log('[JsVecxCore] PC set to fallback BIOS address: 0xF000');
      }
    }
  }
  
  isBiosLoaded(){ return this.biosOk; }
  
  reset(coldReset: boolean = true){
    if (!this.inst) return;
    
    try { 
      this.inst.vecx_reset(); 
      console.log(`[JsVecxCore] Reset successful (${coldReset ? 'cold' : 'warm'} reset)`);
    } catch(e) { 
      console.warn('[JsVecxCore] Reset failed, doing manual reset:', e);
      // Reset manual básico
      this.inst.vector_draw_cnt = 0;
      this.inst.fcycles = 0;
      
      // Recrear funciones de memoria tras fallo de reset
      this.recreateMemoryFunctions();
      this.assignMemoryFunctionsToAllContexts();
    } 
    
    // CONFIGURAR PC Y MEMORIA SEGÚN TIPO DE RESET
    if (coldReset) {
      // COLD RESET: Limpiar Vec_Cold_Flag y ir al inicio de BIOS
      if (this.inst.ram) {
        // Clear Vec_Cold_Flag to force cold start with VECTREX screen and music
        // Vec_Cold_Flag is at $CBFE, which maps to RAM index 0x3FE (0xCBFE & 0x3FF)
        this.inst.ram[0x3FE] = 0x00;  // Clear cold start flag
        this.inst.ram[0x3FF] = 0x00;  // Clear both bytes to ensure it's not $7321
        console.log('[JsVecxCore] Cold reset: Vec_Cold_Flag cleared');
      }
      
      // Set PC to BIOS start for cold start sequence
      if (this.inst.e6809) {
        this.inst.e6809.reg_pc = 0xF000;  // BIOS start address
        console.log('[JsVecxCore] Cold reset: PC set to BIOS start 0xF000');
      }
    } else {
      // WARM RESET: Usar reset vector normal (conserva memoria)
      if (this.inst.rom && this.inst.rom.length >= 0x2000 && this.inst.e6809) {
        // Reset vector está en 0xFFFE-0xFFFF (últimos 2 bytes de ROM)
        const resetVectorLow = this.inst.rom[0x1FFE];   // 0xFFFE - 0xE000 = 0x1FFE 
        const resetVectorHigh = this.inst.rom[0x1FFF];  // 0xFFFF - 0xE000 = 0x1FFF
        const resetVector = (resetVectorHigh << 8) | resetVectorLow;
        
        console.log(`[JsVecxCore] Reset vector bytes: High=0x${resetVectorHigh.toString(16).toUpperCase()}, Low=0x${resetVectorLow.toString(16).toUpperCase()}`);
        console.log(`[JsVecxCore] Reset vector calculated: 0x${resetVector.toString(16).toUpperCase()}`);
        
        if (resetVector !== 0 && resetVector >= 0x1000) {
          this.inst.e6809.reg_pc = resetVector;
          console.log(`[JsVecxCore] Warm reset: PC set to BIOS reset vector: 0x${resetVector.toString(16).toUpperCase()}`);
        } else {
          // Fallback si el reset vector es sospechoso
          this.inst.e6809.reg_pc = 0xF000;
          console.log(`[JsVecxCore] Warm reset: Suspicious reset vector 0x${resetVector.toString(16)}, using fallback 0xF000`);
        }
      } else if (this.inst.e6809) {
        // Fallback básico
        this.inst.e6809.reg_pc = 0xF000;
        console.log('[JsVecxCore] Warm reset: PC set to default BIOS address: 0xF000');
      }
    }
    
    this.frameCounter = 0; 
  }

  loadProgram(bytes: Uint8Array, _base?: number){
    // Para jsvecx, cargar en cartridge (0x0000-0x7FFF)
    if (!this.inst) return;
    
    // Asegurar que el cartucho existe
    if (!this.inst.cart) {
      this.inst.cart = new Array(0x8000).fill(0);
    }
    
    const maxLen = Math.min(bytes.length, 0x8000);
    for (let i = 0; i < maxLen; i++) {
      this.inst.cart[i] = bytes[i];
    }
    console.log(`[JsVecxCore] Program loaded: ${maxLen} bytes to cartridge`);
  }

  runFrame(_maxInstr?: number){
    if (!this.inst) return { stepsRun: 0, vectors: [] };
    
    try { 
      // CRÍTICO: Re-asignar funciones de memoria antes de cada frame 
      // por si jsvecx las pierde durante reset o ejecución
      console.log('[JsVecxCore] Re-assigning memory functions before frame execution...');
      this.assignMemoryFunctionsToAllContexts();
      
      // NUEVO: Monkey-patch vecx_emu para asegurar funciones antes de CADA llamada
      if (!this.vecxEmuPatched && this.inst.vecx_emu) {
        const originalVecxEmu = this.inst.vecx_emu;
        this.inst.vecx_emu = (cycles: number, cyclesDone: number) => {
          // Asegurar funciones JUSTO antes de ejecutar emulación
          console.log('[JsVecxCore] Patched vecx_emu: Re-assigning functions before execution...');
          this.assignMemoryFunctionsToAllContexts();
          return originalVecxEmu.call(this.inst, cycles, cyclesDone);
        };
        this.vecxEmuPatched = true;
        console.log('[JsVecxCore] Patched vecx_emu to inject memory functions');
      }
      
      // Intentar ejecutar un frame usando la función jsvecx
      this.inst.vecx_emu(40000, 0); // Aprox 40K cycles por frame
      
      // Extraer vectores del frame actual
      const vectors: Segment[] = [];
      if (this.inst.vectors_draw && Array.isArray(this.inst.vectors_draw)) {
        console.log(`[JsVecxCore] Processing ${this.inst.vectors_draw.length} raw vectors from JSVecX`);
        
        // Log los primeros vectores para diagnóstico
        this.inst.vectors_draw.slice(0, 3).forEach((v: any, i: number) => {
          console.log(`  Raw Vector ${i}:`, {
            type: typeof v,
            keys: v ? Object.keys(v) : 'null',
            x0: v?.x0, y0: v?.y0, x1: v?.x1, y1: v?.y1, 
            intensity: v?.intensity, color: v?.color,
            hasProps: { x0: 'x0' in v, y0: 'y0' in v, x1: 'x1' in v, y1: 'y1' in v, intensity: 'intensity' in v, color: 'color' in v }
          });
        });
        
        for (const v of this.inst.vectors_draw) {
          if (v && typeof v === 'object') {
            const segment = {
              x0: v.x0 ?? 0,
              y0: v.y0 ?? 0,
              x1: v.x1 ?? 0,
              y1: v.y1 ?? 0,
              intensity: v.color ?? v.intensity ?? 0, // JSVecX usa 'color', no 'intensity'
              frame: this.frameCounter
            };
            vectors.push(segment);
          }
        }
        
        console.log(`[JsVecxCore] Converted ${vectors.length} valid segments`);
        if (vectors.length > 0) {
          console.log(`  First converted segment:`, vectors[0]);
        }
      } else {
        console.log(`[JsVecxCore] No vectors_draw available:`, {
          exists: !!this.inst.vectors_draw,
          isArray: Array.isArray(this.inst.vectors_draw),
          length: this.inst.vectors_draw?.length
        });
      }
      
      this.lastFrameSegments = vectors;
      this.frameCounter++;
      
      console.log(`[JsVecxCore] Frame ${this.frameCounter} completed - ${vectors.length} vectors drawn`);
      
      return { 
        stepsRun: 40000, // Estimado 
        vectors: vectors 
      };
    } catch(e){ 
      console.warn('[JsVecxCore] runFrame failed:', e);
      return { stepsRun: 0, vectors: [] };
    }
  }

  metrics(): MetricsSnapshot | null {
    if (!this.inst) return null;
    
    return {
      total: 0, // No disponible en jsvecx
      unimplemented: 0, // No aplica en jsvecx
      frames: this.frameCounter,
      draw_vl: this.lastFrameSegments.length,
      last_intensity: 0, // No fácilmente disponible
      unique_unimplemented: [], // No aplica
      cycles: this.inst.fcycles || 0,
      top_opcodes: [] // No disponible en jsvecx
    };
  }

  registers(): RegistersSnapshot | null {
    if (!this.inst?.e6809) return null;
    
    const cpu = this.inst.e6809;
    return {
      a: cpu.reg_a || 0,
      b: cpu.reg_b || 0,
      dp: cpu.reg_dp || 0,
      x: cpu.reg_x?.value || 0,
      y: cpu.reg_y?.value || 0,
      u: cpu.reg_u?.value || 0,
      s: cpu.reg_s?.value || 0,
      pc: cpu.reg_pc || 0,
      cycles: this.inst.fcycles || 0,
      frame_count: this.frameCounter,
      last_intensity: 0 // No fácilmente disponible en jsvecx
    };
  }

  getSegmentsShared(): Segment[] { return this.lastFrameSegments; }

  resetStats(){ /* noop */ }
  biosCalls(){ return []; }
  clearBiosCalls(){ /* noop */ }
}