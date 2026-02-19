# Copilot Project Instructions

> **Para detalles técnicos de cualquier sistema, llama `get_project_docs(topic)`.**
> Topics disponibles: `assets`, `joystick`, `buttons`, `const_arrays`, `modules`, `banking`, `draw_line`, `music`, `compiler`, `bios`, `tests`, `meta`, `all`

---

## REGLAS CRÍTICAS - NUNCA VIOLAR

### Git
- Rama principal: `master` (NO `main`)
- macOS: usar `&&`. En Windows PowerShell v5.1: usar `;` (NO `&&`)

### WAIT_RECAL y AUDIO_UPDATE - AUTO-INYECTADOS
- ❌ NUNCA escribir `WAIT_RECAL()` en código VPy — el compilador lo inyecta automáticamente al inicio del `loop()`
- ❌ NUNCA escribir `MUSIC_UPDATE()` o `AUDIO_UPDATE()` — se inyecta automáticamente al FINAL del `loop()`
- ❌ NUNCA escribir `UPDATE_BUTTONS()` — `Read_Btns` se inyecta automáticamente al inicio del `loop()`

### Scopes de Variables
- Variables declaradas en `main()` NO son accesibles en `loop()` — scopes separados
- Declarar variables dentro de la función donde se usan

### BIOS
- NUNCA usar BIOS sintética. Rutas válidas:
  - `ide/frontend/src/assets/bios.bin`
  - `ide/frontend/dist/bios.bin`
- NUNCA usar rutas absolutas fuera del workspace

### Assets (.vec / .vmus)
- Formato SIEMPRE JSON — nunca inventar formatos de texto
- NUNCA inventar nombres de assets — verificar con `project/get_structure` antes de usar `DRAW_VECTOR("name")`
- Usar `project/create_vector` y `project/create_music` (validan JSON automáticamente)

### Código — No Sintético
- Todas las implementaciones deben ser reales, no heurísticas ni simuladas
- No fabricar side effects, no inventar APIs sin verificar código fuente

### Verificación 1:1 (emulator_v2)
- Antes de crear cualquier API: leer el .cpp/.h correspondiente en `vectrexy/libs/emulator/`
- Cada función debe incluir comentario `// C++ Original:` con código fuente real

---

## VPy Language — Recordatorio Rápido

```python
# CORRECTO
def main():
    SET_INTENSITY(127)

def loop():
    # WAIT_RECAL() ← NO escribir, se inyecta solo
    player_x = 0       # declarar aquí, no en main()
    
    joy_x = J1_X()     # -1, 0, +1
    btn1 = J1_BUTTON_1()  # 0=released, 1=pressed
    
    const coords = [10, 20, 30]   # ROM-only, sin RAM
    val = coords[1]               # indexing funciona
    
    DRAW_VECTOR("ship")           # asset debe existir
    PLAY_MUSIC("theme")           # auto-update al final del loop
```

```python
# META para multibank
META ROM_TOTAL_SIZE = 524288
META ROM_BANK_SIZE = 16384
```

---

## Arquitectura del Compilador — Resumen

| Fase | Descripción |
|------|-------------|
| 0 | Asset discovery (assets/vectors/*.vec, assets/music/*.vmus) |
| 1-3 | Parse → Unify (imports resueltos, dot notation → PREFIX_NAME) |
| 4 | Codegen M6809 ASM |
| 5 | Asset embedding (FCB/FDB en ROM) |
| 6.3+ | Multibank split si ROM_TOTAL_SIZE definido |

- Build: `cargo run --bin vectrexc -- build programa.vpy --bin`
- Tests: `cargo test -p vectrex_emulator`

---

## MCP Tools Disponibles (ide/mcp-server/server.js)

| Categoría | Tools |
|-----------|-------|
| Editor | editor_list_documents, editor_read_document, editor_write_document, editor_replace_range, editor_insert_at, editor_delete_range, editor_get_diagnostics |
| Compiler | compiler_build, compiler_get_errors |
| Emulator | emulator_run, emulator_get_state, emulator_stop |
| Memory | memory_dump, memory_list_variables, memory_read_variable, memory_write |
| Debugger | debugger_start, debugger_add_breakpoint, debugger_remove_breakpoint, debugger_get_callstack, debugger_get_registers |
| Project | project_get_structure, project_read_file, project_write_file, project_create, project_open, project_close, project_create_vector, project_create_music |
| **Docs** | **get_project_docs(topic)** ← llamar para detalles técnicos |

### Reglas de uso de tools
- `editor/write_document`: crea O actualiza (siempre funciona)
- `editor/read_document`: solo si el archivo ya está abierto
- `editor/replace_range`: requiere startLine/endLine (NO character offsets)
- `project/read_file`: paths RELATIVAS al project root (ej: `src/main.vpy`)
- NO inventar nombres de tools — solo las listadas arriba

