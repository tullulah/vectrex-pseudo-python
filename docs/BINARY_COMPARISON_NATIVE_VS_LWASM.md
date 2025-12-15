# Comparación Binaria: Native M6809 Assembler vs lwasm
**Fecha**: 2025-11-02  
**Archivo**: test_debug_simple.vpy → test_debug_simple.asm → .bin

## Tamaños
- **Native assembler**: 362 bytes
- **lwasm**: 372 bytes  
- **Diferencia**: 10 bytes (2.7% más grande lwasm)

## Estadísticas de Coincidencia
- **Bytes coincidentes**: 300/362 (82.9%)
- **Bytes diferentes**: 72 (19.1%)
- **Primeras diferencias**: offset $000B (header music1)

## Diferencias Principales

### 1. Header - Music Pointer ($000B-$000C)
```
Offset   Native    lwasm     Descripción
------   ------    -----     -----------
$000B    $00       $FD       music1 high byte
$000C    $00       $0D       music1 low byte
```
**Causa**: `FDB music1` en header. Native assembler escribió $0000 (símbolo no resuelto), lwasm usó `music1 EQU $FD0D` del INCLUDE VECTREX.I.

**Problema**: Native assembler NO procesa símbolos del INCLUDE VECTREX.I.

### 2. Direcciones de JSR ($002A, $003E)
```
Offset   Native    lwasm     Descripción
------   ------    -----     -----------
$002A    $66       $70       JSR target high byte (VECTREX_MOVE_TO)
$003E    $5F       $69       JSR target high byte (VECTREX_SET_INTENSITY)
```
**Causa**: Direcciones de funciones internas diferentes. Native: $0166/$015F, lwasm: $0170/$0169.

**Problema**: Layout de código diferente - probablemente por los 10 bytes extra de lwasm.

### 3. Código Final Completamente Diferente ($012F+)
A partir del offset $012F (303 bytes, 83% del archivo), el código diverge totalmente por 62 bytes continuos.

**Zona afectada**: Final de VECTREX_DRAW_TO + VECTREX_SET_INTENSITY + VECTREX_WAIT_RECAL + DATA SECTION

## Análisis de Causas

### Causa Raíz: INCLUDE no procesado
El native assembler NO está procesando `INCLUDE "include/VECTREX.I"`, lo que causa:
1. ✅ Símbolos BIOS sí funcionan (Wait_Recal, Moveto_d, etc.) - hardcoded en load_vectrex_symbols()
2. ❌ Símbolos de constantes NO funcionan (music1, Vec_Default_Stk probablemente tampoco)
3. ❌ Layout de memoria diferente causa desplazamiento de direcciones

### Por qué funciona parcialmente
- BIOS symbols cargados manualmente en `load_vectrex_symbols()` con direcciones ROM reales
- Código ejecutable compila correctamente (opcodes + operandos)
- Pero constantes del INCLUDE quedan sin resolver → $0000

## Recomendaciones

### Corto Plazo (Workaround)
Agregar símbolos críticos de VECTREX.I a `load_vectrex_symbols()`:
```rust
equates.insert("music1".to_string(), 0xFD0D);
equates.insert("Vec_Default_Stk".to_string(), 0xCBEA);
// ... otros símbolos comunes del INCLUDE
```

### Mediano Plazo (Fix Correcto)
Implementar procesamiento de INCLUDE en `asm_to_binary.rs`:
1. Detectar líneas `INCLUDE "path"`
2. Resolver path relativo (include/, ide/frontend/public/include/)
3. Parsear EQU del archivo incluido
4. Agregar a equates HashMap antes de PASS 1

### Largo Plazo
- Procesar recursivamente INCLUDEs anidados
- Cache de archivos incluidos
- Detección de ciclos de inclusión
- Manejo de paths absolutos y relativos

## Impacto Funcional
**¿El binario native funciona?** Probablemente NO completamente:
- Header con music1=$0000 podría causar crash al intentar reproducir música
- Direcciones JSR internas desplazadas pueden ejecutar código incorrecto
- DATA SECTION con layout diferente podría corromper variables

**Para testing**: Usar lwasm hasta que INCLUDE esté implementado.  
**Para producción**: CRÍTICO implementar INCLUDE antes de release.
