#!/bin/bash

# Script: Clean up multibank_flat.asm by removing duplicate sections and unused code
# This script iterates the compilation until validation passes

FLAT_FILE="/Users/daniel/projects/vectrex-pseudo-python/examples/test_callgraph/src/multibank_temp/multibank_flat.asm"
COMPILER_BIN="/Users/daniel/projects/vectrex-pseudo-python/target/debug/vectrexc"
SOURCE_VPY="/Users/daniel/projects/vectrex-pseudo-python/examples/test_callgraph/src/main.vpy"
VALIDATION_SCRIPT="/Users/daniel/projects/vectrex-pseudo-python/validate_multibank_duplication.sh"

echo "========================================"
echo "MULTIBANK ASM CLEANUP PROCESS"
echo "========================================"
echo ""

ITERATION=0
MAX_ITERATIONS=5

while [ "$ITERATION" -lt "$MAX_ITERATIONS" ]; do
    ((ITERATION++))
    
    echo "🔄 ITERATION #$ITERATION"
    echo "---"
    
    # Recompile
    echo "   Recompiling multibank project..."
    cd /Users/daniel/projects/vectrex-pseudo-python
    cargo run --bin vectrexc -- build "$SOURCE_VPY" --bin 2>&1 | grep -E "(SUCCESS|FAILED|Phase)"
    
    if [ ! -f "$FLAT_FILE" ]; then
        echo "   ❌ Flat file not generated!"
        exit 1
    fi
    
    # Run validation
    echo ""
    echo "   Running validation..."
    
    # Check for issues quietly
    RAM_COUNT=$(grep -c "RAM VARIABLE DEFINITIONS (EQU)" "$FLAT_FILE" 2>/dev/null || echo "0")
    JOYSTICK_COUNT=$(grep -c "JOYSTICK BUILTIN SUBROUTINES" "$FLAT_FILE" 2>/dev/null || echo "0")
    CIRCLE_VARS=$(grep -c "^DRAW_CIRCLE_" "$FLAT_FILE" 2>/dev/null || echo "0")
    CIRCLE_CALLS=$(grep -c "JSR.*[Dd]raw[Cc]ircle" "$FLAT_FILE" 2>/dev/null || echo "0")
    
    echo "   - RAM sections: $RAM_COUNT (should be 1)"
    echo "   - Joystick sections: $JOYSTICK_COUNT (should be 1)"
    echo "   - DRAW_CIRCLE variables: $CIRCLE_VARS"
    echo "   - DRAW_CIRCLE calls: $CIRCLE_CALLS"
    
    # Check if passed
    if [ "$RAM_COUNT" -eq 1 ] && [ "$JOYSTICK_COUNT" -eq 1 ] && [ "$CIRCLE_VARS" -eq 0 ]; then
        echo ""
        echo "   ✅ VALIDATION PASSED!"
        echo ""
        echo "========================================"
        echo "✅ CLEANUP COMPLETE"
        echo "========================================"
        exit 0
    fi
    
    echo ""
    
    if [ "$ITERATION" -ge "$MAX_ITERATIONS" ]; then
        echo "❌ Max iterations reached. Manual review needed."
        exit 1
    fi
done
