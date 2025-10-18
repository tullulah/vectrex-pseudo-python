use crate::ast::*;
use crate::target::{info, CpuArch, Target};
use std::collections::{HashSet, HashMap};
use std::cell::RefCell;

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
        ("DEBUG_PRINT_LABELED", 2),  // label, value     // Nueva función para debug output
    
    // Funciones específicas de vectorlist
    ("DRAW_VL", 2),
    ("FRAME_BEGIN", 1),
    ("VECTOR_PHASE_BEGIN", 0),
    ("WAIT_RECAL", 0),
    ("PLAY_MUSIC1", 0),
    ("DBG_STATIC_VL", 0),
    
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
    pub use crate::backend::arm::emit as emit_arm;
    pub use crate::backend::cortexm::emit as emit_cortexm;
    pub use crate::backend::m6809::emit as emit_6809;
    pub use crate::backend::m6809::emit_with_debug as emit_6809_with_debug;
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
    
    // Paso 1: validación semántica básica (variables / aridad) recolectando warnings.
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    validate_semantics(module, &mut diagnostics);
    let has_errors = diagnostics.iter().any(|d| matches!(d.severity, DiagnosticSeverity::Error));
    if has_errors {
        return (String::new(), None, diagnostics);
    }
    
    // Paso 2: pipeline de optimización (dead_store_elim preserva asignaciones con literales string).
    let optimized = optimize_module(module);
    let ti = info(target);
    
    // If source defines CONST TITLE = "..." let it override CLI title.
    let mut effective = CodegenOptions { ..opts.clone() };
    if let Some(t) = optimized.meta.title_override.clone() { effective.title = t; }
    
    // Generate ASM and debug info
    let (asm, debug_info) = match ti.arch {
        CpuArch::M6809 => {
            let (asm, dbg) = backends_ref::emit_6809_with_debug(&optimized, target, &ti, &effective);
            (asm, Some(dbg))
        },
        CpuArch::Arm => (backends_ref::emit_arm(&optimized, target, &ti, opts), None),
        CpuArch::CortexM => (backends_ref::emit_cortexm(&optimized, target, &ti, opts), None),
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
        CpuArch::Arm => backends_ref::emit_arm(&optimized, target, &ti, opts),
        CpuArch::CortexM => backends_ref::emit_cortexm(&optimized, target, &ti, opts),
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
        let folded: Module = Module { items: current.items.iter().map(opt_item).collect(), meta: current.meta.clone() };
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
        }
    }
    // Validar cada función independientemente.
    for it in &module.items {
        if let Item::Function(func) = it {
            TL_ACCUM.with(|acc| acc.borrow_mut().clear());
            validate_function(func, &globals, diagnostics);
            // Mover errores recolectados (uso/assign/arity) del thread-local
            TL_ACCUM.with(|acc| diagnostics.extend(acc.borrow().iter().cloned()));
        }
    }
}

fn validate_function(f: &Function, globals: &HashSet<String>, diagnostics: &mut Vec<Diagnostic>) {
    // ámbito inicial: globals + params
    let mut scope: Vec<HashSet<String>> = Vec::new();
    scope.push(globals.clone());
    let mut param_set: HashSet<String> = HashSet::new();
    for p in &f.params { param_set.insert(p.clone()); }
    scope.push(param_set);
    // tracking de lecturas para warning de variables no usadas
    let mut reads: HashSet<String> = HashSet::new();
    for stmt in &f.body { validate_stmt_collect(stmt, &mut scope, &mut reads); }
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
fn validate_stmt(stmt: &Stmt, scope: &mut Vec<HashSet<String>>) { validate_stmt_collect(stmt, scope, &mut HashSet::new()); }

fn validate_stmt_collect(stmt: &Stmt, scope: &mut Vec<HashSet<String>>, reads: &mut HashSet<String>) {
    match stmt {
        Stmt::Let { name, value, .. } => { validate_expr_collect(value, scope, reads); declare(name, scope); }
        Stmt::Assign { target, value, .. } => {
            if !is_declared(&target.name, scope) {
                TL_ACCUM.with(|acc| acc.borrow_mut().push(Diagnostic { severity: DiagnosticSeverity::Error, code: DiagnosticCode::UndeclaredAssign, message: format!("SemanticsError: asignación a variable no declarada '{}'. Declárala con 'let {} = ...' antes de usarla.", target.name, target.name), line: Some(target.line), col: Some(target.col) }));
            }
            validate_expr_collect(value, scope, reads);
        }
        Stmt::CompoundAssign { target, value, .. } => {
            // Similar a Assign, pero también leemos la variable del lado izquierdo
            if !is_declared(&target.name, scope) {
                TL_ACCUM.with(|acc| acc.borrow_mut().push(Diagnostic { severity: DiagnosticSeverity::Error, code: DiagnosticCode::UndeclaredAssign, message: format!("SemanticsError: asignación compuesta a variable no declarada '{}'. Declárala con 'let {} = ...' antes de usarla.", target.name, target.name), line: Some(target.line), col: Some(target.col) }));
            }
            reads.insert(target.name.clone()); // Leemos la variable para x += expr
            validate_expr_collect(value, scope, reads);
        }
        Stmt::For { var, start, end, step, body, .. } => {
            validate_expr_collect(start, scope, reads); validate_expr_collect(end, scope, reads); if let Some(se) = step { validate_expr_collect(se, scope, reads); }
            push_scope(scope); // cuerpo loop con var declarada
            declare(var, scope);
            for s in body { validate_stmt_collect(s, scope, reads); }
            pop_scope(scope);
        }
        Stmt::While { cond, body, .. } => {
            validate_expr_collect(cond, scope, reads);
            push_scope(scope);
            for s in body { validate_stmt_collect(s, scope, reads); }
            pop_scope(scope);
        }
        Stmt::If { cond, body, elifs, else_body, .. } => {
            validate_expr_collect(cond, scope, reads);
            push_scope(scope); for s in body { validate_stmt_collect(s, scope, reads); } pop_scope(scope);
            for (ec, eb) in elifs { validate_expr_collect(ec, scope, reads); push_scope(scope); for s in eb { validate_stmt_collect(s, scope, reads); } pop_scope(scope); }
            if let Some(eb) = else_body { push_scope(scope); for s in eb { validate_stmt_collect(s, scope, reads); } pop_scope(scope); }
        }
        Stmt::Switch { expr, cases, default, .. } => {
            validate_expr_collect(expr, scope, reads);
            for (ce, cb) in cases { validate_expr_collect(ce, scope, reads); push_scope(scope); for s in cb { validate_stmt_collect(s, scope, reads); } pop_scope(scope); }
            if let Some(db) = default { push_scope(scope); for s in db { validate_stmt_collect(s, scope, reads); } pop_scope(scope); }
        }
        Stmt::Expr(e, _) => validate_expr_collect(e, scope, reads),
        Stmt::Return(o, _) => { if let Some(e) = o { validate_expr_collect(e, scope, reads); } }
        Stmt::Break { .. } | Stmt::Continue { .. } => {}
    }
}

#[allow(dead_code)]
fn validate_expr(e: &Expr, scope: &mut Vec<HashSet<String>>) { let mut dummy=HashSet::new(); validate_expr_collect(e, scope, &mut dummy); }

fn validate_expr_collect(e: &Expr, scope: &mut Vec<HashSet<String>>, reads: &mut HashSet<String>) {
    match e {
        Expr::Ident(info) => {
            if !is_declared(&info.name, scope) {
                TL_ACCUM.with(|acc| acc.borrow_mut().push(Diagnostic { severity: DiagnosticSeverity::Error, code: DiagnosticCode::UndeclaredVar, message: format!("SemanticsError: uso de variable no declarada '{}'.", info.name), line: Some(info.line), col: Some(info.col) }));
            } else { reads.insert(info.name.clone()); }
        }
        Expr::Call(ci) => {
            if let Some(exp) = expected_builtin_arity(&ci.name) {
                if ci.args.len() != exp {
                    TL_ACCUM.with(|acc| acc.borrow_mut().push(Diagnostic { severity: DiagnosticSeverity::Error, code: DiagnosticCode::ArityMismatch, message: format!("SemanticsErrorArity: llamada a '{}' con {} argumentos; se esperaban {}.", ci.name, ci.args.len(), exp), line: Some(ci.line), col: Some(ci.col) }));
                }
            }
            for a in &ci.args { validate_expr_collect(a, scope, reads); }
        }
        Expr::Binary { left, right, .. } | Expr::Compare { left, right, .. } | Expr::Logic { left, right, .. } => {
            validate_expr_collect(left, scope, reads); validate_expr_collect(right, scope, reads);
        }
        Expr::Not(inner) | Expr::BitNot(inner) => validate_expr_collect(inner, scope, reads),
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
    } 
}

fn opt_function(f: &Function) -> Function {
    Function {
        name: f.name.clone(),
        params: f.params.clone(),
        body: f.body.iter().map(opt_stmt).collect(),
    }
}

fn opt_stmt(s: &Stmt) -> Stmt {
    let line = s.line(); // Preserve original line number
    match s {
    Stmt::Assign { target, value, .. } => Stmt::Assign { target: target.clone(), value: opt_expr(value), line },
    Stmt::Let { name, value, .. } => Stmt::Let { name: name.clone(), value: opt_expr(value), line },
        Stmt::CompoundAssign { target, op, value, .. } => {
            // Transformar x += expr en x = x + expr
            let var_expr = Expr::Ident(IdentInfo { 
                name: target.name.clone(), 
                line: target.line, 
                col: target.col 
            });
            let combined_expr = Expr::Binary { 
                op: *op, 
                left: Box::new(var_expr), 
                right: Box::new(opt_expr(value)) 
            };
            Stmt::Assign { target: target.clone(), value: combined_expr, line }
        },
        Stmt::For { var, start, end, step, body, .. } => Stmt::For {
            var: var.clone(),
            start: opt_expr(start),
            end: opt_expr(end),
            step: step.as_ref().map(opt_expr),
            body: body.iter().map(opt_stmt).collect(),
            line,
        },
        Stmt::While { cond, body, .. } => Stmt::While { cond: opt_expr(cond), body: body.iter().map(opt_stmt).collect(), line },
        Stmt::Expr(e, _) => Stmt::Expr(opt_expr(e), line),
        Stmt::If { cond, body, elifs, else_body, .. } => Stmt::If {
            cond: opt_expr(cond),
            body: body.iter().map(opt_stmt).collect(),
            elifs: elifs.iter().map(|(c, b)| (opt_expr(c), b.iter().map(opt_stmt).collect())).collect(),
            else_body: else_body.as_ref().map(|v| v.iter().map(opt_stmt).collect()),
            line,
        },
        Stmt::Return(o, _) => Stmt::Return(o.as_ref().map(opt_expr), line),
    Stmt::Break { .. } => Stmt::Break { line },
    Stmt::Continue { .. } => Stmt::Continue { line },
    Stmt::Switch { expr, cases, default, .. } => Stmt::Switch { expr: opt_expr(expr), cases: cases.iter().map(|(e,b)| (opt_expr(e), b.iter().map(opt_stmt).collect())).collect(), default: default.as_ref().map(|v| v.iter().map(opt_stmt).collect()), line },
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
        Expr::Not(inner) => {
            let ni = opt_expr(inner);
            if let Expr::Number(v) = ni {
                Expr::Number(if trunc16(v) == 0 { 1 } else { 0 })
            } else {
                Expr::Not(Box::new(ni))
            }
        }
    Expr::Call(ci) => Expr::Call(CallInfo { name: ci.name.clone(), line: ci.line, col: ci.col, args: ci.args.iter().map(opt_expr).collect() }),
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
        }).collect(), 
        meta: m.meta.clone() 
    }
}

fn dce_function(f: &Function) -> Function {
    let mut new_body = Vec::new();
    let mut terminated = false;
    for stmt in &f.body {
        if terminated { break; }
        dce_stmt(stmt, &mut new_body, &mut terminated);
    }
    Function { name: f.name.clone(), params: f.params.clone(), body: new_body }
}

fn dce_stmt(stmt: &Stmt, out: &mut Vec<Stmt>, terminated: &mut bool) {
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
                out.push(Stmt::If { cond: cond.clone(), body: nb, elifs: nelifs, else_body: nelse , line: s.line() });
            }
        },
        Stmt::While { cond, body, .. } => {
            if let Expr::Number(0) = cond { return; }
            let mut nb = Vec::new();
            for s in body { dce_stmt(s, &mut nb, terminated); }
            out.push(Stmt::While { cond: cond.clone(), body: nb , line: s.line() });
        }
        Stmt::For { var, start, end, step, body, .. } => {
            if let (Expr::Number(sv), Expr::Number(ev)) = (start, end) { if sv >= ev { return; } }
            let mut nb = Vec::new();
            for s in body { dce_stmt(s, &mut nb, terminated); }
            out.push(Stmt::For { var: var.clone(), start: start.clone(), end: end.clone(), step: step.clone(), body: nb , line: s.line() });
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
            out.push(Stmt::Switch { expr: expr.clone(), cases: new_cases, default: new_default , line: s.line() });
        }
        Stmt::Return(e, _) => { out.push(Stmt::Return(e.clone(), line)); *terminated = true; }
        Stmt::Assign { target, value, .. } => out.push(Stmt::Assign { target: target.clone(), value: value.clone() , line: s.line() }),
        Stmt::Let { name, value, .. } => out.push(Stmt::Let { name: name.clone(), value: value.clone() , line: s.line() }),
        Stmt::CompoundAssign { .. } => panic!("CompoundAssign should have been transformed to Assign by opt_stmt"),
        Stmt::Expr(e, _) => out.push(Stmt::Expr(e.clone(), line)),
        Stmt::Break { .. } | Stmt::Continue { .. } => out.push(stmt.clone()),
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
                if !used.contains(&target.name) && !expr_has_call(value) && !expr_contains_string_lit(value) {
                } else {
                    collect_reads_expr(value, &mut used);
                    used.insert(target.name.clone());
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
            Stmt::Switch { expr, cases, default, .. } => {
                collect_reads_expr(expr, &mut used);
                for (ce, cb) in cases { collect_reads_expr(ce, &mut used); for s in cb { collect_reads_stmt(s, &mut used); } }
                if let Some(db) = default { for s in db { collect_reads_stmt(s, &mut used); } }
                new_body.push(stmt.clone());
            }
            Stmt::CompoundAssign { .. } => panic!("CompoundAssign should have been transformed to Assign by opt_stmt"),
            Stmt::Break { .. } | Stmt::Continue { .. } => new_body.push(stmt.clone()),
        }
    }
    new_body.reverse();
    Function { name: f.name.clone(), params: f.params.clone(), body: new_body }
}

fn expr_has_call(e: &Expr) -> bool {
    match e {
    Expr::Call(_) => true,
    Expr::Binary { left, right, .. } | Expr::Compare { left, right, .. } | Expr::Logic { left, right, .. } => expr_has_call(left) || expr_has_call(right),
    Expr::Not(inner) | Expr::BitNot(inner) => expr_has_call(inner),
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
        Stmt::Switch { expr, cases, default, .. } => {
            collect_reads_expr(expr, used);
            for (ce, cb) in cases { collect_reads_expr(ce, used); for st in cb { collect_reads_stmt(st, used); } }
            if let Some(db) = default { for st in db { collect_reads_stmt(st, used); } }
        }
        Stmt::CompoundAssign { .. } => panic!("CompoundAssign should have been transformed to Assign by opt_stmt"),
        Stmt::Break { .. } | Stmt::Continue { .. } => {}
    }
}

fn collect_reads_expr(e: &Expr, used: &mut std::collections::HashSet<String>) {
    match e {
        Expr::Ident(n) => {
            used.insert(n.name.clone());
        }
        Expr::Call(ci) => { for a in &ci.args { collect_reads_expr(a, used); } }
        Expr::Binary { left, right, .. }
        | Expr::Compare { left, right, .. }
        | Expr::Logic { left, right, .. } => {
            collect_reads_expr(left, used);
            collect_reads_expr(right, used);
        }
    Expr::Not(inner) | Expr::BitNot(inner) => collect_reads_expr(inner, used),
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
        }).collect(), 
        meta: m.meta.clone() 
    }
}

#[allow(dead_code)]
fn cp_function_with_globals(f: &Function, globals: &std::collections::HashMap<String, i32>) -> Function {
    let mut env = HashMap::<String, i32>::new();
    // preload globals (function-locals can shadow by inserting new value later)
    for (k,v) in globals { env.insert(k.clone(), *v); }
    let mut new_body = Vec::new();
    for stmt in &f.body { new_body.push(cp_stmt(stmt, &mut env)); }
    Function { name: f.name.clone(), params: f.params.clone(), body: new_body }
}

#[allow(dead_code)]
fn cp_stmt(stmt: &Stmt, env: &mut HashMap<String, i32>) -> Stmt {
    match stmt {
        Stmt::Assign { target, value, .. } => {
            let v2 = cp_expr(value, env);
            if let Expr::Number(n) = v2 {
                env.insert(target.name.clone(), n);
                Stmt::Assign { target: target.clone(), value: Expr::Number(n) }
            } else {
                env.remove(&target.name);
                Stmt::Assign { target: target.clone(), value: v2, line: s.line() }
            }
        }
        Stmt::Let { name, value, .. } => {
            let v2 = cp_expr(value, env);
            if let Expr::Number(n) = v2 {
                env.insert(name.clone(), n);
                Stmt::Let { name: name.clone(), value: Expr::Number(n) }
            } else {
                env.remove(name);
                Stmt::Let { name: name.clone(), value: v2, line: s.line() }
            }
        }
        Stmt::Expr(e, _) => Stmt::Expr(cp_expr(e, env), line),
        Stmt::Return(o, _) => Stmt::Return(o.as_ref(, _).map(|e| cp_expr(e, env))),
        Stmt::Break { .. } => Stmt::Break { .. },
        Stmt::Continue { .. } => Stmt::Continue { .. },
        Stmt::While { cond, body, .. } => {
            let c = cp_expr(cond, env);
            let saved = env.clone();
            let mut nb = Vec::new();
            for s in body { nb.push(cp_stmt(s, env)); }
            *env = saved;
            Stmt::While { cond: c, body: nb }
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
            Stmt::For { var: var.clone(), start: s, end: e, step: st, body: nb }
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
            Stmt::If { cond: c, body: nb, elifs: new_elifs, else_body: new_else }
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
            Stmt::Switch { expr: se, cases: new_cases, default: new_default }
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
    Expr::Call(ci) => Expr::Call(CallInfo { name: ci.name.clone(), line: ci.line, col: ci.col, args: ci.args.iter().map(|a| cp_expr(a, env)).collect() }),
        Expr::Number(n) => Expr::Number(*n),
    Expr::StringLit(s) => Expr::StringLit(s.clone()),
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
        }).collect(), 
        meta: m.meta.clone() 
    }
}

fn fold_const_switches_function(f: &Function) -> Function {
    let mut out = Vec::new();
    for s in &f.body { fold_const_switch_stmt(s, &mut out); }
    Function { name: f.name.clone(), params: f.params.clone(), body: out }
}

fn fold_const_switch_stmt(s: &Stmt, out: &mut Vec<Stmt>) {
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
            out.push(Stmt::Switch { expr: expr.clone(), cases: new_cases, default: new_default , line: s.line() });
        }
        Stmt::If { cond, body, elifs, else_body, .. } => {
            let mut nb = Vec::new(); for cs in body { fold_const_switch_stmt(cs, &mut nb); }
            let mut nelifs = Vec::new(); for (ec, eb) in elifs { let mut nb2 = Vec::new(); for cs in eb { fold_const_switch_stmt(cs, &mut nb2); } nelifs.push((ec.clone(), nb2)); }
            let nelse = if let Some(eb) = else_body { let mut nb3 = Vec::new(); for cs in eb { fold_const_switch_stmt(cs, &mut nb3); } Some(nb3) } else { None };
            out.push(Stmt::If { cond: cond.clone(), body: nb, elifs: nelifs, else_body: nelse , line: s.line() });
        }
        Stmt::While { cond, body, .. } => { let mut nb = Vec::new(); for cs in body { fold_const_switch_stmt(cs, &mut nb); } out.push(Stmt::While { cond: cond.clone(), body: nb , line: s.line() }); }
        Stmt::For { var, start, end, step, body, .. } => { let mut nb = Vec::new(); for cs in body { fold_const_switch_stmt(cs, &mut nb); } out.push(Stmt::For { var: var.clone(), start: start.clone(), end: end.clone(), step: step.clone(), body: nb , line: s.line() }); }
    Stmt::Assign { target, value, .. } => out.push(Stmt::Assign { target: target.clone(), value: value.clone() , line: s.line() }),
        Stmt::Let { name, value, .. } => out.push(Stmt::Let { name: name.clone(), value: value.clone() , line: s.line() }),
        Stmt::Expr(e, _) => out.push(Stmt::Expr(e.clone(), line)),
        Stmt::Return(o, _) => out.push(Stmt::Return(o.clone(), line)),
        Stmt::Break { .. } => out.push(Stmt::Break { .. }),
        Stmt::Continue { .. } => out.push(Stmt::Continue { .. }),
        Stmt::CompoundAssign { .. } => panic!("CompoundAssign should be transformed away before fold_const_switch_stmt"),
    }
}
