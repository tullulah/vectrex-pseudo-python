# Flat File Structure Issues

## Problemas Identificados

### 1. Variables VLINE_* en lista global (NO deberían estar)

Líneas 51-58 en multibank_flat.asm contienen:
```asm
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
```

**Problema**: Estas variables son específicas de DRAW_LINE_WRAPPER y deberían estar SOLO en Bank #31 (helper section), no en la lista global de RAM.

**Razón**: Las variables VLINE son auxiliares que solo usa DRAW_LINE_WRAPPER (que está en Bank #31). No son parte de la interfaz global del programa.

**Ubicación correcta**: Deberían estar en la sección RAM de Bank #31, no en la sección global.

### 2. Falta vector de RESET al final

El archivo termina en línea 1432 con:
```asm
; === Multibank Mode: Interrupt Vectors in Bank #31 (Linker) ===
; All vectors handled by multi_bank_linker
; Bank #0-#30: Local 0xFFF0-0xFFFF addresses are unreachable
; Bank #31: Contains complete interrupt vector table (fixed at 0x4000-0x7FFF window)
```

**Falta**: El vector de RESET en $FFFE no está en el archivo flat.

**Debería tener**:
```asm
    ORG $FFFE
    FDB CUSTOM_RESET
```

## Estructura Correcta Esperada

```
┌─────────────────────────────────────────────┐
│ Global RAM Variables (lines ~1-50)          │
├─────────────────────────────────────────────┤
│ CURRENT_ROM_BANK  ✓ (multibank only)        │
│ (other vars needed across all banks)        │
│ NO VLINE_* variables here                   │
└─────────────────────────────────────────────┘
         ↓
┌─────────────────────────────────────────────┐
│ BANKS #0-#30 (Code in ORG $0000)            │
│ (each bank contains functions)              │
└─────────────────────────────────────────────┘
         ↓
┌─────────────────────────────────────────────┐
│ BANK #31 (Fixed, ORG $4000)                 │
│ ┌───────────────────────────────────────┐   │
│ │ CUSTOM_RESET                          │   │
│ │ Builtins (J1X, J1Y, J1B, etc.)        │   │
│ │ DRAW_LINE_WRAPPER + VLINE variables   │   │
│ │ Other helpers                         │   │
│ └───────────────────────────────────────┘   │
└─────────────────────────────────────────────┘
         ↓
┌─────────────────────────────────────────────┐
│ Data Assets                                 │
│ (vectors, music, etc.)                      │
└─────────────────────────────────────────────┘
         ↓
┌─────────────────────────────────────────────┐
│ Interrupt Vectors (ORG $FFFE)               │
│ FDB CUSTOM_RESET                            │
└─────────────────────────────────────────────┘
```

## Acciones Recomendadas

1. **Para VLINE_* variables**:
   - Investigar `core/src/backend/m6809/mod.rs` 
   - Buscar dónde se emiten estas variables globales
   - Hacer que se emitan SOLO en la sección RAM de Bank #31, no en la global

2. **Para vector de RESET**:
   - El linker debe agregar `ORG $FFFE` y `FDB CUSTOM_RESET` al final del archivo flat
   - Ubicación probable: `core/src/backend/m6809/multi_bank_linker.rs`

## Archivos Relevantes

- `examples/test_callgraph/src/multibank_temp/multibank_flat.asm` (archivo con problemas)
- `core/src/backend/m6809/mod.rs` (donde se emiten variables RAM)
- `core/src/backend/m6809/multi_bank_linker.rs` (linker que organiza bancos)
