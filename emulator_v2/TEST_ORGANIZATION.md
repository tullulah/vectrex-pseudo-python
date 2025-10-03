# Test Organization Structure

**Last Updated**: October 3, 2025  
**Total Tests**: 345 passing (1 ignored)

## Directory Structure

```
tests/
├── test_opcodes.rs          # Entry point for opcode tests
├── test_components.rs       # Entry point for component tests
├── test_integration.rs      # Entry point for integration tests
├── test_debug.rs            # Entry point for debug tests (temporary)
│
├── opcodes/                 # CPU Opcode Tests (258 tests)
│   ├── mod.rs
│   ├── arithmetic/          # Arithmetic & Logic Operations (66 tests)
│   │   ├── mod.rs
│   │   ├── test_indexed_arithmetic.rs
│   │   ├── test_addb_subb.rs
│   │   ├── test_logic_b.rs
│   │   ├── test_adda_variants.rs
│   │   ├── test_add_sub_opcodes.rs
│   │   ├── test_arithmetic_corrected.rs
│   │   ├── test_mul_sex_opcodes.rs
│   │   ├── test_and_eor_opcodes.rs
│   │   ├── test_or_opcodes.rs
│   │   ├── test_cmpa_opcodes.rs
│   │   ├── test_cmpb_opcodes.rs
│   │   ├── test_cmpd_opcodes.rs
│   │   ├── test_cmps_opcodes.rs
│   │   ├── test_cmpu_opcodes.rs
│   │   ├── test_cmpx_opcodes.rs
│   │   ├── test_cmpy_opcodes.rs
│   │   ├── test_b_register_opcodes.rs
│   │   └── test_register_control_opcodes.rs
│   │
│   ├── branch/              # Branch & Jump Operations (10 tests)
│   │   ├── mod.rs
│   │   ├── test_lbra_lbsr.rs
│   │   ├── test_jsr_indexed.rs
│   │   └── test_branch_extended_opcodes.rs
│   │
│   ├── data_transfer/       # Load, Store, Transfer (? tests)
│   │   ├── mod.rs
│   │   ├── test_store_16bit.rs
│   │   ├── test_lda.rs
│   │   ├── test_ldb.rs
│   │   ├── test_ldd.rs
│   │   ├── test_ldu.rs
│   │   ├── test_ldx.rs
│   │   ├── test_sta.rs
│   │   ├── test_stb.rs
│   │   ├── test_store_16bit_corrected.rs
│   │   └── test_lea_opcodes.rs
│   │
│   ├── misc/                # Miscellaneous Operations (? tests)
│   │   ├── mod.rs
│   │   ├── test_nop.rs
│   │   ├── test_sync.rs
│   │   ├── test_jmp_modes.rs
│   │   ├── test_tfr_exg_correct.rs
│   │   ├── test_condition_code_opcodes.rs
│   │   ├── test_nop_corrected.rs
│   │   ├── test_basic_opcodes_fixed.rs
│   │   ├── test_extended_addressing_opcodes.rs
│   │   └── test_minimal_opcodes.rs
│   │
│   ├── illegal/             # Illegal Opcodes (? tests)
│   │   ├── mod.rs
│   │   └── test_illegal_opcodes.rs
│   │
│   ├── reserved/            # Reserved Opcodes (16 tests)
│   │   ├── mod.rs
│   │   ├── test_reserved_0x01.rs
│   │   ├── test_reserved_0x02.rs
│   │   ├── test_reserved_0x05.rs
│   │   ├── test_reserved_0x0B.rs
│   │   ├── test_reserved_0x14.rs
│   │   ├── test_reserved_0x15.rs
│   │   ├── test_reserved_0x18.rs
│   │   └── test_reserved_0x1B.rs
│   │
│   └── interrupt/           # Interrupt Operations (4 tests)
│       ├── mod.rs
│       └── test_rti_swi_cwai.rs
│
├── components/              # Component Tests (? tests)
│   ├── mod.rs
│   ├── hardware/            # Hardware Components (? tests)
│   │   ├── mod.rs
│   │   ├── test_psg.rs
│   │   ├── test_screen.rs
│   │   ├── test_shift_register.rs
│   │   └── test_timers.rs
│   │
│   ├── engine/              # Engine Components (7 tests)
│   │   ├── mod.rs
│   │   ├── test_delayed_value_store.rs
│   │   └── test_engine_types.rs
│   │
│   └── memory/              # Memory Devices (? tests)
│       ├── mod.rs
│       └── test_dev_memory_device.rs
│
├── integration/             # Integration Tests (? tests)
│   ├── mod.rs
│   └── test_integration_components.rs
│
└── debug/                   # Debug Tests (temporary)
    ├── mod.rs
    ├── debug_indexed_test.rs
    ├── debug_memory_problem.rs
    ├── debug_mul_opcodes.rs
    ├── debug_orcc_andcc.rs
    ├── debug_step_by_step.rs
    └── test_debug_opcodes.rs
```

## Organization Principles

### 1. One-File-Per-Opcode Pattern
- Each opcode should have its own `.rs` file
- File naming: `test_[opcode_name].rs` (e.g., `test_lda.rs`, `test_adda.rs`)
- For multi-test opcodes, use descriptive names (e.g., `test_adda_variants.rs`)

### 2. Categorization
Tests are organized into logical categories:

#### Opcodes (258 tests)
- **arithmetic/**: ADD, SUB, MUL, DIV, CMP, INC, DEC, AND, OR, EOR, NEG, COM
- **branch/**: BRA, BEQ, BNE, JSR, RTS, LBRA, LBSR, BSR
- **data_transfer/**: LD, ST, LEA, TFR, EXG, PSH, PUL
- **misc/**: NOP, SYNC, JMP, ORCC, ANDCC, DAA, SEX
- **illegal/**: Illegal opcodes that should panic
- **reserved/**: Reserved opcodes (0x01, 0x02, 0x05, 0x0B, 0x14, 0x15, 0x18, 0x1B)
- **interrupt/**: RTI, SWI, CWAI

#### Components
- **hardware/**: PSG, Screen, Shift Register, Timers
- **engine/**: DelayedValueStore, Engine Types
- **memory/**: Memory devices (RAM, ROM, etc.)

#### Integration
- **integration/**: Cross-component integration tests

#### Debug (Temporary)
- **debug/**: Temporary debugging tests (not for production)

### 3. Module Structure
Each category has a `mod.rs` file that declares all test modules:
```rust
// Example: opcodes/arithmetic/mod.rs
pub mod test_adda_variants;
pub mod test_add_sub_opcodes;
pub mod test_arithmetic_corrected;
// ...
```

Entry points in `tests/` root:
- `test_opcodes.rs` - Entry for all opcode tests
- `test_components.rs` - Entry for all component tests
- `test_integration.rs` - Entry for integration tests
- `test_debug.rs` - Entry for debug tests

### 4. Test Naming Convention
```rust
#[test]
fn test_[opcode]_[mode]_0x[hexcode]() {
    // Example: test_adda_immediate_0x8b
    // Example: test_rti_pops_entire_state_0x3B
}
```

### 5. Standard Test Setup
```rust
const RAM_START: u16 = 0xC800;
const STACK_START: u16 = 0xCFFF;

fn setup_emulator() -> (Emulator, Box<dyn MemoryDevice>) {
    let mut emulator = Emulator::new();
    let memory = Box::new(RamDevice::new());
    emulator.memory().add_device(RAM_START, memory.clone()).unwrap();
    emulator.cpu_mut().set_stack_pointer(STACK_START);
    (emulator, memory)
}
```

## Migration Notes

### What Changed (October 3, 2025)
- **Before**: All test files in `tests/` root directory (40+ files)
- **After**: Organized into categorized subdirectories

### Files Moved
- **To `opcodes/arithmetic/`**: 19 test files (ADD, SUB, CMP, AND, OR, EOR, MUL, register ops)
- **To `opcodes/branch/`**: 1 test file (branch operations)
- **To `opcodes/data_transfer/`**: 9 test files (LD, ST, LEA)
- **To `opcodes/misc/`**: 5 test files (NOP, SYNC, condition codes, etc.)
- **To `components/hardware/`**: 4 test files (PSG, Screen, Shift Register, Timers)
- **To `components/engine/`**: 2 test files (DelayedValueStore, Engine Types)
- **To `components/memory/`**: 1 test file (Memory devices)
- **To `integration/`**: 1 test file (component integration)
- **To `debug/`**: 6 test files (debugging tests)

### Files Removed
- `integration_test.rs` - Was just a wrapper for `mod opcodes`, now obsolete
- `test_illegal_opcodes.rs` (from root) - Duplicate, already in opcodes/illegal/
- `test_reserved_opcodes.rs` (from root) - Duplicate, already in opcodes/reserved/

## Test Results

```
Running 4 test suites:
✅ opcodes:      258 passed, 0 failed, 1 ignored
✅ components:    66 passed, 0 failed, 0 ignored
✅ integration:   10 passed, 0 failed, 0 ignored
✅ debug:          4 passed, 0 failed, 0 ignored
✅ lib:            7 passed, 0 failed, 0 ignored

Total: 345 passed, 0 failed, 1 ignored
```

## Best Practices

1. **Create new tests in appropriate category**
   - Opcode test → `tests/opcodes/[category]/`
   - Component test → `tests/components/[category]/`
   - Integration test → `tests/integration/`

2. **Follow one-file-per-opcode pattern**
   - Each opcode gets its own file
   - Multiple addressing modes can share a file

3. **Update mod.rs when adding tests**
   - Add `pub mod test_[name];` to category's `mod.rs`
   - Maintain alphabetical order

4. **Use real BIOS, never synthetic**
   - Load from: `ide/frontend/dist/bios.bin`
   - No simulated side effects

5. **Document test purpose**
   - Add comments explaining what the test verifies
   - Reference MC6809 spec or Vectrexy implementation

## Maintenance

- **Debug tests**: Should be removed or promoted to proper categories when no longer needed
- **Duplicate tests**: Consolidate or remove duplicates
- **Test coverage**: Track coverage per opcode category
- **Documentation**: Update this file when structure changes
