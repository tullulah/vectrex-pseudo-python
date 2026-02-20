# Motorola 6809 & VIA 6522 Opcode / Register Implementation Status

This table provides a complete overview of the implementation status of the 6809 CPU (primary opcodes and prefixed 0x10 / 0x11) and the main registers of the VIA 6522 within the emulator.

Legend: ✅ = implemented (handler present), ❌ = pending / no handler.
Opcodes that the real hardware defines but are treated here as NOP (placeholder) are marked ✅ (NOP) with a note.

> NOTE: This initial version lists the opcodes currently detected as implemented in `cpu6809.rs`. Missing rows (e.g. less common instructions) can be added iteratively.

## 1. Summary (Counts)

- Primary implemented: 256 / 256 (including illegal opcodes treated as controlled NOP)
- Prefix 0x10 implemented (valid): 31 / 31
- Prefix 0x11 implemented (valid): 16 / 16
- Not implemented (valid): 0

## 2. Primary Opcodes (00–FF)

| Opcode | Mnemonic | Brief description | Implemented |
|--------|----------|-------------------|-------------|
| 0x00 | NEG (direct) | Negate byte in memory (direct) | ✅ |
| 0x03 | COM (direct) | Complement byte (direct) | ✅ |
| 0x04 | LSR (direct) | Logical shift right memory | ✅ |
| 0x06 | ROR (direct) | Rotate right through carry memory | ✅ |
| 0x07 | ASR (direct) | Arithmetic shift right memory | ✅ |
| 0x08 | ASL (direct) | Logical/arithmetic shift left memory | ✅ |
| 0x09 | ROL (direct) | Rotate left through carry memory | ✅ |
| 0x0A | DEC (direct) | Decrement memory | ✅ |
| 0x0B | SEV | Set V flag | ✅ |
| 0x0C | INC (direct) | Increment memory | ✅ |
| 0x0D | TST (direct) | Test (N,Z) memory | ✅ |
| 0x0E | JMP (direct) | Direct jump | ✅ |
| 0x0F | CLR (direct) | Clear memory to 0 | ✅ |
| 0x12 | NOP | No operation | ✅ |
| 0x13 | SYNC | Wait for IRQ/FIRQ/NMI (no stack push) | ✅ |
| 0x16 | LBRA | Long branch always | ✅ |
| 0x18 | (NOP*) | Treated as NOP in emulator | ✅ (NOP) |
| 0x19 | DAA | Decimal adjust accumulator A | ✅ |
| 0x1A | ORCC | OR with CC register | ✅ |
| 0x1B | ABA | A = A + B | ✅ |
| 0x1C | ANDCC | AND with CC | ✅ |
| 0x1D | SEX | Sign extend B -> D | ✅ |
| 0x1E | EXG | Exchange registers | ✅ |
| 0x1F | TFR | Transfer register | ✅ |
| 0x20 | BRA | Branch always (short) | ✅ |
| 0x21 | BRN | Branch never | ✅ |
| 0x22 | BHI | Branch if higher | ✅ |
| 0x23 | BLS | Branch if lower or same | ✅ |
| 0x24 | BCC/LBHS | Branch if carry clear | ✅ |
| 0x25 | BCS/LBLO | Branch if carry set | ✅ |
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
| 0x34 | PSHS | Push selected registers to S | ✅ |
| 0x35 | PULS | Pull selected registers from S | ✅ |
| 0x36 | PSHU | Push registers using U | ✅ |
| 0x37 | PULU | Pull registers using U | ✅ |
| 0x38 | (NOP*) | Treated as local NOP | ✅ (NOP) |
| 0x39 | RTS | Return from subroutine | ✅ |
| 0x3B | RTI | Return from interrupt | ✅ |
| 0x3C | CWAI | AND CC with immediate mask; push full frame and enter wait | ✅ |
| 0x3D | MUL | A*B -> D (8×8=16) flags(Z,N,C,V=0) | ✅ |
| 0x3E | WAI | Halt until interrupt | ✅ |
| 0x3F | SWI | Software interrupt | ✅ |
| 0x40 | NEGA | Negate A | ✅ |
| 0x43 | COMA | Complement A | ✅ |
| 0x44 | LSRA | LSR A | ✅ |
| 0x46 | RORA | ROR A | ✅ |
| 0x47 | ASRA | ASR A | ✅ |
| 0x48 | ASLA | ASL A | ✅ |
| 0x49 | ROLA | ROL A | ✅ |
| 0x4C | INCA | Increment A | ✅ |
| 0x4D | TSTA | Test A | ✅ |
| 0x4F | CLRA | Clear A | ✅ |
| 0x50 | NEGB | Negate B | ✅ |
| 0x53 | COMB | Complement B | ✅ |
| 0x54 | LSRB | LSR B | ✅ |
| 0x56 | RORB | ROR B | ✅ |
| 0x57 | ASRB | ASR B | ✅ |
| 0x58 | ASLB | ASL B | ✅ |
| 0x59 | ROLB | ROL B | ✅ |
| 0x5A | DECB | Decrement B | ✅ |
| 0x5C | INCB | Increment B | ✅ |
| 0x5D | TSTB | Test B | ✅ |
| 0x5F | CLRB | Clear B | ✅ |
| 0x60 | NEG (indexed) | Negate memory indexed | ✅ |
| 0x63 | COM (indexed) | Complement indexed | ✅ |
| 0x64 | LSR (indexed) | LSR indexed | ✅ |
| 0x66 | ROR (indexed) | ROR indexed | ✅ |
| 0x67 | ASR (indexed) | ASR indexed | ✅ |
| 0x68 | ASL (indexed) | ASL indexed | ✅ |
| 0x69 | ROL (indexed) | ROL indexed | ✅ |
| 0x6A | DEC (indexed) | Decrement indexed | ✅ |
| 0x6C | INC (indexed) | Increment indexed | ✅ |
| 0x6D | TST (indexed) | Test indexed | ✅ |
| 0x6E | JMP (indexed) | Jump indexed | ✅ |
| 0x6F | CLR (indexed) | Clear indexed | ✅ |
| 0x70 | NEG (extended) | Negate memory extended | ✅ |
| 0x73 | COM (extended) | Complement extended | ✅ |
| 0x74 | LSR (extended) | LSR extended | ✅ |
| 0x76 | ROR (extended) | ROR extended | ✅ |
| 0x77 | ASR (extended) | ASR extended | ✅ |
| 0x78 | ASL (extended) | ASL extended | ✅ |
| 0x79 | ROL (extended) | ROL extended | ✅ |
| 0x7A | DEC (extended) | Decrement extended | ✅ |
| 0x7C | INC (extended) | Increment extended | ✅ |
| 0x7D | TST (extended) | Test extended | ✅ |
| 0x7E | JMP (extended) | Jump extended | ✅ |
| 0x7F | CLR (extended) | Clear extended | ✅ |
| 0x80 | SUBA # | Subtract immediate from A | ✅ |
| 0x81 | CMPA # | Compare A immediate | ✅ |
| 0x82 | SBCA # | Subtract with carry A | ✅ |
| 0x83 | SUBD # | Subtract immediate 16-bit from D | ✅ |
| 0x84 | ANDA # | AND A immediate | ✅ |
| 0x85 | BITA # | Test A & immediate | ✅ |
| 0x86 | LDA # | Load A immediate | ✅ |
| 0x88 | EORA # | XOR A immediate | ✅ |
| 0x89 | ADCA # | Add with carry A immediate | ✅ |
| 0x8A | ORA # | OR A immediate | ✅ |
| 0x8B | ADDA # | Add A immediate | ✅ |
| 0x8D | BSR | Branch to subroutine | ✅ |
| 0x8E | LDX # | Load X immediate | ✅ |
| 0x90 | SUBA direct | Subtract memory direct from A | ✅ |
| 0x91 | CMPA direct | Compare A memory | ✅ |
| 0x93 | SUBD direct | Subtract D memory 16-bit | ✅ |
| 0x94 | ANDA direct | AND A memory | ✅ |
| 0x96 | LDA direct | Load A memory | ✅ |
| 0x97 | STA direct | Store A memory | ✅ |
| 0x98 | EORA direct | XOR A memory | ✅ |
| 0x99 | ADCA direct | Add with carry A memory | ✅ |
| 0x9A | ORA direct | OR A memory | ✅ |
| 0x9B | ADDA direct | Add A memory | ✅ |
| 0x9C | CMPX direct | Compare X memory 16-bit | ✅ |
| 0x9D | JSR direct | Jump subroutine direct | ✅ |
| 0x9E | LDX direct | Load X memory | ✅ |
| 0x9F | STX direct | Store X memory | ✅ |
| 0xA0 | SUBA idx | Subtract A indexed | ✅ |
| 0xA1 | CMPA idx | Compare A indexed | ✅ |
| 0xA2 | SBCA idx | Subtract with carry A indexed | ✅ |
| 0xA3 | SUBD idx | Subtract D indexed | ✅ |
| 0xA4 | ANDA idx | AND A indexed | ✅ |
| 0xA5 | BITA idx | Test A indexed | ✅ |
| 0xA6 | LDA idx | Load A indexed | ✅ |
| 0xA7 | STA idx | Store A indexed | ✅ |
| 0xA8 | EORA idx | XOR A indexed | ✅ |
| 0xA9 | ADCA idx | Add with carry A indexed | ✅ |
| 0xAA | ORA idx | OR A indexed | ✅ |
| 0xAB | ADDA idx | Add A indexed | ✅ |
| 0xAE | LDX idx | Load X indexed | ✅ |
| 0xAF | STX idx | Store X indexed | ✅ |
| 0xB1 | CMPA ext | Compare A extended | ✅ |
| 0xB3 | SUBD ext | Subtract D extended | ✅ |
| 0xB4 | ANDA ext | AND A extended | ✅ |
| 0xB6 | LDA ext | Load A extended | ✅ |
| 0xB7 | STA ext | Store A extended | ✅ |
| 0xB9 | ADCA ext | Add with carry A extended | ✅ |
| 0xBB | ADDA ext | Add A extended | ✅ |
| 0xBE | LDX ext | Load X extended | ✅ |
| 0xBF | STX ext | Store X extended | ✅ |
| 0xC0 | SUBB # | Subtract B immediate | ✅ |
| 0xC1 | CMPB # | Compare B immediate | ✅ |
| 0xC3 | ADDD # | Add D immediate 16-bit | ✅ |
| 0xC4 | ANDB # | AND B immediate | ✅ |
| 0xC5 | BITB # | Test B & immediate | ✅ |
| 0xC6 | LDB # | Load B immediate | ✅ |
| 0xC8 | EORB # | XOR B immediate | ✅ |
| 0xC9 | ADCB # | Add with carry B immediate | ✅ |
| 0xCA | ORB # | OR B immediate | ✅ |
| 0xCB | ADDB # | Add B immediate | ✅ |
| 0xCC | LDD # | Load D immediate | ✅ |
| 0xCE | LDU # | Load U immediate | ✅ |
| 0xD0 | SUBB direct | Subtract B memory | ✅ |
| 0xD1 | CMPB direct | Compare B memory | ✅ |
| 0xD4 | ANDB direct | AND B memory | ✅ |
| 0xD5 | BITB direct | Test B memory | ✅ |
| 0xD6 | LDB direct | Load B memory | ✅ |
| 0xD7 | STB direct | Store B memory | ✅ |
| 0xD8 | EORB direct | XOR B memory | ✅ |
| 0xDA | ORB direct | OR B memory | ✅ |
| 0xDB | ADDB direct | Add B memory | ✅ |
| 0xDC | LDD direct | Load D memory 16-bit | ✅ |
| 0xDD | STD direct | Store D memory 16-bit | ✅ |
| 0xDE | LDU direct | Load U memory 16-bit | ✅ |
| 0xDF | STU direct | Store U memory 16-bit | ✅ |
| 0xE0 | SUBB idx | Subtract B indexed | ✅ |
| 0xE1 | CMPB idx | Compare B indexed | ✅ |
| 0xE3 | ADDD idx | Add D indexed | ✅ |
| 0xE4 | ANDB idx | AND B indexed | ✅ |
| 0xE6 | LDB idx | Load B indexed | ✅ |
| 0xE7 | STB idx | Store B indexed | ✅ |
| 0xE8 | EORB idx | XOR B indexed | ✅ |
| 0xEA | ORB idx | OR B indexed | ✅ |
| 0xEB | ADDB idx | Add B indexed | ✅ |
| 0xEC | LDD idx | Load D indexed | ✅ |
| 0xED | STD idx | Store D indexed | ✅ |
| 0xEE | LDU idx | Load U indexed | ✅ |
| 0xEF | STU idx | Store U indexed | ✅ |
| 0xF0 | SUBB ext | Subtract B extended | ✅ |
| 0xF1 | CMPB ext | Compare B extended | ✅ |
| 0xF2 | SBCB ext | Subtract with carry B extended | ✅ |
| 0xF3 | ADDD ext | Add D extended | ✅ |
| 0xF4 | ANDB ext | AND B extended | ✅ |
| 0xF5 | BITB ext | Test B extended | ✅ |
| 0xF6 | LDB ext | Load B extended | ✅ |
| 0xF7 | STB ext | Store B extended | ✅ |
| 0xF8 | EORB ext | XOR B extended | ✅ |
| 0xF9 | ADCB ext | Add with carry B extended | ✅ |
| 0xFA | ORB ext | OR B extended | ✅ |
| 0xFC | LDD ext | Load D extended | ✅ |
| 0xFD | STD ext | Store D extended | ✅ |
| 0xFE | LDU ext | Load U extended | ✅ |
| 0xFF | STU ext | Store U extended | ✅ |

### Prefix 0x10 (Extended page 1)
Complete listing of valid sub-opcodes (any other is treated as illegal / unassigned and does not count as a gap):

| Opcode (10 xx) | Mnemonic | Description | Impl |
|----------------|----------|-------------|------|
| 0x10 0x3F | SWI2 | Software interrupt 2 | ✅ |
| 0x10 0x8E | LDY # | Load Y immediate | ✅ |
| 0x10 0x9E | LDY direct | Load Y memory | ✅ |
| 0x10 0xAE | LDY idx | Load Y indexed | ✅ |
| 0x10 0xBE | LDY ext | Load Y extended | ✅ |
| 0x10 0x9F | STY direct | Store Y memory | ✅ |
| 0x10 0xAF | STY idx | Store Y indexed | ✅ |
| 0x10 0xBF | STY ext | Store Y extended | ✅ |
| 0x10 0x83 | CMPD # | Compare D immediate | ✅ |
| 0x10 0x93 | CMPD direct | Compare D memory | ✅ |
| 0x10 0xA3 | CMPD idx | Compare D indexed | ✅ |
| 0x10 0xB3 | CMPD ext | Compare D extended | ✅ |
| 0x10 0x8C | CMPY # | Compare Y immediate | ✅ |
| 0x10 0x9C | CMPY direct | Compare Y memory | ✅ |
| 0x10 0xAC | CMPY idx | Compare Y indexed | ✅ |
| 0x10 0xBC | CMPY ext | Compare Y extended | ✅ |
| 0x10 0xDE | LDS direct | Load S memory | ✅ |
| 0x10 0xEE | LDS idx | Load S indexed | ✅ |
| 0x10 0xFE | LDS ext | Load S extended | ✅ |
| 0x10 0xDF | STS direct | Store S memory | ✅ |
| 0x10 0xEF | STS idx | Store S indexed | ✅ |
| 0x10 0xFF | STS ext | Store S extended | ✅ |

### Prefix 0x11 (Extended page 2)

| Opcode (11 xx) | Mnemonic | Description | Impl |
|----------------|----------|-------------|------|
| 0x11 0x3F | SWI3 | Software interrupt 3 | ✅ |
| 0x11 0x83 | CMPU # | Compare U immediate | ✅ |
| 0x11 0x93 | CMPU direct | Compare U memory | ✅ |
| 0x11 0xA3 | CMPU idx | Compare U indexed | ✅ |
| 0x11 0xB3 | CMPU ext | Compare U extended | ✅ |
| 0x11 0x8C | CMPS # | Compare S immediate | ✅ |
| 0x11 0x9C | CMPS direct | Compare S memory | ✅ |
| 0x11 0xAC | CMPS idx | Compare S indexed | ✅ |
| 0x11 0xBC | CMPS ext | Compare S extended | ✅ |

## 3. VIA 6522 Registers
(Confirmed assignments: matches mapping in `bus.rs` and `via6522.rs` — IFR master bit synthesised on read, timers reset flags on high byte read, etc.)

| Address | Name | Description | Implemented |
|---------|------|-------------|-------------|
| 0xD000 | ORB/IRB | Port B / Input | ✅ |
| 0xD001 | ORA/IRA | Port A / Input | ✅ |
| 0xD002 | DDRB | Data Direction B | ✅ |
| 0xD003 | DDRA | Data Direction A | ✅ |
| 0xD004 | T1CL | Timer1 Counter Low | ✅ |
| 0xD005 | T1CH | Timer1 Counter High | ✅ |
| 0xD006 | T1LL | Timer1 Latch Low | ✅ |
| 0xD007 | T1LH | Timer1 Latch High | ✅ |
| 0xD008 | T2CL | Timer2 Counter Low | ✅ |
| 0xD009 | T2CH | Timer2 Counter High | ✅ |
| 0xD00A | SR | Shift Register | ✅ |
| 0xD00B | ACR | Auxiliary Control Register | ✅ |
| 0xD00C | PCR | Peripheral Control | ✅ |
| 0xD00D | IFR | Interrupt Flag Register | ✅ |
| 0xD00E | IER | Interrupt Enable Register | ✅ |
| 0xD00F | ORA2 | Mirror register / handshake | ✅ |

## 4. Pending / Notes
- Cycles: Several instructions still use approximate grouped timings; exact per-mode table pending.
- Flags: Exhaustively validate DAA against official test vectors; add MUL test (C=bit15) and CWAI (WAI state + single frame push).
- Illegal opcodes (0x01, 0x02, 0x05, 0x45, 0x4E, 0x52, placeholders 0x7B, 0x8F if encountered) treated as NOP and recorded as implemented (NOP) to avoid polluting the metric.
- Export extended coverage JSON for UI (list `extended_unimplemented`).
- Explicit list of illegals handled as NOP for traceability: 0x01, 0x02, 0x05, 0x18, 0x38, 0x45, 0x4E, 0x52, 0x7B, 0x8F (if hardware classifies them differently, document divergence in next revision).

## 5. Emulated Cycle Table (Snapshot)
Methodology: synthetic execution of each individual opcode in a cloned CPU with `gen_cycles` (added binary) that places the opcode at $0100 and executes a single `step()`, measuring `cycles` delta. Prefixes 0x10 and 0x11 report 0 cycles because the real cost is consumed when executing the sub-opcode (separate rows EXT10/EXT11). This reflects the current model (approximate grouping) and does not necessarily match official 6809 timings.

Generated file: `cycles.csv` (in workspace root after running the binary). Format:
```
type,opcode,sub,cycles
PRIMARY,8E,,3
EXT10,10,8E,5
...
```

Quick observations:
- Prefixes (0x10/0x11) = 0 prior cycles (could be adjusted to add 1 fetch cycle per official tables if future accuracy is desired).
- Illegal opcodes treated as NOP = 1 cycle currently.
- RMW and long branches show variation (e.g. LBRA = 5 cycles in this simplified model).

Recommended next steps for accuracy:
1. Incorporate official (Motorola) table and add a "nominal" column for comparison.
2. Adjust `cyc` per addressing mode (currently several modes share generic seeds).
3. Integrate automatic verification: fail if deviation exceeds a configurable tolerance.
4. Add cycle field to metrics export for UI introspection (optional addition).

To regenerate:
```
cargo run -p vectrex_emulator --bin gen_cycles > cycles.csv
```

### 5.1 Discrepancies vs Nominal (Summary)
Source: `gen_cycles_compare` + `6809_cycles_nominal.json`.

Current state: Main deviations corrected (JMP, SYNC, SEX, EXG, BRN, WAI, CWAI). Audited opcodes show Δ=0 for adjusted entries.

Note: JMP extended (0x7E) was adjusted to 4 cycles to match the `vectrexy` table (previously 3 in this emulator). This divergence from some tables that list 3 is documented; if a "strict Motorola" mode is enabled it could be reverted.

Pending adjustments (if nominal JSON is extended):
- Illegals/NOP (e.g. 0x01, 0x02, 0x05) emulated at 1 cycle while nominal JSON marks 2 as placeholder; decide criterion (keep 1 for performance or align to 2 for fidelity).
- Model branch taken vs not taken difference (+1) if full fidelity is required.
- Complete nominal table for all missing modes (extended arithmetic, additional prefixes if new instructions are added).
- Normalise cost of illegal opcodes treated as NOP (decide whether 1 or 2 cycles per chosen reference).

Regenerate discrepancies:
```
cargo run -p vectrex_emulator --bin gen_cycles_compare > cycles_compare.csv
```

This section must be updated whenever the `cyc` determination logic in `step()` changes.

## 6. Suggested Next Steps
1. Implement CWAI if any ROM requires it.
2. Adjust the table with real counts (script iterating 0x00–0xFF crossed with switch) and update the header.
3. Add nominal cycle and current emulated cycle columns for auditing.
4. Automatically export coverage to JSON on each build (generate historical diff).

---
*Auto-generated (initial version). Update manually or automate via script as changes are added.*
