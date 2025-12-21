#!/usr/bin/env python3
"""Añade un rectángulo de referencia con los límites de pantalla a todos los .vec"""
import json
import pathlib

vec_dir = pathlib.Path("examples/jetpac/assets/vectors")

for vec_file in vec_dir.glob("*.vec"):
    print(f"Procesando: {vec_file.name}")
    
    # Leer JSON
    data = json.loads(vec_file.read_text())
    
    # Recorrer capas y encontrar la primera (default)
    for layer in data.get("layers", []):
        if layer.get("name") == "default":
            # Crear path de referencia con baja intensidad
            reference_path = {
                "name": "_screen_reference",
                "intensity": 30,  # Muy bajo para no interferir
                "closed": True,
                "points": [
                    {"x": -127, "y": 120},
                    {"x": 126, "y": 120},
                    {"x": 126, "y": -120},
                    {"x": -127, "y": -120}
                ]
            }
            
            # Agregar al final de los paths (para que quede detrás visualmente)
            layer["paths"].append(reference_path)
            break
    
    # Escribir JSON de vuelta
    vec_file.write_text(json.dumps(data, separators=(',', ': '), indent=2))
    print(f"  ✓ Rectángulo de referencia añadido")

print(f"\n✅ Todos los sprites tienen marco de referencia de pantalla")
