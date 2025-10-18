#!/usr/bin/env python3
"""
Fix all Stmt construction sites after adding line field
"""
import re

with open('core/src/codegen.rs', 'r', encoding='utf-8') as f:
    content = f.read()

original = content

# Fix .clone(, _) → .clone(), line
content = re.sub(r'\.clone\(\s*,\s*_\s*\)', r'.clone(), line', content)

# Fix Stmt::Return(e.clone(, _)) → Stmt::Return(e.clone(), line)
content = re.sub(r'Stmt::Return\(([^,]+)\.clone\(\),\s*line\)', r'Stmt::Return(\1.clone(), line)', content)

# Fix Stmt::Expr(e.clone(), line) patterns
content = re.sub(r'Stmt::Expr\(([^,]+)\.clone\(\),\s*line\)', r'Stmt::Expr(\1.clone(), line)', content)

# Fix cp_expr(e, env, _) → cp_expr(e, env), line
content = re.sub(r'cp_expr\(([^,]+),\s*([^,]+),\s*_\s*\)', r'cp_expr(\1, \2), line', content)

# Fix Stmt constructions missing line field - need to extract line from statement being processed
# Pattern: Stmt::XXX { fields } where parent has access to original stmt with line
fixes = [
    # In flatten_blocks - statements being pushed should preserve line from original
    (r'out\.push\(Stmt::If\s*\{\s*cond:\s*([^,]+),\s*body:\s*([^,]+),\s*elifs:\s*([^,]+),\s*else_body:\s*([^}]+)\s*\}\)', 
     r'out.push(Stmt::If { cond: \1, body: \2, elifs: \3, else_body: \4, line: s.line() })'),
    
    (r'out\.push\(Stmt::While\s*\{\s*cond:\s*([^,]+),\s*body:\s*([^}]+)\s*\}\)',
     r'out.push(Stmt::While { cond: \1, body: \2, line: s.line() })'),
    
    (r'out\.push\(Stmt::For\s*\{\s*var:\s*([^,]+),\s*start:\s*([^,]+),\s*end:\s*([^,]+),\s*step:\s*([^,]+),\s*body:\s*([^}]+)\s*\}\)',
     r'out.push(Stmt::For { var: \1, start: \2, end: \3, step: \4, body: \5, line: s.line() })'),
    
    (r'out\.push\(Stmt::Switch\s*\{\s*expr:\s*([^,]+),\s*cases:\s*([^,]+),\s*default:\s*([^}]+)\s*\}\)',
     r'out.push(Stmt::Switch { expr: \1, cases: \2, default: \3, line: s.line() })'),
    
    (r'out\.push\(Stmt::Assign\s*\{\s*target:\s*([^,]+),\s*value:\s*([^}]+)\s*\}\)',
     r'out.push(Stmt::Assign { target: \1, value: \2, line: s.line() })'),
    
    (r'out\.push\(Stmt::Let\s*\{\s*name:\s*([^,]+),\s*value:\s*([^}]+)\s*\}\)',
     r'out.push(Stmt::Let { name: \1, value: \2, line: s.line() })'),
]

for pattern, replacement in fixes:
    content = re.sub(pattern, replacement, content)

# For cp_stmt function - similar pattern
content = re.sub(
    r'Stmt::Assign\s*\{\s*target:\s*target\.clone\(\),\s*value:\s*v2\s*\}',
    r'Stmt::Assign { target: target.clone(), value: v2, line: s.line() }',
    content
)

content = re.sub(
    r'Stmt::Let\s*\{\s*name:\s*name\.clone\(\),\s*value:\s*v2\s*\}',
    r'Stmt::Let { name: name.clone(), value: v2, line: s.line() }',
    content
)

# Fix Stmt::Expr(cp_expr(e, env), line) pattern
content = re.sub(r'Stmt::Expr\(cp_expr\(([^,]+),\s*([^)]+)\),\s*line\)', r'Stmt::Expr(cp_expr(\1, \2), line)', content)

with open('core/src/codegen.rs', 'w', encoding='utf-8') as f:
    f.write(content)

if content != original:
    print("✓ Fixed Stmt constructions in codegen.rs")
    print(f"  Total changes: {len(original) - len(content)} bytes")
else:
    print("No changes made")
