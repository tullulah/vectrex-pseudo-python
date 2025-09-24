#!/usr/bin/env python3
"""
Genera comparación del emulador Rust con debug específico para CLR indexed
"""

import subprocess
import json
import os

def run_rust_emulator_debug():
    """Ejecuta el emulador Rust con debug específico"""
    
    # Usar bios_intro_probe que existe y funciona
    cmd = [
        'cargo', 'run', '--bin', 'bios_intro_probe', '--release'
    ]
    
    # Agregar variable de entorno para trace
    env = os.environ.copy()
    env['BIOS_INTRO_TRACE'] = '1'
    
    try:
        result = subprocess.run(
            cmd, 
            capture_output=True, 
            text=True, 
            cwd='emulator',
            env=env
        )
        
        if result.returncode == 0:
            print("✅ Emulador Rust ejecutado exitosamente")
            print(f"STDOUT:\n{result.stdout}")
            if result.stderr:
                print(f"STDERR:\n{result.stderr}")
            return result.stdout
        else:
            print(f"❌ Error ejecutando emulador: {result.stderr}")
            return None
            
    except Exception as e:
        print(f"❌ Excepción ejecutando emulador: {e}")
        return None

def main():
    print("=== DEBUG ESPECÍFICO PARA CLR INDEXED ===")
    print()
    
    # Ejecutar emulador con trace
    print("Ejecutando emulador Rust con trace...")
    output = run_rust_emulator_debug()
    
    if output:
        # Buscar líneas de debug relacionadas con CLR
        clr_lines = []
        x_change_lines = []
        
        for line in output.split('\n'):
            if 'CLR' in line:
                clr_lines.append(line)
            if '❌ CLR indexed bug' in line:
                x_change_lines.append(line)
        
        print(f"\nLíneas relacionadas con CLR: {len(clr_lines)}")
        for line in clr_lines[:10]:  # Primeras 10
            print(f"  {line}")
        
        print(f"\nLíneas de bug X detectadas: {len(x_change_lines)}")
        for line in x_change_lines:
            print(f"  🚨 {line}")
        
        if x_change_lines:
            print("\n❌ BUG CONFIRMADO: X se está modificando en CLR indexed")
        else:
            print("\n✅ No se detectó el bug en el trace (puede que no se alcance)")
    else:
        print("❌ No se pudo obtener output del emulador")

if __name__ == "__main__":
    main()