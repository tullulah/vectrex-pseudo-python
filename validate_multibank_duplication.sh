#!/bin/bash

# Script: Validate multibank_flat.asm for duplicate sections and unused variables
# Purpose: Check for duplication and unused code that should be cleaned up

FLAT_FILE="/Users/daniel/projects/vectrex-pseudo-python/examples/test_callgraph/src/multibank_temp/multibank_flat.asm"

if [ ! -f "$FLAT_FILE" ]; then
    echo "❌ Error: File not found: $FLAT_FILE"
    exit 1
fi

echo "========================================"
echo "MULTIBANK FLAT ASM VALIDATION REPORT"
echo "========================================"
echo ""

# 1. Check for DRAW_CIRCLE variables
echo "1️⃣  CHECKING DRAW_CIRCLE VARIABLES"
echo "---"

CIRCLE_VARS=("DRAW_CIRCLE_XC" "DRAW_CIRCLE_YC" "DRAW_CIRCLE_RADIUS" "DRAW_CIRCLE_INTENSITY")
CIRCLE_VARS_FOUND=0

for var in "${CIRCLE_VARS[@]}"; do
    if grep -q "^${var}" "$FLAT_FILE"; then
        echo "   ✓ Found variable: $var"
        ((CIRCLE_VARS_FOUND++))
    fi
done

# Check if draw_circle is called
DRAW_CIRCLE_CALLS=$(grep -c "JSR.*[Dd]raw_[Cc]ircle" "$FLAT_FILE" || echo "0")

if [ "$CIRCLE_VARS_FOUND" -gt 0 ] && [ "$DRAW_CIRCLE_CALLS" -eq 0 ]; then
    echo ""
    echo "   ⚠️  WARNING: Found $CIRCLE_VARS_FOUND DRAW_CIRCLE variables but"
    echo "   NO calls to draw_circle (UNUSED CODE)"
else
    echo ""
    echo "   ℹ️  $CIRCLE_VARS_FOUND variables found, $DRAW_CIRCLE_CALLS draw_circle calls"
fi

echo ""

# 2. Count RAM VARIABLE DEFINITIONS section
echo "2️⃣  CHECKING 'RAM VARIABLE DEFINITIONS (EQU)' SECTIONS"
echo "---"

RAM_SECTION_COUNT=$(grep -c "RAM VARIABLE DEFINITIONS (EQU)" "$FLAT_FILE" || echo "0")
echo "   Found: $RAM_SECTION_COUNT occurrence(s)"

if [ "$RAM_SECTION_COUNT" -gt 1 ]; then
    echo "   ⚠️  WARNING: Section appears more than once! Should be 1 only"
    echo ""
    echo "   Locations:"
    grep -n "RAM VARIABLE DEFINITIONS (EQU)" "$FLAT_FILE" | sed 's/^/      /'
else
    echo "   ✓ OK: Section appears exactly once"
fi

echo ""

# 3. Count JOYSTICK BUILTIN SUBROUTINES section
echo "3️⃣  CHECKING 'JOYSTICK BUILTIN SUBROUTINES' SECTIONS"
echo "---"

JOYSTICK_SECTION_COUNT=$(grep -c "JOYSTICK BUILTIN SUBROUTINES" "$FLAT_FILE" || echo "0")
echo "   Found: $JOYSTICK_SECTION_COUNT occurrence(s)"

if [ "$JOYSTICK_SECTION_COUNT" -gt 1 ]; then
    echo "   ⚠️  WARNING: Section appears more than once! Should be 1 only"
    echo ""
    echo "   Locations:"
    grep -n "JOYSTICK BUILTIN SUBROUTINES" "$FLAT_FILE" | sed 's/^/      /'
else
    echo "   ✓ OK: Section appears exactly once"
fi

echo ""

# 4. Check for other duplicate sections that should only appear once
echo "4️⃣  CHECKING OTHER CRITICAL SECTIONS"
echo "---"

SECTIONS=(
    "INCLUDE BIOS"
    "CROSS-BANK CALL WRAPPERS"
    "END CROSS-BANK WRAPPERS"
    "RUNTIME HELPERS SECTION"
    "BIOS VECTORS AND FUNCTIONS"
)

CRITICAL_ERRORS=0

for section in "${SECTIONS[@]}"; do
    count=$(grep -c "$section" "$FLAT_FILE" || echo "0")
    if [ "$count" -gt 1 ]; then
        echo "   ⚠️  '$section': $count occurrences (should be ≤1)"
        ((CRITICAL_ERRORS++))
    fi
done

if [ "$CRITICAL_ERRORS" -eq 0 ]; then
    echo "   ✓ OK: All critical sections appear correct number of times"
fi

echo ""

# 5. Summary
echo "========================================"
echo "SUMMARY"
echo "========================================"

ISSUES=0

if [ "$CIRCLE_VARS_FOUND" -gt 0 ] && [ "$DRAW_CIRCLE_CALLS" -eq 0 ]; then
    echo "❌ ISSUE #1: Unused DRAW_CIRCLE variables (should remove)"
    ((ISSUES++))
fi

if [ "$RAM_SECTION_COUNT" -gt 1 ]; then
    echo "❌ ISSUE #2: RAM_VARIABLE_DEFINITIONS section duplicated ($RAM_SECTION_COUNT times)"
    ((ISSUES++))
fi

if [ "$JOYSTICK_SECTION_COUNT" -gt 1 ]; then
    echo "❌ ISSUE #3: JOYSTICK_BUILTIN_SUBROUTINES section duplicated ($JOYSTICK_SECTION_COUNT times)"
    ((ISSUES++))
fi

if [ "$CRITICAL_ERRORS" -gt 0 ]; then
    echo "❌ ISSUE #4: $CRITICAL_ERRORS critical sections appear multiple times"
    ((ISSUES++))
fi

echo ""

if [ "$ISSUES" -eq 0 ]; then
    echo "✅ VALIDATION PASSED: No issues found!"
    exit 0
else
    echo "⚠️  Found $ISSUES issue(s) to fix"
    exit 1
fi
