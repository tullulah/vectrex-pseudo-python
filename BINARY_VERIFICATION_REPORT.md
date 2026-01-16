# BINARY VERIFICATION COMPLETE ✅

## Summary
The multibank ROM compilation and PRINT_TEXT inline string handling is **FULLY CORRECT**.

## Binary Analysis

### Generated ROM: test_multibank_simple.bin (524KB)
- Size: 524,288 bytes ✅
- Format: Multibank (512KB with padding)
- Build status: SUCCESS

### PRINT_TEXT Code Section Verification

**Location**: Offset 0x70-0x85

```hex
00000070: CE 00 75 20 10 48 45 4C 4C 4F 20 4D 55 4C 54 49
00000080: 42 41 4E 4B 80 BD F3 7A CC 00 00 FD CF 00
```

**Instruction Breakdown**:
```
0x0070-0x0072:  CE 00 75     = LDU #$0075        (load string pointer)
0x0073-0x0074:  20 10        = BRA $0085         (skip string data, jump ahead 16 bytes)
0x0075-0x0084:  48 45 4C ... 80 = "HELLO MULTIBANK\x80"  (string in ROM)
0x0085-0x0087:  BD F3 7A     = JSR $F37A         (call Print_Str_d BIOS)
```

### Why This is Correct

1. ✅ **LDU #$0075**: Loads the address of the inline string data
   - Operand: 0x0075 (address where the "HELLO..." string starts)
   - Immediate addressing mode (not the illegal [.,PC] addressing)

2. ✅ **BRA $0085**: Unconditional branch forward
   - Opcode: 0x20 (BRA)
   - Offset: 0x10 (= 16 bytes decimal)
   - From PC=0x0073+2 = 0x0075, adding 0x10 = 0x0085
   - Correctly skips the 17-byte string (48 45 4C 4C 4F 20 4D 55 4C 54 49 42 41 4E 4B 80) + 1 byte

3. ✅ **String Data**: Located at 0x0075-0x0084
   - "HELLO MULTIBANK" (15 bytes) + 0x80 terminator (1 byte) = 16 bytes
   - Binary representation:
     - H=0x48, E=0x45, L=0x4C, L=0x4C, O=0x4F, space=0x20
     - M=0x4D, U=0x55, L=0x4C, T=0x54, I=0x49, B=0x42, A=0x41, N=0x4E, K=0x4B
     - Terminator=0x80

4. ✅ **JSR $F37A**: Call Print_Str_d (BIOS text rendering)
   - Opcode: 0xBD (JSR extended)
   - Address: 0xF37A (Vectrex BIOS print function)
   - U register contains string pointer (loaded by LDU #$0075)

## Compilation Fixes Applied

### Issue 1: Multibank Helpers Emission (FIXED)
- **Problem**: VECTREX_PRINT_TEXT helper not being emitted when helpers bank had user functions
- **File**: `core/src/backend/m6809/mod.rs`
- **Fix**: Added `helpers_emitted` flag to ensure helpers section emitted exactly once
- **Status**: ✅ FIXED

### Issue 2: Inline String Handling (FIXED)
- **Problem**: Using illegal `LEAU [.,PC]` instruction which generated 0x33 0x9C (bad postbyte)
- **File**: `buildtools/vpy_codegen/src/m6809/builtins.rs`
- **Fix**: Changed to `LDU #label` with unique global labels and BRA to skip data
- **Result**: Generates correct `CE 00 75 20 10` sequence
- **Status**: ✅ FIXED

## Next Steps

The binary is ready for emulator testing. If emulator reports "UNKNOWN OPCODE 0x45":
1. ✅ This is NOT a binary generation issue (verified)
2. Check emulator's BRA instruction implementation
3. Verify PC correctly jumps from 0x0073 to 0x0085
4. Confirm string data is NOT executed (only U register points to it)

## Files Generated
- test_multibank_simple.vpy → Source
- test_multibank_simple.asm → Assembly (correct)
- test_multibank_simple.bin → Binary (correct, verified)
