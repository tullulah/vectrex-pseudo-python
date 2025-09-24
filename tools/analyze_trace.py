# Script Python para analizar trace data desde WASM
# Usar cuando se tenga acceso directo al emulador Rust/WASM

import json
import re
from collections import Counter

def analyze_trace_json(trace_json_str):
    """Analiza trace data en formato JSON del emulador"""
    
    print("=== ANÁLISIS DE TRACE DATA WASM ===")
    
    try:
        trace_data = json.loads(trace_json_str)
    except json.JSONDecodeError as e:
        print(f"Error parsing JSON: {e}")
        return
    
    if not trace_data:
        print("No hay datos de trace")
        return
    
    print(f"Total de entradas en trace: {len(trace_data)}")
    
    # Analizar primeras 50 entradas
    print("\n=== PRIMERAS 50 ENTRADAS (inicio BIOS) ===")
    for i, entry in enumerate(trace_data[:50]):
        pc = f"{entry['pc']:04X}"
        op_str = entry.get('m', 'UNK')
        hex_bytes = entry.get('hex', '')
        operand = entry.get('operand', '')
        repeat = f" [x{entry['repeat']}]" if entry.get('repeat', 0) > 0 else ""
        a = f"{entry['a']:02X}"
        b = f"{entry['b']:02X}"
        x = f"{entry['x']:04X}"
        y = f"{entry['y']:04X}"
        
        print(f"{i:3}: {pc}: {op_str:8} ({hex_bytes}) {operand} A={a} B={b} X={x} Y={y}{repeat}")
    
    # Buscar patrones de bucle
    print("\n=== BÚSQUEDA DE BUCLES ===")
    pc_counts = Counter(entry['pc'] for entry in trace_data)
    hot_pcs = pc_counts.most_common(20)
    
    print("TOP 20 PCs MÁS VISITADOS (posibles bucles):")
    for pc, count in hot_pcs:
        print(f"{pc:04X}: {count} veces")
    
    # Analizar específicamente el bucle F4EB-F4EF
    print("\n=== ANÁLISIS DEL BUCLE F4EB-F4EF ===")
    delay_loop_entries = [entry for entry in trace_data if 0xF4EB <= entry['pc'] <= 0xF4EF]
    print(f"Entradas en rango F4EB-F4EF: {len(delay_loop_entries)}")
    
    if delay_loop_entries:
        print("PRIMERAS 20 entradas del bucle de delay:")
        for i, entry in enumerate(delay_loop_entries[:20]):
            pc = f"{entry['pc']:04X}"
            op_str = entry.get('m', 'UNK')
            y = f"{entry['y']:04X}"
            print(f"{i}: {pc}: {op_str} Y={y}")
        
        print("\nULTIMAS 10 entradas del bucle de delay:")
        for i, entry in enumerate(delay_loop_entries[-10:]):
            pc = f"{entry['pc']:04X}"
            op_str = entry.get('m', 'UNK')
            y = f"{entry['y']:04X}"
            idx = len(delay_loop_entries) - 10 + i
            print(f"{idx}: {pc}: {op_str} Y={y}")
    
    # Analizar últimas entradas
    print("\n=== ÚLTIMAS 20 ENTRADAS ===")
    for i, entry in enumerate(trace_data[-20:]):
        pc = f"{entry['pc']:04X}"
        op_str = entry.get('m', 'UNK')
        hex_bytes = entry.get('hex', '')
        operand = entry.get('operand', '')
        repeat = f" [x{entry['repeat']}]" if entry.get('repeat', 0) > 0 else ""
        a = f"{entry['a']:02X}"
        b = f"{entry['b']:02X}"
        x = f"{entry['x']:04X}"
        y = f"{entry['y']:04X}"
        
        idx = len(trace_data) - 20 + i
        print(f"{idx:4}: {pc}: {op_str:8} ({hex_bytes}) {operand} A={a} B={b} X={x} Y={y}{repeat}")
    
    # Detectar secuencias repetitivas específicas
    print("\n=== DETECCIÓN DE SECUENCIAS REPETITIVAS ===")
    
    # Buscar secuencia específica F4EB-F4EC-F4ED-F4EF (bucle de delay conocido)
    sequence_count = 0
    last_pcs = []
    
    for entry in trace_data:
        pc = entry['pc']
        last_pcs.append(pc)
        if len(last_pcs) > 4:
            last_pcs.pop(0)
        
        # Detectar secuencia F4EB -> F4EC -> F4ED -> F4EF -> F4EB
        if len(last_pcs) == 4 and last_pcs == [0xF4EB, 0xF4EC, 0xF4ED, 0xF4EF]:
            sequence_count += 1
    
    print(f"Secuencias completas F4EB->F4EC->F4ED->F4EF detectadas: {sequence_count}")
    
    # Buscar vectores generados (llamadas a BIOS de vectores)
    print("\n=== BÚSQUEDA DE LLAMADAS BIOS VECTORIALES ===")
    vector_calls = []
    for entry in trace_data:
        pc = entry['pc']
        # Buscar llamadas típicas de vector drawing en BIOS
        if pc in [0xF2A4, 0xF2A6, 0xF2A8, 0xF2AA]:  # Draw vector calls comunes
            vector_calls.append(entry)
    
    print(f"Llamadas BIOS vectoriales encontradas: {len(vector_calls)}")
    if vector_calls:
        print("Primeras 5 llamadas vectoriales:")
        for i, entry in enumerate(vector_calls[:5]):
            pc = f"{entry['pc']:04X}"
            op_str = entry.get('m', 'UNK')
            a = f"{entry['a']:02X}"
            b = f"{entry['b']:02X}"
            print(f"  {i}: {pc}: {op_str} A={a} B={b}")
    
    return trace_data

def export_trace_summary(trace_data, filename="trace_summary.txt"):
    """Exporta un resumen del trace a archivo"""
    
    with open(filename, 'w', encoding='utf-8') as f:
        f.write("=== RESUMEN DE TRACE DATA ===\n\n")
        f.write(f"Total entradas: {len(trace_data)}\n\n")
        
        # PC counts
        pc_counts = Counter(entry['pc'] for entry in trace_data)
        f.write("TOP 50 PCs MÁS VISITADOS:\n")
        for pc, count in pc_counts.most_common(50):
            f.write(f"{pc:04X}: {count} veces\n")
        
        f.write("\n=== TRACE COMPLETO ===\n")
        for i, entry in enumerate(trace_data):
            pc = f"{entry['pc']:04X}"
            op_str = entry.get('m', 'UNK')
            hex_bytes = entry.get('hex', '')
            operand = entry.get('operand', '')
            repeat = f" [x{entry['repeat']}]" if entry.get('repeat', 0) > 0 else ""
            a = f"{entry['a']:02X}"
            b = f"{entry['b']:02X}"
            x = f"{entry['x']:04X}"
            y = f"{entry['y']:04X}"
            u = f"{entry['u']:04X}"
            s = f"{entry['s']:04X}"
            dp = f"{entry['dp']:02X}"
            
            f.write(f"{i:5}: {pc}: {op_str:8} ({hex_bytes}) {operand}{repeat} "
                   f"A={a} B={b} X={x} Y={y} U={u} S={s} DP={dp}\n")
    
    print(f"Resumen exportado a {filename}")

if __name__ == "__main__":
    print("Para usar este script:")
    print("1. Desde consola del navegador con UI abierta:")
    print("   const traceJson = emuCore.emu.trace_log_json();")
    print("   console.log(traceJson);")
    print("2. Copiar el JSON y guardar en archivo 'trace_data.json'")
    print("3. Ejecutar: python analyze_trace.py")
    
    # Intentar cargar trace_data.json si existe
    try:
        with open('trace_data.json', 'r', encoding='utf-8') as f:
            trace_json = f.read()
        
        trace_data = analyze_trace_json(trace_json)
        if trace_data:
            export_trace_summary(trace_data)
    
    except FileNotFoundError:
        print("\nArchivo 'trace_data.json' no encontrado.")
        print("Ejecute los pasos arriba para obtener los datos.")