# Bank System Redesign: Sequential Layout

## Goal
Replace the "fixed bank #31" model with a simpler sequential model that mirrors PRG/CHR separation in Nintendo systems.

## New Architecture

### ROM Layout
```
Bank #0:      Header (0x0000-0x00FF) + main() + loop() + Code
Bank #1:      Code (overflow from bank #0)
Bank #2:      Code (overflow from bank #1)
...
Bank #N-1:    Runtime Helpers (DRAW_LINE_WRAPPER, MUL16, DIV_A, etc.)
```

### Key Changes

1. **Eliminate fixed_bank concept**
   - Remove `BankConfig.fixed_bank` field
   - Last bank (#N-1) is implicitly the helpers bank
   - All other banks (#0 to #N-2) hold VPy code sequentially

2. **Remove boot stub logic**
   - Delete `STA $D000` bank switch code
   - Delete `ORG $0000` / `ORG $4000` dual-origin logic
   - Generate single ORG per bank (bank #0 at $0000, bank #1 at $0000, etc.)

3. **Simplify bank allocator**
   - `bank_optimizer.rs`: Allocate functions sequentially to banks
   - Fill bank #0 first, then bank #1, etc.
   - Reserve last bank for helpers

4. **Update linker**
   - `multi_bank_linker.rs`: Generate one ASM file per bank (or merged with ORG directives)
   - Each bank's code is independent (offset by bank * 16KB in final binary)
   - Helpers always at fixed addresses in last bank

5. **Debug/Breakpoints**
   - Addresses are now absolute within their bank
   - No dynamic bank switching confusion
   - PDB mapping is straightforward

## Files to Modify

- [ ] `core/src/codegen.rs` - Remove fixed_bank from BankConfig
- [ ] `core/src/backend/m6809/mod.rs` - Remove boot stub generation
- [ ] `core/src/backend/m6809/bank_optimizer.rs` - Sequential allocation
- [ ] `core/src/backend/m6809/multi_bank_linker.rs` - Simplify linker logic
- [ ] `core/src/main.rs` - Remove fixed_bank logging

## Migration Path

1. Keep old test projects working (no-bank mode)
2. Projects with `META ROM_TOTAL_SIZE` get new sequential banking
3. Gradually phase out old tests

## Benefits

✅ Simpler mental model
✅ Debugging easier (no dynamic bank confusion)
✅ Better memory utilization
✅ No artificial boot stub
✅ Wrappers only needed for cross-bank calls (rare)
