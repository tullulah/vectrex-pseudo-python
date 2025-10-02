#!/usr/bin/env python3
"""
Script para agregar stubs temporales para las nuevas variantes del AST
"""

import re
import os

def add_missing_patterns(file_path):
    """Agrega patrones faltantes a archivos Rust"""
    
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Patrones para match statements que necesitan stubs
    
    # 1. Para match de Item que necesita Struct
    item_pattern = r'(match it \{[^}]*?)(Item::ExprStatement\([^)]*\)[^,]*,?)(\s*\})'
    def item_replacement(m):
        before, last_item, after = m.groups()
        if 'Item::Struct' not in before:
            return f"{before}{last_item}\n            Item::Struct {{ .. }} => todo!(),{after}"
        return m.group(0)
    
    content = re.sub(item_pattern, item_replacement, content, flags=re.DOTALL)
    
    # 2. Para match de Stmt que necesita CompoundAssign
    stmt_pattern = r'(match stmt? \{[^}]*?)(Stmt::(Break \| Stmt::Continue|Return\([^)]*\)|Switch \{[^}]*\})[^,]*,?)(\s*\})'
    def stmt_replacement(m):
        before, last_stmt, _, after = m.groups()
        if 'CompoundAssign' not in before:
            return f"{before}{last_stmt},\n        Stmt::CompoundAssign {{ .. }} => todo!(),{after}"
        return m.group(0)
    
    content = re.sub(stmt_pattern, stmt_replacement, content, flags=re.DOTALL)
    
    # 3. Para match de Expr que necesita Array, ArrayAccess, FieldAccess, StructLiteral
    expr_pattern = r'(match e(?:xpr)? \{[^}]*?)(Expr::(Number\([^)]*\)|StringLit\([^)]*\))[^,]*,?)(\s*\})'
    def expr_replacement(m):
        before, last_expr, _, after = m.groups()
        if 'Array(' not in before and 'ArrayAccess' not in before:
            new_cases = """        Expr::Array(_) => todo!(),
        Expr::ArrayAccess { .. } => todo!(),
        Expr::FieldAccess { .. } => todo!(),
        Expr::StructLiteral { .. } => todo!(),"""
            return f"{before}{last_expr},\n{new_cases}{after}"
        return m.group(0)
    
    content = re.sub(expr_pattern, expr_replacement, content, flags=re.DOTALL)
    
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(content)
    
    print(f"Updated {file_path}")

def main():
    # Lista de archivos a actualizar
    files_to_update = [
        'core/src/codegen.rs',
        'core/src/backend/m6809.rs',
        'core/src/backend/arm.rs',
        'core/src/backend/cortexm.rs',
        'core/src/backend/string_literals.rs'
    ]
    
    base_path = r'C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python'
    
    for file_rel in files_to_update:
        file_path = os.path.join(base_path, file_rel)
        if os.path.exists(file_path):
            add_missing_patterns(file_path)
        else:
            print(f"File not found: {file_path}")

if __name__ == "__main__":
    main()