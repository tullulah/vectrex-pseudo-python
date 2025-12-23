#!/usr/bin/env python3
"""Fix completo de sfx_doframe - todos los problemas identificados"""

file_path = 'core/src/backend/m6809/emission.rs'

with open(file_path, 'r', encoding='utf-8') as f:
    lines = f.readlines()

# Buscar la línea que contiene "sfx_doframe:"
start_idx = None
for i, line in enumerate(lines):
    if 'sfx_doframe:' in line:
        start_idx = i
        break

if start_idx is None:
    print("❌ No se encontró sfx_doframe")
    exit(1)

# Buscar el final (línea después del último RTS de sfx_endofeffect)
end_idx = None
for i in range(start_idx, len(lines)):
    if 'STD sfx_pointer        ; Clear pointer' in lines[i]:
        # La línea siguiente debe ser RTS, y la siguiente el final de la sección
        if i + 1 < len(lines) and 'RTS' in lines[i+1]:
            end_idx = i + 2
            break

if end_idx is None:
    print("❌ No se encontró el final de sfx_endofeffect")
    exit(1)

print(f"✓ Encontrado: líneas {start_idx} a {end_idx}")

# Código correcto 1:1 con Richard Chadd original
new_code = '''            sfx_doframe:\\n\\
                LDU sfx_pointer        ; Get current frame pointer\\n\\
                LDB ,U                 ; Read flag byte (NO auto-increment)\\n\\
                CMPB #$D0              ; Check end marker (first byte)\\n\\
                BNE sfx_checktonefreq  ; Not end, continue\\n\\
                LDB 1,U                ; Check second byte at offset 1\\n\\
                CMPB #$20              ; End marker $D0 $20?\\n\\
                BEQ sfx_endofeffect    ; Yes, stop\\n\\
            \\n\\
            sfx_checktonefreq:\\n\\
                LEAY 1,U               ; Y = pointer to tone/noise data\\n\\
                LDB ,U                 ; Reload flag byte (Sound_Byte corrupts B)\\n\\
                BITB #$20              ; Bit 5: tone data present?\\n\\
                BEQ sfx_checknoisefreq ; No, skip tone\\n\\
                ; Set tone frequency (channel C = reg 4/5)\\n\\
                LDB 1,U                ; Get first tone byte (fine tune)\\n\\
                LDA #$04               ; Register 4\\n\\
                JSR Sound_Byte         ; Write to PSG\\n\\
                LDB 2,U                ; Get second tone byte (coarse tune)\\n\\
                LDA #$05               ; Register 5\\n\\
                JSR Sound_Byte         ; Write to PSG\\n\\
                LEAY 2,Y               ; Skip 2 tone bytes\\n\\
            \\n\\
            sfx_checknoisefreq:\\n\\
                LDB ,U                 ; Reload flag byte\\n\\
                BITB #$40              ; Bit 6: noise data present?\\n\\
                BEQ sfx_checkvolume    ; No, skip noise\\n\\
                LDB ,Y                 ; Get noise period\\n\\
                LDA #$06               ; Register 6\\n\\
                JSR Sound_Byte         ; Write to PSG\\n\\
                LEAY 1,Y               ; Skip 1 noise byte\\n\\
            \\n\\
            sfx_checkvolume:\\n\\
                LDB ,U                 ; Reload flag byte\\n\\
                ANDB #$0F              ; Get volume from bits 0-3\\n\\
                LDA #$0A               ; Register 10 (volume C)\\n\\
                JSR Sound_Byte         ; Write to PSG\\n\\
            \\n\\
            sfx_checktonedisable:\\n\\
                LDB ,U                 ; Reload flag byte\\n\\
                BITB #$10              ; Bit 4: disable tone?\\n\\
                BEQ sfx_enabletone\\n\\
            sfx_disabletone:\\n\\
                LDB $C807              ; Read mixer shadow (MUST be B register)\\n\\
                ORB #$04               ; Set bit 2 (disable tone C)\\n\\
                LDA #$07               ; Register 7 (mixer)\\n\\
                JSR Sound_Byte         ; Write to PSG\\n\\
                BRA sfx_checknoisedisable  ; Continue to noise check\\n\\
            \\n\\
            sfx_enabletone:\\n\\
                LDB $C807              ; Read mixer shadow (MUST be B register)\\n\\
                ANDB #$FB              ; Clear bit 2 (enable tone C)\\n\\
                LDA #$07               ; Register 7 (mixer)\\n\\
                JSR Sound_Byte         ; Write to PSG\\n\\
            \\n\\
            sfx_checknoisedisable:\\n\\
                LDB ,U                 ; Reload flag byte\\n\\
                BITB #$80              ; Bit 7: disable noise?\\n\\
                BEQ sfx_enablenoise\\n\\
            sfx_disablenoise:\\n\\
                LDB $C807              ; Read mixer shadow (MUST be B register)\\n\\
                ORB #$20               ; Set bit 5 (disable noise C)\\n\\
                LDA #$07               ; Register 7 (mixer)\\n\\
                JSR Sound_Byte         ; Write to PSG\\n\\
                BRA sfx_nextframe      ; Done, update pointer\\n\\
            \\n\\
            sfx_enablenoise:\\n\\
                LDB $C807              ; Read mixer shadow (MUST be B register)\\n\\
                ANDB #$DF              ; Clear bit 5 (enable noise C)\\n\\
                LDA #$07               ; Register 7 (mixer)\\n\\
                JSR Sound_Byte         ; Write to PSG\\n\\
            \\n\\
            sfx_nextframe:\\n\\
                STY sfx_pointer        ; Update pointer for next frame\\n\\
                RTS\\n\\
            \\n\\
            sfx_endofeffect:\\n\\
                ; Stop SFX - set volume to 0\\n\\
                CLR sfx_status         ; Mark as inactive\\n\\
                LDA #$0A               ; Register 10 (volume C)\\n\\
                LDB #$00               ; Volume = 0\\n\\
                JSR Sound_Byte\\n\\
                LDD #$0000\\n\\
                STD sfx_pointer        ; Clear pointer\\n\\
                RTS\\n\\
            \\n"
'''

# Reemplazar
new_lines = lines[:start_idx] + [new_code] + lines[end_idx:]

with open(file_path, 'w', encoding='utf-8') as f:
    f.writelines(new_lines)

print()
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
print("✅ CÓDIGO AHORA 1:1 CON RICHARD CHADD ORIGINAL")
