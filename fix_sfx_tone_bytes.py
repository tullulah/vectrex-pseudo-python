#!/usr/bin/env python3
"""Fix SFX tone frequency byte order in emission.rs"""

file_path = "core/src/backend/m6809/emission.rs"

with open(file_path, 'r') as f:
    content = f.read()

# Find and replace the tone frequency code
old_code = '''                BEQ sfx_checknoisefreq ; No, skip tone\\n\\
                ; Set tone frequency (channel C = reg 4/5)\\n\\
                LDA 1,U                ; High byte\\n\\
                PSHS A\\n\\
                LDA #$04               ; Register 4 (tone C low)\\n\\
                JSR Sound_Byte\\n\\
                PULS A\\n\\
                LDB 2,U                ; Low byte\\n\\
                PSHS B\\n\\
                LDA #$05               ; Register 5 (tone C high)\\n\\
                LDB 2,U\\n\\
                JSR Sound_Byte\\n\\
                PULS B\\n\\
                LEAY 2,Y               ; Skip 2 bytes\\n\\'''

new_code = '''                BEQ sfx_checknoisefreq ; No, skip tone\\n\\
                ; Set tone frequency (channel C = reg 4/5)\\n\\
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

content_new = content.replace(old_code, new_code)

if content_new != content:
    with open(file_path, 'w') as f:
        f.write(content_new)
    print("✓ Fixed SFX tone frequency byte order")
    print("  - Register 4 (low) now gets LOW byte from offset 2,U")
    print("  - Register 5 (high) now gets HIGH byte from offset 1,U")
else:
    print("⚠️  Pattern not found or already fixed")
