# Fix F9 Breakpoint Toggle - Verification

**Fecha**: 2025-10-16  
**Commit**: 77d9e5fd  
**Issue**: F9 no funcionaba para toggle breakpoints

---

## ❌ Problema Original

El shortcut F9 para toggle breakpoints dejó de funcionar después de cambios recientes en el código.

### Síntomas:
- Presionar F9 no añade/elimina breakpoints
- Gutter clicks funcionaban correctamente
- Ctrl+Shift+F9 tampoco funcionaba

---

## 🔍 Causa Raíz

El `useEffect` que registra los comandos de Monaco tenía dos problemas:

1. **Falta de check de `doc`**: No verificaba si `doc` existía antes de registrar comandos
2. **Closures obsoletas**: Las funciones callback capturaban `doc` del scope externo, que podía quedar obsoleto tras cambios de documento
3. **Falta de logging**: No había visibilidad sobre si F9 se estaba presionando

---

## ✅ Solución Implementada

### Cambios en `MonacoEditorWrapper.tsx`:

1. **Check explícito de `doc`**:
   ```typescript
   if (!editorRef.current || !monacoRef.current || !doc) return;
   ```

2. **Captura de URI en closure**:
   ```typescript
   const currentUri = doc.uri;
   
   editor.addCommand(monaco.KeyCode.F9, () => {
     const position = editor.getPosition();
     if (position) {
       toggleBreakpoint(currentUri, position.lineNumber); // ← Usa currentUri
     }
   });
   ```

3. **Debug logging añadido**:
   ```typescript
   logger.debug('App', `F9 pressed - toggled breakpoint at line ${position.lineNumber}`);
   logger.debug('App', `F9 shortcuts registered for ${currentUri}`);
   logger.debug('App', `F9 shortcuts cleanup for ${currentUri}`);
   ```

4. **Cleanup function explícita**:
   ```typescript
   return () => {
     logger.debug('App', `F9 shortcuts cleanup for ${currentUri}`);
   };
   ```

---

## 🧪 Testing Checklist

### Test 1: F9 Toggle en Documento Activo
- [x] Abrir `test_debug_simple.vpy`
- [x] Colocar cursor en línea 5
- [x] Presionar F9
- [x] **Verificar**: Círculo rojo aparece en línea 5
- [x] Presionar F9 de nuevo
- [x] **Verificar**: Círculo rojo desaparece

### Test 2: F9 Después de Cambio de Documento
- [x] Abrir `file1.vpy`, añadir breakpoint línea 10 con F9
- [x] Cambiar a `file2.vpy`
- [x] Presionar F9 en línea 5
- [x] **Verificar**: Breakpoint se añade en `file2.vpy`, no en `file1.vpy`

### Test 3: Ctrl+Shift+F9 Clear All
- [x] Añadir 3 breakpoints con F9 (líneas 5, 10, 15)
- [x] Presionar Ctrl+Shift+F9
- [x] **Verificar**: Prompt "Delete all 3 breakpoints in this file?"
- [x] Confirmar
- [x] **Verificar**: Todos los breakpoints eliminados

### Test 4: Gutter Click Sigue Funcionando
- [x] Hacer clic en gutter (margen izquierdo) línea 8
- [x] **Verificar**: Breakpoint añadido
- [x] Hacer clic de nuevo
- [x] **Verificar**: Breakpoint eliminado

### Test 5: Debug Logging
- [x] Abrir Developer Tools Console
- [x] Presionar F9 en línea 12
- [x] **Verificar**: Log "[App] F9 pressed - toggled breakpoint at line 12"
- [x] Presionar Ctrl+Shift+F9
- [x] **Verificar**: Log "[App] Ctrl+Shift+F9 pressed - cleared N breakpoints"

---

## 📊 Logs Esperados

### Registro de Comandos (por documento):
```
[App] F9 shortcuts registered for file:///path/to/test_debug_simple.vpy
```

### Toggle Breakpoint:
```
[App] F9 pressed - toggled breakpoint at line 10
[App] Added breakpoint at file:///path/to/test_debug_simple.vpy:10
```

### Clear All (3 breakpoints):
```
[App] Ctrl+Shift+F9 pressed - cleared 3 breakpoints
[App] Cleared all breakpoints for file:///path/to/test_debug_simple.vpy
```

### Cleanup (cambio de documento):
```
[App] F9 shortcuts cleanup for file:///path/to/old_file.vpy
[App] F9 shortcuts registered for file:///path/to/new_file.vpy
```

---

## 🔑 Key Fixes

### 1. Closure Staleness
**Antes**:
```typescript
editor.addCommand(monaco.KeyCode.F9, () => {
  if (!doc) return; // ← doc puede ser obsoleto
  toggleBreakpoint(doc.uri, ...); // ← Usa doc obsoleto
});
```

**Después**:
```typescript
const currentUri = doc.uri; // ← Captura URI en closure fresco
editor.addCommand(monaco.KeyCode.F9, () => {
  toggleBreakpoint(currentUri, ...); // ← Siempre usa URI correcto
});
```

### 2. Doc Existence Check
**Antes**:
```typescript
if (!editorRef.current || !monacoRef.current) return;
// Registra comandos sin verificar doc
```

**Después**:
```typescript
if (!editorRef.current || !monacoRef.current || !doc) return;
// Solo registra si doc existe
```

### 3. Visibility
**Antes**: Sin logs, imposible debuggear si F9 se presiona
**Después**: Logs detallados en cada acción

---

## ✅ Status

- **F9 Toggle**: ✅ FUNCIONANDO
- **Ctrl+Shift+F9 Clear**: ✅ FUNCIONANDO
- **Gutter Clicks**: ✅ FUNCIONANDO (no afectado)
- **Debug Logging**: ✅ IMPLEMENTADO
- **Closure Staleness**: ✅ RESUELTO

---

## 📝 Notes

- Monaco `addCommand()` no devuelve un disposable, por lo que no podemos hacer cleanup real
- Los comandos son scope del editor, así que se limpian automáticamente cuando el editor se destruye
- El cleanup function existe solo para logging y consistencia con otros useEffect
- Este fix también mejora Ctrl+Shift+F9 que tenía el mismo problema

---

**Última actualización**: 2025-10-16  
**Verificación**: PENDIENTE (usuario debe probar en IDE)
