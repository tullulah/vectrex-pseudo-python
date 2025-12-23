#!/usr/bin/env python3
"""Fix SFX to reload flag byte before each BITB (Richard Chadd original behavior)"""

file_path = "core/src/backend/m6809/emission.rs"

with open(file_path, 'r') as f:
    content = f.read()

# Fix: Add LDB ,U before each BITB (flag byte gets corrupted by Sound_Byte)
# The original ALWAYS reloads the flag before checking bits

# 1. After sfx_checktonefreq label (first BITB)
old1 = '''            sfx_checktonefreq:\\n\\
                LEAY 1,U               ; Y = pointer to next data/flag\\n\\
                BITB #$20              ; Bit 5: tone data present?\\n\\'''

new1 = '''            sfx_checktonefreq:\\n\\
                LEAY 1,U               ; Y = pointer to next data/flag\\n\\
                LDB ,U                 ; Reload flag byte (Sound_Byte corrupts B)\\n\\
                BITB #$20              ; Bit 5: tone data present?\\n\\'''

content = content.replace(old1, new1)

# 2. Before sfx_checknoisefreq BITB
old2 = '''            sfx_checknoisefreq:\\n\\
                BITB #$40              ; Bit 6: noise data present?\\n\\'''

new2 = '''            sfx_checknoisefreq:\\n\\
                LDB ,U                 ; Reload flag byte\\n\\
                BITB #$40              ; Bit 6: noise data present?\\n\\'''

content = content.replace(old2, new2)

# 3. Before sfx_checktonedisable BITB
old3 = '''            sfx_checktonedisable:\\n\\
                LDA $C807              ; Read mixer (reg 7 shadow)\\n\\
                BITB #$10              ; Bit 4: disable tone?\\n\\'''

new3 = '''            sfx_checktonedisable:\\n\\
                LDB ,U                 ; Reload flag byte\\n\\
                LDA $C807              ; Read mixer (reg 7 shadow)\\n\\
                BITB #$10              ; Bit 4: disable tone?\\n\\'''

content = content.replace(old3, new3)

# 4. In sfx_disabletone before second BITB
old4 = '''            sfx_disabletone:\\n\\
                ORA #$04               ; Set bit 2 (disable tone C)\\n\\
                BITB #$80              ; Bit 7: disable noise?\\n\\'''

new4 = '''            sfx_disabletone:\\n\\
                ORA #$04               ; Set bit 2 (disable tone C)\\n\\
                LDB ,U                 ; Reload flag byte\\n\\
                BITB #$80              ; Bit 7: disable noise?\\n\\'''

content = content.replace(old4, new4)

# 5. In sfx_enabletone before BITB
old5 = '''            sfx_enabletone:\\n\\
                ANDA #$FB              ; Clear bit 2 (enable tone C)\\n\\
                BITB #$80              ; Bit 7: disable noise?\\n\\'''

new5 = '''            sfx_enabletone:\\n\\
                ANDA #$FB              ; Clear bit 2 (enable tone C)\\n\\
                LDB ,U                 ; Reload flag byte\\n\\
                BITB #$80              ; Bit 7: disable noise?\\n\\'''

content = content.replace(old5, new5)

# 6. In sfx_checknoisedisable before BITB (this one is missing entirely!)
# Actually sfx_checknoisedisable doesn't exist in our code, it's merged with enablenoise
# This is handled differently - the original has a separate check

with open(file_path, 'w') as f:
    f.write(content)

print("✓ Fixed SFX to reload flag byte before each BITB")
print("  - Original behavior: LDB ,U before EVERY BITB")
print("  - Reason: Sound_Byte corrupts B register")
print("  - Added 5 LDB ,U instructions")
