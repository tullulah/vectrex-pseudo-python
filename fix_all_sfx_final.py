#!/usr/bin/env python3
"""Fix completo y final de sfx_doframe - TODOS los problemas identificados"""

import re

file_path = 'core/src/backend/m6809/emission.rs'

with open(file_path, 'r', encoding='utf-8') as f:
    content = f.read()

# Encontrar la sección sfx_doframe
start_marker = 'sfx_doframe:'
end_marker = 'sfx_endofeffect:'

start_idx = content.find(start_marker)
if start_idx == -1:
    print("❌ No se encontró sfx_doframe")
    exit(1)

# Buscar el final completo (incluyendo RTS después de sfx_endofeffect)
end_idx = content.find('    RTS\n', content.find(end_marker))
if end_idx == -1:
    print("❌ No se encontró el final de sfx_endofeffect")
    exit(1)
end_idx += len('    RTS\n')

# Reemplazar toda la sección con el código correcto 1:1 del original
new_sfx_code = '''sfx_doframe:
    LDU sfx_pointer        ; Get current frame pointer
    LDB ,U                 ; Read flag byte (NO auto-increment)
    CMPB #$D0              ; Check end marker (first byte)
    BNE sfx_checktonefreq  ; Not end, continue
    LDB 1,U                ; Check second byte at offset 1
    CMPB #$20              ; End marker $D0 $20?
    BEQ sfx_endofeffect    ; Yes, stop

sfx_checktonefreq:
    LEAY 1,U               ; Y = pointer to tone/noise data
    LDB ,U                 ; Reload flag byte (Sound_Byte corrupts B)
    BITB #$20              ; Bit 5: tone data present?
    BEQ sfx_checknoisefreq ; No, skip tone
    ; Set tone frequency (channel C = reg 4/5)
    LDB 1,U                ; Get first tone byte (fine tune)
    LDA #$04               ; Register 4
    JSR Sound_Byte         ; Write to PSG
    LDB 2,U                ; Get second tone byte (coarse tune)
    LDA #$05               ; Register 5
    JSR Sound_Byte         ; Write to PSG
    LEAY 2,Y               ; Skip 2 tone bytes

sfx_checknoisefreq:
    LDB ,U                 ; Reload flag byte
    BITB #$40              ; Bit 6: noise data present?
    BEQ sfx_checkvolume    ; No, skip noise
    LDB ,Y                 ; Get noise period
    LDA #$06               ; Register 6
    JSR Sound_Byte         ; Write to PSG
    LEAY 1,Y               ; Skip 1 noise byte

sfx_checkvolume:
    LDB ,U                 ; Reload flag byte
    ANDB #$0F              ; Get volume from bits 0-3
    LDA #$0A               ; Register 10 (volume C)
    JSR Sound_Byte         ; Write to PSG

sfx_checktonedisable:
    LDB ,U                 ; Reload flag byte
    BITB #$10              ; Bit 4: disable tone?
    BEQ sfx_enabletone
sfx_disabletone:
    LDB $C807              ; Read mixer shadow (MUST be B register)
    ORB #$04               ; Set bit 2 (disable tone C)
    LDA #$07               ; Register 7 (mixer)
    JSR Sound_Byte         ; Write to PSG
    BRA sfx_checknoisedisable  ; Continue to noise check

sfx_enabletone:
    LDB $C807              ; Read mixer shadow (MUST be B register)
    ANDB #$FB              ; Clear bit 2 (enable tone C)
    LDA #$07               ; Register 7 (mixer)
    JSR Sound_Byte         ; Write to PSG

sfx_checknoisedisable:
    LDB ,U                 ; Reload flag byte
    BITB #$80              ; Bit 7: disable noise?
    BEQ sfx_enablenoise
sfx_disablenoise:
    LDB $C807              ; Read mixer shadow (MUST be B register)
    ORB #$20               ; Set bit 5 (disable noise C)
    LDA #$07               ; Register 7 (mixer)
    JSR Sound_Byte         ; Write to PSG
    BRA sfx_nextframe      ; Done, update pointer

sfx_enablenoise:
    LDB $C807              ; Read mixer shadow (MUST be B register)
    ANDB #$DF              ; Clear bit 5 (enable noise C)
    LDA #$07               ; Register 7 (mixer)
    JSR Sound_Byte         ; Write to PSG

sfx_nextframe:
    STY sfx_pointer        ; Update pointer for next frame
    RTS

sfx_endofeffect:
    CLR sfx_status         ; Mark effect as stopped
    LDA #$0A               ; Register 10 (volume C)
    LDB #$00               ; Silence
    JSR Sound_Byte         ; Write to PSG
    LDD #$0000
    STD sfx_pointer        ; Clear pointer
    RTS
'''

# Reemplazar
new_content = content[:start_idx] + new_sfx_code + content[end_idx:]

with open(file_path, 'w', encoding='utf-8') as f:
    f.write(new_content)

print("✅ FIX COMPLETO APLICADO")
print()
print("CAMBIOS REALIZADOS:")
print("==================")
print("1. Línea 3: LDB ,U+ → LDB ,U (eliminado auto-increment)")
print("2. Líneas 6-7: LDA ,U + CMPA → LDB 1,U + CMPB")
print("3. Mixer: LDA $C807 → LDB $C807 (3 ocurrencias)")
print("4. Mixer: ORA/ANDA → ORB/ANDB (todas las operaciones)")
print("5. Eliminados: TODOS los PSHS A/LDB ,S+ (3 ocurrencias)")
print("6. Control flow: RTS → BRA sfx_checknoisedisable (después de sfx_disabletone)")
print("7. Control flow: RTS → BRA sfx_nextframe (después de sfx_disablenoise)")
print("8. Añadido: Label sfx_checknoisedisable")
print()
print("CÓDIGO AHORA 1:1 CON RICHARD CHADD ORIGINAL")
