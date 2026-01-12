# ASM Multibank Test

Prueba minimal en ASM puro (sin VPy compiler) para verificar:
1. Bank switching via $D000
2. Audio (o falta de)
3. Multibank linking

## Archivos

- `bank0.asm` - Boot stub (Bank #0)
  - Emite header Vectrex
  - Escribe bank ID 31 a $D000
  - Salta a START en Bank #31

- `bank31.asm` - Programa principal (Bank #31)
  - Inicializa DP a $D0
  - Llama a Wait_Recal en loop
  - **SIN Init_Music_Buf** (para probar si audio es el problema)

## Compilación

```bash
bash compile.sh
```

Genera: `multibank.bin` (512 KB)

## Prueba en Emulador

La ROM compilada está disponible en:
- IDE: Cargar `test_multibank_asm.bin` desde public folder
- JSVecx: Punto de entrada es $430A (Bank #31 START)

## Qué esperar

✓ Pantalla blanca (Intensity 100 = normal brillo)
✓ Sin ruido de audio (NO hay Init_Music_Buf)
✓ Loop infinito en Wait_Recal

## Si hay problemas

1. **Ruido sigue presente**: Audio es independiente del compilador VPy
2. **ROM no carga**: Verificar que BIOS detecta header en $0000
3. **PC diferente de $430A**: Bank switching no funcionó
