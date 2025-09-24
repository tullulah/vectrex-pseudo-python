import { MetricsSnapshot, RegistersSnapshot, Segment } from './types';

export class JsVecxCore {
  private mod: any = null;
  private inst: any = null;
  private biosOk: boolean = false;
  private frameCounter: number = 0;
  private lastFrameSegments: Segment[] = [];
  private fcInitCached: number | null = null;
  private memScratch: Uint8Array | null = null;

  async init(){
    if (this.mod) return;
    try {
      // Simplificado: sólo intentamos cargar el bundle estático servido desde /public -> /jsvecx/vecx_full.js
      // Razón: Vite emite warning si se importan directamente assets crudos dentro de /public (no pasan por transform).
      // El script jsvecx:bundle garantiza que el archivo exporte VecX y Globals.
      let bundle: any = null;
      try {
        // Nueva ubicación: bundle generado en src/generated/jsvecx/vecx_full.js (incluido en pipeline de TS/Vite)
        // Usamos import relativo explícito para permitir tree-shaking futuro si se particiona.
        const bundlePath = './generated/jsvecx/vecx_full.js';
        bundle = await import(/* @vite-ignore */ bundlePath);
      } catch (e){
        console.warn('[JsVecxCore] no se pudo importar bundle interno generated/jsvecx/vecx_full.js; backend jsvecx inerte', e);
        return;
      }
      
      const VecX = (bundle as any).VecX || (bundle as any).default?.VecX;
      const Globals = (bundle as any).Globals || (bundle as any).default?.Globals;
      if (!VecX) throw new Error('VecX constructor not found');
      this.mod = { VecX, Globals } as any;
      this.inst = new VecX();
      
      // INICIO DE BLOQUE TRY PARA INICIALIZACIÓN JSVECX
      try {
        // Inicializar arrays antes de llamar a vecx_reset para evitar null pointer
        console.log('[JsVecxCore] Initializing arrays before reset...');
        if (this.inst) {
          // Inicializar arrays básicos que vecx_reset necesita
          if (!this.inst.rom) this.inst.rom = new Array(0x2000).fill(0);
          if (!this.inst.cart) this.inst.cart = new Array(0x8000).fill(0);
          if (!this.inst.vectors_draw) this.inst.vectors_draw = [];
          
          // Inicializar registros CPU si es necesario
          if (!this.inst.e6809) {
            this.inst.e6809 = {
              reg_a: 0, reg_b: 0, reg_dp: 0, reg_pc: 0, reg_cc: 0,
              reg_x: { value: 0 }, reg_y: { value: 0 }, reg_u: { value: 0 }, reg_s: { value: 0 }
            };
          }
          
          // Más arrays que jsvecx podría necesitar
          if (!this.inst.ram) this.inst.ram = new Array(0x400).fill(0); // 1K RAM
          this.inst.vector_draw_cnt = 0;
          this.inst.fcycles = 0;
          this.inst.alg_vectoring = 0;
          
          // Arrays adicionales que jsvecx podría necesitar para vecx_reset
          const inst = this.inst as any;
          if (!inst.alg_max_x) inst.alg_max_x = 33000;
          if (!inst.alg_max_y) inst.alg_max_y = 41000;
          if (!inst.alg_dx) inst.alg_dx = 0;
          if (!inst.alg_dy) inst.alg_dy = 0;
          if (!inst.alg_curr_x) inst.alg_curr_x = 0;
          if (!inst.alg_curr_y) inst.alg_curr_y = 0;
          if (!inst.alg_intensity) inst.alg_intensity = 0;
          if (!inst.alg_jch0) inst.alg_jch0 = 128;
          if (!inst.alg_jch1) inst.alg_jch1 = 128;
          if (!inst.alg_jch2) inst.alg_jch2 = 128;
          if (!inst.alg_jch3) inst.alg_jch3 = 128;
          if (!inst.alg_jsh) inst.alg_jsh = new Array(4).fill(0);
          if (!inst.alg_zsh) inst.alg_zsh = 128;
          
          // Arrays VIA que podrían ser necesarios
          if (!inst.via_ora) inst.via_ora = 0;
          if (!inst.via_orb) inst.via_orb = 0;
          if (!inst.via_ddra) inst.via_ddra = 0;
          if (!inst.via_ddrb) inst.via_ddrb = 0;
          if (!inst.via_t1c) inst.via_t1c = 0;
          if (!inst.via_t1l) inst.via_t1l = 0;
          if (!inst.via_t2c) inst.via_t2c = 0;
          if (!inst.via_t2l) inst.via_t2l = 0;
          if (!inst.via_acr) inst.via_acr = 0;
          if (!inst.via_pcr) inst.via_pcr = 0;
          if (!inst.via_ifr) inst.via_ifr = 0;
          if (!inst.via_ier) inst.via_ier = 0;
          if (!inst.via_ca2) inst.via_ca2 = 0;
          if (!inst.via_cb2) inst.via_cb2 = 0;
          
          // Arrays de sonido que podrían ser necesarios
          if (!inst.snd_regs) inst.snd_regs = new Array(16).fill(0);
          if (!inst.dac_a) inst.dac_a = 0;
          if (!inst.dac_b) inst.dac_b = 0;
          if (!inst.dac_c) inst.dac_c = 0;
          if (!inst.dac_d) inst.dac_d = 0;
          
          console.log('[JsVecxCore] Extended arrays initialized for jsvecx compatibility');
          
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
          
          // CRÍTICO: jsvecx está diseñado para ejecutar cartuchos, no solo BIOS
          // Debemos cargar un cartucho mínimo que salte inmediatamente a la BIOS
          console.log('[JsVecxCore] Setting up minimal cartridge to jump to BIOS...');
          
          // VERIFICACIÓN CRÍTICA: Asegurar que el array cart está realmente inicializado
          if (!this.inst.cart || !Array.isArray(this.inst.cart)) {
            console.log('[JsVecxCore] Creating new cart array...');
            this.inst.cart = new Array(0x8000).fill(0);
          } else if (this.inst.cart.length < 0x8000) {
            console.log(`[JsVecxCore] Expanding cart array from ${this.inst.cart.length} to 0x8000...`);
            while (this.inst.cart.length < 0x8000) {
              this.inst.cart.push(0);
            }
          }
          
          // Crear un cartucho mínimo que haga JMP $F000 (inicio típico BIOS)
          // Opcode 7E = JMP Extended, seguido de dirección F000
          this.inst.cart[0] = 0x7E;  // JMP Extended
          this.inst.cart[1] = 0xF0;  // High byte de F000
          this.inst.cart[2] = 0x00;  // Low byte de F000
          
          // También establecer el reset vector del cartucho para apuntar a 0x0000
          // donde está nuestro JMP a la BIOS
          this.inst.cart[0x7FFE] = 0x00;  // Reset vector low byte (apunta a 0x0000)
          this.inst.cart[0x7FFF] = 0x00;  // Reset vector high byte
          
          // Verificar que los datos se escribieron correctamente
          console.log(`[JsVecxCore] Cart verification: [0]=${this.inst.cart[0]}, [1]=${this.inst.cart[1]}, [2]=${this.inst.cart[2]}`);
          console.log(`[JsVecxCore] Reset vector: [0x7FFE]=${this.inst.cart[0x7FFE]}, [0x7FFF]=${this.inst.cart[0x7FFF]}`);
          
          // VERIFICAR ROM también
          if (!this.inst.rom || !Array.isArray(this.inst.rom)) {
            console.log('[JsVecxCore] Creating new rom array...');
            this.inst.rom = new Array(0x2000).fill(0);
          } else if (this.inst.rom.length < 0x2000) {
            console.log(`[JsVecxCore] Expanding rom array from ${this.inst.rom.length} to 0x2000...`);
            while (this.inst.rom.length < 0x2000) {
              this.inst.rom.push(0);
            }
          }
          console.log(`[JsVecxCore] ROM array verification: length=${this.inst.rom.length}, isArray=${Array.isArray(this.inst.rom)}`);
          
          console.log('[JsVecxCore] Minimal cartridge setup complete, attempting vecx_reset...');
          try {
            this.inst.vecx_reset();
            console.log('[JsVecxCore] vecx_reset completed successfully');
            
            // Después del reset, verificar que el PC esté en 0x0000 (inicio del cartucho)
            if (this.inst.e6809?.reg_pc === 0) {
              console.log('[JsVecxCore] PC correctly initialized to 0x0000 (cartridge start)');
            } else {
              console.warn(`[JsVecxCore] Unexpected PC after reset: 0x${this.inst.e6809?.reg_pc?.toString(16)}`);
            }
          } catch (resetError) {
            console.warn('[JsVecxCore] vecx_reset failed, doing manual initialization:', resetError);
            // Reset manual - forzar PC a 0x0000 para ejecutar nuestro JMP
            this.inst.vector_draw_cnt = 0;
            this.inst.fcycles = 0;
            if (this.inst.e6809) {
              this.inst.e6809.reg_pc = 0x0000; // Empezar en el cartucho donde está nuestro JMP
              this.inst.e6809.reg_a = 0;
              this.inst.e6809.reg_b = 0;
              this.inst.e6809.reg_dp = 0;
              this.inst.e6809.reg_x = { value: 0 };
              this.inst.e6809.reg_y = { value: 0 };
              this.inst.e6809.reg_u = { value: 0x8000 }; // Stack inicial típico
              this.inst.e6809.reg_s = { value: 0x8000 };
              
              console.log(`[JsVecxCore] Manual reset - PC set to 0x${this.inst.e6809.reg_pc.toString(16)} (cartridge start)`);
            }
            
            // RECREAR FUNCIONES DE MEMORIA tras fallo en reset
            console.log('[JsVecxCore] Recreating memory bus functions after reset failure...');
            this.recreateMemoryFunctions();
            this.assignMemoryFunctionsToAllContexts();
          }
        }
        
      } catch (e){
        console.warn('[JsVecxCore] jsvecx initialization failed', e);
      }
    } catch (e){
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
    
    // 1. Asignar al CPU
    if (this.inst.e6809) {
      console.log('[JsVecxCore] Assigning memory functions to CPU...');
      (this.inst.e6809 as any).read8 = this.inst.read8;
      (this.inst.e6809 as any).write8 = this.inst.write8;
    }
    
    // 2. Asignar a Globals del módulo
    if (this.mod?.Globals) {
      console.log('[JsVecxCore] Assigning memory functions to Globals...');
      (this.mod.Globals as any).read8 = this.inst.read8;
      (this.mod.Globals as any).write8 = this.inst.write8;
    }
    
    // 3. Asignar al módulo completo
    if (this.mod) {
      (this.mod as any).read8 = this.inst.read8;
      (this.mod as any).write8 = this.inst.write8;
      
      // 4. Y al HEAP también por si acaso
      if ((this.mod as any).HEAP8) {
        (this.mod as any).HEAP8.read8 = this.inst.read8;
        (this.mod as any).HEAP8.write8 = this.inst.write8;
      }
    }
    
    // 5. Asignar a la instancia global de window 
    (window as any).read8 = this.inst.read8;
    (window as any).write8 = this.inst.write8;
    
    console.log('[JsVecxCore] Memory functions assigned to ALL contexts (inst, e6809, Globals, module, HEAP8, window)');
    
    // Debug final de función assignments
    console.log('[JsVecxCore] Function assignment verification:');
    console.log('  inst.read8 =', typeof this.inst.read8);
    console.log('  e6809.read8 =', typeof (this.inst.e6809 as any)?.read8);
    console.log('  Globals.read8 =', typeof (this.mod?.Globals as any)?.read8);
    console.log('  module.read8 =', typeof (this.mod as any)?.read8);
    console.log('  window.read8 =', typeof (window as any).read8);
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
  }
  
  isBiosLoaded(){ return this.biosOk; }
  
  reset(){
    if (!this.inst) return;
    
    try { 
      this.inst.vecx_reset(); 
      console.log('[JsVecxCore] Reset successful');
    } catch(e) { 
      console.warn('[JsVecxCore] Reset failed, doing manual reset:', e);
      // Reset manual con reasignación de funciones
      this.inst.vector_draw_cnt = 0;
      this.inst.fcycles = 0;
      if (this.inst.e6809) {
        this.inst.e6809.reg_pc = 0x0000;
        this.inst.e6809.reg_a = 0;
        this.inst.e6809.reg_b = 0;
        this.inst.e6809.reg_dp = 0;
        this.inst.e6809.reg_x = { value: 0 };
        this.inst.e6809.reg_y = { value: 0 };
        this.inst.e6809.reg_u = { value: 0x8000 };
        this.inst.e6809.reg_s = { value: 0x8000 };
      }
      
      // Recrear funciones de memoria tras fallo de reset
      this.recreateMemoryFunctions();
      this.assignMemoryFunctionsToAllContexts();
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
      // Intentar ejecutar un frame usando la función jsvecx
      this.inst.vecx_emu(40000, 0); // Aprox 40K cycles por frame
      
      // Extraer vectores del frame actual
      const vectors: Segment[] = [];
      if (this.inst.vectors_draw && Array.isArray(this.inst.vectors_draw)) {
        for (const v of this.inst.vectors_draw) {
          if (v && typeof v === 'object') {
            vectors.push({
              x0: v.x0 || 0,
              y0: v.y0 || 0,
              x1: v.x1 || 0,
              y1: v.y1 || 0,
              intensity: v.intensity || 0
            });
          }
        }
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
      cycles: this.inst.fcycles || 0,
      cyclesPerSecond: 1_500_000, // Estándar Vectrex
      frameCount: this.frameCounter,
      vectorsDrawn: this.lastFrameSegments.length,
      pcCurrent: this.inst.e6809?.reg_pc || 0,
      stackSize: 0, // No fácilmente disponible en jsvecx
      maxStackSize: 0
    };
  }

  registers(): RegistersSnapshot | null {
    if (!this.inst?.e6809) return null;
    
    const cpu = this.inst.e6809;
    return {
      pc: cpu.reg_pc || 0,
      a: cpu.reg_a || 0,
      b: cpu.reg_b || 0,
      dp: cpu.reg_dp || 0,
      x: cpu.reg_x?.value || 0,
      y: cpu.reg_y?.value || 0,
      u: cpu.reg_u?.value || 0,
      s: cpu.reg_s?.value || 0,
      cc: cpu.reg_cc || 0
    };
  }

  getSegmentsShared(): Segment[] { return this.lastFrameSegments; }

  resetStats(){ /* noop */ }
  biosCalls(){ return []; }
  clearBiosCalls(){ /* noop */ }
}