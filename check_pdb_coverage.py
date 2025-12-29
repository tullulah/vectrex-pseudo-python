#!/usr/bin/env python3
"""
Script para comparar líneas con código en .vpy vs líneas mapeadas en .pdb
"""
import json
import re
import sys

def extract_pdb_lines(pdb_path):
    """Extrae números de línea del lineMap en el .pdb"""
    with open(pdb_path, 'r') as f:
        pdb_data = json.load(f)
    
    line_map = pdb_data.get('lineMap', {})
    # Las claves pueden venir como strings (JSON) o números, normalizar a ints
    pdb_lines = set()
    for key in line_map.keys():
        try:
            pdb_lines.add(int(key))
        except (ValueError, TypeError):
            pass
    return pdb_lines

def extract_vpy_code_lines(vpy_path):
    """Extrae números de línea que contienen código en el .vpy"""
    code_lines = set()
    
    with open(vpy_path, 'r', encoding='utf-8') as f:
        for line_num, line in enumerate(f, start=1):
            stripped = line.strip()
            
            # Ignorar líneas vacías y comentarios puros
            if not stripped:
                continue
            if stripped.startswith('#'):
                continue
            
            # Líneas con código (incluso si tienen comentario al final)
            code_lines.add(line_num)
    
    return code_lines

def analyze_coverage(vpy_path, pdb_path):
    """Analiza qué líneas de código NO están en el .pdb"""
    vpy_lines = extract_vpy_code_lines(vpy_path)
    pdb_lines = extract_pdb_lines(pdb_path)
    
    missing_lines = sorted(vpy_lines - pdb_lines)
    
    print(f"📊 Análisis de cobertura .pdb")
    print(f"{'='*60}")
    print(f"Total líneas con código en .vpy: {len(vpy_lines)}")
    print(f"Total líneas mapeadas en .pdb:  {len(pdb_lines)}")
    print(f"Líneas FALTANTES en .pdb:       {len(missing_lines)}")
    print(f"Cobertura: {len(pdb_lines)/len(vpy_lines)*100:.1f}%")
    print()
    
    if missing_lines:
        print(f"❌ Líneas con código NO mapeadas en .pdb:")
        print(f"{'='*60}")
        
        # Agrupar líneas consecutivas en rangos
        ranges = []
        start = missing_lines[0]
        end = start
        
        for line in missing_lines[1:]:
            if line == end + 1:
                end = line
            else:
                ranges.append((start, end))
                start = line
                end = line
        ranges.append((start, end))
        
        # Imprimir rangos
        for start, end in ranges:
            if start == end:
                print(f"  Línea {start}")
            else:
                print(f"  Líneas {start}-{end} ({end-start+1} líneas)")
        
        print()
        
        # Mostrar contenido de líneas faltantes
        print(f"📝 Contenido de líneas faltantes:")
        print(f"{'='*60}")
        with open(vpy_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
            
            for start, end in ranges[:5]:  # Mostrar primeros 5 rangos
                if start == end:
                    print(f"\n{start}: {lines[start-1].rstrip()}")
                else:
                    print(f"\n--- Rango {start}-{end} ---")
                    for i in range(start, min(end+1, start+5)):  # Max 5 líneas por rango
                        print(f"{i}: {lines[i-1].rstrip()}")
                    if end - start > 4:
                        print(f"  ... (+{end-start-4} líneas más)")
    else:
        print("✅ Todas las líneas con código están mapeadas en .pdb")
    
    return missing_lines

if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Uso: python check_pdb_coverage.py <archivo.vpy> <archivo.pdb>")
        sys.exit(1)
    
    vpy_path = sys.argv[1]
    pdb_path = sys.argv[2]
    
    analyze_coverage(vpy_path, pdb_path)
