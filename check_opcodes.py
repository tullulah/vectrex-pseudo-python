import json

# Cargar el trace
with open('emulator/startup_trace.json', 'r') as f:
    trace = json.load(f)

# Buscar entradas del bucle F4EB-F4EF
f4eb_entries = [e for e in trace if e['pc'] in range(0xF4EB, 0xF4F0)]

print("F4EB-F4EF opcodes:")
for e in f4eb_entries[:12]:  # Mostrar 3 iteraciones del bucle completo
    pc = e['pc']
    op = e['op']
    m = e.get('m', 'UNK')
    b_val = e.get('b', 0)
    z_flag = e.get('flags', 0) & 0x04 != 0  # bit 2 es Z flag
    print(f"PC: {pc:04X}, op: 0x{op:02X}, m: {m}, B: {b_val:02X}, Z: {z_flag}")