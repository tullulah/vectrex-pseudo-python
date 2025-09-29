import pathlib

# Read BIOS data
data = pathlib.Path(r'C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin').read_bytes()
base = 0xE000  # BIOS mapped at E000-FFFF

# Reset vector at 0xFFFE (last 2 bytes of BIOS)
print(f'BIOS size: {len(data)} bytes (0x{len(data):04X})')
reset_vec_offset = 0xFFFE - base
if reset_vec_offset + 1 < len(data):
    reset_addr = (data[reset_vec_offset+1] << 8) | data[reset_vec_offset] 
    print(f'Reset vector at 0xFFFE points to: 0x{reset_addr:04X}')
else:
    print(f'Reset vector offset 0x{reset_vec_offset:04X} is beyond BIOS size')

# Also check what's at F000 (typical BIOS start)  
f000_offset = 0xF000 - base
print(f'Byte at F000: 0x{data[f000_offset]:02X}')
print(f'Bytes at F000-F003: {" ".join(f"0x{data[f000_offset+i]:02X}" for i in range(4))}')

# Check bytes around reset vector
print(f'Bytes at FFFC-FFFF: {" ".join(f"0x{data[0xFFFC-base+i]:02X}" for i in range(4))}')