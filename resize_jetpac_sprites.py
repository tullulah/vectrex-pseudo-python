#!/usr/bin/env python3
"""Redimensiona todos los sprites de Jetpac dividiéndolos por 2"""
import json
import pathlib

vec_dir = pathlib.Path("examples/jetpac/assets/vectors")

for vec_file in vec_dir.glob("*.vec"):
    print(f"Procesando: {vec_file.name}")
    
    # Leer JSON
    data = json.loads(vec_file.read_text())
    
    # Recorrer todas las capas y paths
    for layer in data.get("layers", []):
        for path in layer.get("paths", []):
            # Dividir todas las coordenadas x, y por 2 y redondear
            for point in path.get("points", []):
                if "x" in point:
                    point["x"] = int(round(point["x"] / 2))
                if "y" in point:
                    point["y"] = int(round(point["y"] / 2))
    
    # Escribir JSON de vuelta (sin espacios extra)
    vec_file.write_text(json.dumps(data, separators=(',', ': '), indent=2))
    print(f"  ✓ Redimensionado correctamente")

print(f"\n✅ Todos los sprites han sido redimensionados (÷2)")
