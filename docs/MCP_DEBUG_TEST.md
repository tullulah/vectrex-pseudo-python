# MCP Debug Tools - Test Session

**Fecha**: 2026-01-06  
**Objetivo**: Probar las nuevas herramientas MCP de debugging implementadas en el servidor

## Herramientas MCP Implementadas

### Nuevas Herramientas Añadidas (2026-01-06)

**Archivo modificado**: `ide/electron/src/mcp/server.ts`

#### 1. Gestión de Breakpoints
- ✅ `debugger/add_breakpoint` (ya existía)
- ✅ `debugger/remove_breakpoint` - Elimina breakpoint específico
- ✅ `debugger/list_breakpoints` - Lista todos los breakpoints activos
- ✅ `debugger/clear_breakpoints` - Elimina todos los breakpoints

#### 2. Control de Ejecución Step-by-Step
- ✅ `debugger/step_into` - F11 (entra en funciones)
- ✅ `debugger/step_over` - F10 (ejecuta sin entrar)
- ✅ `debugger/step_out` - Shift+F11 (sale de función)
- ✅ `debugger/continue` - F5 (continúa hasta breakpoint)
- ✅ `debugger/pause` - Pausa ejecución
- ✅ `debugger/start` - **NUEVO** (Ctrl+F5, inicia sesión de debug)

#### 3. Memoria
- ✅ `memory/dump` (ya existía)
- ✅ `memory/list_variables` (ya existía)
- ✅ `memory/read_variable` (ya existía)
- ✅ `memory/write` - **NUEVO** (escribe valor en memoria RAM)

#### 4. Inspección
- ✅ `debugger/get_callstack` (ya existía)

**Total**: 14 herramientas MCP de debugging

## Implementación Técnica

### Patrón de Implementación

Todas las herramientas siguen este patrón:

```typescript
private async toolName(params: any): Promise<any> {
  if (!this.mainWindow) {
    throw new Error('No main window available');
  }

  // Opción 1: Ejecutar JavaScript en el renderer
  const result = await this.mainWindow.webContents.executeJavaScript(`
    (function() {
      const store = window.__*Store__;
      // ... operación
      return { success: true, data: ... };
    })()
  `);
  
  // Opción 2: Enviar mensaje via postMessage
  await this.mainWindow.webContents.executeJavaScript(`
    (function() {
      window.postMessage({ type: 'debug-step-into' }, '*');
      return { success: true };
    })()
  `);
  
  return result;
}
```

### Herramienta debugger/start

**Ubicación**: `ide/electron/src/mcp/server.ts` líneas ~350 (registro) y ~1640 (implementación)

```typescript
private async debugStart(params: any): Promise<any> {
  if (!this.mainWindow) {
    throw new Error('No main window available');
  }

  // Send debug.start command via window.postMessage
  await this.mainWindow.webContents.executeJavaScript(`
    (function() {
      window.postMessage({ type: 'command', command: 'debug.start' }, '*');
      return { success: true };
    })()
  `);

  return { success: true, message: 'Debug session started' };
}
```

**Comportamiento esperado**:
1. Compila el proyecto actual sin auto-run
2. Setea `loadingForDebug: true` en debugStore
3. Carga el binario en el emulador en modo pausado
4. Sincroniza breakpoints con JSVecx
5. Estado final: `debugState='paused'`, esperando en PC de entrada

### Herramienta memory/write

**Ubicación**: `ide/electron/src/mcp/server.ts` líneas ~240 (registro) y ~1545 (implementación)

**Parámetros**:
- `address` (number): Dirección de memoria (0xC800-0xCFFF para RAM)
- `value` (number): Valor a escribir (0-255 para 8-bit, 0-65535 para 16-bit)
- `size` (number, opcional): 1 o 2 bytes (default: 1)

**Validación**:
- Rango RAM: 0xC800-0xCFFF (512 bytes)
- Valor 8-bit: 0-255
- Valor 16-bit: 0-65535

**Retorno**: Confirmación con valor leído después de escribir

## Secuencia de Prueba

### Test Básico: Breakpoint + Step Into

**Proyecto**: `examples/test_bp_min`

**Código VPy** (`src/main.vpy`):
```python
META TITLE = "BP Test"

def main():
    SET_INTENSITY(100)

def loop():
    SET_INTENSITY(50)
    PRINT_TEXT(-50, 0, "GET READY")  # <- Línea 8 (breakpoint aquí)
```

**Pasos**:

1. **Añadir breakpoint**:
   ```typescript
   mcp_vpy_ide_debugger_add_breakpoint({
     uri: "file:///Users/daniel/projects/vectrex-pseudo-python/examples/test_bp_min/src/main.vpy",
     line: 8
   })
   ```

2. **Iniciar debug** (en lugar de run normal):
   ```typescript
   mcp_vpy_ide_debugger_start()
   ```
   
   **Diferencia vs `emulator/run`**:
   - `emulator/run` → ejecución normal, no se detiene en breakpoints
   - `debugger/start` → sesión de debug, se detiene en breakpoints

3. **Verificar estado**:
   ```typescript
   mcp_vpy_ide_emulator_get_state()
   ```
   
   **Esperado**: 
   ```json
   {
     "state": "paused",
     "pc": "0x???",  // PC en línea 8 del VPy
     "debugState": "stopped"
   }
   ```

4. **Step Into** (F11):
   ```typescript
   mcp_vpy_ide_debugger_step_into()
   ```
   
   **Esperado**: 
   - Cambia a vista ASM
   - Muestra primera instrucción de PRINT_TEXT
   - **NO ejecuta** la instrucción automáticamente
   
   **Fix aplicado** (2026-01-06):
   - `main.tsx` líneas 863-876: Conditional message dispatch
   - Si `asmDebuggingMode=false` → envía `debug-switch-to-asm` (solo cambio de vista)
   - Si `asmDebuggingMode=true` → envía `debug-step-into` (ejecución real)
   - `EmulatorPanel.tsx` líneas 984-1025: Nuevo handler para `debug-switch-to-asm`

5. **Continuar con Step Over** (F10):
   ```typescript
   mcp_vpy_ide_debugger_step_over()
   ```

6. **Continuar ejecución** (F5):
   ```typescript
   mcp_vpy_ide_debugger_continue()
   ```

## Problema Actual: Timeouts en MCP

### Síntoma

```
ERROR while calling tool: MPC -32603: IPC request timeout
```

### Diagnóstico

1. **Servidor MCP IPC interno** (Electron): ✅ FUNCIONANDO
   ```
   [electron] [MCP IPC] Server listening on port 9123
   ```

2. **Servidor MCP externo** (stdio): ✅ CORRIENDO
   ```bash
   ps aux | grep mcp-server
   # node /Users/.../ide/mcp-server/server.js --stdio
   ```

3. **Compilación TypeScript**: ✅ SIN ERRORES
   ```bash
   cd ide/electron && npm run build  # OK
   ```

### Posibles Causas

1. **Cliente MCP no configurado**: VSCode/Copilot no tiene configurado el servidor MCP externo
2. **Timeout muy corto**: El cliente tiene un timeout de conexión muy breve
3. **Protocolo de comunicación**: El servidor stdio espera mensajes JSON-RPC pero no los recibe

### Solución Temporal

**Usar el IDE manualmente** para probar que el debugging funciona:

1. Abrir `examples/test_bp_min/src/main.vpy`
2. Añadir breakpoint en línea 8 (clic en margen izquierdo)
3. Presionar **Ctrl+F5** (Start Debugging)
4. Verificar que el emulador se para en el breakpoint
5. Presionar **F11** (Step Into) y verificar:
   - ✅ Cambia a vista ASM
   - ✅ Muestra primera instrucción (línea correcta)
   - ✅ **NO ejecuta** automáticamente
6. Presionar **F10** (Step Over) para avanzar instrucción por instrucción

## Fixes Previos Aplicados (Contexto)

### 1. Step Into Auto-Execution Bug (2026-01-06)

**Problema**: Al hacer Step Into desde VPy, el emulador ejecutaba automáticamente la primera instrucción ASM.

**Causa**: `vecx.js` interceptaba el mensaje `debug-step-into` y ejecutaba `vecx.debugStepInto()` antes de que EmulatorPanel cambiara la vista.

**Fix**: 
- `main.tsx` líneas 863-876: Mensaje condicional basado en `asmDebuggingMode`
- VPy mode → `debug-switch-to-asm` (solo vista)
- ASM mode → `debug-step-into` (ejecución)

### 2. Labels en Address Map (2026-01-06)

**Problema**: Debugger se paraba en líneas de etiquetas (e.g., línea 133) en lugar de instrucciones (línea 135).

**Fix**: `core/src/backend/asm_address_mapper.rs` líneas 86-93
- Labels sincronizan `current_address` pero **NO se insertan** en `asm_line_map`
- Solo instrucciones ejecutables tienen addresses en el mapa

### 3. Comment Lines en Address Map (2026-01-06)

**Fix**: `asm_address_mapper.rs`
- Skip lines starting with `;`
- Solo instrucciones reales en address map

## Próximos Pasos

1. **Reiniciar VSCode** para recargar cliente MCP
2. **Verificar configuración MCP** en VSCode settings
3. **Probar herramientas MCP** después del reinicio:
   ```typescript
   // Test sequence
   mcp_vpy_ide_debugger_add_breakpoint(...)
   mcp_vpy_ide_debugger_start()
   mcp_vpy_ide_emulator_get_state()
   mcp_vpy_ide_debugger_step_into()
   ```
4. **Si MCP sigue fallando**: Usar IDE manualmente y reportar resultados

## Estado Final

- ✅ **Código TypeScript**: Compilado sin errores
- ✅ **14 herramientas MCP**: Registradas e implementadas
- ⏳ **Cliente MCP**: Necesita reinicio de VSCode
- 🔄 **Testing**: Pendiente de reinicio

## Archivos Modificados

1. `ide/electron/src/mcp/server.ts`:
   - Líneas 262-340: Registros de nuevas herramientas
   - Líneas 1545-1635: Implementación `memoryWrite()`
   - Líneas 1640-1660: Implementación `debugStart()`
   - Líneas 1665-1800: Implementaciones step/breakpoint tools

2. `ide/frontend/public/jsvecx_deploy/vecx.js`:
   - Línea 615-622: Comentado log excesivo de CARTRIDGE CODE

3. `core/src/backend/asm_address_mapper.rs`:
   - Líneas 86-93: Fix labels (no insertar en map)

4. `ide/frontend/src/main.tsx`:
   - Líneas 863-876: Conditional Step Into message

5. `ide/frontend/src/components/panels/EmulatorPanel.tsx`:
   - Líneas 984-1025: Handler `debug-switch-to-asm`

---

**Nota**: Después de reiniciar VSCode, las herramientas MCP deberían estar disponibles. Si siguen los timeouts, probar manualmente el debugging en el IDE para verificar que los fixes de navegación funcionan correctamente.
