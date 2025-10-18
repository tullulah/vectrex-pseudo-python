#!/usr/bin/env python3
"""
Fix Stmt pattern matching after adding line field to all variants
"""
import re

# Read codegen.rs
with open('core/src/codegen.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# Patterns to fix (order matters - most specific first)
replacements = [
    # Specific patterns with all fields
    (r'Stmt::Assign\s*\{\s*target,\s*value\s*\}', r'Stmt::Assign { target, value, .. }'),
    (r'Stmt::Let\s*\{\s*name,\s*value\s*\}', r'Stmt::Let { name, value, .. }'),
    (r'Stmt::For\s*\{\s*var,\s*start,\s*end,\s*step,\s*body\s*\}', r'Stmt::For { var, start, end, step, body, .. }'),
    (r'Stmt::While\s*\{\s*cond,\s*body\s*\}', r'Stmt::While { cond, body, .. }'),
    (r'Stmt::If\s*\{\s*cond,\s*body,\s*elifs,\s*else_body\s*\}', r'Stmt::If { cond, body, elifs, else_body, .. }'),
    (r'Stmt::Switch\s*\{\s*expr,\s*cases,\s*default\s*\}', r'Stmt::Switch { expr, cases, default, .. }'),
    (r'Stmt::CompoundAssign\s*\{\s*target,\s*op,\s*value\s*\}', r'Stmt::CompoundAssign { target, op, value, .. }'),
    
    # Enum variants without fields (become structs with line field)
    (r'Stmt::Break(?!\s*\{)', r'Stmt::Break { .. }'),
    (r'Stmt::Continue(?!\s*\{)', r'Stmt::Continue { .. }'),
    
    # Tuple variants
    (r'Stmt::Return\(([^)]+)\)(?!\s*,\s*_)', r'Stmt::Return(\1, _)'),
    (r'Stmt::Expr\(([^)]+)\)(?!\s*,\s*_)', r'Stmt::Expr(\1, _)'),
]

original = content
for pattern, replacement in replacements:
    content = re.sub(pattern, replacement, content)

# Write back
with open('core/src/codegen.rs', 'w', encoding='utf-8') as f:
    f.write(content)

# Report changes
if content != original:
    print("✓ Fixed Stmt patterns in codegen.rs")
    # Count matches
    for pattern, _ in replacements:
        count = len(re.findall(pattern, original))
        if count > 0:
            print(f"  - Fixed {count} occurrences of: {pattern[:50]}...")
else:
    print("No changes needed")
