#!/usr/bin/env python3
"""Fix byte order for tone data: big-endian data → little-endian PSG"""

with open('core/src/backend/m6809/emission.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# Buscar y reemplazar la sección de tone
# Los datos están en formato big-endian: FCB high, low
# Pero PSG necesita little-endian: reg 4=low, reg 5=high

old_pattern = "LDB 1,U                ; Get first tone byte (fine tune)"
new_pattern = "LDB 2,U                ; Get LOW byte (fine tune)"

if old_pattern in content:
    # Reemplazar primero el comentario del primer byte
    content = content.replace(
        "LDB 1,U                ; Get first tone byte (fine tune)",
        "LDB 2,U                ; Get LOW byte (fine tune)"
    )
    
    # Reemplazar el comentario del segundo byte
    content = content.replace(
        "LDB 2,U                ; Get second tone byte (coarse tune)",
        "LDB 1,U                ; Get HIGH byte (coarse tune)"
    )
    
    print("✅ FIXED: Tone byte order corrected")
    print("   Data format: big-endian (high, low)")
    print("   PSG expects: little-endian (low=reg4, high=reg5)")
    print("   Change: 1,U → 2,U (for register 4)")
    print("           2,U → 1,U (for register 5)")
else:
    print("❌ Pattern not found")

with open('core/src/backend/m6809/emission.rs', 'w', encoding='utf-8') as f:
    f.write(content)
