# Motorola 6809 & VIA 6522 Opcode / Register Implementation Status

Esta tabla ofrece una visión completa del estado de implementación del CPU 6809 (opcodes primarios y prefijados 0x10 / 0x11) y de los registros principales del VIA 6522 dentro del emulador.

Leyenda Implementado: ✅ = implementado (handler presente), ❌ = pendiente / sin handler.  
Para opcodes que el hardware real define pero que aquí se tratan como NOP (placeholder) se marca ✅ (NOP) con nota.

> NOTA: Esta primera versión lista los opcodes actualmente detectados como implementados en el archivo `cpu6809.rs`. Si faltan filas (p.ej. instrucciones menos usadas) se pueden añadir iterativamente.

## 1. Tabla Resumen (Conteo)

- Primarios implementados: 256 / 256 (incluyendo ilegales tratados como NOP controlado)
- Prefijo 0x10 implementados (válidos): 31 / 31
- Prefijo 0x11 implementados (válidos): 16 / 16
- **VIA 6522 registros implementados: 8 / 16** (solo timers, SR, IFR/IER con handlers específicos)
- No implementados (válidos): 0 (CPU); 8 (VIA registros)

## 2. Opcodes Primarios (00–FF)

| Opcode | Mnemonic | Descripción breve | Implementado |
|--------|----------|-------------------|--------------|
| 0x00 | NEG (direct) | Negar byte en memoria (direct) | ✅ |
| 0x03 | COM (direct) | Complemento byte (direct) | ✅ |
| 0x04 | LSR (direct) | Desplaza derecha lógico memoria | ✅ |
| 0x06 | ROR (direct) | Rotate right con carry memoria | ✅ |
| 0x07 | ASR (direct) | Shift aritmético derecha memoria | ✅ |
| 0x08 | ASL (direct) | Shift lógico/aritm izq memoria | ✅ |
| 0x09 | ROL (direct) | Rotate left con carry memoria | ✅ |
| 0x0A | DEC (direct) | Decremento memoria | ✅ |
| 0x0B | SEV | Set V flag | ✅ |
| 0x0C | INC (direct) | Incremento memoria | ✅ |
| 0x0D | TST (direct) | Test (N,Z) memoria | ✅ |
| 0x0E | JMP (direct) | Salto directo | ✅ |
| 0x0F | CLR (direct) | Pone 0 en memoria | ✅ |
| 0x12 | NOP | No operación | ✅ |
| 0x13 | SYNC | Espera hasta IRQ/FIRQ/NMI (no apila) | ✅ |
| 0x16 | LBRA | Long branch always | ✅ |
| 0x18 | (NOP*) | Tratado como NOP en emulador | ✅ (NOP) |
| 0x19 | DAA | Ajuste decimal acumulador A | ✅ |
| 0x1A | ORCC | OR con registro CC | ✅ |
| 0x1B | ABA | A = A + B | ✅ |
| 0x1C | ANDCC | AND con CC | ✅ |
| 0x1D | SEX | Sign extend B -> D | ✅ |
| 0x1E | EXG | Intercambia registros | ✅ |
| 0x1F | TFR | Transfiere registro | ✅ |
| 0x20 | BRA | Branch always corto | ✅ |
| 0x21 | BRN | Branch never | ✅ |
| 0x22 | BHI | Branch if hi | ✅ |
| 0x23 | BLS | Branch if low/ same | ✅ |
| 0x24 | BCC/LBHS | Branch if Carry clear | ✅ |
| 0x25 | BCS/LBLO | Branch if Carry set | ✅ |
| 0x26 | BNE | Branch if Z=0 | ✅ |
| 0x27 | BEQ | Branch if Z=1 | ✅ |
| 0x28 | BVC | Branch if V=0 | ✅ |
| 0x29 | BVS | Branch if V=1 | ✅ |
| 0x2A | BPL | Branch if N=0 | ✅ |
| 0x2B | BMI | Branch if N=1 | ✅ |
| 0x2C | BGE | Branch if N^V=0 | ✅ |
| 0x2D | BLT | Branch if N^V=1 | ✅ |
| 0x2E | BGT | Branch if Z=0 & N^V=0 | ✅ |
| 0x2F | BLE | Branch if Z=1 or N^V=1 | ✅ |
| 0x30 | LEAX | Load Effective Address X | ✅ |
| 0x31 | LEAY | LEA Y | ✅ |
| 0x32 | LEAS | LEA S | ✅ |
| 0x33 | LEAU | LEA U | ✅ |
| 0x34 | PSHS | Push selected regs S | ✅ |
| 0x35 | PULS | Pull selected regs S | ✅ |
| 0x36 | PSHU | Push regs using U | ✅ |
| 0x37 | PULU | Pull regs using U | ✅ |
| 0x38 | (NOP*) | Marcado NOP local | ✅ (NOP) |
| 0x39 | RTS | Return subrutina | ✅ |
| 0x3B | RTI | Return from interrupt | ✅ |
| 0x3C | CWAI | AND CC con máscara inmediata; push frame completo y entra en wait | ✅ |
| 0x3D | MUL | A*B -> D (8x8=16) flags(Z,N,C,V=0) | ✅ |
| 0x3E | WAI | Halt hasta interrupt | ✅ |
| 0x3F | SWI | Software interrupt | ✅ |
| 0x40 | NEGA | Neg A | ✅ |
| 0x43 | COMA | Complemento A | ✅ |
| 0x44 | LSRA | LSR A | ✅ |
| 0x46 | RORA | ROR A | ✅ |
| 0x47 | ASRA | ASR A | ✅ |
| 0x48 | ASLA | ASL A | ✅ |
| 0x49 | ROLA | ROL A | ✅ |
| 0x4C | INCA | Inc A | ✅ |
| 0x4D | TSTA | Test A | ✅ |
| 0x4F | CLRA | Clear A | ✅ |
| 0x50 | NEGB | Neg B | ✅ |
| 0x53 | COMB | Complemento B | ✅ |
| 0x54 | LSRB | LSR B | ✅ |
| 0x56 | RORB | ROR B | ✅ |
| 0x57 | ASRB | ASR B | ✅ |
| 0x58 | ASLB | ASL B | ✅ |
| 0x59 | ROLB | ROL B | ✅ |
| 0x5A | DECB | Dec B | ✅ |
| 0x5C | INCB | Inc B | ✅ |
| 0x5D | TSTB | Test B | ✅ |
| 0x5F | CLRB | Clear B | ✅ |
| 0x60 | NEG (indexed) | Neg memoria idx | ✅ |
| 0x63 | COM (indexed) | Complemento idx | ✅ |
| 0x64 | LSR (indexed) | LSR idx | ✅ |
| 0x66 | ROR (indexed) | ROR idx | ✅ |
| 0x67 | ASR (indexed) | ASR idx | ✅ |
| 0x68 | ASL (indexed) | ASL idx | ✅ |
| 0x69 | ROL (indexed) | ROL idx | ✅ |
| 0x6A | DEC (indexed) | Dec idx | ✅ |
| 0x6C | INC (indexed) | Inc idx | ✅ |
| 0x6D | TST (indexed) | Test idx | ✅ |
| 0x6E | JMP (indexed) | Jump idx | ✅ |
| 0x6F | CLR (indexed) | Clear idx | ✅ |
| 0x70 | NEG (extended) | Neg memoria ext | ✅ |
| 0x73 | COM (extended) | Complemento ext | ✅ |
| 0x74 | LSR (extended) | LSR ext | ✅ |
| 0x76 | ROR (extended) | ROR ext | ✅ |
| 0x77 | ASR (extended) | ASR ext | ✅ |
| 0x78 | ASL (extended) | ASL ext | ✅ |
| 0x79 | ROL (extended) | ROL ext | ✅ |
| 0x7A | DEC (extended) | Dec ext | ✅ |
| 0x7C | INC (extended) | Inc ext | ✅ |
| 0x7D | TST (extended) | Test ext | ✅ |
| 0x7E | JMP (extended) | Jump ext | ✅ |
| 0x7F | CLR (extended) | Clear ext | ✅ |
| 0x80 | SUBA # | Resta imm a A | ✅ |
| 0x81 | CMPA # | Compara A imm | ✅ |
| 0x82 | SBCA # | Sub con carry A | ✅ |
| 0x83 | SUBD # | Resta imm16 a D | ✅ |
| 0x84 | ANDA # | AND A imm | ✅ |
| 0x85 | BITA # | Test A & imm | ✅ |
| 0x86 | LDA # | Load A imm | ✅ |
| 0x88 | EORA # | XOR A imm | ✅ |
| 0x89 | ADCA # | Add c A imm | ✅ |
| 0x8A | ORA # | OR A imm | ✅ |
| 0x8B | ADDA # | Add A imm | ✅ |
| 0x8D | BSR | Branch to subroutine | ✅ |
| 0x8E | LDX # | Load X imm | ✅ |
| 0x90 | SUBA direct | Resta mem direct | ✅ |
| 0x91 | CMPA direct | Compara A mem | ✅ |
| 0x93 | SUBD direct | Resta D mem16 | ✅ |
| 0x94 | ANDA direct | AND A mem | ✅ |
| 0x96 | LDA direct | Load A mem | ✅ |
| 0x97 | STA direct | Store A mem | ✅ |
| 0x98 | EORA direct | XOR A mem | ✅ |
| 0x99 | ADCA direct | Add c A mem | ✅ |
| 0x9A | ORA direct | OR A mem | ✅ |
| 0x9B | ADDA direct | Add A mem | ✅ |
| 0x9C | CMPX direct | Compara X mem16 | ✅ |
| 0x9D | JSR direct | Jump subr direct | ✅ |
| 0x9E | LDX direct | Load X mem | ✅ |
| 0x9F | STX direct | Store X mem | ✅ |
| 0xA0 | SUBA idx | Resta A idx | ✅ |
| 0xA1 | CMPA idx | Compara A idx | ✅ |
| 0xA2 | SBCA idx | Sub carry A idx | ✅ |
| 0xA3 | SUBD idx | Resta D idx | ✅ |
| 0xA4 | ANDA idx | AND A idx | ✅ |
| 0xA5 | BITA idx | Test A idx | ✅ |
| 0xA6 | LDA idx | Load A idx | ✅ |
| 0xA7 | STA idx | Store A idx | ✅ |
| 0xA8 | EORA idx | XOR A idx | ✅ |
| 0xA9 | ADCA idx | Add c A idx | ✅ |
| 0xAA | ORA idx | OR A idx | ✅ |
| 0xAB | ADDA idx | Add A idx | ✅ |
| 0xAE | LDX idx | Load X idx | ✅ |
| 0xAF | STX idx | Store X idx | ✅ |
| 0xB1 | CMPA ext | Compara A ext | ✅ |
| 0xB3 | SUBD ext | Resta D ext | ✅ |
| 0xB4 | ANDA ext | AND A ext | ✅ |
| 0xB6 | LDA ext | Load A ext | ✅ |
| 0xB7 | STA ext | Store A ext | ✅ |
| 0xB9 | ADCA ext | Add c A ext | ✅ |
| 0xBB | ADDA ext | Add A ext | ✅ |
| 0xBE | LDX ext | Load X ext | ✅ |
| 0xBF | STX ext | Store X ext | ✅ |
| 0xC0 | SUBB # | Resta B imm | ✅ |
| 0xC1 | CMPB # | Compara B imm | ✅ |
| 0xC3 | ADDD # | Suma D imm16 | ✅ |
| 0xC4 | ANDB # | AND B imm | ✅ |
| 0xC5 | BITB # | Test B & imm | ✅ |
| 0xC6 | LDB # | Load B imm | ✅ |
| 0xC8 | EORB # | XOR B imm | ✅ |
| 0xC9 | ADCB # | Add c B imm | ✅ |
| 0xCA | ORB # | OR B imm | ✅ |
| 0xCB | ADDB # | Add B imm | ✅ |
| 0xCC | LDD # | Load D imm | ✅ |
| 0xCE | LDU # | Load U imm | ✅ |
| 0xD0 | SUBB direct | Resta B mem | ✅ |
| 0xD1 | CMPB direct | Compara B mem | ✅ |
| 0xD4 | ANDB direct | AND B mem | ✅ |
| 0xD5 | BITB direct | Test B mem | ✅ |
| 0xD6 | LDB direct | Load B mem | ✅ |
| 0xD7 | STB direct | Store B mem | ✅ |
| 0xD8 | EORB direct | XOR B mem | ✅ |
| 0xDA | ORB direct | OR B mem | ✅ |
| 0xDB | ADDB direct | Add B mem | ✅ |
| 0xDC | LDD direct | Load D mem16 | ✅ |
| 0xDD | STD direct | Store D mem16 | ✅ |
| 0xDE | LDU direct | Load U mem16 | ✅ |
| 0xDF | STU direct | Store U mem16 | ✅ |
| 0xE0 | SUBB idx | Resta B idx | ✅ |
| 0xE1 | CMPB idx | Compara B idx | ✅ |
| 0xE3 | ADDD idx | Suma D idx | ✅ |
| 0xE4 | ANDB idx | AND B idx | ✅ |
| 0xE6 | LDB idx | Load B idx | ✅ |
| 0xE7 | STB idx | Store B idx | ✅ |
| 0xE8 | EORB idx | XOR B idx | ✅ |
| 0xEA | ORB idx | OR B idx | ✅ |
| 0xEB | ADDB idx | Add B idx | ✅ |
| 0xEC | LDD idx | Load D idx | ✅ |
| 0xED | STD idx | Store D idx | ✅ |
| 0xEE | LDU idx | Load U idx | ✅ |
| 0xEF | STU idx | Store U idx | ✅ |
| 0xF0 | SUBB ext | Resta B ext | ✅ |
| 0xF1 | CMPB ext | Compara B ext | ✅ |
| 0xF2 | SBCB ext | Sub carry B ext | ✅ |
| 0xF3 | ADDD ext | Suma D ext | ✅ |
| 0xF4 | ANDB ext | AND B ext | ✅ |
| 0xF5 | BITB ext | Test B ext | ✅ |
| 0xF6 | LDB ext | Load B ext | ✅ |
| 0xF7 | STB ext | Store B ext | ✅ |
| 0xF8 | EORB ext | XOR B ext | ✅ |
| 0xF9 | ADCB ext | Add c B ext | ✅ |
| 0xFA | ORB ext | OR B ext | ✅ |
| 0xFC | LDD ext | Load D ext | ✅ |
| 0xFD | STD ext | Store D ext | ✅ |
| 0xFE | LDU ext | Load U ext | ✅ |
| 0xFF | STU ext | Store U ext | ✅ |

### Prefijo 0x10 (Página extendida 1)
Listado completo de sub‑opcodes válidos (cualquier otro se trata como ilegal / no asignado y no cuenta como brecha):

| Opcode (10 xx) | Mnemonic | Descripción | Impl |
|----------------|----------|-------------|------|
| 0x10 0x3F | SWI2 | Software interrupt 2 | ✅ |
| 0x10 0x8E | LDY # | Load Y imm | ✅ |
| 0x10 0x9E | LDY direct | Load Y mem | ✅ |
| 0x10 0xAE | LDY idx | Load Y idx | ✅ |
| 0x10 0xBE | LDY ext | Load Y ext | ✅ |
| 0x10 0x9F | STY direct | Store Y mem | ✅ |
| 0x10 0xAF | STY idx | Store Y idx | ✅ |
| 0x10 0xBF | STY ext | Store Y ext | ✅ |
| 0x10 0x83 | CMPD # | Compare D imm | ✅ |
| 0x10 0x93 | CMPD direct | Compare D mem | ✅ |
| 0x10 0xA3 | CMPD idx | Compare D idx | ✅ |
| 0x10 0xB3 | CMPD ext | Compare D ext | ✅ |
| 0x10 0x8C | CMPY # | Compare Y imm | ✅ |
| 0x10 0x9C | CMPY direct | Compare Y mem | ✅ |
| 0x10 0xAC | CMPY idx | Compare Y idx | ✅ |
| 0x10 0xBC | CMPY ext | Compare Y ext | ✅ |
| 0x10 0xDE | LDS direct | Load S mem | ✅ |
| 0x10 0xEE | LDS idx | Load S idx | ✅ |
| 0x10 0xFE | LDS ext | Load S ext | ✅ |
| 0x10 0xDF | STS direct | Store S mem | ✅ |
| 0x10 0xEF | STS idx | Store S idx | ✅ |
| 0x10 0xFF | STS ext | Store S ext | ✅ |

### Prefijo 0x11 (Página extendida 2)

| Opcode (11 xx) | Mnemonic | Descripción | Impl |
|----------------|----------|-------------|------|
| 0x11 0x3F | SWI3 | Software interrupt 3 | ✅ |
| 0x11 0x83 | CMPU # | Compare U imm | ✅ |
| 0x11 0x93 | CMPU direct | Compare U mem | ✅ |
| 0x11 0xA3 | CMPU idx | Compare U idx | ✅ |
| 0x11 0xB3 | CMPU ext | Compare U ext | ✅ |
| 0x11 0x8C | CMPS # | Compare S imm | ✅ |
| 0x11 0x9C | CMPS direct | Compare S mem | ✅ |
| 0x11 0xAC | CMPS idx | Compare S idx | ✅ |
| 0x11 0xBC | CMPS ext | Compare S ext | ✅ |

## 3. VIA 6522 Registros
(Asignaciones confirmadas: coincide con mapeo en `bus.rs` y `via6522.rs` – lectura IFR master bit sintetizado, timers resetean flags al leer alto, etc.)

| Dirección | Nombre | Descripción | Implementado |
|-----------|--------|------------|--------------|
| 0xD000 | ORB/IRB | Puerto B / Entrada | ❌ |
| 0xD001 | ORA/IRA | Puerto A / Entrada | ❌ |
| 0xD002 | DDRB | Data Direction B | ❌ |
| 0xD003 | DDRA | Data Direction A | ❌ |
| 0xD004 | T1CL | Timer1 Counter Low | ✅ |
| 0xD005 | T1CH | Timer1 Counter High | ✅ |
| 0xD006 | T1LL | Timer1 Latch Low | ❌ |
| 0xD007 | T1LH | Timer1 Latch High | ❌ |
| 0xD008 | T2CL | Timer2 Counter Low | ✅ |
| 0xD009 | T2CH | Timer2 Counter High | ✅ |
| 0xD00A | SR | Shift Register | ✅ |
| 0xD00B | ACR | Aux Control Reg | ✅ |
| 0xD00C | PCR | Peripheral Control | ❌ |
| 0xD00D | IFR | Interrupt Flag | ✅ |
| 0xD00E | IER | Interrupt Enable | ✅ |
| 0xD00F | ORA2 | Registro espejo / handshake | ❌ |

## 4. Pendientes / Notas
- Ciclos: varias instrucciones aún usan tiempos agrupados aproximados; pendiente tabla exacta por modo.
- Flags: Validar exhaustivamente DAA contra vectores oficiales; añadir test de MUL (cálculo C=bit15) y CWAI (estado WAI + frame push único).
- Instrucciones ilegales (0x01,0x02,0x05,0x45,0x4E,0x52, placeholders 0x7B,0x8F si aparecen) se tratan como NOP y registran cobertura como implementadas (NOP) para no contaminar métrica.
- Exportar JSON de cobertura extendida para UI (lista `extended_unimplemented`).
 - Listado explícito de ilegales manejados como NOP para trazabilidad: 0x01,0x02,0x05,0x18,0x38,0x45,0x4E,0x52,0x7B,0x8F (si el hardware los clasifica distintos, documentar divergencia en próxima revisión).

## 5. Tabla de Ciclos Emulados (Snapshot)
Metodología: ejecución sintética de cada opcode individual en un CPU clonado con `gen_cycles` (bin añadido) que coloca el opcode en $0100 y ejecuta un único `step()`, midiendo `cycles` delta. Los prefijos 0x10 y 0x11 reportan 0 ciclos porque el coste real se consume al ejecutar el sub‑opcode (separado en filas EXT10/EXT11). Esto refleja el modelo actual (agrupación aproximada) y no necesariamente los tiempos oficiales del 6809.

Archivo generado: `cycles.csv` (en raíz de workspace tras correr el bin). Formato:
```
type,opcode,sub,cycles
PRIMARY,8E,,3
EXT10,10,8E,5
...
```

Observaciones rápidas:
- Prefijos (0x10/0x11) = 0 ciclos previos (se podría ajustar para sumar 1 ciclo de fetch adicional según tablas oficiales si se desea precisión futura).
- Instrucciones ilegales tratadas como NOP = 1 ciclo actualmente.
- RMW y saltos largos muestran variación (e.g. LBRA = 5 ciclos en este modelo simplificado).

Próximos pasos recomendados para exactitud:
1. Incorporar tabla oficial (Motorola) y columna "nominal" para comparar.
2. Ajustar `cyc` por modo de direccionamiento (actualmente varios modos comparten seeds genéricos).
3. Integrar verificación automática: fallo si desviación > tolerancia (configurable).
4. Añadir campo de ciclos a la exportación de métricas para introspección en la UI (agregado opcional).

Para regenerar:
```
cargo run -p vectrex_emulator --bin gen_cycles > cycles.csv
```

### 5.1 Discrepancias vs Nominal (Resumen)
Fuente: `gen_cycles_compare` + `6809_cycles_nominal.json`.

Estado actual: Principales desvíos corregidos (JMP, SYNC, SEX, EXG, BRN, WAI, CWAI). Los opcodes auditados muestran Δ=0 para las entradas ajustadas.

Nota: JMP extended (0x7E) se ajustó a 4 ciclos para coincidir con la tabla de `vectrexy` (anteriormente 3 en este emulador). Documentamos esta divergencia respecto a algunas tablas que listan 3; si se habilita modo "estricto Motorola" podría revertirse.

Próximos ajustes pendientes (si se amplía nominal JSON):
- Ilegales/NOP (ej. 0x01,0x02,0x05) emulados a 1 ciclo mientras nominal JSON marca 2 como placeholder; decidir criterio (mantener 1 para rendimiento o alinear a 2 por fidelidad).
- Modelar diferencia branch taken vs not taken (+1) si se requiere fidelidad completa.
- Completar tabla nominal para todos los modos faltantes (aritmética extendida, prefijos adicionales si se añaden nuevas instrucciones).
- Normalizar coste de instrucciones ilegales tratadas como NOP (decidir si 1 o 2 ciclos según referencia escogida).

Regenerar discrepancias:
```
cargo run -p vectrex_emulator --bin gen_cycles_compare > cycles_compare.csv
```

Esta sección debe actualizarse cuando cambie la lógica de determinación de `cyc` en `step()`.

## 5. Próximos Pasos Sugeridos
1. **Implementar registros VIA faltantes**: ORB/ORA (0xD000/0xD001), DDR A/B (0xD002/0xD003), T1 Latches (0xD006/0xD007), PCR (0xD00C), ORA2 (0xD00F) necesitan handlers específicos más allá del almacenamiento genérico.
2. Implementar CWAI si alguna ROM lo necesita.  
3. Ajustar tabla con conteos reales (script que recorra 0x00-0xFF y cruce con switch) y actualizar cabecera.  
4. Añadir columna de ciclos nominales y ciclos actuales emulados para auditoría.  
5. Exportar cobertura a JSON automáticamente en cada build (generar diff histórico).

## 6. Notas sobre Implementación VIA

**Registros con handlers específicos implementados:**
- Timers (0xD004/0xD005, 0xD008/0xD009): Lógica completa de contador, latch, IFR clear
- IFR/IER (0xD00D/0xD00E): Síntesis master bit, set/clear semantics, recompute_irq
- ACR (0xD00B): Control modes para timers/PB7
- SR (0xD00A): Shift register con modo 4

**Registros solo con almacenamiento genérico (funcionalidad limitada):**
- Puertos A/B (0xD000/0xD001): Solo `regs[r] = val` sin lógica de dirección
- DDR A/B (0xD002/0xD003): Solo almacenamiento, **la lógica DDR real está en CPU**
- T1 Latches (0xD006/0xD007): Solo almacenamiento, sin diferenciación vs counter
- PCR (0xD00C): Solo almacenamiento sin control de líneas CA1/CA2/CB1/CB2
- ORA2 (0xD00F): Solo almacenamiento sin handshake

**Implicación para vectores de texto:** Como se descubrió, los DDR A/B están manejados en el CPU (`cpu.ddr_a`/`cpu.ddr_b`) y afectan directamente si el integrator se actualiza. La inicialización a 0xFF (output mode) fue crítica para que los vectores de texto aparezcan.

---
_Generado automáticamente (versión inicial). Actualizar manualmente o automatizar script según se agreguen cambios._
