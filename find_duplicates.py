import pathlib

arith = pathlib.Path(r'C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\emulator_v2\tests\opcodes\arithmetic')
files = [f.name for f in arith.glob('*.rs')]

bases = [f for f in files if '_variants' not in f and f != 'mod.rs']
variants = [f for f in files if '_variants' in f]

for base in bases:
    vname = base.replace('.rs', '_variants.rs')
    if vname in variants:
        print(f"{base} duplica a {vname}")
