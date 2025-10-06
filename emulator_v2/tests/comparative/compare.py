#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Comparative Test Tool - Vectrexy vs Rust Emulator
Compara estados JSON y reporta diferencias con colores
"""

import json
import sys
import os
from typing import Any, Dict, List, Tuple
from dataclasses import dataclass

# Fix Windows console encoding for emoji support
if sys.platform == 'win32':
    import codecs
    sys.stdout = codecs.getwriter('utf-8')(sys.stdout.buffer, 'strict')
    sys.stderr = codecs.getwriter('utf-8')(sys.stderr.buffer, 'strict')

# ANSI color codes
class Colors:
    RESET = '\033[0m'
    RED = '\033[91m'
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    MAGENTA = '\033[95m'
    CYAN = '\033[96m'
    BOLD = '\033[1m'

@dataclass
class Difference:
    path: str
    expected: Any
    vectrexy: Any
    rust: Any
    
    def severity(self) -> str:
        """Determina severidad de la diferencia"""
        if self.expected == self.vectrexy == self.rust:
            return "OK"
        elif self.expected == self.vectrexy:
            return "RUST_DIFF"  # Solo Rust difiere
        elif self.expected == self.rust:
            return "VECTREXY_DIFF"  # Solo Vectrexy difiere
        else:
            return "BOTH_DIFF"  # Ambos difieren del esperado

def compare_values(path: str, expected: Any, vectrexy: Any, rust: Any, 
                   differences: List[Difference], tolerance: float = 0.01, 
                   missing_fields: List[str] = None):
    """Compara valores recursivamente con tolerancia para floats"""
    
    if missing_fields is None:
        missing_fields = []
    
    # Si expected es None, skip this field (don't care)
    if expected is None:
        return
    
    # Detectar campos faltantes en vectrexy o rust
    if vectrexy is None or rust is None:
        missing_fields.append(f"{path} (vectrexy={vectrexy is not None}, rust={rust is not None})")
        return
    
    # Floats con tolerancia
    if isinstance(expected, (float, int)) and isinstance(vectrexy, (float, int)) and isinstance(rust, (float, int)):
        exp_f = float(expected)
        vec_f = float(vectrexy)
        rus_f = float(rust)
        
        if abs(exp_f - vec_f) > tolerance or abs(exp_f - rus_f) > tolerance:
            differences.append(Difference(path, exp_f, vec_f, rus_f))
        return
    
    # Diccionarios
    if isinstance(expected, dict) and isinstance(vectrexy, dict) and isinstance(rust, dict):
        all_keys = set(expected.keys()) | set(vectrexy.keys()) | set(rust.keys())
        for key in all_keys:
            new_path = f"{path}.{key}" if path else key
            exp_val = expected.get(key, None)
            vec_val = vectrexy.get(key, None)
            rus_val = rust.get(key, None)
            compare_values(new_path, exp_val, vec_val, rus_val, differences, tolerance, missing_fields)
        return
    
    # Listas
    if isinstance(expected, list) and isinstance(vectrexy, list) and isinstance(rust, list):
        max_len = max(len(expected), len(vectrexy), len(rust))
        for i in range(max_len):
            new_path = f"{path}[{i}]"
            exp_val = expected[i] if i < len(expected) else None
            vec_val = vectrexy[i] if i < len(vectrexy) else None
            rus_val = rust[i] if i < len(rust) else None
            compare_values(new_path, exp_val, vec_val, rus_val, differences, tolerance, missing_fields)
        return
    
    # Comparación directa
    if expected != vectrexy or expected != rust:
        differences.append(Difference(path, expected, vectrexy, rust))

def print_header(test_name: str):
    """Imprime cabecera del reporte"""
    print(f"\n{Colors.BOLD}{Colors.CYAN}{'='*80}{Colors.RESET}")
    print(f"{Colors.BOLD}{Colors.CYAN}  COMPARATIVE TEST: {test_name}{Colors.RESET}")
    print(f"{Colors.BOLD}{Colors.CYAN}{'='*80}{Colors.RESET}\n")

def print_summary(differences: List[Difference]):
    """Imprime resumen de diferencias"""
    severities = {'OK': 0, 'RUST_DIFF': 0, 'VECTREXY_DIFF': 0, 'BOTH_DIFF': 0}
    for diff in differences:
        sev = diff.severity()
        severities[sev] = severities.get(sev, 0) + 1
    
    print(f"\n{Colors.BOLD}SUMMARY:{Colors.RESET}")
    print(f"  Total differences: {len(differences)}")
    print(f"  {Colors.RED}Rust differs:      {severities['RUST_DIFF']}{Colors.RESET}")
    print(f"  {Colors.YELLOW}Vectrexy differs:  {severities['VECTREXY_DIFF']}{Colors.RESET}")
    print(f"  {Colors.MAGENTA}Both differ:       {severities['BOTH_DIFF']}{Colors.RESET}")

def print_difference(diff: Difference):
    """Imprime una diferencia con formato colorido"""
    sev = diff.severity()
    
    # Colorear según severidad
    if sev == "RUST_DIFF":
        color = Colors.RED
        symbol = "❌"
    elif sev == "VECTREXY_DIFF":
        color = Colors.YELLOW
        symbol = "⚠️"
    elif sev == "BOTH_DIFF":
        color = Colors.MAGENTA
        symbol = "🔥"
    else:
        return  # No imprimir si no hay diferencia
    
    print(f"{color}{symbol} {diff.path}{Colors.RESET}")
    print(f"  Expected:  {Colors.GREEN}{diff.expected}{Colors.RESET}")
    print(f"  Vectrexy:  {Colors.CYAN}{diff.vectrexy}{Colors.RESET}")
    print(f"  Rust:      {Colors.BLUE}{diff.rust}{Colors.RESET}")
    
    # Calcular deltas si son numéricos
    if isinstance(diff.expected, (int, float)) and isinstance(diff.vectrexy, (int, float)) and isinstance(diff.rust, (int, float)):
        delta_vec = diff.vectrexy - diff.expected
        delta_rus = diff.rust - diff.expected
        print(f"  Δ Vectrexy: {delta_vec:+.6f}")
        print(f"  Δ Rust:     {delta_rus:+.6f}")
    print()

def main():
    # Soportar 2 o 3 argumentos
    if len(sys.argv) < 3:
        print("Usage: python compare.py <expected.json> <output1.json> [output2.json]")
        print("Example (2-way): python compare.py expected.json rust_output.json")
        print("Example (3-way): python compare.py expected.json vectrexy_out.json rust_out.json")
        sys.exit(1)
    
    expected_path = sys.argv[1]
    
    # Modo 2-way vs 3-way (sys.argv[0] es el nombre del script)
    if len(sys.argv) == 3:
        # Modo 2-way: expected vs rust solamente
        rust_path = sys.argv[2]
        vectrexy_path = None
    else:
        # Modo 3-way: expected vs vectrexy vs rust
        vectrexy_path = sys.argv[2]
        rust_path = sys.argv[3]
    
    # Cargar JSONs
    try:
        print(f"Loading expected from: {expected_path}")
        with open(expected_path, 'r', encoding='utf-8-sig') as f:
            expected = json.load(f)
        
        print(f"Loading rust from: {rust_path}")
        with open(rust_path, 'r', encoding='utf-8-sig') as f:
            rust = json.load(f)
        
        # En modo 2-way, usar expected como vectrexy (para compatibilidad)
        if vectrexy_path is None:
            vectrexy = expected
        else:
            print(f"Loading vectrexy from: {vectrexy_path}")
            with open(vectrexy_path, 'r', encoding='utf-8-sig') as f:
                vectrexy = json.load(f)
    except Exception as e:
        print(f"{Colors.RED}Error loading JSON files: {e}{Colors.RESET}")
        import traceback
        traceback.print_exc()
        sys.exit(1)
    
    # Comparar
    differences = []
    missing_fields = []
    
    # Extraer nombre del test de la ruta (soportar Windows y Unix paths)
    import os
    test_name = os.path.basename(os.path.dirname(expected_path))
    if test_name == "." or test_name == "":
        test_name = os.path.basename(expected_path).replace(".json", "")
    
    # IGNORE_FIELDS: Campos conocidos que pueden diferir por inicialización BIOS
    # Para tests CPU puros (sin interacción con VIA), ignorar timers
    ignore_fields = []
    if test_name.startswith("cpu_"):
        # Tests CPU puros - ignorar timers VIA (diferencia esperada por init BIOS)
        ignore_fields = ["via.timer1_counter", "via.timer2_counter"]
        print(f"{Colors.YELLOW}Note: Ignoring {ignore_fields} (BIOS initialization difference){Colors.RESET}")
    
    # Filtrar diferencias ignoradas
    def should_ignore(path: str) -> bool:
        for ignore_pattern in ignore_fields:
            if path == ignore_pattern:
                return True
        return False
    
    compare_values("", expected, vectrexy, rust, differences, missing_fields=missing_fields)
    
    # Filtrar campos ignorados DESPUÉS de compare_values
    differences = [d for d in differences if not should_ignore(d.path)]
    
    # Reportar
    print_header(test_name)
    
    # Mostrar campos faltantes si los hay
    if missing_fields:
        print(f"{Colors.YELLOW}{Colors.BOLD}⚠️  WARNING: Some fields missing in outputs:{Colors.RESET}")
        for field in missing_fields[:10]:  # Limitar a 10 primeros
            print(f"  {Colors.YELLOW}{field}{Colors.RESET}")
        if len(missing_fields) > 10:
            print(f"  {Colors.YELLOW}... and {len(missing_fields) - 10} more{Colors.RESET}")
        print()
    
    if not differences:
        print(f"{Colors.GREEN}{Colors.BOLD}✅ ALL TESTS PASSED!{Colors.RESET}")
        print(f"{Colors.GREEN}Vectrexy and Rust outputs match perfectly.{Colors.RESET}")
        
        # Mostrar resumen de valores comparados para verificar que realmente se testeó algo
        print(f"\n{Colors.BOLD}Verified fields:{Colors.RESET}")
        def count_fields(obj, prefix=""):
            count = 0
            if isinstance(obj, dict):
                for key, value in obj.items():
                    full_key = f"{prefix}.{key}" if prefix else key
                    if isinstance(value, (dict, list)):
                        count += count_fields(value, full_key)
                    else:
                        count += 1
                        # Mostrar algunos valores de ejemplo
                        if count <= 10:  # Limitar a primeros 10 campos
                            print(f"  {Colors.CYAN}{full_key}{Colors.RESET} = {Colors.GREEN}{value}{Colors.RESET}")
            elif isinstance(obj, list):
                for i, item in enumerate(obj):
                    count += count_fields(item, f"{prefix}[{i}]")
            return count
        
        total_fields = count_fields(expected)
        if total_fields > 10:
            print(f"  {Colors.YELLOW}... and {total_fields - 10} more fields{Colors.RESET}")
        print(f"\n{Colors.BOLD}Total fields compared: {total_fields}{Colors.RESET}")
        
        sys.exit(0)
    
    # Imprimir diferencias
    print(f"{Colors.BOLD}DIFFERENCES FOUND:{Colors.RESET}\n")
    for diff in differences:
        print_difference(diff)
    
    print_summary(differences)
    
    # Exit code según severidad
    critical_diffs = sum(1 for d in differences if d.severity() in ["RUST_DIFF", "BOTH_DIFF"])
    if critical_diffs > 0:
        print(f"\n{Colors.RED}{Colors.BOLD}❌ TEST FAILED - {critical_diffs} critical differences{Colors.RESET}")
        sys.exit(1)
    else:
        print(f"\n{Colors.YELLOW}⚠️  TEST PASSED WITH WARNINGS - Only Vectrexy differs{Colors.RESET}")
        sys.exit(0)

if __name__ == "__main__":
    main()
