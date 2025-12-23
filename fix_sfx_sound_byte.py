#!/usr/bin/env python3
"""Fix SFX to match Richard Chadd's original code exactly"""

file_path = "core/src/backend/m6809/emission.rs"

with open(file_path, 'r') as f:
    content = f.read()

# Find and replace tone frequency code
old_tone = '''                ; Set tone frequency (channel C = reg 4/5)\\n\\
                LDA 2,U                ; Low byte (FIXED: was 1,U)\\n\\
                PSHS A\\n\\
                LDA #$04               ; Register 4 (tone C low)\\n\\
                JSR Sound_Byte\\n\\
                PULS A\\n\\
                LDB 1,U                ; High byte (FIXED: was 2,U)\\n\\
                PSHS B\\n\\
                LDA #$05               ; Register 5 (tone C high)\\n\\
                LDB 1,U\\n\\
                JSR Sound_Byte\\n\\
                PULS B\\n\\
                LEAY 2,Y               ; Skip 2 bytes\\n\\'''

new_tone = '''                ; Set tone frequency (channel C = reg 4/5)\\n\\
                LDB 1,U                ; Get first data byte\\n\\
                LDA #$04               ; Register 4\\n\\
                JSR Sound_Byte         ; Set tone freq\\n\\
                LDB 2,U                ; Get second data byte\\n\\
                LDA #$05               ; Register 5\\n\\
                JSR Sound_Byte         ; Set tone freq\\n\\
                LEAY 2,Y               ; Skip 2 bytes\\n\\'''

content = content.replace(old_tone, new_tone)

# Find and replace noise code (also has unnecessary push/pull)
old_noise = '''                BEQ sfx_checkvolume    ; No, skip noise\\n\\
                LDA ,Y                 ; Get noise period\\n\\
                PSHS A\\n\\
                LDA #$06               ; Register 6 (noise period)\\n\\
                LDB ,Y\\n\\
                JSR Sound_Byte\\n\\
                PULS A\\n\\
                LEAY 1,Y               ; Skip 1 byte\\n\\'''

new_noise = '''                BEQ sfx_checkvolume    ; No, skip noise\\n\\
                LDB ,Y                 ; Get noise period\\n\\
                LDA #$06               ; Register 6\\n\\
                JSR Sound_Byte         ; Set noise freq\\n\\
                LEAY 1,Y               ; Skip 1 byte\\n\\'''

content = content.replace(old_noise, new_noise)

# Find and replace volume code
old_volume = '''            sfx_checkvolume:\\n\\
                TFR B,A                ; Get flag byte in A\\n\\
                ANDA #$0F              ; Extract volume (bits 0-3)\\n\\
                PSHS A\\n\\
                LDA #$0A               ; Register 10 (volume C)\\n\\
                LDB ,S+                ; Get volume\\n\\
                JSR Sound_Byte\\n\\'''

new_volume = '''            sfx_checkvolume:\\n\\
                LDB ,U                 ; Set volume on channel 3\\n\\
                ANDB #$0F              ; Get volume from bits 0-3\\n\\
                LDA #$0A               ; Register 10\\n\\
                JSR Sound_Byte         ; Set volume\\n\\'''

content = content.replace(old_volume, new_volume)

with open(file_path, 'w') as f:
    f.write(content)

print("✓ Fixed SFX code to match Richard Chadd original")
print("  - Removed unnecessary PSHS/PULS")
print("  - Sound_Byte convention: A=register, B=value")
print("  - Tone: LDB 1,U / LDB 2,U (not LDA)")
print("  - Noise: LDB ,Y (not LDA)")
print("  - Volume: LDB ,U + ANDB (not TFR + ANDA)")
