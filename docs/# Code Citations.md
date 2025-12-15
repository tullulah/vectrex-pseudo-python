# Code Citations

## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&CC.Value) {
        CC
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&CC.Value) {
        CC.Negative = CalcNeg
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow =
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpA
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpAND(uint8_t& reg
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpAND(uint8_t& reg) {
    uint8_t
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpAND(uint8_t& reg) {
    uint8_t value = ReadOperandValue8
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpAND(uint8_t& reg) {
    uint8_t value = ReadOperandValue8<LookupCpuOp
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpAND(uint8_t& reg) {
    uint8_t value = ReadOperandValue8<LookupCpuOp(page, opCode).addr
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpAND(uint8_t& reg) {
    uint8_t value = ReadOperandValue8<LookupCpuOp(page, opCode).addrMode>();
    reg = reg &
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpAND(uint8_t& reg) {
    uint8_t value = ReadOperandValue8<LookupCpuOp(page, opCode).addrMode>();
    reg = reg & value;
    // For AND
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpAND(uint8_t& reg) {
    uint8_t value = ReadOperandValue8<LookupCpuOp(page, opCode).addrMode>();
    reg = reg & value;
    // For ANDCC, we don'
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC.Negative = CalcNeg
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow =
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void Op
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpEOR(uint8_t&
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpEOR(uint8_t& reg) {
    uint8_
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpEOR(uint8_t& reg) {
    uint8_t value = ReadOperandValue8
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpEOR(uint8_t& reg) {
    uint8_t value = ReadOperandValue8<LookupCpuO
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpEOR(uint8_t& reg) {
    uint8_t value = ReadOperandValue8<LookupCpuOp(page, opCode).a
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpEOR(uint8_t& reg) {
    uint8_t value = ReadOperandValue8<LookupCpuOp(page, opCode).addrMode>();
    reg ^=
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpEOR(uint8_t& reg) {
    uint8_t value = ReadOperandValue8<LookupCpuOp(page, opCode).addrMode>();
    reg ^= value;
    CC.Negative
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpEOR(uint8_t& reg) {
    uint8_t value = ReadOperandValue8<LookupCpuOp(page, opCode).addrMode>();
    reg ^= value;
    CC.Negative = CalcNegative(reg
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpEOR(uint8_t& reg) {
    uint8_t value = ReadOperandValue8<LookupCpuOp(page, opCode).addrMode>();
    reg ^= value;
    CC.Negative = CalcNegative(reg);
    CC.Zero = Cal
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpEOR(uint8_t& reg) {
    uint8_t value = ReadOperandValue8<LookupCpuOp(page, opCode).addrMode>();
    reg ^= value;
    CC.Negative = CalcNegative(reg);
    CC.Zero = CalcZero(reg);
    
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpEOR(uint8_t& reg) {
    uint8_t value = ReadOperandValue8<LookupCpuOp(page, opCode).addrMode>();
    reg ^= value;
    CC.Negative = CalcNegative(reg);
    CC.Zero = CalcZero(reg);
    CC.Overflow = 0;
```


## License: MIT
https://github.com/amaiorano/vectrexy/blob/9d165af2e617960d7f32e81efff54fdab4738c8c/libs/emulator/src/Cpu.cpp

```
&reg != &CC.Value) {
        CC.Negative = CalcNegative(reg);
        CC.Zero = CalcZero(reg);
        CC.Overflow = 0;
    }
}

template <int page, uint8_t opCode>
void OpEOR(uint8_t& reg) {
    uint8_t value = ReadOperandValue8<LookupCpuOp(page, opCode).addrMode>();
    reg ^= value;
    CC.Negative = CalcNegative(reg);
    CC.Zero = CalcZero(reg);
    CC.Overflow = 0;
}
```

