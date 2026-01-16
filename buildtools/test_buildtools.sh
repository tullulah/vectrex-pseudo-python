#!/bin/bash
# Test script to verify buildtools pipeline
# Usage: ./test_buildtools.sh

set -e

echo "═══════════════════════════════════════════════════════════════"
echo "Building buildtools pipeline..."
echo "═══════════════════════════════════════════════════════════════"

cd "$(dirname "$0")"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test vpy_loader
echo -e "${YELLOW}[1/9] Testing vpy_loader...${NC}"
cd vpy_loader
if cargo test --lib 2>&1 | grep -q "test result: ok"; then
    echo -e "${GREEN}✓ vpy_loader: PASS${NC}"
else
    echo -e "${RED}✗ vpy_loader: FAIL${NC}"
    exit 1
fi
cd ..

# Check remaining placeholders compile
echo -e "${YELLOW}[2/9] Checking vpy_parser compiles...${NC}"
cd vpy_parser && cargo build --lib > /dev/null 2>&1 && echo -e "${GREEN}✓ vpy_parser: OK${NC}" && cd ..

echo -e "${YELLOW}[3/9] Checking vpy_unifier compiles...${NC}"
cd vpy_unifier && cargo build --lib > /dev/null 2>&1 && echo -e "${GREEN}✓ vpy_unifier: OK${NC}" && cd ..

echo -e "${YELLOW}[4/9] Checking vpy_bank_allocator compiles...${NC}"
cd vpy_bank_allocator && cargo build --lib > /dev/null 2>&1 && echo -e "${GREEN}✓ vpy_bank_allocator: OK${NC}" && cd ..

echo -e "${YELLOW}[5/9] Checking vpy_codegen compiles...${NC}"
cd vpy_codegen && cargo build --lib > /dev/null 2>&1 && echo -e "${GREEN}✓ vpy_codegen: OK${NC}" && cd ..

echo -e "${YELLOW}[6/9] Checking vpy_assembler compiles...${NC}"
cd vpy_assembler && cargo build --lib > /dev/null 2>&1 && echo -e "${GREEN}✓ vpy_assembler: OK${NC}" && cd ..

echo -e "${YELLOW}[7/9] Checking vpy_linker compiles...${NC}"
cd vpy_linker && cargo build --lib > /dev/null 2>&1 && echo -e "${GREEN}✓ vpy_linker: OK${NC}" && cd ..

echo -e "${YELLOW}[8/9] Checking vpy_binary_writer compiles...${NC}"
cd vpy_binary_writer && cargo build --lib > /dev/null 2>&1 && echo -e "${GREEN}✓ vpy_binary_writer: OK${NC}" && cd ..

echo -e "${YELLOW}[9/9] Checking vpy_debug_gen compiles...${NC}"
cd vpy_debug_gen && cargo build --lib > /dev/null 2>&1 && echo -e "${GREEN}✓ vpy_debug_gen: OK${NC}" && cd ..

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo -e "${GREEN}All buildtools crates compiled successfully!${NC}"
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "Next: Port vpy_parser from core/src/parser.rs"
