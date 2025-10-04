#!/usr/bin/env python3
"""
Limpia archivos de test generados - remueve mod tests wrapper innecesario
"""

from pathlib import Path
import re

BASE_DIR = Path('emulator_v2/tests/opcodes')

def clean_test_file(file_path):
    """Remueve mod tests wrapper y arregla estructura"""
    content = file_path.read_text(encoding='utf-8')
    
    # Detectar si tiene mod tests wrapper
    if 'mod tests {' not in content:
        return False  # Ya está limpio
    
    # Remover el mod tests wrapper
    # Patrón: #[cfg(test)]\nmod tests {\n    use super::*;\n
    content = re.sub(r'#\[cfg\(test\)\]\s*\nmod tests \{\s*\n\s*use super::\*;\s*\n', '', content, flags=re.MULTILINE)
    
    # Remover líneas que solo contienen }
    lines = content.split('\n')
    cleaned_lines = []
    
    for line in lines:
        # Saltar líneas que solo tienen } (cierre del mod tests)
        if line.strip() == '}' and len(cleaned_lines) > 0:
            # Verificar si es el cierre de una función o del mod
            # Si la línea anterior no es una llave de cierre, probablemente sea del mod
            prev_stripped = cleaned_lines[-1].strip() if cleaned_lines else ''
            if not prev_stripped or prev_stripped.startswith('#[test]') or prev_stripped == '':
                continue  # Saltar este cierre (es del mod tests)
        
        cleaned_lines.append(line)
    
    # Eliminar líneas vacías excesivas al final
    while cleaned_lines and cleaned_lines[-1].strip() == '':
        cleaned_lines.pop()
    
    new_content = '\n'.join(cleaned_lines)
    
    # Escribir de vuelta
    file_path.write_text(new_content, encoding='utf-8')
    return True

def main():
    print("=" * 80)
    print("LIMPIEZA DE ARCHIVOS DE TEST GENERADOS")
    print("=" * 80)
    
    total_files = 0
    cleaned_files = 0
    
    for rs_file in BASE_DIR.rglob('test_*.rs'):
        total_files += 1
        if clean_test_file(rs_file):
            cleaned_files += 1
            print(f"OK {rs_file.relative_to(BASE_DIR)}")
    
    print("\n" + "=" * 80)
    print(f"Limpieza completada!")
    print(f"   Archivos procesados: {total_files}")
    print(f"   Archivos limpiados: {cleaned_files}")
    print("=" * 80)

if __name__ == '__main__':
    main()
