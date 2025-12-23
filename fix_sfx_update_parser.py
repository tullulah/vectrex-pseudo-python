#!/usr/bin/env python3
"""Add SFX_UPDATE to is_known_builtin() in parser.rs"""

import re

file_path = "core/src/parser.rs"

with open(file_path, 'r') as f:
    content = f.read()

# Find the is_known_builtin function and add SFX_UPDATE to the list
old_pattern = r'(fn is_known_builtin\(name: &str\) -> bool \{\s*matches!\(name,\s*// Core builtins.*?"WAIT_RECAL" \| "SET_ORIGIN" \| "MUSIC_UPDATE" \| "STOP_MUSIC" \|)'

new_replacement = r'\1\n        "SFX_UPDATE" |'

content_new = re.sub(old_pattern, new_replacement, content, flags=re.DOTALL)

if content_new != content:
    with open(file_path, 'w') as f:
        f.write(content_new)
    print("✓ Added SFX_UPDATE to is_known_builtin() in parser.rs")
else:
    print("⚠️  Pattern not found - checking if SFX_UPDATE already exists...")
    if 'SFX_UPDATE' in content:
        print("✓ SFX_UPDATE already in parser.rs")
    else:
        print("❌ Manual edit required")
