#!/usr/bin/env python3
"""Add BITB instruction support to binary emitter and parser"""

# Step 1: Add BITB methods to BinaryEmitter
with open('core/src/backend/m6809_binary_emitter.rs', 'r') as f:
    emitter_content = f.read()

# Find where to insert (after ANDB)
andb_pos = emitter_content.find('    /// ANDB #immediate (opcode 0xC4)')
if andb_pos == -1:
    print("ERROR: Could not find ANDB in binary emitter")
    exit(1)

# Find end of andb_immediate function
end_pos = emitter_content.find('    }', andb_pos)
end_pos = emitter_content.find('\n', end_pos) + 1

# New BITB methods (opcodes from M6809 manual)
bitb_code = '''
    /// BITB #immediate (opcode 0xC5)
    pub fn bitb_immediate(&mut self, value: u8) {
        self.record_line_mapping();
        self.emit(0xC5);
        self.emit(value);
    }

    /// BITB extended (opcode 0xF5)
    pub fn bitb_extended(&mut self, addr: u16) {
        self.record_line_mapping();
        self.emit(0xF5);
        self.emit_word(addr);
    }
'''

new_emitter = emitter_content[:end_pos] + bitb_code + emitter_content[end_pos:]

with open('core/src/backend/m6809_binary_emitter.rs', 'w') as f:
    f.write(new_emitter)

print("✓ Added BITB methods to BinaryEmitter")

# Step 2: Add BITB parser to asm_to_binary
with open('core/src/backend/asm_to_binary.rs', 'r') as f:
    parser_content = f.read()

# Find where to add in instruction match
andb_match = parser_content.find('        "ANDB" => emit_andb(emitter, operand, equates),')
if andb_match == -1:
    print("ERROR: Could not find ANDB match statement")
    exit(1)

end_line = parser_content.find('\n', andb_match) + 1
bitb_match = '        "BITB" => emit_bitb(emitter, operand, equates),\n'

new_parser = parser_content[:end_line] + bitb_match + parser_content[end_line:]

# Find where to add emit_bitb function (after emit_andb)
emit_andb_pos = new_parser.find('fn emit_andb(emitter: &mut BinaryEmitter, operand: &str, equates: &HashMap<String, u16>) -> Result<(), String> {')
if emit_andb_pos == -1:
    print("ERROR: Could not find emit_andb function")
    exit(1)

# Find end of emit_andb function
brace_count = 0
pos = emit_andb_pos
found_first_brace = False
while pos < len(new_parser):
    if new_parser[pos] == '{':
        brace_count += 1
        found_first_brace = True
    elif new_parser[pos] == '}':
        brace_count -= 1
        if found_first_brace and brace_count == 0:
            pos = new_parser.find('\n', pos) + 1
            break
    pos += 1

# Add emit_bitb function
emit_bitb_code = '''
fn emit_bitb(emitter: &mut BinaryEmitter, operand: &str, equates: &HashMap<String, u16>) -> Result<(), String> {
    if operand.starts_with('#') {
        let val = parse_immediate(&operand[1..])?;
        emitter.bitb_immediate(val);
        Ok(())
    } else {
        match evaluate_expression(operand, equates) {
            Ok(addr) => {
                emitter.bitb_extended(addr);
                Ok(())
            }
            Err(msg) => {
                if msg.starts_with("SYMBOL:") {
                    let symbol = &msg[7..];
                    emitter.add_symbol_ref(symbol, false, 2);
                    emitter.emit(0xF5); // BITB extended
                    emitter.emit_word(0);
                    Ok(())
                } else {
                    Err(msg)
                }
            }
        }
    }
}
'''

final_parser = new_parser[:pos] + emit_bitb_code + new_parser[pos:]

with open('core/src/backend/asm_to_binary.rs', 'w') as f:
    f.write(final_parser)

print("✓ Added BITB parser to asm_to_binary")
print("✓ BITB instruction fully implemented (immediate 0xC5, extended 0xF5)")
