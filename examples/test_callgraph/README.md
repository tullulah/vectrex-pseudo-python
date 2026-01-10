# Call Graph Test - Bank Switching Demo

Este proyecto demuestra el sistema de **automatic bank switching** del compilador VPy.

## Propósito

Test de la funcionalidad de **cross-bank call wrappers** (TODO #8):
- Detecta automáticamente llamadas entre funciones en diferentes bancos
- Genera wrappers con código de bank switching (PSHS/PULS + cambio de $4000)
- Redirige JSR calls para usar los wrappers automáticamente

## Estructura del Proyecto

```
test_callgraph/
├── test_callgraph.vpyproj     # Archivo de proyecto
├── src/
│   └── main.vpy               # Código principal con múltiples funciones
└── assets/
    └── vectors/
        ├── player.vec         # Sprite del jugador (triángulo)
        └── enemy.vec          # Sprite del enemigo (cuadrado)
```

## Call Graph

El programa tiene la siguiente jerarquía de llamadas:

```
main() [Bank #31 - fixed]
  └─> init_game() [Bank #0 - auto-assigned]
      ├─> load_assets()
      └─> setup_player()
  └─> game_loop() [Bank #0 - auto-assigned]
      ├─> update_player()
      │   ├─> check_input()
      │   └─> move_player()
      ├─> update_enemies()
      │   └─> move_enemy(id)
      └─> draw_all() [Bank #0 - auto-assigned]
          ├─> draw_player()
          └─> draw_enemies()

loop() [Bank #31 - fixed]
  └─> game_loop() [Bank #0 - auto-assigned]
```

## Cross-Bank Calls Detectadas

El analizador detecta **3 llamadas cross-bank**:

1. `main() → init_game()` (Bank #31 → Bank #0)
2. `main() → game_loop()` (Bank #31 → Bank #0)  
3. `loop() → game_loop()` (Bank #31 → Bank #0)

## Wrappers Generados

Para cada cross-bank call, el compilador genera automáticamente:

```asm
init_game_BANK_WRAPPER:
    PSHS A              ; Save A register
    LDA $4000           ; Read current bank register
    PSHS A              ; Save current bank on stack
    LDA #0              ; Load target bank ID (Bank #0)
    STA $4000           ; Switch to target bank
    JSR init_game       ; Call real function
    PULS A              ; Restore original bank from stack
    STA $4000           ; Switch back to original bank
    PULS A              ; Restore A register
    RTS
```

## Cómo Probar

### 1. Compilar
```bash
cargo run --bin vectrexc -- build examples/test_callgraph/src/main.vpy
```

### 2. Verificar Cross-Bank Calls
```bash
grep "JSR.*_BANK_WRAPPER" examples/test_callgraph/src/main.asm
```

Deberías ver:
```asm
JSR init_game_BANK_WRAPPER
JSR game_loop_BANK_WRAPPER
JSR game_loop_BANK_WRAPPER
```

### 3. Ejecutar en IDE
- Abre el IDE: `./run-ide.sh`
- Abre el proyecto: `examples/test_callgraph/test_callgraph.vpyproj`
- Compila y ejecuta
- Verifica que no haya errores de bank switching

### 4. Verificar en Emulador
El emulador debe:
- Cargar el ROM multi-banco correctamente
- Ejecutar las funciones en sus bancos asignados
- Manejar los cambios de banco automáticamente

## Configuración de ROM

```vpy
META ROM_TOTAL_SIZE = 524288    # 512KB total (32 bancos × 16KB)
META ROM_BANK_SIZE = 16384      # 16KB por banco
```

## Asignación de Bancos (Automática)

El optimizador asigna funciones a bancos usando algoritmo **Best Fit Decreasing**:

- **Bank #31** (fixed): `main()`, `loop()` - funciones críticas
- **Bank #0** (auto): Resto de funciones del juego

## Assets

### player.vec
Triángulo que apunta hacia arriba (sprite del jugador):
```json
{
  "points": [
    {"x": 0, "y": 20},     // Punta superior
    {"x": -15, "y": -10},  // Base izquierda
    {"x": 15, "y": -10}    // Base derecha
  ]
}
```

### enemy.vec
Cuadrado (sprite del enemigo):
```json
{
  "points": [
    {"x": -10, "y": 10},   // Superior izquierda
    {"x": 10, "y": 10},    // Superior derecha
    {"x": 10, "y": -10},   // Inferior derecha
    {"x": -10, "y": -10}   // Inferior izquierda
  ]
}
```

## Verificación de Funcionalidad

### ✅ TODO #8 Part 1 - Wrapper Generation
- [x] Wrappers generados en ASM
- [x] Estructura correcta (PSHS/PULS + bank switch)
- [x] 3 wrappers detectados correctamente

### ✅ TODO #8 Part 2 - JSR Redirection
- [x] JSR calls usan wrappers automáticamente
- [x] Mismo-banco: JSR directo
- [x] Cross-banco: JSR wrapper

### ⏳ Pendiente Testing
- [ ] Ejecutar en emulador
- [ ] Verificar que game_loop se ejecute correctamente
- [ ] Confirmar que draw_player/draw_enemies rendericen sprites
- [ ] Validar que no haya crashes por bank switching

## Próximos Pasos (TODO #9)

Una vez confirmado que los wrappers funcionan:
- Generar secciones ASM separadas por banco
- Cada función emitida en su banco con ORG correcto
- Preparar para linker multi-banco

## Notas Técnicas

### Bank Register
- **Address**: `$4000`
- **Write**: Cambiar banco actual (0-31)
- **Read**: Obtener banco actual

### Preservación de Registros
Los wrappers preservan:
- **A register**: Push/pop antes y después
- **Stack**: Guarda banco original en stack
- **PC**: RTS retorna correctamente

### Performance
- **Overhead por cross-bank call**: ~20 cycles extra
- **Acceptable**: Las llamadas cross-bank son raras
- **Optimización**: Same-bank calls siguen siendo JSR directo
