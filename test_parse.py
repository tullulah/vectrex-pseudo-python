asm = open('examples/test_debug_simple.asm', encoding='utf-8').read()
addr = 0x0000

# Test FCC calculation
test_line = '    FCC "g GCE 1982"'
start = test_line.find('"')
end = test_line.rfind('"')
print(f'FCC test: start={start}, end={end}, len={end-start-1}')
print(f'Expected: 10 (g GCE 1982)')
print()

for i, line in enumerate(asm.split('\n')[:40], 1):
    trimmed = line.strip()
    if trimmed.startswith('ORG '):
        addr_str = trimmed.split()[1]
        addr = int(addr_str.replace('$', '0x'), 16)
        print(f'{i:3d} ORG detected: {addr_str} -> 0x{addr:04X}')
    elif trimmed.startswith('FCC '):
        if '"' in trimmed:
            s = trimmed[trimmed.find('"')+1:trimmed.rfind('"')]
            print(f'{i:3d} FCC: len={len(s)} addr=0x{addr:04X} -> 0x{addr+len(s):04X} content="{s}"')
            addr += len(s)
    elif trimmed.startswith('FCB '):
        print(f'{i:3d} FCB: addr=0x{addr:04X} -> 0x{addr+1:04X}')
        addr += 1
    elif trimmed.startswith('FDB '):
        print(f'{i:3d} FDB: addr=0x{addr:04X} -> 0x{addr+2:04X}')
        addr += 2
    elif '; VPy_LINE' in trimmed:
        line_num = trimmed.split(':')[1]
        print(f'{i:3d} MARKER Line {line_num} at 0x{addr:04X}')
