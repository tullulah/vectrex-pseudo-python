# LSP Quick Fixes Testing Guide

## ✅ Phase 1-3 COMPLETADO

Se han implementado exitosamente:

### Phase 1: Variable Usage Analysis
- ✅ Estructuras `VariableUsage` y `UsageAnalysis`
- ✅ Análisis recursivo del AST
- ✅ Tracking de declaraciones, lecturas y escrituras

### Phase 2: Diagnostics Generation
- ✅ WARNING para variables no usadas
- ✅ HINT para sugerencias de const
- ✅ Mensajes bilingües (en/es)

### Phase 3: Code Actions (Quick Fixes)
- ✅ Método `code_action()` implementado
- ✅ Quick Fix 1: "Convert to const"
- ✅ Quick Fix 2: "Remove unused variable"
- ✅ WorkspaceEdit con TextEdit

## 🧪 Testing en VSCode

### Test 1: Archivo de Prueba Simple

1. **Abrir archivo de prueba**:
   - Archivo: `test_lsp_diagnostics.vpy`
   - Ubicación: `/Users/daniel/projects/vectrex-pseudo-python/test_lsp_diagnostics.vpy`

2. **Reiniciar LSP en VSCode**:
   - Presiona `Cmd+Shift+P`
   - Escribe "Reload Window" y presiona Enter
   - Esto recargará el LSP con el nuevo código

3. **Verificar diagnostics**:
   - Línea 8: `unused_var = 42` 
     - Debe aparecer WARNING con texto gris/tachado
     - Mensaje: "Variable 'unused_var' is declared but never used"
   
   - Línea 9: `num_locations = 17`
     - Debe aparecer HINT con subrayado amarillo
     - Mensaje: "Variable 'num_locations' never changes - consider 'const' to save RAM (2 bytes)"

4. **Probar Quick Fixes**:
   
   **Quick Fix 1: Convert to const**
   - Hover sobre `num_locations = 17`
   - Debe aparecer 💡 lightbulb icon
   - Click en el lightbulb o presiona `Cmd+.`
   - Debe aparecer: "Convert 'num_locations' to const"
   - Click en la opción
   - **Resultado esperado**: Línea cambia a `const num_locations = 17`
   
   **Quick Fix 2: Remove unused**
   - Hover sobre `unused_var = 42`
   - Click en lightbulb o presiona `Cmd+.`
   - Debe aparecer: "Remove unused variable 'unused_var'"
   - Click en la opción
   - **Resultado esperado**: Línea se elimina completamente

### Test 2: Proyecto Real (Pang)

1. **Abrir archivo Pang**:
   - Archivo: `examples/pang/src/main.vpy`
   - Buscar las siguientes variables:
     - `num_locations = 17` (línea ~15) → HINT
     - `hook_max_y = 40` (línea ~19) → HINT
     - `player_speed = 2` (línea ~20) → HINT

2. **Aplicar Quick Fixes**:
   - Convertir las 3 variables a const
   - Verificar que el juego sigue compilando:
     ```bash
     cd /Users/daniel/projects/vectrex-pseudo-python
     cargo run --bin vectrexc -- build examples/pang/src/main.vpy --bin
     ```

3. **Beneficio esperado**:
   - Ahorro de RAM: 6 bytes (3 variables × 2 bytes cada una)
   - Binario más pequeño (sin código de inicialización)

## 🎯 Checklist de Verificación

- [ ] LSP reiniciado (Reload Window)
- [ ] test_lsp_diagnostics.vpy abierto
- [ ] WARNING visible en unused_var (texto gris)
- [ ] HINT visible en num_locations (subrayado amarillo)
- [ ] Lightbulb 💡 aparece al hover
- [ ] Quick Fix "Convert to const" funciona
- [ ] Quick Fix "Remove unused variable" funciona
- [ ] Pang muestra 3 HINTs para const
- [ ] Pang compila después de aplicar fixes
- [ ] Binary size reducido (verificar con `ls -lh`)

## 📊 Próximos Pasos (Phase 4)

Si todo funciona correctamente:

1. **Refinamiento de mensajes**:
   - Hacer mensajes más descriptivos
   - Agregar ejemplos en hover
   - Mejorar traducción al español

2. **Diagnostics adicionales**:
   - Variables no inicializadas
   - Variables write-only (nunca leídas)
   - Sugerencias de `let` vs declaración implícita

3. **LSP Features extendidos**:
   - Code lens para mostrar usage count
   - Inlay hints para tipos inferidos
   - Signature help para builtins

## 🐛 Troubleshooting

**Si no aparecen diagnostics:**
1. Verificar que el LSP esté corriendo: `ps aux | grep vpy_lsp`
2. Reiniciar VSCode completamente (no solo Reload Window)
3. Verificar logs: VSCode → Output → VPy Language Server

**Si lightbulb no aparece:**
1. Verificar que estás en la línea correcta (no en línea vacía)
2. Intentar presionar `Cmd+.` directamente sobre el warning/hint
3. Verificar que diagnostic tenga código: "unused-variable" o "suggest-const"

**Si Quick Fix no aplica cambios:**
1. Verificar que el archivo no sea read-only
2. Verificar que el workspace esté guardado
3. Intentar manual: copiar nuevo texto y reemplazar línea

## ✨ Expected Output Examples

**Antes del Quick Fix (num_locations)**:
```python
num_locations = 17       # 💡 HINT: Variable 'num_locations' never changes - consider 'const' to save RAM (2 bytes)
```

**Después del Quick Fix**:
```python
const num_locations = 17  # ✅ Ahora es const, ahorra 2 bytes RAM
```

**Antes del Quick Fix (unused_var)**:
```python
unused_var = 42          # ⚠️ WARNING: Variable 'unused_var' is declared but never used
```

**Después del Quick Fix**:
```python
                         # ✅ Línea eliminada
```

---

Última actualización: 2025-01-05 01:22 AM
