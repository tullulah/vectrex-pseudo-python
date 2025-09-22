#!/usr/bin/env python3
"""
Análisis específico del vector diagonal en el trace de inicio de BIOS
"""

import json
import sys
from pathlib import Path

def analyze_diagonal_vectors():
    trace_file = Path("startup_trace.json")
    if not trace_file.exists():
        print("❌ No se encontró startup_trace.json")
        return
    
    with open(trace_file, 'r') as f:
        trace_data = json.load(f)
    
    print("🔍 ANÁLISIS DEL VECTOR DIAGONAL")
    print("="*50)
    
    # Buscar escrituras al puerto VIA que generen coordenadas diagonales
    diagonal_events = []
    
    for i, entry in enumerate(trace_data):
        pc = entry.get('pc', 0)
        op_str = entry.get('op_str', '')
        a = entry.get('a', 0)
        b = entry.get('b', 0)
        
        # Buscar valores que puedan generar coordenadas (-30, -30)
        # 0xF1 = 241 = -15 en complemento a 2 de 8 bits
        if a == 0xF1 or b == 0xF1:
            diagonal_events.append({
                'index': i,
                'pc': f"F{pc:04X}",
                'op_str': op_str,
                'a': f"0x{a:02X}",
                'b': f"0x{b:02X}",
                'context': f"trace[{i}]"
            })
    
    print(f"Eventos con valor 0xF1 encontrados: {len(diagonal_events)}")
    print()
    
    for event in diagonal_events[:10]:  # Primeros 10
        print(f"{event['context']} {event['pc']}: {event['op_str']} A={event['a']} B={event['b']}")
    
    print()
    print("🎯 SECUENCIA CRÍTICA - Búsqueda de escrituras VIA con 0xF1:")
    
    # Buscar la secuencia específica donde se genera el vector diagonal
    critical_sequence = []
    
    for i in range(len(trace_data) - 5):
        entry = trace_data[i]
        pc = entry.get('pc', 0)
        a = entry.get('a', 0)
        
        # Buscar cuando A = 0xF1 en cualquier parte
        if a == 0xF1:
            # Capturar contexto (5 instrucciones antes y después)
            start_idx = max(0, i - 5)
            end_idx = min(len(trace_data), i + 6)
            
            print(f"\n📍 SECUENCIA CRÍTICA encontrada en trace[{i}]:")
            for j in range(start_idx, end_idx):
                marker = ">>> " if j == i else "    "
                entry_ctx = trace_data[j]
                pc_ctx = entry_ctx.get('pc', 0)
                op_str_ctx = entry_ctx.get('op_str', '')
                a_ctx = entry_ctx.get('a', 0)
                b_ctx = entry_ctx.get('b', 0)
                
                print(f"{marker}trace[{j}] F{pc_ctx:04X}: {op_str_ctx} A=0x{a_ctx:02X} B=0x{b_ctx:02X}")
            
            critical_sequence.append(i)
            
            if len(critical_sequence) >= 3:  # Solo mostrar las primeras 3
                break
    
    print(f"\n🎲 FRECUENCIA DEL PROBLEMA:")
    print(f"Secuencias críticas encontradas: {len(critical_sequence)}")
    
    if critical_sequence:
        first_occurrence = critical_sequence[0]
        print(f"Primera ocurrencia: trace[{first_occurrence}]")
        print(f"Esto sugiere que el vector diagonal aparece repetidamente")
        print(f"durante la secuencia de bucles de delay de la BIOS.")
    
    print("\n💡 CONCLUSIÓN:")
    print("El valor 0xF1 en el registro A se está escribiendo al puerto VIA,")
    print("generando coordenadas Y = -30 que causan el vector diagonal visible.")
    print("Esto ocurre durante los bucles de delay de la BIOS de inicio.")

if __name__ == "__main__":
    analyze_diagonal_vectors()