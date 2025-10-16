# Debug Split View Implementation - October 16, 2025

## 🎯 **Visión General**

Implementación de un debugger híbrido con vista dividida (split view) que muestra **código VPy y ASM sincronizados en tiempo real**, con soporte para breakpoints dinámicos y control de ejecución paso a paso.

## 📁 **Componentes Implementados**

### 1. **DebugSplitView.tsx** - Vista Principal Split

**Ubicación**: `ide/frontend/src/components/DebugSplitView.tsx`

**Características**:
- ✅ Dos editores Monaco lado a lado (VPy + ASM)
- ✅ Sincronización de scroll proporcional
- ✅ Decoraciones de breakpoints (círculos rojos)
- ✅ Decoración de línea actual (flecha amarilla + highlight)
- ✅ Marcado simultáneo en ambos paneles
- ✅ Click en gutter para toggle breakpoints (solo VPy)
- ✅ Modo read-only durante debugging
- ✅ Headers con nombre de archivo y dirección PC

**Props**:
```typescript
interface DebugSplitViewProps {
  vpyContent: string;      // Código fuente VPy
  asmContent: string;       // Assembly generado
  currentDocument: { uri: string; path: string } | null;
}
```

### 2. **DebugToolbar.tsx** - Barra de Controles

**Ubicación**: `ide/frontend/src/components/DebugToolbar.tsx`

**Botones**:
- ▶️ **Run/Continue** (F5) - Iniciar o continuar ejecución
- ⏸️ **Pause** - Pausar ejecución en curso
- ⏹️ **Stop** - Detener y resetear
- ↗️ **Step Over** (F10) - Ejecutar línea completa
- ↘️ **Step Into** (F11) - Entrar a función
- ↖️ **Step Out** (Shift+F11) - Salir de función

**Info Display**:
- Línea VPy actual
- Dirección PC (Program Counter)
- Ciclos ejecutados
- FPS actual
- Estado (STOPPED / RUNNING / PAUSED)

**Atajos de Teclado**:
```
F5         → Continue/Run
F10        → Step Over
F11        → Step Into
Shift+F11  → Step Out
```

### 3. **debugStore.ts** - Estado Global del Debugger

**Ubicación**: `ide/frontend/src/state/debugStore.ts`

**Estado**:
```typescript
{
  state: 'stopped' | 'running' | 'paused',
  currentVpyLine: number | null,
  currentAsmAddress: string | null,
  pdbData: PdbData | null,
  callStack: CallFrame[],
  registers: Registers,
  totalCycles: number,
  currentFps: number
}
```

**Acciones**:
- `setState()` - Cambiar estado del debugger
- `loadPdbData()` - Cargar símbolos de debug (.pdb)
- `run()`, `pause()`, `stop()` - Control de ejecución
- `stepOver()`, `stepInto()`, `stepOut()` - Stepping
- `onBreakpointAdded()`, `onBreakpointRemoved()` - Sincronización dinámica

**Mensajes a JSVecx** (vía `postMessage`):
```javascript
{ type: 'debug-continue' }
{ type: 'debug-pause' }
{ type: 'debug-stop' }
{ type: 'debug-step-over', targetAddress: '0x0050' }
{ type: 'debug-step-into', isNativeCall: false }
{ type: 'debug-step-out' }
{ type: 'debug-add-breakpoint', address: '0x0050', line: 10 }
{ type: 'debug-remove-breakpoint', address: '0x0050', line: 10 }
```

### 4. **editorStore.ts** - Integración de Breakpoints

**Modificación**: Añadido import de `debugStore` y notificación en `toggleBreakpoint()`

**Flujo de Breakpoint Dinámico**:
```
Usuario click gutter → toggleBreakpoint(uri, line)
                        ↓
         Actualiza estado local (Set)
                        ↓
         Notifica debugStore.onBreakpointAdded/Removed()
                        ↓
      Si debugging activo → postMessage a JSVecx
                        ↓
     JSVecx añade/quita breakpoint SIN reiniciar
```

**Ventaja**: Breakpoints pueden añadirse/quitarse **en mitad de la ejecución** sin reiniciar el programa.

### 5. **Estilos CSS**

#### **global.css** - Decoraciones de Debug

```css
/* Breakpoint (círculo rojo) */
.debug-breakpoint {
  background: #e51400;
  width: 12px;
  height: 12px;
  border-radius: 50%;
}

/* Línea actual VPy (flecha amarilla + highlight) */
.debug-current-line {
  background: rgba(255, 255, 0, 0.2);
}

.debug-current-line-arrow {
  border-width: 6px 0 6px 10px;
  border-color: transparent transparent transparent #ffff00;
}

/* Instrucción ASM actual */
.debug-current-asm-line {
  background: rgba(255, 255, 0, 0.15);
  border-left: 2px solid #ffff00;
}
```

#### **DebugToolbar.css** - Botones y UI

- Botones con colores semánticos (azul=run, rojo=stop, amarillo=pause)
- Efectos hover y active
- Badge de estado (STOPPED/RUNNING/PAUSED)
- Info panel con valores monoespaciados

## 🔄 **Flujo de Ejecución**

### Escenario 1: Usuario presiona F5 (Run)

```
1. Usuario: Presiona F5
              ↓
2. DebugToolbar: Detecta keydown, llama run()
              ↓
3. debugStore: setState('running')
              ↓
4. debugStore: postMessage({ type: 'debug-continue' })
              ↓
5. JSVecx: Inicia emulación, ejecuta opcodes
              ↓
6. JSVecx: Breakpoint detectado en PC = 0x0050
              ↓
7. JSVecx: postMessage({ type: 'debugger-paused', pc: '0x0050', ... })
              ↓
8. debugStore: setState('paused')
              ↓
9. debugStore: setCurrentVpyLine(10) [via pdb.lineMap]
              ↓
10. DebugSplitView: Re-render con decoraciones
              ↓
11. UI: Muestra flecha amarilla en línea 10 VPy + ASM correspondiente
```

### Escenario 2: Usuario añade breakpoint durante ejecución

```
1. Usuario: Click en gutter línea 15 (programa corriendo)
              ↓
2. editorStore: toggleBreakpoint('main.vpy', 15)
              ↓
3. editorStore: Añade línea 15 a Set de breakpoints
              ↓
4. editorStore: Llama debugStore.onBreakpointAdded('main.vpy', 15)
              ↓
5. debugStore: Busca address en pdb.lineMap['15'] → '0x0080'
              ↓
6. debugStore: postMessage({ type: 'debug-add-breakpoint', address: '0x0080' })
              ↓
7. JSVecx: Añade 0x0080 a breakpoint set (SIN REINICIAR)
              ↓
8. JSVecx: Continúa ejecución normalmente
              ↓
9. JSVecx: Al llegar a PC=0x0080 → Pausa automáticamente
              ↓
10. UI: Actualiza vista con nueva línea actual
```

## 🎨 **UI/UX Layout**

```
┌──────────────────────────────────────────────────────────────┐
│ DebugToolbar                                                  │
│ [▶️ Run] [⏹️ Stop] | [↗️ Step Over] [↘️ Step Into] [↖️ Step Out]│
│ Line: 10 | PC: 0xC890 | Cycles: 5,234 | FPS: 60.0 | PAUSED  │
├───────────────────────────┬──────────────────────────────────┤
│ 🐍 VPy Source             │ ⚙️ Assembly                      │
│ test_debug_simple.vpy     │ PC: 0xC890                       │
├───────────────────────────┼──────────────────────────────────┤
│                           │                                  │
│  4 def main():            │ START:                           │
│  5 ●   WAIT_RECAL()       │     LDA #$80                     │
│  6     SET_INTENSITY(5)   │     STA VIA_t1_cnt_lo            │
│                           │     JSR VECTREX_WAIT_RECAL       │
│  9 def loop():            │     ...                          │
│ 10 ► ● MOVE(0, 0)         │ MAIN:                            │
│ 11     DRAW_TO(50, 0)     │     JSR Wait_Recal               │
│ 12     DRAW_TO(50, 50)    │ ►   LDA #$80                     │
│ 13     DRAW_TO(0, 50)     │     STA VIA_t1_cnt_lo            │
│ 14     DRAW_TO(0, 0)      │     JSR LOOP_BODY                │
│                           │ LOOP_BODY:                       │
│                           │     JSR VECTREX_MOVE_TO          │
│                           │     ...                          │
└───────────────────────────┴──────────────────────────────────┘

Símbolos:
● = Breakpoint activo (círculo rojo)
► = Línea ejecutándose (flecha amarilla + highlight)
```

## 📊 **Datos Necesarios del .pdb**

### Formato JSON Mínimo Requerido

```json
{
  "version": "1.0",
  "source": "test_debug_simple.vpy",
  "binary": "test_debug_simple.bin",
  "entry_point": "0x0000",
  "symbols": {
    "START": "0x0020",
    "MAIN": "0x0050",
    "LOOP_BODY": "0x0080"
  },
  "lineMap": {
    "5": "0x0020",   // main() - WAIT_RECAL()
    "6": "0x0030",   // main() - SET_INTENSITY(5)
    "10": "0x0050",  // loop() - MOVE(0, 0)
    "11": "0x0060",  // loop() - DRAW_TO(50, 0)
    "12": "0x0070",  // loop() - DRAW_TO(50, 50)
    "13": "0x0080",  // loop() - DRAW_TO(0, 50)
    "14": "0x0090"   // loop() - DRAW_TO(0, 0)
  },
  "functions": {
    "main": {
      "startLine": 4,
      "endLine": 6,
      "address": "0x0020",
      "type": "vpy"
    },
    "loop": {
      "startLine": 9,
      "endLine": 14,
      "address": "0x0050",
      "type": "vpy"
    }
  },
  "nativeCalls": {
    "5": "VECTREX_WAIT_RECAL",
    "6": "VECTREX_SET_INTENSITY",
    "10": "VECTREX_MOVE_TO",
    "11": "VECTREX_DRAW_TO",
    "12": "VECTREX_DRAW_TO",
    "13": "VECTREX_DRAW_TO",
    "14": "VECTREX_DRAW_TO"
  }
}
```

## 🚀 **Estado de Implementación**

### ✅ Fase 1 - Componentes UI (COMPLETADO)
- [x] DebugSplitView con editores Monaco sincronizados
- [x] DebugToolbar con botones de control
- [x] Decoraciones de breakpoints y línea actual
- [x] Atajos de teclado (F5, F10, F11, Shift+F11)
- [x] Estilos CSS (VS2022-style)
- [x] debugStore con estado y acciones
- [x] Integración editorStore → debugStore

### 🔲 Fase 2 - Backend JSVecx (PENDIENTE)
- [ ] Modificar `e6809.js` para detectar breakpoints
- [ ] Implementar `stepOver()` / `stepInto()` / `stepOut()`
- [ ] Tracking de call stack (JSR/BSR/RTS)
- [ ] postMessage events a IDE
- [ ] Añadir/quitar breakpoints dinámicamente

### 🔲 Fase 3 - .pdb Enhanced (PENDIENTE)
- [ ] Rastrear direcciones reales durante codegen
- [ ] Poblar `lineMap` con mapeos VPy → ASM
- [ ] Añadir sección `functions` con start/end lines
- [ ] Añadir sección `nativeCalls` para Step Into

### 🔲 Fase 4 - Desensambladao ASM (FUTURO)
- [ ] Parser de opcodes 6809 en JavaScript
- [ ] Generación dinámica de ASM con direcciones
- [ ] Highlighting de instrucción actual
- [ ] Annotations de símbolos (JSR VECTREX_WAIT_RECAL, etc.)

## 🔧 **Integración con App Principal**

Para usar DebugSplitView en la app, reemplazar el editor normal con:

```typescript
import { DebugSplitView } from './components/DebugSplitView';
import { DebugToolbar } from './components/DebugToolbar';

function App() {
  const debugState = useDebugStore(s => s.state);
  const currentDoc = useEditorStore(s => s.documents.find(d => d.uri === s.active));
  
  // Load .pdb when opening VPy file
  useEffect(() => {
    if (currentDoc?.path.endsWith('.vpy')) {
      const pdbPath = currentDoc.path.replace('.vpy', '.pdb');
      fetch(pdbPath)
        .then(res => res.json())
        .then(pdb => useDebugStore.getState().loadPdbData(pdb))
        .catch(err => console.warn('No .pdb found:', err));
    }
  }, [currentDoc?.path]);
  
  // Load corresponding ASM
  const [asmContent, setAsmContent] = useState('');
  useEffect(() => {
    if (currentDoc?.path.endsWith('.vpy')) {
      const asmPath = currentDoc.path.replace('.vpy', '.asm');
      fetch(asmPath)
        .then(res => res.text())
        .then(asm => setAsmContent(asm))
        .catch(err => console.warn('No .asm found:', err));
    }
  }, [currentDoc?.path]);
  
  return (
    <>
      {debugState !== 'stopped' && <DebugToolbar />}
      
      {debugState !== 'stopped' ? (
        <DebugSplitView
          vpyContent={currentDoc?.content || ''}
          asmContent={asmContent}
          currentDocument={currentDoc}
        />
      ) : (
        <NormalEditorView />
      )}
    </>
  );
}
```

## 📝 **Notas de Desarrollo**

1. **Sincronización de Scroll**: Calculada proporcionalmente (ratio líneas ASM / líneas VPy)
2. **Breakpoints Solo en VPy**: ASM es read-only, breakpoints se setean en código fuente
3. **Direcciones Placeholder**: Actualmente todos los símbolos apuntan a 0x0000 (Phase 3 pendiente)
4. **Performance**: Monaco deltaDecorations es eficiente, pero limitar re-renders innecesarios
5. **Circular Dependency**: editorStore importa debugStore (OK) - debugStore NO importa editorStore

## 🎯 **Próximos Pasos Críticos**

1. **Implementar JSVecx breakpoint detection** (`e6809.js`)
2. **Poblar .pdb con line mappings reales** (Phase 2A del compiler)
3. **Integrar DebugSplitView en App principal** (reemplazar editor normal)
4. **Probar flujo completo**: F5 → breakpoint → Step Over → Step Into

---

**Fecha de Implementación**: 16 de octubre de 2025  
**Estado**: UI Components Complete ✅ | Backend Pending 🔲  
**Archivos Nuevos**: 4 (DebugSplitView.tsx, DebugToolbar.tsx, DebugToolbar.css, este documento)  
**Archivos Modificados**: 3 (debugStore.ts, editorStore.ts, global.css)
