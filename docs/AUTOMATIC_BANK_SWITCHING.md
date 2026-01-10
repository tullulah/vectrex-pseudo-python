# Automatic Bank Switching Specification

**Version**: 1.0  
**Date**: 2026-01-10  
**Status**: Design Phase

## Overview

VPy implementa **automatic bank switching** transparente para el developer. El compilador analiza el código, asigna funciones a bancos automáticamente, y genera wrappers para llamadas cross-bank. El developer solo configura el tamaño total de ROM, el resto es automático.

## Goals

✅ **Zero Configuration**: Developer no menciona bancos en código  
✅ **Transparent**: Same code funciona en 64KB o 4MB ROM  
✅ **Optimal**: Algoritmo agrupa funciones relacionadas  
✅ **Safe**: No bank switching bugs posibles  
✅ **Debuggable**: Stack traces muestran banco actual

## Architecture

### Memory Map

```
ROM Layout:
  0x0000-0x3FFF: Banked ROM window (16KB) ← Cambia según registro
  0x4000-0x7FFF: Fixed ROM (8KB)          ← Siempre banco último (FIXED_BANK)
  
Hardware Register:
  0x4000: ROM_BANK_REG (write-only)
  
RAM Variables:
  CURRENT_ROM_BANK: 1 byte (tracking de banco actual)
```

### Bank Configuration (META Directives)

```python
# Developer configura solo tamaño total:
META ROM_TOTAL_SIZE = 524288   # 512KB
META ROM_BANK_SIZE = 16384     # 16KB (default, opcional)

# Compilador calcula automáticamente:
# ROM_BANK_COUNT = 524288 / 16384 = 32 bancos
# FIXED_BANK = 31 (último banco, siempre visible en 0x4000-0x7FFF)
# BANKED_BANKS = 0-30 (disponibles para swapping en 0x0000-0x3FFF)
```

### Fixed Bank Strategy

**Banco Fijo** (último banco, no switcheable):
- `main()` function
- Interrupt handlers (BIOS vectors)
- Wrappers para cross-bank calls
- Runtime de bank switching (SWITCH_TO_BANK)
- Hot functions (llamadas frecuentemente)

**Bancos Swappables** (0 a N-2):
- Funciones de usuario
- Assets (vectores, música)
- Datos const grandes

## Compilation Pipeline

### Phase 1: Call Graph Analysis

**Input**: AST del programa VPy  
**Output**: Call graph con frecuencias

```rust
struct CallGraph {
    nodes: HashMap<String, FunctionNode>,  // nombre → info función
    edges: Vec<CallEdge>,                   // arista = llamada
}

struct FunctionNode {
    name: String,
    size_bytes: usize,          // Tamaño estimado en ASM
    is_interrupt: bool,         // Handler de interrupt?
    call_frequency: u32,        // Cuántas veces se llama
}

struct CallEdge {
    from: String,               // Función caller
    to: String,                 // Función callee
    frequency: u32,             // Veces que se llama en runtime
}
```

**Algoritmo**:
```rust
fn build_call_graph(module: &Module) -> CallGraph {
    let mut graph = CallGraph::new();
    
    // 1. Crear nodos (una entrada por función)
    for item in &module.items {
        if let Item::Function(func) = item {
            let node = FunctionNode {
                name: func.name.clone(),
                size_bytes: estimate_function_size(func),
                is_interrupt: is_interrupt_handler(&func.name),
                call_frequency: 0,  // Se calcula después
            };
            graph.nodes.insert(func.name.clone(), node);
        }
    }
    
    // 2. Crear aristas (llamadas entre funciones)
    for item in &module.items {
        if let Item::Function(func) = item {
            for call in find_function_calls(&func.body) {
                graph.add_edge(
                    func.name.clone(),
                    call.target.clone(),
                    estimate_call_frequency(&call)
                );
            }
        }
    }
    
    // 3. Calcular frecuencias (propagación desde main)
    graph.calculate_frequencies();
    
    graph
}
```

**Ejemplo Call Graph**:
```
main (size: 100 bytes, freq: 1)
  ├─→ level_1_init (size: 200 bytes, freq: 1)
  │     ├─→ load_sprites (size: 300 bytes, freq: 1)
  │     └─→ init_enemies (size: 150 bytes, freq: 1)
  └─→ game_loop (size: 400 bytes, freq: 10000)
        ├─→ update_player (size: 250 bytes, freq: 10000)
        ├─→ update_enemies (size: 300 bytes, freq: 10000)
        └─→ draw_all (size: 500 bytes, freq: 10000)
              ├─→ draw_player (size: 200 bytes, freq: 10000)
              └─→ draw_enemies (size: 250 bytes, freq: 10000)
```

### Phase 2: Function Clustering

**Goal**: Agrupar funciones que se llaman mucho entre sí (minimizar cross-bank calls)

```rust
fn cluster_functions(graph: &CallGraph) -> Vec<Cluster> {
    let mut clusters = Vec::new();
    
    // 1. Identificar "hot paths" (caminos frecuentes)
    let hot_paths = graph.find_hot_paths(threshold: 1000);
    
    // 2. Crear cluster por cada hot path
    for path in hot_paths {
        let cluster = Cluster {
            functions: path.nodes.clone(),
            total_size: path.nodes.iter().map(|n| n.size_bytes).sum(),
            call_frequency: path.total_frequency,
        };
        clusters.push(cluster);
    }
    
    // 3. Merge clusters pequeños (menor que mitad de banco)
    merge_small_clusters(&mut clusters, ROM_BANK_SIZE / 2);
    
    clusters
}

struct Cluster {
    functions: Vec<String>,
    total_size: usize,
    call_frequency: u32,
}
```

### Phase 3: Bank Assignment (Bin Packing)

**Goal**: Asignar clusters a bancos minimizando fragmentación

```rust
fn assign_banks(
    clusters: Vec<Cluster>,
    config: &BankConfig
) -> HashMap<String, u8> {
    let mut assignments = HashMap::new();
    let mut banks = vec![Bank::new(); config.rom_bank_count];
    
    // Banco fijo (último) reservado para hot code
    let fixed_bank_id = config.rom_bank_count - 1;
    
    // 1. Asignar funciones críticas al banco fijo
    for func in find_critical_functions(&clusters) {
        assignments.insert(func.clone(), fixed_bank_id);
        banks[fixed_bank_id].add(func);
    }
    
    // 2. Sort clusters por frecuencia (más llamados primero)
    let mut sorted_clusters = clusters;
    sorted_clusters.sort_by_key(|c| std::cmp::Reverse(c.call_frequency));
    
    // 3. First-Fit Decreasing bin packing
    for cluster in sorted_clusters {
        // Encontrar banco con espacio suficiente
        let bank_id = banks.iter_mut()
            .position(|b| b.has_space(cluster.total_size))
            .unwrap_or_else(|| panic!("ROM size too small"));
        
        // Asignar todas las funciones del cluster a ese banco
        for func in &cluster.functions {
            assignments.insert(func.clone(), bank_id as u8);
            banks[bank_id].add(func.clone());
        }
    }
    
    assignments  // Mapa: función → banco
}

struct Bank {
    id: u8,
    functions: Vec<String>,
    used_size: usize,
    max_size: usize,
}

impl Bank {
    fn has_space(&self, size: usize) -> bool {
        self.used_size + size <= self.max_size
    }
}
```

**Funciones Críticas** (siempre en banco fijo):
- `main()`
- Interrupt handlers
- Funciones llamadas > 1000 veces
- Wrappers de cross-bank calls (generados después)

### Phase 4: Cross-Bank Call Wrapper Generation

**Goal**: Generar automáticamente wrappers para llamadas entre bancos

```rust
fn generate_wrappers(
    graph: &CallGraph,
    assignments: &HashMap<String, u8>
) -> Vec<String> {
    let mut wrappers = Vec::new();
    
    for edge in &graph.edges {
        let caller_bank = assignments[&edge.from];
        let callee_bank = assignments[&edge.to];
        
        // Si están en diferentes bancos, generar wrapper
        if caller_bank != callee_bank {
            let wrapper = format_wrapper(
                &edge.to,
                callee_bank,
                caller_bank
            );
            wrappers.push(wrapper);
        }
    }
    
    wrappers
}
```

**Wrapper Template** (ASM generado):
```asm
; Wrapper para llamar función en otro banco
; Ejemplo: main (banco 31) llama level_1_init (banco 0)

CALL_BANK0_level_1_init:
    ; 1. Guardar banco actual
    LDA CURRENT_ROM_BANK
    PSHS A
    
    ; 2. Cambiar a banco destino
    LDA #0                  ; Banco 0
    STA CURRENT_ROM_BANK
    STA $4000               ; Escribir registro hardware
    
    ; 3. Llamar función real
    JSR level_1_init
    
    ; 4. Restaurar banco original
    PULS A
    STA CURRENT_ROM_BANK
    STA $4000
    
    RTS

; Todos los wrappers van en FIXED_BANK (banco 31)
; Así están siempre visibles desde cualquier banco
```

### Phase 5: ASM Section Generation

**Output**: ASM dividido en secciones por banco

```asm
; ========================================
; BANK 0 (0x0000-0x3FFF cuando swapped in)
; ========================================
    ORG $0000
BANK0_START:

level_1_init:
    JSR CALL_BANK0_load_sprites  ; Same bank, pero usa wrapper por si acaso
    JSR CALL_BANK0_init_enemies
    RTS

load_sprites:
    ; ... código ...
    RTS

init_enemies:
    ; ... código ...
    RTS

BANK0_END:
    ; Padding hasta 16KB
    FILL $FF, $4000 - (* - BANK0_START)

; ========================================
; BANK 1 (0x0000-0x3FFF cuando swapped in)
; ========================================
    ORG $0000
BANK1_START:

game_loop:
    JSR CALL_BANK1_update_player
    JSR CALL_BANK1_update_enemies
    JSR CALL_BANK1_draw_all
    RTS

; ... más funciones de banco 1 ...

BANK1_END:
    FILL $FF, $4000 - (* - BANK1_START)

; ========================================
; FIXED BANK (banco 31, siempre en 0x4000-0x7FFF)
; ========================================
    ORG $4000
FIXED_BANK_START:

main:
    ; Inicializar banco
    LDA #31
    STA CURRENT_ROM_BANK
    STA $4000
    
    ; Llamar init de nivel (banco 0)
    JSR CALL_BANK0_level_1_init
    
    ; Loop principal (banco 1)
MAINLOOP:
    JSR CALL_BANK1_game_loop
    BRA MAINLOOP

; --- Cross-bank call wrappers ---
CALL_BANK0_level_1_init:
    ; ... wrapper code (ver arriba) ...
    
CALL_BANK0_load_sprites:
    ; ... wrapper code ...

CALL_BANK1_game_loop:
    ; ... wrapper code ...

; --- Variables RAM ---
CURRENT_ROM_BANK: FCB 31  ; Banco actual

FIXED_BANK_END:
```

### Phase 6: Multi-Bank Binary Generation

**Output**: Archivo .rom con todos los bancos concatenados

```rust
fn generate_multi_bank_rom(
    banks: Vec<BankData>,
    config: &BankConfig
) -> Vec<u8> {
    let mut rom = Vec::new();
    
    // Concatenar bancos en orden (0, 1, 2, ..., N-1)
    for bank in banks {
        let mut bank_data = bank.assemble();
        
        // Padding a tamaño exacto de banco
        bank_data.resize(config.rom_bank_size, 0xFF);
        
        rom.extend_from_slice(&bank_data);
    }
    
    rom
}
```

**ROM Layout en archivo**:
```
Offset        Bank    Description
--------------------------------------
0x00000       0       Banco 0 (16KB)
0x04000       1       Banco 1 (16KB)
0x08000       2       Banco 2 (16KB)
...
0x7C000       31      Banco fijo (último 16KB)
```

## Developer Experience

### Código VPy (Sin mencionar bancos)

```python
# examples/bigame/src/main.vpy

META TITLE = "Big Game"
META ROM_TOTAL_SIZE = 524288  # 512KB - ¡Solo esto!

# Developer escribe código NORMAL:

def main():
    SET_INTENSITY(127)
    level_1_init()
    
    while True:
        WAIT_RECAL()
        game_loop()

def level_1_init():
    load_sprites()
    init_enemies()

def load_sprites():
    # Cargar 50 sprites
    for i in range(50):
        LOAD_LEVEL(i)

def game_loop():
    update_player()
    update_enemies()
    draw_all()

def update_player():
    # Lógica compleja
    pass

def update_enemies():
    # Lógica compleja
    pass

def draw_all():
    draw_player()
    draw_enemies()

# ¡NO hay @bank, NO hay LOAD_ROM_BANK!
# Compilador hace TODO automáticamente:
# - Analiza call graph
# - Agrupa funciones relacionadas
# - Asigna a bancos
# - Genera wrappers
```

### Compilation Output (Verbose Mode)

```
$ cargo run --bin vectrexc -- build bigame.vpy --verbose

Phase 1: Parse AST... OK
Phase 2: Semantic analysis... OK
Phase 3: Call graph analysis...
  ✓ Found 8 functions
  ✓ Built 12 call edges
  ✓ Calculated frequencies
  ✓ Hot functions: main, game_loop, draw_all

Phase 4: Function clustering...
  ✓ Cluster 0: [main, game_loop, update_player] (1.2KB, 10000 calls/sec)
  ✓ Cluster 1: [level_1_init, load_sprites] (0.5KB, 1 call)
  ✓ Cluster 2: [draw_all, draw_player, draw_enemies] (1.0KB, 10000 calls/sec)

Phase 5: Bank assignment...
  ✓ Bank 0: Cluster 1 (level initialization)
  ✓ Bank 31 (fixed): Cluster 0, Cluster 2, wrappers
  ✓ Total banks used: 2 / 32
  ✓ Fixed bank size: 2.8KB / 8KB (35% full)
  ✓ Cross-bank calls: 2 (level_1_init, load_sprites)

Phase 6: Wrapper generation...
  ✓ Generated CALL_BANK0_level_1_init
  ✓ Generated CALL_BANK0_load_sprites

Phase 7: Assembly emission...
  ✓ Bank 0: 512 bytes
  ✓ Bank 31 (fixed): 2.8KB
  ✓ Total ROM: 32KB / 512KB (6% used)

Phase 8: Multi-bank ROM generation...
  ✓ Written: build/bigame.rom (512KB)

Compilation successful!
```

### Debug Information

**Stack Trace con Banco Actual**:
```
Runtime Error: Division by zero at line 42
Call stack:
  [Bank 31] main() at main.vpy:10
  [Bank 31] game_loop() at main.vpy:15
  [Bank 31] update_player() at main.vpy:30
  [Bank 0]  calculate_velocity() at main.vpy:42  ← Error aquí
  
Current bank: 0
ROM map: 2 banks in use (0, 31)
```

## Implementation Details

### Data Structures

```rust
// core/src/codegen.rs
pub struct BankConfig {
    pub rom_bank_size: u32,      // 16384 (16KB)
    pub rom_total_size: u32,     // 524288 (512KB)
    pub rom_bank_count: u8,      // 32
    pub fixed_bank: u8,          // 31 (último)
    pub rom_bank_reg: u16,       // 0x4000
}

pub struct FunctionBankMap {
    pub assignments: HashMap<String, u8>,  // función → banco
    pub wrappers: Vec<WrapperCode>,        // wrappers generados
}

pub struct WrapperCode {
    pub name: String,           // CALL_BANK0_level_1_init
    pub target_func: String,    // level_1_init
    pub target_bank: u8,        // 0
    pub asm_code: String,       // Código ASM generado
}

// core/src/backend/m6809/call_graph.rs (NEW)
pub struct CallGraph {
    pub nodes: HashMap<String, FunctionNode>,
    pub edges: Vec<CallEdge>,
}

pub struct FunctionNode {
    pub name: String,
    pub size_bytes: usize,
    pub is_critical: bool,
    pub call_frequency: u32,
}

pub struct CallEdge {
    pub from: String,
    pub to: String,
    pub frequency: u32,
}

// core/src/backend/m6809/bank_optimizer.rs (NEW)
pub struct BankOptimizer {
    config: BankConfig,
    graph: CallGraph,
}

impl BankOptimizer {
    pub fn assign_banks(&self) -> FunctionBankMap;
    pub fn generate_wrappers(&self, map: &FunctionBankMap) -> Vec<WrapperCode>;
}
```

### Files to Create/Modify

**New Files**:
- `core/src/backend/m6809/call_graph.rs` - Call graph analysis
- `core/src/backend/m6809/bank_optimizer.rs` - Bank assignment algorithm
- `core/src/backend/m6809/multi_bank_linker.rs` - ROM generation

**Modified Files**:
- `core/src/parser.rs` - Parse META ROM_TOTAL_SIZE
- `core/src/codegen.rs` - Add BankConfig to CodegenOptions
- `core/src/backend/m6809/mod.rs` - Integrate bank system
- `core/src/backend/m6809/emission.rs` - Emit per-bank sections
- `ide/frontend/src/components/panels/EmulatorPanel.tsx` - Load multi-bank ROMs

## Hardware Requirements

### Minimum (128KB)

```
Components:
- 1× 27C010 EPROM (128KB) - $5
- 1× 74HC373 Latch - $1
- 1× 74HC00 NAND gates - $1
Total: ~$10

Banks: 8 × 16KB
Fixed bank: Bank 7 (0x4000-0x7FFF)
Swappable banks: 0-6 (0x0000-0x3FFF)
```

### Recommended (512KB)

```
Components:
- 1× 29F040 Flash ROM (512KB) - $8
- 1× 74HC373 Latch - $1
- 1× 74HC139 Decoder - $1
Total: ~$12

Banks: 32 × 16KB
Fixed bank: Bank 31
Swappable banks: 0-30
```

### Maximum (4MB)

```
Components:
- 1× 29F320 Flash ROM (4MB) - $15
- 1× ATF1504 CPLD - $5
Total: ~$20

Banks: 256 × 16KB
Fixed bank: Bank 255
Swappable banks: 0-254
```

## Optimization Strategies

### Hot Code in Fixed Bank

Funciones con `call_frequency > 1000` van al banco fijo:
- Evita overhead de bank switching
- Mejor performance

### Function Clustering

Algoritmo agrupa funciones que se llaman mucho:
- Minimiza cross-bank calls
- Mejor cache locality (aunque 6809 no tiene cache)

### Wrapper Caching

Si función A llama B 100 veces, wrapper se ejecuta 100 veces:
- Overhead: ~30 cycles por llamada
- Total: 3000 cycles
- Alternativa: Mover B al mismo banco (0 overhead)

### Bin Packing Efficiency

First-Fit Decreasing vs Best-Fit:
- FFD: Más rápido, 80-90% eficiencia
- BF: Más lento, 90-95% eficiencia
- VPy usa FFD (suficiente para juegos)

## Future Enhancements

### Phase 2 (Optional Features)

- [ ] Manual hints: `@bank(fixed)` decorator
- [ ] Profile-guided optimization (usar datos de runtime)
- [ ] Compressed banks (descomprimir on-load)
- [ ] Bank preloading hints (predict next bank)

### Phase 3 (Advanced)

- [ ] RAM banking (similar system para cartridge RAM)
- [ ] Overlay system (múltiples niveles no cargados simultáneamente)
- [ ] Hot-reload (cambiar banco sin interrumpir game loop)

## References

- Game Boy Bank Switching: https://gbdev.io/pandocs/Bank_Switching.html
- NES Mappers: https://www.nesdev.org/wiki/Mapper
- Vectrex Hardware: https://vectrex.fandom.com/wiki/Vectrex_Technical_Information
- Bin Packing Algorithms: https://en.wikipedia.org/wiki/Bin_packing_problem

---

**Last Updated**: 2026-01-10  
**Author**: VPy Compiler Team  
**Status**: ✅ Ready for Implementation
