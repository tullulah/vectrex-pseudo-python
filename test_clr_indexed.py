#!/usr/bin/env python3
"""
Test específico para verificar el comportamiento del emulador con CLR 11,X
"""

import subprocess
import json
import tempfile
import os

def create_test_binary():
    """Crea un binario específico para testear CLR 11,X"""
    test_code = """
    ORG $C800
START:
    LDX #$C800     ; X = C800
    CLR 11,X       ; CLR [X+11] = CLR [C80B], X NO debe cambiar
    NOP            ; Punto de parada
    """
    
    # Escribir archivo temporal
    with tempfile.NamedTemporaryFile(mode='w', suffix='.asm', delete=False) as f:
        f.write(test_code)
        asm_file = f.name
    
    bin_file = asm_file.replace('.asm', '.bin')
    
    # Compilar con lwasm
    try:
        result = subprocess.run(['lwasm', '-f', 'raw', '-o', bin_file, asm_file], 
                              capture_output=True, text=True)
        if result.returncode != 0:
            print(f"Error compilando: {result.stderr}")
            return None
    except FileNotFoundError:
        print("lwasm no encontrado, creando binario manualmente")
        # Código máquina manual:
        # LDX #$C800 = 8E C8 00
        # CLR 11,X   = 6F 8B  (6F = CLR indexed, 8B = postbyte X+11)
        # NOP        = 12
        machine_code = bytes([0x8E, 0xC8, 0x00, 0x6F, 0x8B, 0x12])
        with open(bin_file, 'wb') as f:
            f.write(machine_code)
    
    # Limpiar archivo temporal
    os.unlink(asm_file)
    return bin_file

def test_emulator():
    """Prueba el emulador Rust con el código de test"""
    bin_file = create_test_binary()
    if not bin_file:
        return None
    
    try:
        # Ejecutar emulador
        result = subprocess.run([
            'cargo', 'run', '--bin', 'emulator_test_runner', '--',
            '--cart', bin_file, '--steps', '10', '--start', 'C800'
        ], capture_output=True, text=True, cwd='emulator')
        
        if result.returncode != 0:
            print(f"Error ejecutando emulador: {result.stderr}")
            return None
        
        # Parsear salida JSON
        try:
            output = json.loads(result.stdout)
            return output
        except json.JSONDecodeError:
            print(f"Output no es JSON válido: {result.stdout}")
            return None
            
    finally:
        if bin_file and os.path.exists(bin_file):
            os.unlink(bin_file)
    
    return None

def main():
    print("=== Test específico para CLR 11,X ===")
    print()
    
    # Test emulador Rust
    print("Testeando emulador Rust...")
    result = test_emulator()
    
    if result:
        initial_state = result.get('initial_state', {})
        final_state = result.get('final_state', {})
        trace = result.get('trace', [])
        
        print(f"Estado inicial X: {initial_state.get('x', 'N/A'):04X}")
        print(f"Estado final X:   {final_state.get('x', 'N/A'):04X}")
        print()
        
        # Mostrar traza relevante
        print("Traza de ejecución:")
        for i, step in enumerate(trace[:6]):  # Primeros 6 pasos
            pc = step.get('pc', 0)
            instruction = step.get('instruction', '')
            x_val = step.get('x', 0)
            print(f"Paso {i+1}: PC={pc:04X} X={x_val:04X} {instruction}")
        
        # Verificar si X cambió incorrectamente
        initial_x = initial_state.get('x', 0)
        final_x = final_state.get('x', 0)
        
        if initial_x != final_x:
            print(f"\n❌ ERROR: X cambió de {initial_x:04X} a {final_x:04X}")
            print("CLR 11,X NO debería modificar X")
        else:
            print(f"\n✅ CORRECTO: X se mantuvo en {initial_x:04X}")
    else:
        print("❌ No se pudo ejecutar el test")

if __name__ == "__main__":
    main()