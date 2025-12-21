use crate::ast::*;
use crate::target::{info, CpuArch, Target};
use std::collections::{HashSet, HashMap};
use std::cell::RefCell;

use crate::struct_layout::{StructRegistry, build_struct_registry, StructLayout};

// ---------------- Diagnostics (S8) ----------------
// Canal estructurado para warnings (y pronto errores S9).
// S8: warnings estructurados.
// S9: errores semánticos ahora también se recolectan (ya no panic) y se devuelven para que el
// consumidor decida si abortar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticSeverity { Warning, Error }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticCode {
    UnusedVar,
    UndeclaredVar,
    UndeclaredAssign,
    ArityMismatch,
    UndefinedStruct,     // Phase 2: Struct not found
    StructRegistryError, // Phase 2: Error building struct registry
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub code: DiagnosticCode,
    pub message: String,
    pub line: Option<usize>,
    pub col: Option<usize>,
}

thread_local! {
    static TL_ACCUM: RefCell<Vec<Diagnostic>> = RefCell::new(Vec::new());
}

// Tabla centralizada de builtins (nombre normalizado sin prefijo VECTREX_) -> aridad.
// Mantener sincronizada con backend m6809 (emit_builtin_call / scan_expr_runtime).
static BUILTIN_ARITIES: &[(&str, usize)] = &[
    // Funciones unificadas (global + vectorlist)
    ("MOVE", 2),            // was MOVE_TO
    ("PRINT_TEXT", 3),
    ("DRAW_TO", 2),
    ("DRAW_LINE", 5),
    ("SET_ORIGIN", 0),
    ("SET_INTENSITY", 1),
    ("DEBUG_PRINT", 1),
    ("DEBUG_PRINT_LABELED", 2),  // label, value
    ("DEBUG_PRINT_STR", 1),      // string variable
    
    // Asset functions (new)
    ("DRAW_VECTOR", 3),     // Draw vector asset at position: name, x, y
    ("DRAW_VECTOR_EX", 4),  // Draw vector with transformations: name, x, y, mirror (0=normal, 1=flip X)
    ("PLAY_MUSIC", 1),      // Play background music in loop: name
    ("PLAY_SFX", 1),        // Play sound effect (one-shot): name
    ("MUSIC_UPDATE", 0),    // Process music events per frame
    ("SFX_UPDATE", 0),      // Process SFX envelope/pitch per frame
    ("STOP_MUSIC", 0),      // Stop background music
    
    // Malban algorithm (vector list processing)
    ("DRAW_VECTOR_LIST", 4), // Draw vector list: (list_ptr, y, x, scale)
    
    // Funciones específicas de vectorlist
    ("DRAW_VL", 2),
    ("FRAME_BEGIN", 1),
    ("VECTOR_PHASE_BEGIN", 0),
    ("WAIT_RECAL", 0),
    ("PLAY_MUSIC1", 0),
    ("DBG_STATIC_VL", 0),
    
    // Math functions
    ("ABS", 1),             // Absolute value
    
    // Array functions
    ("LEN", 1),             // Get array length
    
    // Inline assembly
    ("ASM", 1),             // Inline assembly string
    
    // Joystick 1 input functions (default = digital)
    ("J1_X", 0),            // Read Joystick 1 X axis (digital: -1/0/+1)
    ("J1_Y", 0),            // Read Joystick 1 Y axis (digital: -1/0/+1)
    ("J1_X_DIGITAL", 0),    // Explicit digital version (-1/0/+1)
    ("J1_Y_DIGITAL", 0),    // Explicit digital version (-1/0/+1)
    ("J1_X_ANALOG", 0),     // Analog version (-127 to +127)
    ("J1_Y_ANALOG", 0),     // Analog version (-127 to +127)
    ("J1_BUTTON_1", 0),     // Read J1 button 1 (0=released, 1=pressed)
    ("J1_BUTTON_2", 0),     // Read J1 button 2
    ("J1_BUTTON_3", 0),     // Read J1 button 3
    ("J1_BUTTON_4", 0),     // Read J1 button 4
    
    // Joystick 2 input functions (default = digital)
    ("J2_X", 0),            // Read Joystick 2 X axis (digital: -1/0/+1)
    ("J2_Y", 0),            // Read Joystick 2 Y axis (digital: -1/0/+1)
    ("J2_X_DIGITAL", 0),    // Explicit digital version (-1/0/+1)
    ("J2_Y_DIGITAL", 0),    // Explicit digital version (-1/0/+1)
    ("J2_X_ANALOG", 0),     // Analog version (-127 to +127)
    ("J2_Y_ANALOG", 0),     // Analog version (-127 to +127)
    ("J2_BUTTON_1", 0),     // Read J2 button 1 (0=released, 1=pressed)
    ("J2_BUTTON_2", 0),     // Read J2 button 2
    ("J2_BUTTON_3", 0),     // Read J2 button 3
    ("J2_BUTTON_4", 0),     // Read J2 button 4
    
    // Compatibilidad hacia atrás (deprecated)
    ("MOVE_TO", 2),         // deprecated: use MOVE
];

fn expected_builtin_arity(name: &str) -> Option<usize> {
    let upper = name.to_ascii_uppercase();
    let core = if let Some(stripped) = upper.strip_prefix("VECTREX_") { stripped } else { upper.as_str() };
    for (n,a) in BUILTIN_ARITIES { if *n == core { return Some(*a); } }
    None
}

// Re-export backend emitters under stable names.
mod backends_ref {
    // pub use crate::backend::arm::emit as emit_arm;  // Desactivado
    // pub use crate::backend::cortexm::emit as emit_cortexm;  // Desactivado
    pub use crate::backend::m6809::emit as emit_6809;
    pub use crate::backend::m6809::emit_with_debug as emit_6809_with_debug;
}

// Asset information for compilation
#[derive(Clone, Debug)]
pub struct AssetInfo {
    pub name: String,      // Asset name without extension (e.g., "player", "theme")
    pub path: String,      // Full path to asset file
    pub asset_type: AssetType,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssetType {
    Vector,  // .vec file
    Music,   // .vmus file (background music, loops)
    Sfx,     // .vsfx file (sound effect, parametric SFXR-style)
}

// CodegenOptions: options affecting generation (title, etc.).
#[derive(Clone)]
pub struct CodegenOptions {
    pub title: String,
    pub auto_loop: bool, // if false, backend must not emit implicit frame loop
    pub diag_freeze: bool,  // instrument init steps with DIAG_COUNTER
    pub force_extended_jsr: bool, // avoid direct-page JSR generation for safety
    // --- New options (Vectrex specific) ---
    pub _bank_size: u32,              // (unused) if >0, ALIGN to this power-of-two (e.g. 4096 or 8192)
    pub per_frame_silence: bool,     // insert JSR VECTREX_SILENCE in frame loop
    pub debug_init_draw: bool,       // draw a small debug vector in INIT to confirm execution
    pub blink_intensity: bool,       // replace fixed INTENSITY_5F with blinking pattern
    pub exclude_ram_org: bool,       // emit RAM variables as EQU instead of ORG-ing into RAM (keeps ROM size small)
    pub fast_wait: bool,             // replace BIOS Wait_Recal with simulated wrapper
    pub source_path: Option<String>, // ruta del archivo fuente para calcular includes relativos
    pub assets: Vec<AssetInfo>,      // Assets to embed in ROM (.vec, .vmus files)
    pub const_values: std::collections::BTreeMap<String, i32>, // Constant values for inlining (nombre_uppercase → valor)
    pub structs: StructRegistry, // Struct layout information (Phase 2)
    pub type_context: HashMap<String, String>, // Maps variable names to struct types (e.g., "p" -> "Point")
    // future: fast_wait_counter could toggle increment of a frame counter
}

// emit_asm: optimize module then dispatch to selected backend.
pub fn emit_asm(module: &Module, target: Target, opts: &CodegenOptions) -> String {
    let (asm, diags) = emit_asm_with_diagnostics(module, target, opts);
    
    // Print all diagnostics to stderr
    for d in &diags {
        match d.severity {
            DiagnosticSeverity::Warning => eprintln!("[warn] {}", d.message),
            DiagnosticSeverity::Error => eprintln!("[error] {}", d.message),
        }
    }
    
    // Return empty string if there were any errors
    let has_errors = diags.iter().any(|d| matches!(d.severity, DiagnosticSeverity::Error));
    if has_errors {
        eprintln!("[codegen] Code generation failed due to {} error(s)", 
                 diags.iter().filter(|d| matches!(d.severity, DiagnosticSeverity::Error)).count());
        return String::new();
    }
    
    asm
}

// emit_asm_with_debug: Same as emit_asm but also returns debug information for .pdb generation
// Currently only M6809/Vectrex backend supports debug info generation
pub fn emit_asm_with_debug(module: &Module, target: Target, opts: &CodegenOptions) 
    -> (String, Option<crate::backend::debug_info::DebugInfo>, Vec<Diagnostic>) 
{
    use crate::target::CpuArch;
    
    // Phase 2 Step 1: Build struct registry from module
    let struct_registry = match build_struct_registry(&module.items) {
        Ok(registry) => registry,
        Err(e) => {
            let mut diagnostics = vec![Diagnostic {
                severity: DiagnosticSeverity::Error,
                code: DiagnosticCode::StructRegistryError,
                message: e,
                line: None,
                col: None,
            }];
            return (String::new(), None, diagnostics);
        }
    };
    
    // Paso 1: validación semántica básica (variables / aridad) recolectando warnings.
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    let type_context = validate_semantics_with_structs(module, &struct_registry, &mut diagnostics);
    let has_errors = diagnostics.iter().any(|d| matches!(d.severity, DiagnosticSeverity::Error));
    if has_errors {
        return (String::new(), None, diagnostics);
    }
    
    // Paso 2: pipeline de optimización (dead_store_elim preserva asignaciones con literales string).
    let optimized = optimize_module(module);
    let ti = info(target);
    
    // If source defines CONST TITLE = "..." let it override CLI title.
    let mut effective = CodegenOptions { 
        structs: struct_registry, // Add struct registry to options
        type_context, // Add type context for method resolution
        ..opts.clone() 
    };
    if let Some(t) = optimized.meta.title_override.clone() { effective.title = t; }
    
    // Generate ASM and debug info
    let (asm, debug_info) = match ti.arch {
        CpuArch::M6809 => {
            let (asm, dbg) = backends_ref::emit_6809_with_debug(&optimized, target, &ti, &effective);
            (asm, Some(dbg))
        },
        CpuArch::Arm => panic!("ARM backend desactivado temporalmente"),
        CpuArch::CortexM => panic!("Cortex-M backend desactivado temporalmente"),
    };
    
    (asm, debug_info, diagnostics)
}

// Nueva API estructurada (S8). Mantiene mismo comportamiento pero devuelve diagnostics.
pub fn emit_asm_with_diagnostics(module: &Module, target: Target, opts: &CodegenOptions) -> (String, Vec<Diagnostic>) {
    // Paso 1: validación semántica básica (variables / aridad) recolectando warnings.
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    validate_semantics(module, &mut diagnostics);
    let has_errors = diagnostics.iter().any(|d| matches!(d.severity, DiagnosticSeverity::Error));
    if has_errors {
        return (String::new(), diagnostics);
    }
    // Paso 2: pipeline de optimización (dead_store_elim preserva asignaciones con literales string).
    let optimized = optimize_module(module);
    let ti = info(target);
    // If source defines CONST TITLE = "..." let it override CLI title.
    let mut effective = CodegenOptions { ..opts.clone() };
    if let Some(t) = optimized.meta.title_override.clone() { effective.title = t; }
    // Pass music/copyright through metas hashmap for backend (reuse existing fields via metas)
    if optimized.meta.music_override.is_some() { /* backend reads module.meta.music_override */ }
    let asm = match ti.arch {
        CpuArch::M6809 => backends_ref::emit_6809(&optimized, target, &ti, &effective),
        CpuArch::Arm => panic!("ARM backend desactivado temporalmente"),
        CpuArch::CortexM => panic!("Cortex-M backend desactivado temporalmente"),
    };
    (asm, diagnostics)
}

// optimize_module: iterative fixpoint optimization pipeline (max 5 iterations).
// Pass order per iteration:
// 1. opt_item / opt_expr: constant folding, algebraic simplifications (16-bit truncation)
// 2. dead_code_elim: prune unreachable code and empty loops
// 3. propagate_constants: forward constant propagation with branch merging
// 4. dead_store_elim: eliminate unused assignments without side-effects
// 5. fold_const_switches: replace switch whose expression & cases are all constant numbers with selected body (or default)
#[allow(dead_code)]
pub fn debug_optimize_module_for_tests(m: &Module) -> Module { optimize_module(m) }

fn optimize_module(m: &Module) -> Module {
    // Enable ONLY safe optimizations - disable problematic ones that eliminate arithmetic operations
    let mut current = m.clone();
    for _ in 0..5 {
        let folded: Module = Module { items: current.items.iter().map(opt_item).collect(), meta: current.meta.clone(), imports: current.imports.clone() };
        let dce = dead_code_elim(&folded);
        // DISABLE propagate_constants - eliminates arithmetic operations incorrectly
        let cp = dce; // Skip constant propagation
        // DISABLE dead_store_elim - eliminates variable assignments incorrectly  
        let ds = cp; // Skip dead store elimination
        // Enable fold_const_switches - this is safe for control flow
        let sw = fold_const_switches(&ds);
        if sw == current {
            break;
        }
        current = sw;
    }
    current
}

// ---------------- Semántica básica ----------------
// validate_semantics: asegura que toda variable usada ha sido declarada previamente en su ámbito
// (modelo simple: ámbitos anidados para funciones y bucles). No hace shadowing complejo; permite
// shadowing por Let local (esto sobrescribe variable anterior). Las Const y GlobalLet son visibles
// para todas las funciones (ya que se resolvieron en parse a este AST plano y el lenguaje actual
// no define módulos). Las params son visibles en el cuerpo de la función.
pub fn validate_semantics(module: &Module, diagnostics: &mut Vec<Diagnostic>) {
    // Recolectar globals declaradas (Const + GlobalLet + VectorList nombres no son variables de expr)
    let mut globals: HashSet<String> = HashSet::new();
    for it in &module.items {
        match it {
            Item::Const { name, .. } | Item::GlobalLet { name, .. } => { globals.insert(name.clone()); },
            Item::VectorList { .. } => {},
            Item::Function(_) => {},
            Item::ExprStatement(_) => {}, // Expression statements no definen globals
            Item::Export(_) => {}, // Export declarations don't define globals
            Item::StructDef(_) => {}, // Struct definitions don't define globals
        }
    }
    
    // Recolectar nombres de todas las funciones definidas en el módulo
    let mut defined_functions: HashSet<String> = HashSet::new();
    for it in &module.items {
        if let Item::Function(func) = it {
            defined_functions.insert(func.name.clone());
        }
        // Also add struct names so they can be "called" as constructors
        if let Item::StructDef(struct_def) = it {
            defined_functions.insert(struct_def.name.clone());
        }
    }
    
    // Recolectar todas las variables locales de cada función para detección de cross-function usage
    let mut function_locals: HashMap<String, HashSet<String>> = HashMap::new();
    for it in &module.items {
        if let Item::Function(func) = it {
            let mut locals = HashSet::new();
            collect_function_locals(&func.body, &mut locals, &globals);
            function_locals.insert(func.name.clone(), locals);
        }
    }
    
    // Validar cada función independientemente.
    for it in &module.items {
        if let Item::Function(func) = it {
            TL_ACCUM.with(|acc| acc.borrow_mut().clear());
            validate_function(func, &globals, &function_locals, &defined_functions, diagnostics);
            // Mover errores recolectados (uso/assign/arity) del thread-local
            TL_ACCUM.with(|acc| diagnostics.extend(acc.borrow().iter().cloned()));
        }
    }
}

// Helper para recolectar todas las variables locales declaradas en una función
fn collect_function_locals(stmts: &[Stmt], locals: &mut HashSet<String>, globals: &HashSet<String>) {
    for stmt in stmts {
        match stmt {
            Stmt::Let { name, .. } => { locals.insert(name.clone()); }
            // NEW: Primera asignación a nombre no global es declaración implícita
            Stmt::Assign { target, .. } => {
                if let crate::ast::AssignTarget::Ident { name, .. } = target {
                    if !globals.contains(name) {
                        locals.insert(name.clone());
                    }
                }
            }
            Stmt::For { var, body, .. } => {
                locals.insert(var.clone());
                collect_function_locals(body, locals, globals);
            }
            Stmt::While { body, .. } => collect_function_locals(body, locals, globals),
            Stmt::If { body, elifs, else_body, .. } => {
                collect_function_locals(body, locals, globals);
                for (_, elif_body) in elifs {
                    collect_function_locals(elif_body, locals, globals);
                }
                if let Some(else_body) = else_body {
                    collect_function_locals(else_body, locals, globals);
                }
            }
            Stmt::Switch { cases, default, .. } => {
                for (_, case_body) in cases {
                    collect_function_locals(case_body, locals, globals);
                }
                if let Some(default_body) = default {
                    collect_function_locals(default_body, locals, globals);
                }
            }
            _ => {}
        }
    }
}

// Helper to collect type information from struct initializations
fn collect_function_types(
    stmts: &[Stmt], 
    type_context: &mut HashMap<String, String>,
    struct_registry: &StructRegistry
) {
    for stmt in stmts {
        match stmt {
            Stmt::Let { name, value, .. } => {
                if let Some(struct_name) = extract_struct_type(value, struct_registry) {
                    type_context.insert(name.clone(), struct_name);
                }
            }
            Stmt::Assign { target, value, .. } => {
                if let crate::ast::AssignTarget::Ident { name, .. } = target {
                    if let Some(struct_name) = extract_struct_type(value, struct_registry) {
                        type_context.insert(name.clone(), struct_name);
                    }
                }
            }
            Stmt::For { body, .. } => collect_function_types(body, type_context, struct_registry),
            Stmt::While { body, .. } => collect_function_types(body, type_context, struct_registry),
            Stmt::If { body, elifs, else_body, .. } => {
                collect_function_types(body, type_context, struct_registry);
                for (_, elif_body) in elifs {
                    collect_function_types(elif_body, type_context, struct_registry);
                }
                if let Some(else_body) = else_body {
                    collect_function_types(else_body, type_context, struct_registry);
                }
            }
            Stmt::Switch { cases, default, .. } => {
                for (_, case_body) in cases {
                    collect_function_types(case_body, type_context, struct_registry);
                }
                if let Some(default_body) = default {
                    collect_function_types(default_body, type_context, struct_registry);
                }
            }
            _ => {}
        }
    }
}

// Extract struct type name from expression (for StructInit)
fn extract_struct_type(expr: &Expr, struct_registry: &StructRegistry) -> Option<String> {
    match expr {
        Expr::StructInit { struct_name, .. } => {
            // Verify struct exists in registry
            if struct_registry.contains_key(struct_name) {
                Some(struct_name.clone())
            } else {
                None
            }
        }
        Expr::Call(ci) => {
            // Check if this is a struct constructor call
            // Constructor calls look like: Entity(x, y, dx, dy)
            if struct_registry.contains_key(&ci.name) {
                Some(ci.name.clone())
            } else {
                None
            }
        }
        _ => None
    }
}

// validate_semantics_with_structs: Extended semantic validation with struct support (Phase 2)
pub fn validate_semantics_with_structs(
    module: &Module, 
    struct_registry: &StructRegistry,
    diagnostics: &mut Vec<Diagnostic>
) -> HashMap<String, String> {
    // First, do standard validation
    validate_semantics(module, diagnostics);
    
    // Phase 2: Collect type information from struct initializations
    let mut type_context = HashMap::new();
    for item in &module.items {
        if let Item::Function(func) = item {
            collect_function_types(&func.body, &mut type_context, struct_registry);
            validate_function_structs(func, struct_registry, diagnostics);
        } else if let Item::StructDef(s) = item {
            // Also collect types from struct methods
            for method in &s.methods {
                collect_function_types(&method.body, &mut type_context, struct_registry);
            }
        }
    }
    
    type_context
}

fn validate_function_structs(
    func: &Function,
    struct_registry: &StructRegistry,
    diagnostics: &mut Vec<Diagnostic>
) {
    for stmt in &func.body {
        validate_stmt_structs(stmt, struct_registry, diagnostics);
    }
}

fn validate_stmt_structs(
    stmt: &Stmt,
    struct_registry: &StructRegistry,
    diagnostics: &mut Vec<Diagnostic>
) {
    match stmt {
        Stmt::Assign { target, value, .. } => {
            // Validate field access in assignment target
            if let crate::ast::AssignTarget::FieldAccess { target: obj_expr, field, .. } = target {
                validate_field_access_assignment(obj_expr, field, struct_registry, diagnostics);
            }
            validate_expr_structs(value, struct_registry, diagnostics);
        }
        Stmt::Expr(expr, _) => {
            validate_expr_structs(expr, struct_registry, diagnostics);
        }
        Stmt::If { cond, body, elifs, else_body, .. } => {
            validate_expr_structs(cond, struct_registry, diagnostics);
            for s in body {
                validate_stmt_structs(s, struct_registry, diagnostics);
            }
            for (elif_cond, elif_body) in elifs {
                validate_expr_structs(elif_cond, struct_registry, diagnostics);
                for s in elif_body {
                    validate_stmt_structs(s, struct_registry, diagnostics);
                }
            }
            if let Some(eb) = else_body {
                for s in eb {
                    validate_stmt_structs(s, struct_registry, diagnostics);
                }
            }
        }
        Stmt::While { cond, body, .. } => {
            validate_expr_structs(cond, struct_registry, diagnostics);
            for s in body {
                validate_stmt_structs(s, struct_registry, diagnostics);
            }
        }
        Stmt::For { body, .. } => {
            for s in body {
                validate_stmt_structs(s, struct_registry, diagnostics);
            }
        }
        Stmt::Return(Some(expr), _) => {
            validate_expr_structs(expr, struct_registry, diagnostics);
        }
        Stmt::Switch { expr, cases, default, .. } => {
            validate_expr_structs(expr, struct_registry, diagnostics);
            for (case_expr, case_body) in cases {
                validate_expr_structs(case_expr, struct_registry, diagnostics);
                for s in case_body {
                    validate_stmt_structs(s, struct_registry, diagnostics);
                }
            }
            if let Some(def_body) = default {
                for s in def_body {
                    validate_stmt_structs(s, struct_registry, diagnostics);
                }
            }
        }
        _ => {}
    }
}

fn validate_expr_structs(
    expr: &Expr,
    struct_registry: &StructRegistry,
    diagnostics: &mut Vec<Diagnostic>
) {
    match expr {
        Expr::StructInit { struct_name, source_line, col } => {
            // First check if this is actually a builtin function call, not a struct init
            let is_builtin = BUILTIN_ARITIES.iter().any(|(name, _)| *name == struct_name.as_str());
            
            if !is_builtin && !struct_registry.contains_key(struct_name) {
                diagnostics.push(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    code: DiagnosticCode::UndefinedStruct,
                    message: format!("Undefined struct '{}'", struct_name),
                    line: Some(*source_line),
                    col: Some(*col),
                });
            }
        }
        Expr::FieldAccess { target, field, source_line, col } => {
            // For now, we can't easily determine the type of target expression
            // This will be improved in Phase 3 with type inference
            // For simple cases like `obj.field`, we could track variable types
            // TODO Phase 3: Add type tracking to validate field exists on the struct type
            validate_expr_structs(target, struct_registry, diagnostics);
        }
        Expr::Binary { left, right, .. } | Expr::Compare { left, right, .. } | Expr::Logic { left, right, .. } => {
            validate_expr_structs(left, struct_registry, diagnostics);
            validate_expr_structs(right, struct_registry, diagnostics);
        }
        Expr::Not(inner) | Expr::BitNot(inner) => {
            validate_expr_structs(inner, struct_registry, diagnostics);
        }
        Expr::Call(call_info) => {
            for arg in &call_info.args {
                validate_expr_structs(arg, struct_registry, diagnostics);
            }
        }
        Expr::List(elements) => {
            for elem in elements {
                validate_expr_structs(elem, struct_registry, diagnostics);
            }
        }
        Expr::Index { target, index } => {
            validate_expr_structs(target, struct_registry, diagnostics);
            validate_expr_structs(index, struct_registry, diagnostics);
        }
        _ => {}
    }
}

fn validate_field_access_assignment(
    _obj_expr: &Expr,
    _field: &str,
    _struct_registry: &StructRegistry,
    _diagnostics: &mut Vec<Diagnostic>
) {
    // Phase 2: Basic validation - just ensure struct registry is available
    // Phase 3 will add full type tracking to validate field exists on specific struct type
    // For now, we'll validate at runtime that the field access is correct
}

fn validate_function(f: &Function, globals: &HashSet<String>, function_locals: &HashMap<String, HashSet<String>>, defined_functions: &HashSet<String>, diagnostics: &mut Vec<Diagnostic>) {
    // ámbito inicial: globals + params
    let mut scope: Vec<HashSet<String>> = Vec::new();
    scope.push(globals.clone());
    let mut param_set: HashSet<String> = HashSet::new();
    for p in &f.params { param_set.insert(p.clone()); }
    scope.push(param_set);
    // tracking de lecturas para warning de variables no usadas
    let mut reads: HashSet<String> = HashSet::new();
    for stmt in &f.body { validate_stmt_collect(stmt, &mut scope, &mut reads, &f.name, function_locals, defined_functions); }
    // Advertencias (stderr) para variables declaradas pero no leídas (excluye params por ahora)
    let mut declared: HashSet<String> = HashSet::new();
    for frame in &scope { for v in frame { declared.insert(v.clone()); } }
    for d in declared {
        if !reads.contains(&d) && !f.params.contains(&d) && !globals.contains(&d) {
            diagnostics.push(Diagnostic { severity: DiagnosticSeverity::Warning, code: DiagnosticCode::UnusedVar, message: format!("[unused-var] funcion='{}' var='{}'", f.name, d), line: None, col: None });
        }
    }
}

fn push_scope(scope: &mut Vec<HashSet<String>>) { scope.push(HashSet::new()); }
fn pop_scope(scope: &mut Vec<HashSet<String>>) { scope.pop(); }

fn declare(name: &str, scope: &mut Vec<HashSet<String>>) { if let Some(top) = scope.last_mut() { top.insert(name.to_string()); } }

fn is_declared(name: &str, scope: &Vec<HashSet<String>>) -> bool {
    for s in scope.iter().rev() { if s.contains(name) { return true; } }
    false
}

#[allow(dead_code)]
fn validate_stmt(stmt: &Stmt, scope: &mut Vec<HashSet<String>>) { 
    validate_stmt_collect(stmt, scope, &mut HashSet::new(), "unknown", &HashMap::new(), &HashSet::new()); 
}

fn validate_stmt_collect(
    stmt: &Stmt, 
    scope: &mut Vec<HashSet<String>>, 
    reads: &mut HashSet<String>,
    current_func: &str,
    function_locals: &HashMap<String, HashSet<String>>,
    defined_functions: &HashSet<String>
) {
    match stmt {
        Stmt::Let { name, value, .. } => { 
            validate_expr_collect(value, scope, reads, current_func, function_locals, defined_functions); 
            declare(name, scope); 
        }
        Stmt::Assign { target, value, .. } => {
            match target {
                crate::ast::AssignTarget::Ident { name, source_line, col } => {
                    // NEW: Primera asignación a nombre no declarado es declaración implícita
                    // (matching behavior de collect_locals en backend)
                    if !is_declared(name, scope) {
                        declare(name, scope); // Declaración implícita
                    }
                }
                crate::ast::AssignTarget::Index { target: array_expr, index, .. } => {
                    // For indexed assignment, validate both target and index expressions
                    validate_expr_collect(array_expr, scope, reads, current_func, function_locals, defined_functions);
                    validate_expr_collect(index, scope, reads, current_func, function_locals, defined_functions);
                }
                crate::ast::AssignTarget::FieldAccess { target: obj_expr, field, .. } => {
                    // For field assignment, validate the target object expression
                    validate_expr_collect(obj_expr, scope, reads, current_func, function_locals, defined_functions);
                    // Field names are not variables, so no declaration needed
                }
            }
            validate_expr_collect(value, scope, reads, current_func, function_locals, defined_functions);
        }
        Stmt::CompoundAssign { target, value, .. } => {
            // Similar a Assign, pero también leemos la variable del lado izquierdo
            match target {
                crate::ast::AssignTarget::Ident { name, source_line, col } => {
                    if !is_declared(name, scope) {
                        TL_ACCUM.with(|acc| acc.borrow_mut().push(Diagnostic { severity: DiagnosticSeverity::Error, code: DiagnosticCode::UndeclaredAssign, message: format!("SemanticsError: asignación compuesta a variable no declarada '{}'. Declárala primero con '{} = ...' antes de usar '{} += ...'", name, name, name), line: Some(*source_line), col: Some(*col) }));
                    }
                    reads.insert(name.clone()); // Leemos la variable para x += expr
                }
                crate::ast::AssignTarget::Index { target: array_expr, index, .. } => {
                    // For indexed compound assignment, validate both target and index expressions
                    validate_expr_collect(array_expr, scope, reads, current_func, function_locals, defined_functions);
                    validate_expr_collect(index, scope, reads, current_func, function_locals, defined_functions);
                }
                crate::ast::AssignTarget::FieldAccess { target: obj_expr, field, .. } => {
                    // For field compound assignment, validate the target object
                    validate_expr_collect(obj_expr, scope, reads, current_func, function_locals, defined_functions);
                }
            }
            validate_expr_collect(value, scope, reads, current_func, function_locals, defined_functions);
        }
        Stmt::For { var, start, end, step, body, .. } => {
            validate_expr_collect(start, scope, reads, current_func, function_locals, defined_functions); 
            validate_expr_collect(end, scope, reads, current_func, function_locals, defined_functions); 
            if let Some(se) = step { 
                validate_expr_collect(se, scope, reads, current_func, function_locals, defined_functions); 
            }
            push_scope(scope); // cuerpo loop con var declarada
            declare(var, scope);
            for s in body { validate_stmt_collect(s, scope, reads, current_func, function_locals, defined_functions); }
            pop_scope(scope);
        }
        Stmt::ForIn { var, iterable, body, .. } => {
            validate_expr_collect(iterable, scope, reads, current_func, function_locals, defined_functions); 
            push_scope(scope);
            declare(var, scope);
            for s in body { validate_stmt_collect(s, scope, reads, current_func, function_locals, defined_functions); }
            pop_scope(scope);
        }
        Stmt::While { cond, body, .. } => {
            validate_expr_collect(cond, scope, reads, current_func, function_locals, defined_functions);
            push_scope(scope);
            for s in body { validate_stmt_collect(s, scope, reads, current_func, function_locals, defined_functions); }
            pop_scope(scope);
        }
        Stmt::If { cond, body, elifs, else_body, .. } => {
            validate_expr_collect(cond, scope, reads, current_func, function_locals, defined_functions);
            push_scope(scope); 
            for s in body { validate_stmt_collect(s, scope, reads, current_func, function_locals, defined_functions); } 
            pop_scope(scope);
            for (ec, eb) in elifs { 
                validate_expr_collect(ec, scope, reads, current_func, function_locals, defined_functions); 
                push_scope(scope); 
                for s in eb { validate_stmt_collect(s, scope, reads, current_func, function_locals, defined_functions); } 
                pop_scope(scope); 
            }
            if let Some(eb) = else_body { 
                push_scope(scope); 
                for s in eb { validate_stmt_collect(s, scope, reads, current_func, function_locals, defined_functions); } 
                pop_scope(scope); 
            }
        }
        Stmt::Switch { expr, cases, default, .. } => {
            validate_expr_collect(expr, scope, reads, current_func, function_locals, defined_functions);
            for (ce, cb) in cases { 
                validate_expr_collect(ce, scope, reads, current_func, function_locals, defined_functions); 
                push_scope(scope); 
                for s in cb { validate_stmt_collect(s, scope, reads, current_func, function_locals, defined_functions); } 
                pop_scope(scope); 
            }
            if let Some(db) = default { 
                push_scope(scope); 
                for s in db { validate_stmt_collect(s, scope, reads, current_func, function_locals, defined_functions); } 
                pop_scope(scope); 
            }
        }
        Stmt::Expr(e, _) => validate_expr_collect(e, scope, reads, current_func, function_locals, defined_functions),
        Stmt::Return(o, _) => { 
            if let Some(e) = o { 
                validate_expr_collect(e, scope, reads, current_func, function_locals, defined_functions); 
            } 
        }
        Stmt::Break { .. } | Stmt::Continue { .. } | Stmt::Pass { .. } => {}
    }
}

#[allow(dead_code)]
fn validate_expr(e: &Expr, scope: &mut Vec<HashSet<String>>) { 
    let mut dummy=HashSet::new(); 
    validate_expr_collect(e, scope, &mut dummy, "unknown", &HashMap::new(), &HashSet::new()); 
}

fn validate_expr_collect(
    e: &Expr, 
    scope: &mut Vec<HashSet<String>>, 
    reads: &mut HashSet<String>,
    current_func: &str,
    function_locals: &HashMap<String, HashSet<String>>,
    defined_functions: &HashSet<String>
) {
    match e {
        Expr::Ident(info) => {
            if !is_declared(&info.name, scope) {
                // Variable no está en scope actual, verificar si está en otra función
                let mut found_in_other_func = None;
                for (func_name, locals) in function_locals.iter() {
                    if func_name != current_func && locals.contains(&info.name) {
                        found_in_other_func = Some(func_name.clone());
                        break;
                    }
                }
                
                let error_msg = if let Some(other_func) = found_in_other_func {
                    format!(
                        "SemanticsError: variable '{}' declarada en función '{}' no es accesible en '{}'. \
                        Las funciones en VPy tienen scopes separados (no comparten variables). \
                        Solución: declara '{}' dentro de '{}' donde la necesitas.",
                        info.name, other_func, current_func, info.name, current_func
                    )
                } else {
                    format!("SemanticsError: uso de variable no declarada '{}'.", info.name)
                };
                
                TL_ACCUM.with(|acc| acc.borrow_mut().push(Diagnostic { 
                    severity: DiagnosticSeverity::Error, 
                    code: DiagnosticCode::UndeclaredVar, 
                    message: error_msg, 
                    line: Some(info.source_line), 
                    col: Some(info.col) 
                }));
            } else { 
                reads.insert(info.name.clone()); 
            }
        }
        Expr::Call(ci) => {
            // Verificar si es builtin o función definida
            if let Some(exp) = expected_builtin_arity(&ci.name) {
                // Es builtin - verificar aridad
                if ci.args.len() != exp {
                    TL_ACCUM.with(|acc| acc.borrow_mut().push(Diagnostic { severity: DiagnosticSeverity::Error, code: DiagnosticCode::ArityMismatch, message: format!("SemanticsErrorArity: llamada a '{}' con {} argumentos; se esperaban {}.", ci.name, ci.args.len(), exp), line: Some(ci.source_line), col: Some(ci.col) }));
                }
            } else {
                // No es builtin - verificar que la función existe
                if !defined_functions.contains(&ci.name) {
                    TL_ACCUM.with(|acc| acc.borrow_mut().push(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        code: DiagnosticCode::UndeclaredVar,
                        message: format!("Unknown function '{}'", ci.name),
                        line: Some(ci.source_line),
                        col: Some(ci.col)
                    }));
                }
            }
            for a in &ci.args { validate_expr_collect(a, scope, reads, current_func, function_locals, defined_functions); }
        }
        Expr::MethodCall(mc) => {
            // Method calls: validate target and arguments
            // TODO: Add method resolution and struct type checking
            validate_expr_collect(&mc.target, scope, reads, current_func, function_locals, defined_functions);
            for a in &mc.args { validate_expr_collect(a, scope, reads, current_func, function_locals, defined_functions); }
        }
        Expr::Binary { left, right, .. } | Expr::Compare { left, right, .. } | Expr::Logic { left, right, .. } => {
            validate_expr_collect(left, scope, reads, current_func, function_locals, defined_functions); 
            validate_expr_collect(right, scope, reads, current_func, function_locals, defined_functions);
        }
        Expr::Not(inner) | Expr::BitNot(inner) => validate_expr_collect(inner, scope, reads, current_func, function_locals, defined_functions),
        Expr::List(elements) => {
            for elem in elements {
                validate_expr_collect(elem, scope, reads, current_func, function_locals, defined_functions);
            }
        }
        Expr::Index { target, index } => {
            validate_expr_collect(target, scope, reads, current_func, function_locals, defined_functions);
            validate_expr_collect(index, scope, reads, current_func, function_locals, defined_functions);
        }
        Expr::StructInit { .. } => {
            // Struct initialization - validation happens in semantic analysis phase
            // TODO: Verify struct exists in Phase 2
        }
        Expr::FieldAccess { target, .. } => {
            // Field access - validate the target object
            validate_expr_collect(target, scope, reads, current_func, function_locals, defined_functions);
            // Field names are not variables, so no additional validation needed here
        }
        Expr::Number(_) | Expr::StringLit(_) => {}
    }
}

fn opt_item(it: &Item) -> Item { 
    match it { 
        Item::Function(f) => Item::Function(opt_function(f)), 
        Item::Const { name, value } => Item::Const { name: name.clone(), value: opt_expr(value) }, 
        Item::GlobalLet { name, value } => Item::GlobalLet { name: name.clone(), value: opt_expr(value) }, 
        Item::VectorList { name, entries } => Item::VectorList { name: name.clone(), entries: entries.clone() },
        Item::ExprStatement(expr) => Item::ExprStatement(opt_expr(expr)),
        Item::Export(e) => Item::Export(e.clone()),
        Item::StructDef(s) => Item::StructDef(s.clone()), // Structs don't need optimization
    } 
}

fn opt_function(f: &Function) -> Function {
    Function {
        name: f.name.clone(),
        line: f.line,
        params: f.params.clone(),
        body: f.body.iter().map(opt_stmt).collect(),
    }
}

fn opt_stmt(s: &Stmt) -> Stmt {
    let source_line = s.source_line(); // Preserve original line number
    match s {
    Stmt::Assign { target, value, .. } => Stmt::Assign { target: target.clone(), value: opt_expr(value), source_line },
    Stmt::Let { name, value, .. } => Stmt::Let { name: name.clone(), value: opt_expr(value), source_line },
        Stmt::CompoundAssign { target, op, value, .. } => {
            // Transformar x += expr en x = x + expr
            let var_expr = match target {
                crate::ast::AssignTarget::Ident { name, source_line, col } => {
                    Expr::Ident(IdentInfo { 
                        name: name.clone(), 
                        source_line: *source_line, 
                        col: *col 
                    })
                }
                crate::ast::AssignTarget::Index { target: array_expr, index, source_line, .. } => {
                    // For array[i] += expr, we need array[i] as the left side
                    Expr::Index { 
                        target: array_expr.clone(), 
                        index: index.clone()
                    }
                }
                crate::ast::AssignTarget::FieldAccess { target: obj_expr, field, source_line, col } => {
                    // For obj.field += expr, we need obj.field as the left side
                    Expr::FieldAccess {
                        target: obj_expr.clone(),
                        field: field.clone(),
                        source_line: *source_line,
                        col: *col,
                    }
                }
            };
            let combined_expr = Expr::Binary { 
                op: *op, 
                left: Box::new(var_expr), 
                right: Box::new(opt_expr(value)) 
            };
            Stmt::Assign { target: target.clone(), value: combined_expr, source_line }
        },
        Stmt::For { var, start, end, step, body, .. } => Stmt::For {
            var: var.clone(),
            start: opt_expr(start),
            end: opt_expr(end),
            step: step.as_ref().map(opt_expr),
            body: body.iter().map(opt_stmt).collect(),
            source_line,
        },
        Stmt::ForIn { var, iterable, body, .. } => Stmt::ForIn {
            var: var.clone(),
            iterable: opt_expr(iterable),
            body: body.iter().map(opt_stmt).collect(),
            source_line,
        },
        Stmt::While { cond, body, .. } => Stmt::While { cond: opt_expr(cond), body: body.iter().map(opt_stmt).collect(), source_line },
        Stmt::Expr(e, _) => Stmt::Expr(opt_expr(e), source_line),
        Stmt::If { cond, body, elifs, else_body, .. } => Stmt::If {
            cond: opt_expr(cond),
            body: body.iter().map(opt_stmt).collect(),
            elifs: elifs.iter().map(|(c, b)| (opt_expr(c), b.iter().map(opt_stmt).collect())).collect(),
            else_body: else_body.as_ref().map(|v| v.iter().map(opt_stmt).collect()),
            source_line,
        },
        Stmt::Return(o, _) => Stmt::Return(o.as_ref().map(opt_expr), source_line),
    Stmt::Break { .. } => Stmt::Break { source_line },
    Stmt::Continue { .. } => Stmt::Continue { source_line },
    Stmt::Pass { .. } => Stmt::Pass { source_line },
    Stmt::Switch { expr, cases, default, .. } => Stmt::Switch { expr: opt_expr(expr), cases: cases.iter().map(|(e,b)| (opt_expr(e), b.iter().map(opt_stmt).collect())).collect(), default: default.as_ref().map(|v| v.iter().map(opt_stmt).collect()), source_line },
    }
}

const INT_MASK: i32 = 0xFFFF; // unify 16-bit integer model across backends
fn trunc16(v: i32) -> i32 { v & INT_MASK }

fn opt_expr(e: &Expr) -> Expr {
    match e {
        Expr::Binary { op, left, right } => {
            let l = opt_expr(left);
            let r = opt_expr(right);
            match (&l, &r, op) {
                (Expr::Number(0), Expr::Number(_), BinOp::Add) => return r.clone(),
                (Expr::Number(_), Expr::Number(0), BinOp::Add) => return l.clone(),
                (Expr::Number(_), Expr::Number(0), BinOp::Sub) => return l.clone(),
                (_, Expr::Number(1), BinOp::Mul) => return l.clone(),
                (Expr::Number(1), _, BinOp::Mul) => return r.clone(),
                (_, Expr::Number(0), BinOp::Mul) | (Expr::Number(0), _, BinOp::Mul) => return Expr::Number(0),
                (_, Expr::Number(1), BinOp::Div) => return l.clone(),
                _ => {}
            }
            if let (Expr::Number(a), Expr::Number(b)) = (&l, &r) {
                let raw = match op {
                    BinOp::Add => a.wrapping_add(*b),
                    BinOp::Sub => a.wrapping_sub(*b),
                    BinOp::Mul => a.wrapping_mul(*b),
                    BinOp::Div => if *b != 0 { a / b } else { *a },
                    BinOp::FloorDiv => if *b != 0 { a / b } else { *a }, // División entera (igual que Div en enteros)
                    BinOp::Mod => if *b != 0 { a % b } else { *a },
                    BinOp::Shl => a.wrapping_shl((*b & 0xF) as u32),
                    BinOp::Shr => ((*a as u32) >> (*b & 0xF)) as i32,
                    BinOp::BitAnd => a & b,
                    BinOp::BitOr => a | b,
                    BinOp::BitXor => a ^ b,
                };
                Expr::Number(trunc16(raw))
            } else {
                // Bitwise identities / annihilators
                match op {
                    BinOp::BitAnd => {
                        if matches!(r, Expr::Number(0)) || matches!(l, Expr::Number(0)) { return Expr::Number(0); }
                        if let Expr::Number(n) = r { if n == 0xFFFF { return l; } }
                        if let Expr::Number(n) = l { if n == 0xFFFF { return r; } }
                    }
                    BinOp::BitOr => {
                        if matches!(r, Expr::Number(0)) { return l; }
                        if matches!(l, Expr::Number(0)) { return r; }
                    }
                    BinOp::BitXor => {
                        if matches!(r, Expr::Number(0)) { return l; }
                        if matches!(l, Expr::Number(0)) { return r; }
                    }
                    BinOp::Mod => {
                        if matches!(r, Expr::Number(1)) { return Expr::Number(0); }
                        if matches!(l, Expr::Number(0)) { return Expr::Number(0); }
                    }
                    BinOp::Shl | BinOp::Shr => {
                        if matches!(r, Expr::Number(0)) { return l; }
                        if matches!(l, Expr::Number(0)) { return Expr::Number(0); }
                    }
                    _ => {}
                }
                Expr::Binary { op: *op, left: Box::new(l), right: Box::new(r) }
            }
        }
        Expr::BitNot(inner) => {
            let i2 = opt_expr(inner);
            if let Expr::Number(n) = i2 { Expr::Number(trunc16(!n)) } else { Expr::BitNot(Box::new(i2)) }
        }
        Expr::Compare { op, left, right } => {
            let l = opt_expr(left);
            let r = opt_expr(right);
            if let (Expr::Number(a), Expr::Number(b)) = (&l, &r) {
                let a16 = trunc16(*a);
                let b16 = trunc16(*b);
                let res = match op {
                    CmpOp::Eq => a16 == b16,
                    CmpOp::Ne => a16 != b16,
                    CmpOp::Lt => a16 < b16,
                    CmpOp::Le => a16 <= b16,
                    CmpOp::Gt => a16 > b16,
                    CmpOp::Ge => a16 >= b16,
                };
                Expr::Number(if res { 1 } else { 0 })
            } else {
                Expr::Compare { op: *op, left: Box::new(l), right: Box::new(r) }
            }
        }
        Expr::Logic { op, left, right } => {
            let l = opt_expr(left);
            let r = opt_expr(right);
            if let (Expr::Number(a), Expr::Number(b)) = (&l, &r) {
                let lv = trunc16(*a) != 0;
                let rv = trunc16(*b) != 0;
                let res = match op { LogicOp::And => lv && rv, LogicOp::Or => lv || rv };
                Expr::Number(if res { 1 } else { 0 })
            } else {
                Expr::Logic { op: *op, left: Box::new(l), right: Box::new(r) }
            }
        }
        Expr::List(elements) => Expr::List(elements.iter().map(opt_expr).collect()),
        Expr::Index { target, index } => Expr::Index { 
            target: Box::new(opt_expr(target)), 
            index: Box::new(opt_expr(index)) 
        },
        Expr::StructInit { struct_name, source_line, col } => {
            // Struct initialization doesn't need optimization
            Expr::StructInit { 
                struct_name: struct_name.clone(), 
                source_line: *source_line, 
                col: *col 
            }
        }
        Expr::FieldAccess { target, field, source_line, col } => {
            // Optimize the target object expression
            Expr::FieldAccess {
                target: Box::new(opt_expr(target)),
                field: field.clone(),
                source_line: *source_line,
                col: *col,
            }
        }
        Expr::Not(inner) => {
            let ni = opt_expr(inner);
            if let Expr::Number(v) = ni {
                Expr::Number(if trunc16(v) == 0 { 1 } else { 0 })
            } else {
                Expr::Not(Box::new(ni))
            }
        }
    Expr::Call(ci) => Expr::Call(CallInfo { name: ci.name.clone(), source_line: ci.source_line, col: ci.col, args: ci.args.iter().map(opt_expr).collect() }),
    Expr::MethodCall(mc) => Expr::MethodCall(MethodCallInfo { 
        target: Box::new(opt_expr(&mc.target)), 
        method_name: mc.method_name.clone(), 
        source_line: mc.source_line, 
        col: mc.col, 
        args: mc.args.iter().map(opt_expr).collect() 
    }),
    Expr::Ident(i) => Expr::Ident(i.clone()),
    Expr::Number(n) => Expr::Number(trunc16(*n)),
    Expr::StringLit(s) => Expr::StringLit(s.clone()),
    }
}

// dead_code_elim: prune unreachable branches / empty loops.
fn dead_code_elim(m: &Module) -> Module {
    Module { 
        items: m.items.iter().map(|it| match it { 
            Item::Function(f) => Item::Function(dce_function(f)), 
            Item::Const { name, value } => Item::Const { name: name.clone(), value: value.clone() }, 
            Item::GlobalLet { name, value } => Item::GlobalLet { name: name.clone(), value: value.clone() }, 
            Item::VectorList { name, entries } => Item::VectorList { name: name.clone(), entries: entries.clone() },
            Item::ExprStatement(expr) => Item::ExprStatement(expr.clone()),
            Item::Export(e) => Item::Export(e.clone()),
            Item::StructDef(s) => Item::StructDef(s.clone()),
        }).collect(), 
        meta: m.meta.clone(),
        imports: m.imports.clone()
    }
}

fn dce_function(f: &Function) -> Function {
    let mut new_body = Vec::new();
    let mut terminated = false;
    for stmt in &f.body {
        if terminated { break; }
        dce_stmt(stmt, &mut new_body, &mut terminated);
    }
    Function { name: f.name.clone(), line: f.line, params: f.params.clone(), body: new_body }
}

fn dce_stmt(stmt: &Stmt, out: &mut Vec<Stmt>, terminated: &mut bool) {
    let source_line = stmt.source_line();  // Capture before match
    match stmt {
        Stmt::If { cond, body, elifs, else_body, .. } => match cond {
            Expr::Number(n) => {
                if *n != 0 {
                    for s in body { dce_stmt(s, out, terminated); if *terminated { return; } }
                } else {
                    let mut taken = false;
                    for (ec, eb) in elifs {
                        if let Expr::Number(v) = ec {
                            if *v != 0 {
                                for s in eb { dce_stmt(s, out, terminated); }
                                taken = true;
                                break;
                            }
                        }
                    }
                    if !taken {
                        if let Some(eb) = else_body {
                            for s in eb { dce_stmt(s, out, terminated); if *terminated { return; } }
                        }
                    }
                }
            }
            _ => {
                let mut nb = Vec::new();
                for s in body { dce_stmt(s, &mut nb, terminated); }
                let mut nelifs = Vec::new();
                for (ec, eb) in elifs {
                    let mut nb2 = Vec::new();
                    for s in eb { dce_stmt(s, &mut nb2, terminated); }
                    nelifs.push((ec.clone(), nb2));
                }
                let nelse = else_body.as_ref().map(|v| {
                    let mut vv = Vec::new();
                    for s in v { dce_stmt(s, &mut vv, terminated); }
                    vv
                });
                out.push(Stmt::If { cond: cond.clone(), body: nb, elifs: nelifs, else_body: nelse , source_line: source_line });
            }
        },
        Stmt::While { cond, body, .. } => {
            if let Expr::Number(0) = cond { return; }
            let mut nb = Vec::new();
            for s in body { dce_stmt(s, &mut nb, terminated); }
            out.push(Stmt::While { cond: cond.clone(), body: nb , source_line: source_line });
        }
        Stmt::For { var, start, end, step, body, .. } => {
            if let (Expr::Number(sv), Expr::Number(ev)) = (start, end) { if sv >= ev { return; } }
            let mut nb = Vec::new();
            for s in body { dce_stmt(s, &mut nb, terminated); }
            out.push(Stmt::For { var: var.clone(), start: start.clone(), end: end.clone(), step: step.clone(), body: nb , source_line: source_line });
        }
        Stmt::ForIn { var, iterable, body, .. } => {
            let mut nb = Vec::new();
            for s in body { dce_stmt(s, &mut nb, terminated); }
            out.push(Stmt::ForIn { var: var.clone(), iterable: iterable.clone(), body: nb , source_line: source_line });
        }
        Stmt::Switch { expr, cases, default, .. } => {
            // Keep all arms; could prune unreachable constant-match arms later
            let mut new_cases = Vec::new();
            for (ce, cb) in cases {
                let mut nb = Vec::new();
                for s in cb { dce_stmt(s, &mut nb, terminated); }
                new_cases.push((ce.clone(), nb));
            }
            let new_default = if let Some(db) = default {
                let mut nb = Vec::new();
                for s in db { dce_stmt(s, &mut nb, terminated); }
                Some(nb)
            } else { None };
            out.push(Stmt::Switch { expr: expr.clone(), cases: new_cases, default: new_default , source_line: source_line });
        }
        Stmt::Return(e, _) => { out.push(Stmt::Return(e.clone(), source_line)); *terminated = true; }
        Stmt::Assign { target, value, .. } => out.push(Stmt::Assign { target: target.clone(), value: value.clone() , source_line: source_line }),
        Stmt::Let { name, value, .. } => out.push(Stmt::Let { name: name.clone(), value: value.clone() , source_line: source_line }),
        Stmt::CompoundAssign { .. } => panic!("CompoundAssign should have been transformed to Assign by opt_stmt"),
        Stmt::Expr(e, _) => out.push(Stmt::Expr(e.clone(), source_line)),
        Stmt::Break { .. } | Stmt::Continue { .. } | Stmt::Pass { .. } => out.push(stmt.clone()),
    }
}

#[allow(dead_code)]
fn dse_function(f: &Function) -> Function {
    use std::collections::HashSet;
    let mut used: HashSet<String> = HashSet::new();
    let mut new_body: Vec<Stmt> = Vec::new();
    for stmt in f.body.iter().rev() {
        match stmt {
            Stmt::Assign { target, value, .. } => {
                let should_keep = match target {
                    crate::ast::AssignTarget::Ident { name, .. } => {
                        used.contains(name) || expr_has_call(value) || expr_contains_string_lit(value)
                    }
                    crate::ast::AssignTarget::Index { .. } => {
                        // Array assignments always kept (side effects)
                        true
                    }
                    crate::ast::AssignTarget::FieldAccess { .. } => {
                        // Field assignments always kept (side effects)
                        true
                    }
                };
                
                if should_keep {
                    collect_reads_expr(value, &mut used);
                    if let crate::ast::AssignTarget::Ident { name, .. } = target {
                        used.insert(name.clone());
                    }
                    new_body.push(stmt.clone());
                }
            }
            Stmt::Let { name, value, .. } => {
                if !used.contains(name) && !expr_has_call(value) && !expr_contains_string_lit(value) {
                } else {
                    collect_reads_expr(value, &mut used);
                    used.insert(name.clone());
                    new_body.push(stmt.clone());
                }
            }
            Stmt::Expr(e, _) => { collect_reads_expr(e, &mut used); new_body.push(stmt.clone()); }
            Stmt::Return(o, _) => { if let Some(e) = o { collect_reads_expr(e, &mut used); } new_body.push(stmt.clone()); }
            Stmt::If { cond, body, elifs, else_body, .. } => {
                collect_reads_expr(cond, &mut used);
                
                // For IF statements, we need to be conservative about dead store elimination
                // because variables assigned inside the IF might be used outside the IF
                // Simply collect all reads from all branches without optimization
                for s in body { collect_reads_stmt(s, &mut used); }
                for (ec, eb) in elifs { 
                    collect_reads_expr(ec, &mut used); 
                    for s in eb { collect_reads_stmt(s, &mut used); } 
                }
                if let Some(eb) = else_body { 
                    for s in eb { collect_reads_stmt(s, &mut used); } 
                }
                new_body.push(stmt.clone());
            }
            Stmt::While { cond, body, .. } => { collect_reads_expr(cond, &mut used); for s in body { collect_reads_stmt(s, &mut used); } new_body.push(stmt.clone()); }
            Stmt::For { var, start, end, step, body, .. } => {
                collect_reads_expr(start, &mut used);
                collect_reads_expr(end, &mut used);
                if let Some(se) = step { collect_reads_expr(se, &mut used); }
                for s in body { collect_reads_stmt(s, &mut used); }
                used.insert(var.clone());
                new_body.push(stmt.clone());
            }
            Stmt::ForIn { var, iterable, body, .. } => {
                collect_reads_expr(iterable, &mut used);
                for s in body { collect_reads_stmt(s, &mut used); }
                used.insert(var.clone());
                new_body.push(stmt.clone());
            }
            Stmt::Switch { expr, cases, default, .. } => {
                collect_reads_expr(expr, &mut used);
                for (ce, cb) in cases { collect_reads_expr(ce, &mut used); for s in cb { collect_reads_stmt(s, &mut used); } }
                if let Some(db) = default { for s in db { collect_reads_stmt(s, &mut used); } }
                new_body.push(stmt.clone());
            }
            Stmt::CompoundAssign { .. } => panic!("CompoundAssign should have been transformed to Assign by opt_stmt"),
            Stmt::Break { .. } | Stmt::Continue { .. } | Stmt::Pass { .. } => new_body.push(stmt.clone()),
        }
    }
    new_body.reverse();
    Function { name: f.name.clone(), line: f.line, params: f.params.clone(), body: new_body }
}

fn expr_has_call(e: &Expr) -> bool {
    match e {
    Expr::Call(_) => true,
    Expr::Binary { left, right, .. } | Expr::Compare { left, right, .. } | Expr::Logic { left, right, .. } => expr_has_call(left) || expr_has_call(right),
    Expr::Not(inner) | Expr::BitNot(inner) => expr_has_call(inner),
        Expr::List(elements) => elements.iter().any(expr_has_call),
        Expr::Index { target, index } => expr_has_call(target) || expr_has_call(index),
        _ => false,
    }
}

// expr_contains_string_lit: returns true if expression tree contains any string literal
fn expr_contains_string_lit(e: &Expr) -> bool {
    match e {
        Expr::StringLit(_) => true,
        Expr::Binary { left, right, .. }
        | Expr::Compare { left, right, .. }
        | Expr::Logic { left, right, .. } => expr_contains_string_lit(left) || expr_contains_string_lit(right),
    Expr::Call(ci) => ci.args.iter().any(expr_contains_string_lit),
        Expr::Not(inner) | Expr::BitNot(inner) => expr_contains_string_lit(inner),
        Expr::List(elements) => elements.iter().any(expr_contains_string_lit),
        Expr::Index { target, index } => expr_contains_string_lit(target) || expr_contains_string_lit(index),
        _ => false,
    }
}

fn collect_reads_stmt(s: &Stmt, used: &mut std::collections::HashSet<String>) {
    match s {
    Stmt::Assign { value, .. } => collect_reads_expr(value, used),
    Stmt::Let { value, .. } => collect_reads_expr(value, used),
        Stmt::Expr(e, _) => collect_reads_expr(e, used),
        Stmt::Return(o, _) => { if let Some(e) = o { collect_reads_expr(e, used); } }
        Stmt::If { cond, body, elifs, else_body, .. } => {
            collect_reads_expr(cond, used);
            for st in body { collect_reads_stmt(st, used); }
            for (ec, eb) in elifs { collect_reads_expr(ec, used); for st in eb { collect_reads_stmt(st, used); } }
            if let Some(eb) = else_body { for st in eb { collect_reads_stmt(st, used); } }
        }
        Stmt::While { cond, body, .. } => { collect_reads_expr(cond, used); for st in body { collect_reads_stmt(st, used); } }
        Stmt::For { start, end, step, body, .. } => {
            collect_reads_expr(start, used);
            collect_reads_expr(end, used);
            if let Some(se) = step { collect_reads_expr(se, used); }
            for st in body { collect_reads_stmt(st, used); }
        }
        Stmt::ForIn { iterable, body, .. } => {
            collect_reads_expr(iterable, used);
            for st in body { collect_reads_stmt(st, used); }
        }
        Stmt::Switch { expr, cases, default, .. } => {
            collect_reads_expr(expr, used);
            for (ce, cb) in cases { collect_reads_expr(ce, used); for st in cb { collect_reads_stmt(st, used); } }
            if let Some(db) = default { for st in db { collect_reads_stmt(st, used); } }
        }
        Stmt::CompoundAssign { .. } => panic!("CompoundAssign should have been transformed to Assign by opt_stmt"),
        Stmt::Break { .. } | Stmt::Continue { .. } | Stmt::Pass { .. } => {}
    }
}

fn collect_reads_expr(e: &Expr, used: &mut std::collections::HashSet<String>) {
    match e {
        Expr::Ident(n) => {
            used.insert(n.name.clone());
        }
        Expr::Call(ci) => { for a in &ci.args { collect_reads_expr(a, used); } }
        Expr::MethodCall(mc) => { 
            collect_reads_expr(&mc.target, used);
            for a in &mc.args { collect_reads_expr(a, used); } 
        }
        Expr::Binary { left, right, .. }
        | Expr::Compare { left, right, .. }
        | Expr::Logic { left, right, .. } => {
            collect_reads_expr(left, used);
            collect_reads_expr(right, used);
        }
    Expr::Not(inner) | Expr::BitNot(inner) => collect_reads_expr(inner, used),
        Expr::List(elements) => {
            for elem in elements {
                collect_reads_expr(elem, used);
            }
        }
        Expr::Index { target, index } => {
            collect_reads_expr(target, used);
            collect_reads_expr(index, used);
        }
        Expr::StructInit { .. } => {
            // Struct initialization doesn't read variables
        }
        Expr::FieldAccess { target, .. } => {
            // Field access reads the target object
            collect_reads_expr(target, used);
        }
        Expr::Number(_) => {}
    Expr::StringLit(_) => {}
    }
}

// propagate_constants: simple forward constant propagation with branch merging.
#[allow(dead_code)]
fn propagate_constants(m: &Module) -> Module {
    use std::collections::HashMap;
    let mut globals: HashMap<String, i32> = HashMap::new();
    // Collect global const numeric values (only if literal number after folding)
    for it in &m.items {
        if let Item::Const { name, value: Expr::Number(n) } = it {
            globals.insert(name.clone(), *n);
        }
    }
    Module { 
        items: m.items.iter().map(|it| match it { 
            Item::Function(f) => Item::Function(cp_function_with_globals(f, &globals)), 
            Item::Const { name, value } => Item::Const { name: name.clone(), value: value.clone() }, 
            Item::GlobalLet { name, value } => Item::GlobalLet { name: name.clone(), value: value.clone() }, 
            Item::VectorList { name, entries } => Item::VectorList { name: name.clone(), entries: entries.clone() },
            Item::ExprStatement(expr) => Item::ExprStatement(expr.clone()),
            Item::Export(e) => Item::Export(e.clone()),
            Item::StructDef(s) => Item::StructDef(s.clone()),
        }).collect(), 
        meta: m.meta.clone(),
        imports: m.imports.clone()
    }
}

#[allow(dead_code)]
fn cp_function_with_globals(f: &Function, globals: &std::collections::HashMap<String, i32>) -> Function {
    let mut env = HashMap::<String, i32>::new();
    // preload globals (function-locals can shadow by inserting new value later)
    for (k,v) in globals { env.insert(k.clone(), *v); }
    let mut new_body = Vec::new();
    for stmt in &f.body { new_body.push(cp_stmt(stmt, &mut env)); }
    Function { name: f.name.clone(), line: f.line, params: f.params.clone(), body: new_body }
}

#[allow(dead_code)]
fn cp_stmt(stmt: &Stmt, env: &mut HashMap<String, i32>) -> Stmt {
    let source_line = stmt.source_line(); // Capture before match
    match stmt {
        Stmt::Assign { target, value, .. } => {
            let v2 = cp_expr(value, env);
            match target {
                crate::ast::AssignTarget::Ident { name, .. } => {
                    if let Expr::Number(n) = v2 {
                        env.insert(name.clone(), n);
                        Stmt::Assign { target: target.clone(), value: Expr::Number(n), source_line }
                    } else {
                        env.remove(name);
                        Stmt::Assign { target: target.clone(), value: v2, source_line }
                    }
                }
                crate::ast::AssignTarget::Index { .. } => {
                    // Array indexing - can't propagate constants through
                    Stmt::Assign { target: target.clone(), value: v2, source_line }
                }
                crate::ast::AssignTarget::FieldAccess { .. } => {
                    // Field access - can't propagate constants through (Phase 3)
                    Stmt::Assign { target: target.clone(), value: v2, source_line }
                }
            }
        }
        Stmt::Let { name, value, .. } => {
            let v2 = cp_expr(value, env);
            if let Expr::Number(n) = v2 {
                env.insert(name.clone(), n);
                Stmt::Let { name: name.clone(), value: Expr::Number(n), source_line }
            } else {
                env.remove(name);
                Stmt::Let { name: name.clone(), value: v2, source_line }
            }
        }
        Stmt::Expr(e, _) => Stmt::Expr(cp_expr(e, env), source_line),
        Stmt::Return(o, _) => Stmt::Return(o.as_ref().map(|e| cp_expr(e, env)), source_line),
        Stmt::Break { .. } => Stmt::Break { source_line },
        Stmt::Continue { .. } => Stmt::Continue { source_line },
        Stmt::Pass { .. } => Stmt::Pass { source_line },
        Stmt::While { cond, body, .. } => {
            let c = cp_expr(cond, env);
            let saved = env.clone();
            let mut nb = Vec::new();
            for s in body { nb.push(cp_stmt(s, env)); }
            *env = saved;
            Stmt::While { cond: c, body: nb, source_line }
        }
        Stmt::For { var, start, end, step, body, .. } => {
            let s = cp_expr(start, env);
            let e = cp_expr(end, env);
            let st = step.as_ref().map(|x| cp_expr(x, env));
            let saved = env.clone();
            env.remove(var);
            let mut nb = Vec::new();
            for sstmt in body { nb.push(cp_stmt(sstmt, env)); }
            *env = saved;
            Stmt::For { var: var.clone(), start: s, end: e, step: st, body: nb, source_line }
        }
        Stmt::ForIn { var, iterable, body, .. } => {
            let it = cp_expr(iterable, env);
            let saved = env.clone();
            env.remove(var);
            let mut nb = Vec::new();
            for sstmt in body { nb.push(cp_stmt(sstmt, env)); }
            *env = saved;
            Stmt::ForIn { var: var.clone(), iterable: it, body: nb, source_line }
        }
        Stmt::If { cond, body, elifs, else_body, .. } => {
            let c = cp_expr(cond, env);
            let base_env = env.clone();
            let mut then_env = base_env.clone();
            let mut nb = Vec::new();
            for s in body { nb.push(cp_stmt(s, &mut then_env)); }
            let mut new_elifs = Vec::new();
            let mut branch_envs: Vec<HashMap<String, i32>> = vec![then_env.clone()];
            for (ec, eb) in elifs {
                let ec2 = cp_expr(ec, env);
                let mut eenv = base_env.clone();
                let mut eb_new = Vec::new();
                for s in eb { eb_new.push(cp_stmt(s, &mut eenv)); }
                branch_envs.push(eenv.clone());
                new_elifs.push((ec2, eb_new));
            }
            let new_else = if let Some(eb) = else_body {
                let mut eenv = base_env.clone();
                let mut eb_new = Vec::new();
                for s in eb { eb_new.push(cp_stmt(s, &mut eenv)); }
                branch_envs.push(eenv.clone());
                Some(eb_new)
            } else {
                None
            };
            if !branch_envs.is_empty() {
                let first = branch_envs[0].clone();
                let mut merged = HashMap::new();
                'outer: for (k, v) in first {
                    for be in &branch_envs[1..] {
                        if be.get(&k) != Some(&v) { continue 'outer; }
                    }
                    merged.insert(k, v);
                }
                *env = merged;
            }
            Stmt::If { cond: c, body: nb, elifs: new_elifs, else_body: new_else, source_line }
        }
        Stmt::Switch { expr, cases, default, .. } => {
            let se = cp_expr(expr, env);
            let mut new_cases = Vec::new();
            for (ce, cb) in cases {
                let ce2 = cp_expr(ce, env);
                let saved = env.clone();
                let mut nb = Vec::new();
                for s in cb { nb.push(cp_stmt(s, env)); }
                *env = saved; // conservative merge
                new_cases.push((ce2, nb));
            }
            let new_default = if let Some(db) = default {
                let saved = env.clone();
                let mut nb = Vec::new();
                for s in db { nb.push(cp_stmt(s, env)); }
                *env = saved;
                Some(nb)
            } else { None };
            Stmt::Switch { expr: se, cases: new_cases, default: new_default, source_line }
        }
        Stmt::CompoundAssign { .. } => panic!("CompoundAssign should have been transformed to Assign by opt_stmt"),
    }
}

#[allow(dead_code)]
fn cp_expr(e: &Expr, env: &HashMap<String, i32>) -> Expr {
    match e {
    Expr::Ident(name) => env.get(&name.name).map(|v| Expr::Number(*v)).unwrap_or_else(|| Expr::Ident(name.clone())),
        Expr::Binary { op, left, right } => Expr::Binary { op: *op, left: Box::new(cp_expr(left, env)), right: Box::new(cp_expr(right, env)) },
        Expr::Compare { op, left, right } => Expr::Compare { op: *op, left: Box::new(cp_expr(left, env)), right: Box::new(cp_expr(right, env)) },
        Expr::Logic { op, left, right } => Expr::Logic { op: *op, left: Box::new(cp_expr(left, env)), right: Box::new(cp_expr(right, env)) },
    Expr::Not(inner) => Expr::Not(Box::new(cp_expr(inner, env))),
    Expr::BitNot(inner) => Expr::BitNot(Box::new(cp_expr(inner, env))),
        Expr::List(elements) => Expr::List(elements.iter().map(|e| cp_expr(e, env)).collect()),
        Expr::Index { target, index } => Expr::Index { 
            target: Box::new(cp_expr(target, env)), 
            index: Box::new(cp_expr(index, env)) 
        },
    Expr::Call(ci) => Expr::Call(CallInfo { name: ci.name.clone(), source_line: ci.source_line, col: ci.col, args: ci.args.iter().map(|a| cp_expr(a, env)).collect() }),
    Expr::MethodCall(mc) => Expr::MethodCall(MethodCallInfo {
        target: Box::new(cp_expr(&mc.target, env)),
        method_name: mc.method_name.clone(),
        args: mc.args.iter().map(|a| cp_expr(a, env)).collect(),
        source_line: mc.source_line,
        col: mc.col,
    }),
        Expr::Number(n) => Expr::Number(*n),
    Expr::StringLit(s) => Expr::StringLit(s.clone()),
    Expr::StructInit { .. } => e.clone(), // Phase 3 - no constant propagation
    Expr::FieldAccess { target, field, source_line, col } => Expr::FieldAccess { 
        target: Box::new(cp_expr(target, env)), 
        field: field.clone(), 
        source_line: *source_line, 
        col: *col 
    },
    }
}

// fold_const_switches: if a switch expression is a constant number and all case values are constant numbers,
// select the matching case (or default) and inline its body, removing the switch. Conservatively keeps semantics.
fn fold_const_switches(m: &Module) -> Module {
    Module { 
        items: m.items.iter().map(|it| match it { 
            Item::Function(f) => Item::Function(fold_const_switches_function(f)), 
            Item::Const { name, value } => Item::Const { name: name.clone(), value: value.clone() }, 
            Item::GlobalLet { name, value } => Item::GlobalLet { name: name.clone(), value: value.clone() }, 
            Item::VectorList { name, entries } => Item::VectorList { name: name.clone(), entries: entries.clone() },
            Item::ExprStatement(expr) => Item::ExprStatement(expr.clone()),
            Item::Export(e) => Item::Export(e.clone()),
            Item::StructDef(s) => Item::StructDef(s.clone()),
        }).collect(), 
        meta: m.meta.clone(),
        imports: m.imports.clone()
    }
}

fn fold_const_switches_function(f: &Function) -> Function {
    let mut out = Vec::new();
    for s in &f.body { fold_const_switch_stmt(s, &mut out); }
    Function { name: f.name.clone(), line: f.line, params: f.params.clone(), body: out }
}

fn fold_const_switch_stmt(s: &Stmt, out: &mut Vec<Stmt>) {
    let source_line = s.source_line(); // Capture before match
    match s {
        Stmt::Switch { expr, cases, default, .. } => {
            if let Expr::Number(v) = expr {
                let mut all_numeric = true;
                for (ce, _) in cases { if !matches!(ce, Expr::Number(_)) { all_numeric = false; break; } }
                if all_numeric {
                    let mut matched: Option<&Vec<Stmt>> = None;
                    for (ce, body) in cases {
                        if let Expr::Number(cv) = ce { if (cv & 0xFFFF) == (v & 0xFFFF) { matched = Some(body); break; } }
                    }
                    let chosen: &Vec<Stmt> = if let Some(b) = matched { b } else if let Some(db) = default { db } else { &Vec::new() };
                    for cs in chosen { fold_const_switch_stmt(cs, out); }
                    return;
                }
            }
            // Recurse normally if not folded
            let mut new_cases = Vec::new();
            for (ce, cb) in cases {
                let mut nb = Vec::new();
                for cs in cb { fold_const_switch_stmt(cs, &mut nb); }
                new_cases.push((ce.clone(), nb));
            }
            let new_default = if let Some(db) = default {
                let mut nb = Vec::new();
                for cs in db { fold_const_switch_stmt(cs, &mut nb); }
                Some(nb)
            } else { None };
            out.push(Stmt::Switch { expr: expr.clone(), cases: new_cases, default: new_default , source_line: source_line });
        }
        Stmt::If { cond, body, elifs, else_body, .. } => {
            let mut nb = Vec::new(); for cs in body { fold_const_switch_stmt(cs, &mut nb); }
            let mut nelifs = Vec::new(); for (ec, eb) in elifs { let mut nb2 = Vec::new(); for cs in eb { fold_const_switch_stmt(cs, &mut nb2); } nelifs.push((ec.clone(), nb2)); }
            let nelse = if let Some(eb) = else_body { let mut nb3 = Vec::new(); for cs in eb { fold_const_switch_stmt(cs, &mut nb3); } Some(nb3) } else { None };
            out.push(Stmt::If { cond: cond.clone(), body: nb, elifs: nelifs, else_body: nelse , source_line: source_line });
        }
        Stmt::While { cond, body, .. } => { let mut nb = Vec::new(); for cs in body { fold_const_switch_stmt(cs, &mut nb); } out.push(Stmt::While { cond: cond.clone(), body: nb , source_line: source_line }); }
        Stmt::For { var, start, end, step, body, .. } => { let mut nb = Vec::new(); for cs in body { fold_const_switch_stmt(cs, &mut nb); } out.push(Stmt::For { var: var.clone(), start: start.clone(), end: end.clone(), step: step.clone(), body: nb , source_line: source_line }); }
        Stmt::ForIn { var, iterable, body, .. } => { let mut nb = Vec::new(); for cs in body { fold_const_switch_stmt(cs, &mut nb); } out.push(Stmt::ForIn { var: var.clone(), iterable: iterable.clone(), body: nb , source_line: source_line }); }
    Stmt::Assign { target, value, .. } => out.push(Stmt::Assign { target: target.clone(), value: value.clone() , source_line: source_line }),
        Stmt::Let { name, value, .. } => out.push(Stmt::Let { name: name.clone(), value: value.clone() , source_line: source_line }),
        Stmt::Expr(e, _) => out.push(Stmt::Expr(e.clone(), source_line)),
        Stmt::Return(o, _) => out.push(Stmt::Return(o.clone(), source_line)),
        Stmt::Break { .. } => out.push(Stmt::Break { source_line }),
        Stmt::Continue { .. } => out.push(Stmt::Continue { source_line }),
        Stmt::Pass { .. } => out.push(Stmt::Pass { source_line }),
        Stmt::CompoundAssign { .. } => panic!("CompoundAssign should be transformed away before fold_const_switch_stmt"),
    }
}
