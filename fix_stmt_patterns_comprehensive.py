#!/usr/bin/env python3
"""
Comprehensive fix for Stmt patterns - Phase 1
Fixes most common errors automatically
"""
import re

def fix_file(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original = content
    
    # 1. Fix Break/Continue as unit variants → struct variants
    content = re.sub(r'\|\s*Stmt::Break(?!\s*\{)', r'| Stmt::Break { .. }', content)
    content = re.sub(r'\|\s*Stmt::Continue(?!\s*\{)', r'| Stmt::Continue { .. }', content)
    content = re.sub(r'==\s*Stmt::Break(?!\s*\{)', r'matches!(*_stmt, Stmt::Break { .. })', content)
    content = re.sub(r'==\s*Stmt::Continue(?!\s*\{)', r'matches!(*_stmt, Stmt::Continue { .. })', content)
    
    # 2. Fix tuple variant patterns (only in match arms, not constructions)
    # Look for patterns like: Stmt::Expr(x) => or Stmt::Return(x) =>
    content = re.sub(r'Stmt::Expr\(([^,)]+)\)\s*=>', r'Stmt::Expr(\1, _) =>', content)
    content = re.sub(r'Stmt::Return\(([^,)]+)\)\s*=>', r'Stmt::Return(\1, _) =>', content)
    
    # 3. Add .. to struct patterns that don't have it
    def add_ellipsis(match):
        stmt_type = match.group(1)
        fields = match.group(2).strip()
        
        # Skip if already has .. or source_line
        if '..' in fields or 'source_line' in fields:
            return match.group(0)
        
        # Don't add if it's a construction (has values being assigned)
        if ':' in fields and not fields.endswith(','):
            # This looks like a construction: { field: value }
            # vs a pattern: { field, other_field }
            return match.group(0)
        
        # Add ..
        return f'Stmt::{stmt_type} {{ {fields}, .. }}'
    
    # Apply to common Stmt variants
    for variant in ['Assign', 'Let', 'For', 'While', 'If', 'Switch', 'CompoundAssign']:
        pattern = rf'Stmt::{variant}\s*\{{([^}}]+)\}}'
        
        def add_ellipsis_for_variant(match):
            fields = match.group(1).strip()
            
            # Skip if already has .. or source_line
            if '..' in fields or 'source_line' in fields:
                return match.group(0)
            
            # Don't add if it's a construction (has values being assigned)
            if ':' in fields and not fields.endswith(','):
                # This looks like a construction: { field: value }
                # vs a pattern: { field, other_field }
                return match.group(0)
            
            # Add ..
            return f'Stmt::{variant} {{ {fields}, .. }}'
        
        content = re.sub(pattern, add_ellipsis_for_variant, content)
    
    # 4. Fix obvious syntax errors from previous scripts
    # .as_ref(, _) → .as_ref()
    content = re.sub(r'\.as_ref\(\s*,\s*_\s*\)', r'.as_ref()', content)
    
    # opt_expr(e, _) → opt_expr(e)  
    content = re.sub(r'opt_expr\(([^,)]+),\s*_\s*\)', r'opt_expr(\1)', content)
    
    # cp_expr(e, env, _) → cp_expr(e, env)
    content = re.sub(r'cp_expr\(([^,]+),\s*([^,]+),\s*_\s*\)', r'cp_expr(\1, \2)', content)
    
    # Write back
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(content)
    
    if content != original:
        return True
    return False

# Fix all relevant files
files = [
    'core/src/codegen.rs',
    'core/src/backend/m6809.rs',
    'core/src/backend/cortexm.rs',
    'core/src/backend/arm.rs',
]

fixed_count = 0
for filepath in files:
    try:
        if fix_file(filepath):
            print(f"✓ Fixed {filepath}")
            fixed_count += 1
    except FileNotFoundError:
        pass  # Skip files that don't exist

print(f"\n✓ Fixed {fixed_count} files")
print("\nNext: Run 'cargo build' to see remaining errors")
