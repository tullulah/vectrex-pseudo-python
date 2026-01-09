warning: unused import: `StructLayout`
 --> core/src/codegen.rs:6:67
  |
6 | use crate::struct_layout::{StructRegistry, build_struct_registry, StructLayout};
  |                                                                   ^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `FieldDef`
 --> core/src/struct_layout.rs:8:29
  |
8 | use crate::ast::{StructDef, FieldDef};
  |                             ^^^^^^^^

warning: unused import: `Stmt`
 --> core/src/backend/m6809/builtins.rs:2:24
  |
2 | use crate::ast::{Expr, Stmt};
  |                        ^^^^

warning: unused import: `AssignTarget`
 --> core/src/backend/m6809/statements.rs:2:18
  |
2 | use crate::ast::{AssignTarget, Expr, Stmt};
  |                  ^^^^^^^^^^^^

warning: unused import: `emit_builtin_call`
 --> core/src/backend/m6809/statements.rs:4:42
  |
4 | use super::{LoopCtx, FuncCtx, emit_expr, emit_builtin_call, fresh_label, LineTracker};
  |                                          ^^^^^^^^^^^^^^^^^

warning: unused import: `Function`
 --> core/src/backend/m6809/analysis.rs:2:31
  |
2 | use crate::ast::{BinOp, Expr, Function, Item, Module, Stmt};
  |                               ^^^^^^^^

warning: unused import: `std::collections::HashSet`
   --> core/src/backend/m6809/analysis.rs:180:5
    |
180 | use std::collections::HashSet;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `Expr`
 --> core/src/backend/m6809/emission.rs:2:42
  |
2 | use crate::ast::{Function, Stmt, Module, Expr};
  |                                          ^^^^

warning: unused import: `collect_locals`
 --> core/src/backend/m6809/emission.rs:4:42
  |
4 | use super::{LoopCtx, FuncCtx, emit_stmt, collect_locals, collect_locals_with_params, RuntimeUsage, LineTracker, DebugInfo};
  |                                          ^^^^^^^^^^^^^^

warning: unused import: `std::collections::BTreeSet`
 --> core/src/backend/m6809/collectors.rs:4:5
  |
4 | use std::collections::BTreeSet;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `emission::*`
  --> core/src/backend/m6809/mod.rs:23:9
   |
23 | pub use emission::*;
   |         ^^^^^^^^^^^

warning: unused imports: `CmpOp` and `LogicOp`
  --> core/src/backend/m6809/mod.rs:32:25
   |
32 | use crate::ast::{BinOp, CmpOp, Expr, Function, Item, LogicOp, Module, Stmt};
   |                         ^^^^^                        ^^^^^^^

warning: unused import: `Ordering`
  --> core/src/backend/m6809/mod.rs:38:37
   |
38 | use std::sync::atomic::{AtomicBool, Ordering};
   |                                     ^^^^^^^^

warning: unused import: `std::collections::BTreeMap`
  --> core/src/backend/m6809/mod.rs:39:5
   |
39 | use std::collections::BTreeMap;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused macro definition: `check_depth`
  --> core/src/backend/m6809/mod.rs:44:14
   |
44 | macro_rules! check_depth {
   |              ^^^^^^^^^^^
   |
   = note: `#[warn(unused_macros)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `Function`
   --> core/src/lsp.rs:869:32
    |
869 | use crate::ast::{Module, Item, Function, Stmt, Expr, AssignTarget, IdentInfo, CallInfo};
    |                                ^^^^^^^^

warning: unreachable pattern
  --> core/src/parser.rs:12:9
   |
11 |         "SFX_UPDATE" |
   |         ------------ matches all the relevant values
12 |         "SFX_UPDATE" |
   |         ^^^^^^^^^^^^ no value can reach this
   |
   = note: `#[warn(unreachable_patterns)]` (part of `#[warn(unused)]`) on by default

warning: variable does not need to be mutable
   --> core/src/codegen.rs:555:17
    |
555 |             let mut diagnostics = vec![Diagnostic {
    |                 ----^^^^^^^^^^^
    |                 |
    |                 help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `field`
   --> core/src/codegen.rs:952:37
    |
952 |         Expr::FieldAccess { target, field, source_line, col } => {
    |                                     ^^^^^ help: try ignoring the field: `field: _`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `source_line`
   --> core/src/codegen.rs:952:44
    |
952 |         Expr::FieldAccess { target, field, source_line, col } => {
    |                                            ^^^^^^^^^^^ help: try ignoring the field: `source_line: _`

warning: unused variable: `col`
   --> core/src/codegen.rs:952:57
    |
952 |         Expr::FieldAccess { target, field, source_line, col } => {
    |                                                         ^^^ help: try ignoring the field: `col: _`

warning: unused variable: `source_line`
    --> core/src/codegen.rs:1045:57
     |
1045 |                 crate::ast::AssignTarget::Ident { name, source_line, col } => {
     |                                                         ^^^^^^^^^^^ help: try ignoring the field: `source_line: _`

warning: unused variable: `col`
    --> core/src/codegen.rs:1045:70
     |
1045 |                 crate::ast::AssignTarget::Ident { name, source_line, col } => {
     |                                                                      ^^^ help: try ignoring the field: `col: _`

warning: unused variable: `field`
    --> core/src/codegen.rs:1057:75
     |
1057 |                 crate::ast::AssignTarget::FieldAccess { target: obj_expr, field, .. } => {
     |                                                                           ^^^^^ help: try ignoring the field: `field: _`

warning: unused variable: `field`
    --> core/src/codegen.rs:1079:75
     |
1079 |                 crate::ast::AssignTarget::FieldAccess { target: obj_expr, field, .. } => {
     |                                                                           ^^^^^ help: try ignoring the field: `field: _`

warning: unused variable: `source_line`
    --> core/src/codegen.rs:1322:78
     |
1322 |                 crate::ast::AssignTarget::Index { target: array_expr, index, source_line, .. } => {
     |                                                                              ^^^^^^^^^^^ help: try ignoring the field: `source_line: _`

warning: variable `something_changed` is assigned to, but never used
   --> core/src/musres.rs:205:21
    |
205 |             let mut something_changed = false;
    |                     ^^^^^^^^^^^^^^^^^
    |
    = note: consider using `_something_changed` instead

warning: value assigned to `something_changed` is never read
   --> core/src/musres.rs:211:17
    |
211 |                 something_changed = true;
    |                 ^^^^^^^^^^^^^^^^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: value assigned to `something_changed` is never read
   --> core/src/musres.rs:218:17
    |
218 |                 something_changed = true;
    |                 ^^^^^^^^^^^^^^^^^
    |
    = help: maybe it is overwritten before being read?

warning: value assigned to `something_changed` is never read
   --> core/src/musres.rs:226:21
    |
226 |                     something_changed = true;
    |                     ^^^^^^^^^^^^^^^^^
    |
    = help: maybe it is overwritten before being read?

warning: value assigned to `something_changed` is never read
   --> core/src/musres.rs:235:21
    |
235 |                     something_changed = true;
    |                     ^^^^^^^^^^^^^^^^^
    |
    = help: maybe it is overwritten before being read?

warning: unused variable: `skip_label`
    --> core/src/backend/m6809/builtins.rs:1274:25
     |
1274 |                     let skip_label = fresh_label("DEBUG_SKIP_DATA");
     |                         ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_skip_label`

warning: unused variable: `idx_label`
   --> core/src/backend/m6809/statements.rs:239:17
    |
239 |             let idx_label = format!("_FORIN_IDX_{}", ls.replace("L_", ""));
    |                 ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_idx_label`

warning: unused variable: `has_audio_calls`
   --> core/src/backend/m6809/mod.rs:715:13
    |
715 |         let has_audio_calls = has_music_calls || has_sfx_calls;
    |             ^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_has_audio_calls`

warning: unreachable pattern
  --> core/src/backend/m6809_opcodes.rs:53:9
   |
32 |         0x90 | 0x91 | 0x92 | 0x93 | 0x94 | 0x95 | 0x97 | 0x98 | 0x99 | 0x9A | 0x9B | 0x9C => 1,  // Direct (sin 0x9D, 0x9E, 0x9F)
   |                                                   ---- matches all the relevant values
...
53 |         0x97 | 0xB6 | 0xB7 | 0xBD | 0xBE | 0xBF | 0xD7 | 0xF6 | 0xF7 | 0xFC | 0xFD | 0xFE | 0xFF |
   |         ^^^^ no value can reach this

warning: unreachable pattern
   --> core/src/backend/asm_to_binary.rs:561:9
    |
544 |         "CLRA" => { emitter.clra(); Ok(()) },
    |         ------ matches all the relevant values
...
561 |         "CLRA" => { emitter.clra(); Ok(()) },
    |         ^^^^^^ no value can reach this

warning: value assigned to `last_valid_address` is never read
  --> core/src/backend/asm_address_mapper.rs:46:13
   |
46 |     let mut last_valid_address = 0u16; // Track last address for non-code lines
   |             ^^^^^^^^^^^^^^^^^^
   |
   = help: maybe it is overwritten before being read?

warning: value assigned to `last_valid_address` is never read
  --> core/src/backend/asm_address_mapper.rs:92:13
   |
92 |             last_valid_address = runtime_address;
   |             ^^^^^^^^^^^^^^^^^^
   |
   = help: maybe it is overwritten before being read?

warning: unused variable: `asm_lines`
   --> core/src/backend/asm_address_mapper.rs:176:39
    |
176 | fn calculate_header_offset_deprecated(asm_lines: &[&str]) -> u16 {
    |                                       ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_asm_lines`

warning: variable `parsed_module` is assigned to, but never used
   --> core/src/lsp.rs:638:21
    |
638 |             let mut parsed_module: Option<Module> = None;
    |                     ^^^^^^^^^^^^^
    |
    = note: consider using `_parsed_module` instead

warning: value assigned to `parsed_module` is never read
   --> core/src/lsp.rs:654:17
    |
654 |                 parsed_module = Some(module);
    |                 ^^^^^^^^^^^^^
    |
    = help: maybe it is overwritten before being read?

warning: variable does not need to be mutable
   --> core/src/lsp.rs:879:21
    |
879 |                 let mut usage = VariableUsage {
    |                     ----^^^^^
    |                     |
    |                     help: remove this `mut`

warning: unused variable: `var`
    --> core/src/lsp.rs:1004:25
     |
1004 |             Stmt::For { var, start, end, step, body, .. } => {
     |                         ^^^-
     |                         |
     |                         help: try removing the field

warning: method `unread_identifier` is never used
   --> core/src/parser.rs:806:8
    |
140 | impl<'a> Parser<'a> {
    | ------------------- method in this implementation
...
806 |     fn unread_identifier(&mut self, name:String) { self.pos-=1; if let TokenKind::Identifier(s)=&self.tokens[self.pos].kind { assert_eq!(...
    |        ^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: associated function `format_fcb2` is never used
   --> core/src/vecres.rs:267:8
    |
167 | impl VecResource {
    | ---------------- associated function in this implementation
...
267 |     fn format_fcb2(v1: i8, v2: i8) -> String {
    |        ^^^^^^^^^^^

warning: associated function `midi_to_bios_freq` is never used
   --> core/src/musres.rs:411:8
    |
100 | impl MusicResource {
    | ------------------ associated function in this implementation
...
411 |     fn midi_to_bios_freq(midi_note: u8) -> u8 {
    |        ^^^^^^^^^^^^^^^^^

warning: static `LAST_END_SET` is never used
  --> core/src/backend/m6809/mod.rs:41:8
   |
41 | static LAST_END_SET: AtomicBool = AtomicBool::new(false);
   |        ^^^^^^^^^^^^

warning: function `parse_indexed_postbyte` is never used
    --> core/src/backend/asm_to_binary.rs:2753:4
     |
2753 | fn parse_indexed_postbyte(operand: &str, _emitter: &mut BinaryEmitter) -> Result<u8, String> {
     |    ^^^^^^^^^^^^^^^^^^^^^^

warning: creating a shared reference to mutable static
   --> core/src/backend/asm_to_binary.rs:251:9
    |
251 |         INCLUDE_DIR.clone().or_else(|| std::env::current_dir().ok())
    |         ^^^^^^^^^^^^^^^^^^^ shared reference to mutable static
    |
    = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
    = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference is created for it while the shared reference lives
    = note: `#[warn(static_mut_refs)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning: `vectrex_lang` (lib) generated 49 warnings (run `cargo fix --lib -p vectrex_lang` to apply 18 suggestions)
warning: value assigned to `last_valid_address` is never read
  --> core/src/backend/asm_address_mapper.rs:46:13
   |
46 |     let mut last_valid_address = 0u16; // Track last address for non-code lines
   |             ^^^^^^^^^^^^^^^^^^
   |
   = help: maybe it is overwritten before being read?
   = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: value assigned to `something_changed` is never read
   --> core/src/musres.rs:211:17
    |
211 |                 something_changed = true;
    |                 ^^^^^^^^^^^^^^^^^
    |
    = help: maybe it is overwritten before being read?

warning: unused variable: `line_map`
   --> core/src/main.rs:683:43
    |
683 |                 let (binary_symbol_table, line_map, org) = assemble_bin(&out_path, use_lwasm, include_dir).map_err(|e| {
    |                                           ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_line_map`

warning: unused variable: `org`
   --> core/src/main.rs:683:53
    |
683 |                 let (binary_symbol_table, line_map, org) = assemble_bin(&out_path, use_lwasm, include_dir).map_err(|e| {
    |                                                     ^^^ help: if this is intentional, prefix it with an underscore: `_org`

warning: unused variable: `include_dir`
    --> core/src/main.rs:1034:38
     |
1034 | fn assemble_dual(asm_path: &PathBuf, include_dir: Option<&PathBuf>) -> Result<()> {
     |                                      ^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_include_dir`

warning: variant `Let` is never constructed
   --> core/src/ast.rs:112:2
    |
110 | pub enum Stmt {
    |          ---- variant in this enum
111 |     Assign { target: AssignTarget, value: Expr, source_line: usize },
112 |     Let { name: String, value: Expr, source_line: usize },
    |     ^^^
    |
    = note: `Stmt` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: method `unread_identifier` is never used
   --> core/src/parser.rs:806:8
    |
140 | impl<'a> Parser<'a> {
    | ------------------- method in this implementation
...
806 |     fn unread_identifier(&mut self, name:String) { self.pos-=1; if let TokenKind::Identifier(s)=&self.tokens[self.pos].kind { assert_eq!(...
    |        ^^^^^^^^^^^^^^^^^

warning: field `diag_freeze` is never read
   --> core/src/codegen.rs:207:9
    |
204 | pub struct CodegenOptions {
    |            -------------- field in this struct
...
207 |     pub diag_freeze: bool,  // instrument init steps with DIAG_COUNTER
    |         ^^^^^^^^^^^
    |
    = note: `CodegenOptions` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

warning: fields `uses_mul`, `uses_div`, and `uses_music` are never read
  --> core/src/backend/m6809/analysis.rs:16:9
   |
15 | pub struct RuntimeUsage {
   |            ------------ fields in this struct
16 |     pub uses_mul: bool,
   |         ^^^^^^^^
17 |     pub uses_div: bool,
   |         ^^^^^^^^
18 |     pub uses_music: bool,
   |         ^^^^^^^^^^
   |
   = note: `RuntimeUsage` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

warning: function `collect_all_vars` is never used
 --> core/src/backend/m6809/collectors.rs:6:8
  |
6 | pub fn collect_all_vars(module: &Module) -> Vec<String> {
  |        ^^^^^^^^^^^^^^^^

warning: method `get_address` is never used
  --> core/src/backend/m6809/ram_layout.rs:53:12
   |
20 | impl RamLayout {
   | -------------- method in this implementation
...
53 |     pub fn get_address(&self, name: &str) -> Option<u16> {
   |            ^^^^^^^^^^^

warning: fields `current_address` and `enabled` are never read
  --> core/src/backend/m6809/address_tracker.rs:11:5
   |
10 | pub struct AddressTracker {
   |            -------------- fields in this struct
11 |     current_address: u16,
   |     ^^^^^^^^^^^^^^^
12 |     enabled: bool,
   |     ^^^^^^^
   |
   = note: `AddressTracker` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: associated functions `current`, `advance`, `set`, `disable`, `enable`, and `comment` are never used
  --> core/src/backend/m6809/address_tracker.rs:30:12
   |
15 | impl AddressTracker {
   | ------------------- associated functions in this implementation
...
30 |     pub fn current() -> u16 {
   |            ^^^^^^^
...
36 |     pub fn advance(bytes: u16) {
   |            ^^^^^^^
...
46 |     pub fn set(address: u16) {
   |            ^^^
...
54 |     pub fn disable() {
   |            ^^^^^^^
...
62 |     pub fn enable() {
   |            ^^^^^^
...
70 |     pub fn comment() -> String {
   |            ^^^^^^^

warning: function `emit_with_addr` is never used
  --> core/src/backend/m6809/address_tracker.rs:77:8
   |
77 | pub fn emit_with_addr(instruction: &str) -> String {
   |        ^^^^^^^^^^^^^^

warning: function `estimate_instruction_size` is never used
  --> core/src/backend/m6809/address_tracker.rs:86:4
   |
86 | fn estimate_instruction_size(instruction: &str) -> u16 {
   |    ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: methods `lbeq_label`, `lbne_label`, `ldx_immediate_sym`, `ldy_immediate_sym`, and `ldu_immediate_sym` are never used
   --> core/src/backend/m6809_binary_emitter.rs:337:12
    |
 27 | impl BinaryEmitter {
    | ------------------ methods in this implementation
...
337 |     pub fn lbeq_label(&mut self, label: &str) {
    |            ^^^^^^^^^^
...
362 |     pub fn lbne_label(&mut self, label: &str) {
    |            ^^^^^^^^^^
...
846 |     pub fn ldx_immediate_sym(&mut self, symbol: &str) {
    |            ^^^^^^^^^^^^^^^^^
...
884 |     pub fn ldy_immediate_sym(&mut self, symbol: &str) {
    |            ^^^^^^^^^^^^^^^^^
...
946 |     pub fn ldu_immediate_sym(&mut self, symbol: &str) {
    |            ^^^^^^^^^^^^^^^^^

warning: function `escape_ascii` is never used
  --> core/src/backend/string_literals.rs:66:8
   |
66 | pub fn escape_ascii(s: &str) -> String {
   |        ^^^^^^^^^^^^

warning: method `add_variable` is never used
   --> core/src/backend/debug_info.rs:160:12
    |
136 | impl DebugInfo {
    | -------------- method in this implementation
...
160 |     pub fn add_variable(&mut self, name: String, address: u16, size: usize, var_type: &str, decl_line: Option<usize>) {
    |            ^^^^^^^^^^^^

warning: constant `VMUS_EXTENSION` is never used
  --> core/src/musres.rs:11:11
   |
11 | pub const VMUS_EXTENSION: &str = "vmus";
   |           ^^^^^^^^^^^^^^

warning: associated items `save`, `new`, `midi_to_bios_freq`, and `compile_to_binary` are never used
   --> core/src/musres.rs:109:12
    |
100 | impl MusicResource {
    | ------------------ associated items in this implementation
...
109 |     pub fn save(&self, path: &Path) -> Result<()> {
    |            ^^^^
...
116 |     pub fn new(name: &str) -> Self {
    |            ^^^
...
411 |     fn midi_to_bios_freq(midi_note: u8) -> u8 {
    |        ^^^^^^^^^^^^^^^^^
...
420 |     pub fn compile_to_binary(&self) -> Vec<u8> {
    |            ^^^^^^^^^^^^^^^^^

warning: enum `EventType` is never used
   --> core/src/musres.rs:476:6
    |
476 | enum EventType {
    |      ^^^^^^^^^

warning: function `compile_vmus_to_asm` is never used
   --> core/src/musres.rs:482:8
    |
482 | pub fn compile_vmus_to_asm(input: &Path, output: &Path) -> Result<()> {
    |        ^^^^^^^^^^^^^^^^^^^

warning: function `compile_vmus_to_binary` is never used
   --> core/src/musres.rs:494:8
    |
494 | pub fn compile_vmus_to_binary(input: &Path, output: &Path) -> Result<()> {
    |        ^^^^^^^^^^^^^^^^^^^^^^

warning: constant `VSFX_EXTENSION` is never used
  --> core/src/sfxres.rs:12:11
   |
12 | pub const VSFX_EXTENSION: &str = "vsfx";
   |           ^^^^^^^^^^^^^^

warning: multiple associated items are never used
   --> core/src/sfxres.rs:250:12
    |
241 | impl SfxResource {
    | ---------------- associated items in this implementation
...
250 |     pub fn save(&self, path: &Path) -> Result<()> {
    |            ^^^^
...
272 |     pub fn preset_laser() -> Self {
    |            ^^^^^^^^^^^^
...
302 |     pub fn preset_explosion() -> Self {
    |            ^^^^^^^^^^^^^^^^
...
337 |     pub fn preset_powerup() -> Self {
    |            ^^^^^^^^^^^^^^
...
372 |     pub fn preset_hit() -> Self {
    |            ^^^^^^^^^^
...
402 |     pub fn preset_jump() -> Self {
    |            ^^^^^^^^^^^
...
432 |     pub fn preset_blip() -> Self {
    |            ^^^^^^^^^^^

warning: constant `VPLAY_EXTENSION` is never used
  --> core/src/levelres.rs:11:11
   |
11 | pub const VPLAY_EXTENSION: &str = "vplay";
   |           ^^^^^^^^^^^^^^^

warning: field `name` is never read
  --> core/src/struct_layout.rs:14:9
   |
13 | pub struct StructLayout {
   |            ------------ field in this struct
14 |     pub name: String,
   |         ^^^^
   |
   = note: `StructLayout` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: field `size` is never read
  --> core/src/struct_layout.rs:24:9
   |
21 | pub struct FieldLayout {
   |            ----------- field in this struct
...
24 |     pub size: usize,
   |         ^^^^
   |
   = note: `FieldLayout` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: method `field_offset` is never used
  --> core/src/struct_layout.rs:75:12
   |
27 | impl StructLayout {
   | ----------------- method in this implementation
...
75 |     pub fn field_offset(&self, field_name: &str) -> Option<usize> {
   |            ^^^^^^^^^^^^

warning: `vectrex_lang` (bin "vectrexc") generated 69 warnings (40 duplicates)
    Finished `release` profile [optimized] target(s) in 0.03s
     Running `/Users/daniel/projects/vectrex-pseudo-python/target/release/vectrexc build src/main.vpy`
=== COMPILATION PIPELINE START ===
Input file: src/main.vpy
Target: Vectrex
Binary generation: disabled
Phase 1: Reading source file...
✓ Phase 1 SUCCESS: Read 351 characters
Phase 2: Lexical analysis (tokenization)...
✓ Phase 2 SUCCESS: Generated 45 tokens
Phase 3: Syntax analysis (parsing)...
✓ Phase 3 SUCCESS: Parsed module with 2 top-level items
Phase 0: Asset discovery...
✓ Discovered 7 asset(s):
  - coin (Vector)
  - square (Vector)
  - bubble_huge (Vector)
  - bubble_large (Vector)
  - mountain (Vector)
  - fuji_bg (Vector)
  - test_level (Level)
Phase 4: Code generation (ASM emission)...
[DEBUG] Found SHOW_LEVEL call at analysis
[DEBUG] Found asset usage: test_level (LOAD_LEVEL)
[DEBUG] Level 'test_level' references vector: fuji_bg
[DEBUG] Level 'test_level' references vector: bubble_huge
[DEBUG] Level 'test_level' references vector: coin
[DEBUG] Level 'test_level' references vector: bubble_large
[DEBUG] Used assets: {"bubble_large", "fuji_bg", "test_level", "bubble_huge", "coin"}
[DEBUG] Available assets: ["coin", "square", "bubble_huge", "bubble_large", "mountain", "fuji_bg", "test_level"]
[DEBUG] Assets to embed: 5
✓ Phase 4 SUCCESS: Generated 44060 bytes of assembly
Phase 5: Writing assembly file...
✓ Phase 5 SUCCESS: Written to src/main.asm (target=vectrex)
Phase 5.5: Writing debug symbols file (.pdb)...
✓ Phase 5.5 SUCCESS: Debug symbols written to src/main.pdb
Phase 6: Binary assembly skipped (not requested or target not Vectrex)
=== COMPILATION PIPELINE COMPLETE ===
