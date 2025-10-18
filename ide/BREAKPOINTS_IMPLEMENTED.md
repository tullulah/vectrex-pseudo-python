# Breakpoints Implementation - VS2022 Style

## ✅ Implementado (2025-10-16)

### Funcionalidades Core

1. **Toggle Breakpoint (F9)**
   - Presionar F9 en cualquier línea añade/quita breakpoint
   - Visual: círculo rojo en el margen (glyph margin)
   - Estado persistente mientras el archivo esté abierto

2. **Clear All Breakpoints (Ctrl+Shift+F9)**
   - Elimina todos los breakpoints del archivo actual
   - Atajo de teclado estándar VS2022

3. **Gutter Click Toggle**
   - Click en el margen izquierdo (glyph margin) para añadir/quitar breakpoint
   - Hover muestra tooltip "Breakpoint"
   - Visual feedback con hover (rojo más brillante)

### Arquitectura

**Estado (editorStore.ts):**
```typescript
breakpoints: Record<string, Set<number>>; // uri -> líneas (1-indexed)
toggleBreakpoint(uri: string, lineNumber: number): void;
clearAllBreakpoints(uri?: string): void;
```

**UI (MonacoEditorWrapper.tsx):**
- Decoraciones Monaco para renderizar círculos rojos
- Event handlers para F9 y Ctrl+Shift+F9
- Mouse handler para gutter clicks (target.type === 2)

**Estilos (global.css):**
```css
.breakpoint-glyph {
  background: #e51400; /* Rojo VS2022 */
  width: 12px;
  height: 12px;
  border-radius: 50%; /* Círculo perfecto */
  box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.3);
}
```

### Logging

```
App: Added breakpoint at file:///path/file.vpy:15
App: Removed breakpoint at file:///path/file.vpy:15
App: Cleared all breakpoints for file:///path/file.vpy
```

## 📋 Próximos Pasos (JSVecx Debugger)

1. **Debugger Backend en JSVecx**
   - Instrumentar e6809.js para capturar breakpoints
   - Pausar ejecución cuando PC == breakpoint address
   - Mapear líneas VPy → direcciones ASM

2. **Debugger UI en IDE**
   - Panel "Debug" con variables/registros
   - Call stack visual
   - Step Over / Step Into / Step Out
   - Continue / Pause buttons

3. **Watch Variables**
   - Inspector de memoria en tiempo real
   - Watch expressions (ej: `var_x`, `@0xC800`)

4. **Disassembly View**
   - Mostrar ASM alrededor de PC actual
   - Highlight de línea actual en ejecución

## 🎯 Ventajas vs. Emulador Rust

- ✅ **Funciona perfectamente** - Sin skew, centrado correcto
- ✅ **Integración directa** - JavaScript puro, sin WASM
- ✅ **Debugging nativo** - Browser DevTools compatible
- ✅ **Desarrollo rápido** - Sin compilación, cambios instantáneos
- ✅ **Mantenible** - Código limpio y documentado

## 📝 Notas de Implementación

- **Line numbers**: 1-indexed (Monaco estándar)
- **Decorations**: Automáticamente actualizadas cuando cambia `breakpoints` store
- **Persistencia**: Solo en memoria (no se guardan en disco)
- **Multi-archivo**: Cada archivo tiene su propio set de breakpoints
- **Performance**: Decoraciones optimizadas con `deltaDecorations()`

## 🔧 Testing

1. Abrir archivo `.vpy` en el IDE
2. Presionar **F9** en una línea → debe aparecer círculo rojo
3. Presionar **F9** de nuevo → círculo desaparece
4. Click en margen izquierdo → toggle breakpoint
5. **Ctrl+Shift+F9** → limpia todos los breakpoints

---

**Estado**: ✅ Listo para testing
**Fecha**: 2025-10-16
**Próximo**: Implementar debugger backend en JSVecx
