#!/usr/bin/env python3
"""
Script para actualizar todos los pattern matches de Stmt en el código
para incluir el campo `line` añadido.
"""

import re
import sys

def update_file(filepath):
    """Actualiza un archivo con los nuevos patrones de Stmt"""
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original = content
    
    # Patrones a actualizar (sin .. al final)
    replacements = [
        # Break y Continue - cambiar de unit a struct variant
        (r'\bStmt::Break\b(?!\s*\{)', 'Stmt::Break { .. }'),
        (r'\bStmt::Continue\b(?!\s*\{)', 'Stmt::Continue { .. }'),
        
        # Return - cambiar de tuple a struct variant
        (r'Stmt::Return\(([^)]+)\)', r'Stmt::Return { value: \1, .. }'),
        
        # While, For, If, Switch, Let - añadir .. si no lo tienen
        (r'Stmt::While\s*\{\s*cond\s*,\s*body\s*\}', 'Stmt::While { cond, body, .. }'),
        (r'Stmt::For\s*\{\s*var\s*,\s*start\s*,\s*end\s*,\s*step\s*,\s*body\s*\}', 'Stmt::For { var, start, end, step, body, .. }'),
        (r'Stmt::If\s*\{\s*cond\s*,\s*body\s*,\s*elifs\s*,\s*else_body\s*\}', 'Stmt::If { cond, body, elifs, else_body, .. }'),
        (r'Stmt::Switch\s*\{\s*expr\s*,\s*cases\s*,\s*default\s*\}', 'Stmt::Switch { expr, cases, default, .. }'),
    ]
    
    for pattern, replacement in replacements:
        content = re.sub(pattern, replacement, content)
    
    # Casos especiales en construcción de Stmt
    # Estos necesitan el campo `line` explícito
    # Ejemplo: Stmt::Let { name: name.clone(), value: opt_expr(value) }
    # Debe ser: Stmt::Let { name: name.clone(), value: opt_expr(value), line: *line }
    
    if content != original:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        return True
    return False

if __name__ == '__main__':
    files = [
        'core/src/backend/m6809.rs',
        'core/src/codegen.rs',
    ]
    
    for f in files:
        try:
            if update_file(f):
                print(f"Updated: {f}")
            else:
                print(f"No changes: {f}")
        except Exception as e:
            print(f"Error updating {f}: {e}", file=sys.stderr)
