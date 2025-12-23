#!/usr/bin/env python3
"""Comparación completa línea por línea del código AYFX"""

# Original de Richard Chadd (vide/template/ayfxPlayer.i)
original = """
sfx_doframe: 
    LDU      sfx_pointer
    LDB      ,U
    CMPB     #$D0
    BNE      sfx_checktonefreq
    LDB      1,U
    CMPB     #$20
    BEQ      sfx_endofeffect
sfx_checktonefreq: 
    LEAY     1,U
    LDB      ,U
    BITB     #%00100000
    BEQ      sfx_checknoisefreq
    LDB      1,U
    LDA      #$04
    JSR      Sound_Byte
    LDB      2,U
    LDA      #$05
    JSR      Sound_Byte
    LEAY     2,Y
sfx_checknoisefreq: 
    LDB      ,U
    BITB     #%01000000
    BEQ      sfx_checkvolume
    LDB      ,Y
    LDA      #$06
    JSR      Sound_Byte
    LEAY     1,Y
sfx_checkvolume: 
    LDB      ,U
    ANDB     #%00001111
    LDA      #$0A
    JSR      Sound_Byte
sfx_checktonedisable: 
    LDB      ,U
    BITB     #%00010000
    BEQ      sfx_enabletone
sfx_disabletone: 
    LDB      $C807
    ORB      #%00000100
    LDA      #$07
    JSR      Sound_Byte
    BRA      sfx_checknoisedisable
sfx_enabletone: 
    LDB      $C807
    ANDB     #%11111011
    LDA      #$07
    JSR      Sound_Byte
sfx_checknoisedisable: 
    LDB      ,U
    BITB     #%10000000
    BEQ      sfx_enablenoise
sfx_disablenoise: 
    LDB      $C807
    ORB      #%00100000
    LDA      #$07
    JSR      Sound_Byte
    BRA      sfx_nextframe
sfx_enablenoise: 
    LDB      $C807
    ANDB     #%11011111
    LDA      #$07
    JSR      Sound_Byte
sfx_nextframe: 
    STY      sfx_pointer
    RTS
sfx_endofeffect: 
    LDB      #$00
    LDA      #$0A
    JSR      Sound_Byte
    LDD      #$0000
    STD      sfx_pointer
    STA      sfx_status
    RTS
""".strip().split('\n')

# Nuestro código (leer del ASM generado)
import subprocess
result = subprocess.run(['sed', '-n', '208,310p', 'examples/sfx_buttons/src/test_minimal_sfx.asm'], 
                       capture_output=True, text=True)
nuestro = result.stdout.strip().split('\n')

print("=" * 80)
print("COMPARACIÓN COMPLETA LÍNEA POR LÍNEA")
print("=" * 80)
print()

differences = []
line_num = 0

# Normalizar y comparar
for i, (orig, nuestro_line) in enumerate(zip(original, nuestro[:len(original)]), 1):
    orig_clean = orig.strip().replace('%', '$').replace('  ', ' ')
    nuestro_clean = nuestro_line.strip()
    
    if orig_clean != nuestro_clean and orig_clean and nuestro_clean:
        differences.append((i, orig_clean, nuestro_clean))

print(f"Total de diferencias encontradas: {len(differences)}")
print()

for num, orig, nuestro_val in differences:
    print(f"Línea {num}:")
    print(f"  ORIGINAL: {orig}")
    print(f"  NUESTRO:  {nuestro_val}")
    print()

print("=" * 80)
print("RESUMEN DE PROBLEMAS:")
print("=" * 80)
problems = []
for num, orig, nuestro_val in differences:
    if 'LDB ,U+' in nuestro_val and 'LDB ,U' in orig:
        problems.append(f"- Línea {num}: Usa ,U+ en lugar de ,U (incremento innecesario)")
    elif 'LDA ,U' in nuestro_val and 'LDB' in orig:
        problems.append(f"- Línea {num}: Usa LDA en lugar de LDB")
    elif 'CMPA' in nuestro_val and 'CMPB' in orig:
        problems.append(f"- Línea {num}: Usa CMPA en lugar de CMPB")
    elif 'ORA' in nuestro_val and 'ORB' in orig:
        problems.append(f"- Línea {num}: Usa ORA en lugar de ORB")
    elif 'ANDA' in nuestro_val and 'ANDB' in orig:
        problems.append(f"- Línea {num}: Usa ANDA en lugar de ANDB")
    elif 'PSHS A' in nuestro_val and 'PSHS' not in orig:
        problems.append(f"- Línea {num}: PSHS/PULS innecesario")
    elif 'BRA sfx_checknoisedisable' in orig and 'BRA' not in nuestro_val:
        problems.append(f"- Línea {num}: Falta BRA sfx_checknoisedisable")
    elif 'sfx_checknoisedisable:' in orig and 'sfx_checknoisedisable' not in nuestro_val:
        problems.append(f"- Línea {num}: Falta label sfx_checknoisedisable")

for p in set(problems):
    print(p)
