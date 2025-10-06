; Test BIOS Print_Str - Vector Output Comparison
; Este test ejecuta la BIOS hasta Print_Str para renderizar texto
; y compara los vectores generados entre JSVecx y Rust
;
; Objetivo: Detectar el problema de "skewed letters" comparando vectores pixel por pixel
;
; Estrategia:
; 1. Cargar BIOS (0xE000-0xFFFF)
; 2. Reset CPU (PC → 0xF000)
; 3. Ejecutar BIOS hasta Print_Str (0xF495)
; 4. Capturar vectores generados
; 5. Comparar vectores entre emuladores
;
; NOTA: Este test NO necesita código custom - solo ejecuta BIOS real
; El binario es vacío (solo placeholder) porque usamos BIOS embebida

        ORG $C800

; Placeholder - este test solo ejecuta BIOS, no código custom
; El test runner ejecutará hasta Print_Str automáticamente

nop_loop:
        NOP
        BRA nop_loop

        END
