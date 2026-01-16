# Phase 6.8 PDB Issue - Wrong ASM Source Addresses

## Status Update (2026-01-15)

✅ **FIXED**: PDB output location (now writes to correct project directory)
✅ **FIXED**: PDB addresses now parsed from bank_*.asm files (correct post-linker addresses)

**Implementation**: Phase 6.8 now parses all bank_*.asm files, assembles them to extract symbol tables, and updates debug_info with correct addresses before writing PDB.

## Original Problem (RESOLVED)

Phase 6.8 was generating `.pdb` file with addresses from the **pre-linker unified ASM**, but should use addresses from the **post-linker individual bank ASMs** (`bank_00.asm`, `bank_01.asm`, etc.).

**Solution Implemented**:
- New function `update_debug_info_from_banks()` parses all bank_*.asm files
- Assembles each bank to extract symbol table with addresses
- Updates `debug_info.symbols` with final addresses
- Updates `debug_info.functions` with correct addresses

## Current Flow (BROKEN)

```
Phase 4: emit_asm_with_debug("test_multibank_pdb.vpy")
  └─ Creates: test_multibank_pdb.asm (unified, single file)
  └─ Creates: debug_info (from unified ASM)

Phase 5: Write test_multibank_pdb.asm to disk
  └─ Single file with ALL code

Phase 6.7: Multibank Linker
  └─ Splits test_multibank_pdb.asm into:
     - bank_00.asm
     - bank_01.asm
     - ...
     - bank_31.asm
  └─ debug_info is NOT updated

Phase 6.8: Write PDB
  └─ Uses ORIGINAL debug_info (from unified ASM)
  └─ PDB points to addresses in test_multibank_pdb.asm
  ❌ WRONG: Runtime uses bank_00.asm, bank_01.asm, etc.
```

## Why This Is Wrong

When you set a breakpoint in the debugger:
1. Debugger reads `.pdb` file generated in Phase 6.8
2. PDB says "function main is at address 0x00E1 in test_multibank_pdb.asm"
3. ROM loads bank_00.bin with different address mapping
4. Breakpoint triggers at wrong address or not at all
5. **Debugging broken**

## Root Cause

Phase 6.8 writes `debug_info` that was populated in Phase 4 from **unified ASM**.
After Phase 6.7 (linker), the actual runtime uses **split bank ASMs** with different addresses.

## The Fix Needed

Phase 6.8 should:
1. Read ALL `bank_*.asm` files from multibank_temp/
2. Re-populate debug_info with addresses from split banks
3. Write PDB with CORRECT addresses

Current code (BROKEN):
```rust
// Phase 6.8: Write PDB for multibank (after linker has processed all banks)
if has_multibank && bin && tgt == target::Target::Vectrex {
    if let Some(ref mut dbg) = debug_info_mut {
        eprintln!("Phase 6.8: Writing debug symbols file (.pdb) for multibank...");
        let pdb_path = out_path.with_extension("pdb");
        
        match dbg.to_json() {  // ❌ WRONG: uses old unified ASM addresses
            Ok(json) => {
                fs::write(&pdb_path, json)?;
```

Needed code (CORRECT):
```rust
// Phase 6.8: Write PDB for multibank (after linker has processed all banks)
if has_multibank && bin && tgt == target::Target::Vectrex {
    if let Some(ref mut dbg) = debug_info_mut {
        eprintln!("Phase 6.8: Updating PDB with bank addresses...");
        
        // Read bank_*.asm files and update debug_info
        let multibank_temp_dir = out_path.parent().unwrap().join("multibank_temp");
        for bank_num in 0..num_banks {
            let bank_asm = multibank_temp_dir.join(format!("bank_{:02}.asm", bank_num));
            if bank_asm.exists() {
                // Parse bank ASM and update addresses in debug_info
                // ...code to read and merge bank addresses...
            }
        }
        
        let pdb_path = out_path.with_extension("pdb");
        match dbg.to_json() {  // ✅ CORRECT: uses updated addresses from banks
            Ok(json) => {
                fs::write(&pdb_path, json)?;
```

## Impact

- **Current behavior**: PDB is written but points to wrong ASM source
- **Debugger behavior**: Breakpoints may not work correctly in multibank mode
- **Single-bank mode**: Unaffected (Phase 6.6 handles it correctly)

## Solution Priority

- **Severity**: High (debugging completely broken for multibank)
- **Complexity**: Medium (need to parse bank ASMs after linker)
- **Timeline**: Should be fixed before multibank debugging is used

## Testing

After fix:
1. Compile test_multibank_pdb with Phase 6.7 working (fix VAR_ARG2 first)
2. Check that .pdb references bank_*.asm files
3. Set breakpoint in JSVecx debugger
4. Verify breakpoint hits correct code location
