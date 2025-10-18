#!/usr/bin/env python3
"""
Comprehensive fix for all Stmt-related compilation errors
"""
import re

# Read file
with open('core/src/codegen.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# Fix 1: Stmt::Break and Stmt::Continue comparisons (== Stmt::Break is invalid, use matches!)
content = re.sub(r'==\s*Stmt::Break\b', r'matches!(*stmt, Stmt::Break { .. })', content)
content = re.sub(r'==\s*Stmt::Continue\b', r'matches!(*stmt, Stmt::Continue { .. })', content)
content = re.sub(r'\|\s*Stmt::Break\b(?!\s*\{)', r'| Stmt::Break { .. }', content)
content = re.sub(r'\|\s*Stmt::Continue\b(?!\s*\{)', r'| Stmt::Continue { .. }', content)

# Fix 2: Tuple variant patterns missing second argument (e.g., Stmt::Expr(e) should be Stmt::Expr(e, _))
# Already done in previous script, but let's be thorough
content = re.sub(r'Stmt::Expr\(([^,)]+)\)(?!\s*,\s*_)(?![^)]*=>)', r'Stmt::Expr(\1, _)', content)
content = re.sub(r'Stmt::Return\(([^,)]+)\)(?!\s*,\s*_)(?![^)]*=>)', r'Stmt::Return(\1, _)', content)

# Fix 3: Constructions that still use "line" instead of "source_line"
content = re.sub(r'Stmt::(\w+)\s*\{([^}]*?)\bline:', r'Stmt::\1 {\2source_line:', content)

# Fix 4: Variable "line" that conflicts with line! macro - should be source_line
# But preserve cases where it's actually intentional (like file line numbers for debugging)
# Only in statement-related contexts

# Fix 5: Missing ".." in patterns
# This is tricky - need to add .. to patterns that don't have it
def add_ellipsis_if_missing(match):
    content = match.group(1)
    if '..' not in content and 'source_line' not in content:
        # Add .. before closing brace
        return match.group(0)[:-1] + ', .. }'
    return match.group(0)

content = re.sub(r'Stmt::Assign\s*\{([^}]+)\}', add_ellipsis_if_missing, content)
content = re.sub(r'Stmt::Let\s*\{([^}]+)\}', add_ellipsis_if_missing, content)
content = re.sub(r'Stmt::For\s*\{([^}]+)\}', add_ellipsis_if_missing, content)
content = re.sub(r'Stmt::While\s*\{([^}]+)\}', add_ellipsis_if_missing, content)
content = re.sub(r'Stmt::If\s*\{([^}]+)\}', add_ellipsis_if_missing, content)
content = re.sub(r'Stmt::Switch\s*\{([^}]+)\}', add_ellipsis_if_missing, content)
content = re.sub(r'Stmt::CompoundAssign\s*\{([^}]+)\}', add_ellipsis_if_missing, content)

# Write back
with open('core/src/codegen.rs', 'w', encoding='utf-8') as f:
    f.write(content)

print("✓ Applied comprehensive fixes to codegen.rs")
