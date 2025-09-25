import re
from pathlib import Path

vectrexy_path = Path('vectrexy_backup/libs/emulator/include/emulator/CpuOpCodes.h')
rust_path = Path('emulator_v2/src/core/cpu_op_codes.rs')

# Parse Vectrexy
with open(vectrexy_path, 'r') as f:
    content = f.read()
pattern = r'{\s*0x([0-9A-Fa-f]+),\s*"([^"]+)",\s*AddressingMode::(\w+)\s*,\s*(\d+),\s*(\d+),\s*"([^"]+)"\s*}'
v_matches = re.findall(pattern, content, re.MULTILINE)
vectrexy_opcodes = {int(m[0], 16): m[1] for m in v_matches}

# Parse Rust
with open(rust_path, 'r') as f:
    content = f.read()
pattern = r'0x([0-9A-Fa-f]+)\s*=>\s*CpuOp\s*{\s*op_code:\s*0x[0-9A-Fa-f]+,\s*name:\s*"([^"]+)"'
r_matches = re.findall(pattern, content)
rust_opcodes = {int(m[0], 16): m[1] for m in r_matches}

missing = set(vectrexy_opcodes.keys()) - set(rust_opcodes.keys())
non_illegal = [op for op in missing if vectrexy_opcodes[op] != 'Illegal']

print('OPCODES FALTANTES (excluyendo Illegal):')
for op in sorted(non_illegal):
    print(f'  0x{op:02X}: {vectrexy_opcodes[op]}')

print(f'\nTotal faltantes (sin Illegal): {len(non_illegal)}/256 = {(len(non_illegal)/256)*100:.1f}%')
print(f'Compliance real (sin Illegal): {((256-len(non_illegal))/256)*100:.1f}%')