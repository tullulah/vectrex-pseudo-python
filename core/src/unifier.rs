//! AST Unifier for multi-file VPy projects.
//!
//! Takes multiple parsed modules and merges them into a single unified AST
//! with proper symbol resolution and namespace prefixing.

use std::collections::{HashMap, HashSet};
use std::path::Path;
use crate::ast::*;
use crate::resolver::ModuleResolver;
use anyhow::{bail, Result};

/// Context for symbol resolution during unification
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct SymbolTable {
    /// Global symbols: name -> (module_path, original_name)
    pub globals: HashMap<String, (String, String)>,
    /// Exported symbols per module: module_path -> set of exported names
    pub exports: HashMap<String, HashSet<String>>,
    /// Import aliases: local_name -> (module_path, original_name)
    pub aliases: HashMap<String, (String, String)>,
}

/// Configuration for AST unification
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UnifyOptions {
    /// Prefix module names to avoid collisions (e.g., `utils_math_clamp`)
    pub prefix_symbols: bool,
    /// Separator for prefixed names
    pub prefix_separator: String,
    /// Remove unused code (tree shaking)
    pub tree_shake: bool,
}

impl Default for UnifyOptions {
    fn default() -> Self {
        Self {
            prefix_symbols: true,
            prefix_separator: "_".to_string(),
            tree_shake: true, // Enabled by default now (2026-01-15)
        }
    }
}

/// Result of AST unification
#[allow(dead_code)]
#[derive(Debug)]
pub struct UnifiedModule {
    /// The merged module
    pub module: Module,
    /// Symbol table for debugging
    pub symbols: SymbolTable,
    /// Mapping of original (module, symbol) -> unified name
    pub name_map: HashMap<(String, String), String>,
}

/// Detects circular imports in module dependency graph
/// Returns error if a cycle is found (e.g., A→B→C→A)
fn detect_circular_imports(
    resolver: &ModuleResolver,
) -> Result<()> {
    let modules = resolver.get_all_modules();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    
    // Build dependency graph: module -> list of modules it imports
    for module in &modules {
        let module_id = module_id_from_path(&module.path);
        let mut deps = Vec::new();
        
        for import in &module.module.imports {
            let imported_id = import.module_path.join("_");
            if !deps.contains(&imported_id) {
                deps.push(imported_id);
            }
        }
        
        graph.insert(module_id, deps);
    }
    
    // DFS-based cycle detection
    let mut visited = HashSet::new();
    let mut rec_stack: HashSet<String> = HashSet::new();
    
    fn dfs(
        node: &str,
        graph: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Result<()> {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());
        
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    dfs(neighbor, graph, visited, rec_stack, path)?;
                } else if rec_stack.contains(neighbor) {
                    // Found a cycle!
                    let cycle_start = path.iter().position(|x| x == neighbor).unwrap_or(0);
                    let cycle: Vec<String> = path[cycle_start..].iter().map(|x| x.clone()).collect();
                    let cycle_str = cycle.join(" → ");
                    bail!(
                        "Circular import detected: {} → {}. Please reorganize your imports to break the cycle.",
                        cycle_str, neighbor
                    );
                }
            }
        }
        
        path.pop();
        rec_stack.remove(node);
        Ok(())
    }
    
    // Check each unvisited node
    let all_modules: Vec<String> = graph.keys().cloned().collect();
    for module_id in all_modules {
        if !visited.contains(&module_id) {
            let mut path = Vec::new();
            let mut rec_stack = HashSet::new();
            dfs(&module_id, &graph, &mut visited, &mut rec_stack, &mut path)?;
        }
    }
    
    Ok(())
}

/// Detects name conflicts when multiple modules export the same symbol
/// Issues warnings (not errors) since last import can override
fn detect_name_conflicts(
    symbols: &SymbolTable,
) -> Result<()> {
    // Build a map of symbol_name -> list of modules that export it
    let mut symbol_sources: HashMap<String, Vec<String>> = HashMap::new();
    
    for (module_id, exports) in &symbols.exports {
        for exported_name in exports {
            symbol_sources
                .entry(exported_name.clone())
                .or_insert_with(Vec::new)
                .push(module_id.clone());
        }
    }
    
    // Check for conflicts (same symbol from multiple modules)
    for (symbol_name, sources) in symbol_sources {
        if sources.len() > 1 {
            // Sort for consistent output
            let mut sorted_sources = sources.clone();
            sorted_sources.sort();
            
            eprintln!(
                "⚠️  WARNING: Symbol '{}' is exported by multiple modules: {}",
                symbol_name,
                sorted_sources.join(", ")
            );
            eprintln!(
                "    This may cause unexpected behavior. Consider using module.symbol() notation for clarity."
            );
        }
    }
    
    Ok(())
}

/// Unify multiple modules into a single AST
pub fn unify_modules(
    resolver: &ModuleResolver,
    entry_module: &str,
    options: &UnifyOptions,
) -> Result<UnifiedModule> {
    // Phase 0: Detect circular imports before processing
    detect_circular_imports(resolver)?;
    
    let modules = resolver.get_all_modules();
    
    if modules.is_empty() {
        bail!("No modules to unify");
    }
    
    let mut symbols = SymbolTable::default();
    let mut name_map: HashMap<(String, String), String> = HashMap::new();
    let mut unified_items: Vec<Item> = Vec::new();
    let mut unified_meta = ModuleMeta::default();
    
    // Phase 1: Collect all exports from all modules
    for module in &modules {
        let module_id = module_id_from_path(&module.path);
        let mut module_exports = HashSet::new();
        
        // Check for explicit exports
        let mut has_explicit_exports = false;
        for item in &module.module.items {
            if let Item::Export(e) = item {
                has_explicit_exports = true;
                for sym in &e.symbols {
                    module_exports.insert(sym.clone());
                }
            }
        }
        
        // If no explicit exports, export all top-level definitions
        if !has_explicit_exports {
            for item in &module.module.items {
                match item {
                    Item::Function(f) => { module_exports.insert(f.name.clone()); }
                    Item::Const { name, .. } => { module_exports.insert(name.clone()); }
                    Item::GlobalLet { name, .. } => { module_exports.insert(name.clone()); }
                    Item::VectorList { name, .. } => { module_exports.insert(name.clone()); }
                    _ => {}
                }
            }
        }
        
        symbols.exports.insert(module_id, module_exports);
    }
    
    // Phase 1b: Detect name conflicts across modules
    detect_name_conflicts(&symbols)?;
    
    // Phase 2: Build import aliases for each module
    for module in &modules {
        let module_id = module_id_from_path(&module.path);
        
        for import in &module.module.imports {
            let imported_module_id = import.module_path.join("_");
            
            // Phase 2.5: Validate imported module exists
            if !symbols.exports.contains_key(&imported_module_id) {
                let module_name = import.module_path.join("::");
                bail!(
                    "Cannot find module '{}' imported from '{}'. Available modules: {}",
                    module_name,
                    module_id,
                    symbols.exports.keys()
                        .map(|k| k.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            
            match &import.symbols {
                ImportSymbols::Named(syms) => {
                    for sym in syms {
                        let local_name = sym.alias.as_ref().unwrap_or(&sym.name);
                        let key = format!("{}::{}", module_id, local_name);
                        symbols.aliases.insert(key, (imported_module_id.clone(), sym.name.clone()));
                    }
                }
                ImportSymbols::All => {
                    // Import all exports from the module
                    if let Some(exports) = symbols.exports.get(&imported_module_id) {
                        for name in exports {
                            let key = format!("{}::{}", module_id, name);
                            symbols.aliases.insert(key, (imported_module_id.clone(), name.clone()));
                        }
                    }
                }
                ImportSymbols::Module { alias } => {
                    let local_name = alias.as_ref()
                        .map(|s| s.clone())
                        .unwrap_or_else(|| import.module_path.last().cloned().unwrap_or_default());
                    let key = format!("{}::{}", module_id, local_name);
                    symbols.aliases.insert(key, (imported_module_id.clone(), "*".to_string()));
                }
            }
        }
    }
    
    // Phase 3: FIRST PASS - Generate unified names for ALL symbols
    for module in &modules {
        let module_id = module_id_from_path(&module.path);
        let path_str = module.path.to_string_lossy().to_string();
        let is_entry = module_id == entry_module 
            || path_str.contains(entry_module)
            || path_str.ends_with(&format!("{}.vpy", entry_module));
        
        for item in &module.module.items {
            match item {
                Item::Function(f) => {
                    let unified_name = generate_unified_name(&module_id, &f.name, is_entry, options);
                    name_map.insert((module_id.clone(), f.name.clone()), unified_name);
                }
                Item::Const { name, .. } => {
                    let unified_name = generate_unified_name(&module_id, name, is_entry, options);
                    name_map.insert((module_id.clone(), name.clone()), unified_name);
                }
                Item::GlobalLet { name, .. } => {
                    let unified_name = generate_unified_name(&module_id, name, is_entry, options);
                    name_map.insert((module_id.clone(), name.clone()), unified_name);
                }
                Item::VectorList { name, .. } => {
                    let unified_name = generate_unified_name(&module_id, name, is_entry, options);
                    name_map.insert((module_id.clone(), name.clone()), unified_name);
                }
                _ => {}
            }
        }
    }
    
    // Phase 4: SECOND PASS - Rewrite items with resolved references
    for module in &modules {
        let module_id = module_id_from_path(&module.path);
        let path_str = module.path.to_string_lossy().to_string();
        let is_entry = module_id == entry_module 
            || path_str.contains(entry_module)
            || path_str.ends_with(&format!("{}.vpy", entry_module));
        
        // Use entry module's meta
        if is_entry {
            unified_meta = module.module.meta.clone();
        }
        
        for item in &module.module.items {
            match item {
                Item::Function(f) => {
                    let unified_name = name_map.get(&(module_id.clone(), f.name.clone()))
                        .cloned()
                        .unwrap_or_else(|| f.name.clone());
                    
                    // Rewrite function with unified name and resolved references
                    let unified_func = rewrite_function(f, &unified_name, &module_id, &symbols, &name_map, options);
                    unified_items.push(Item::Function(unified_func));
                }
                Item::Const { name, value, source_line } => {
                    let unified_name = name_map.get(&(module_id.clone(), name.clone()))
                        .cloned()
                        .unwrap_or_else(|| name.clone());
                    
                    let unified_value = rewrite_expr(value, &module_id, &symbols, &name_map, options);
                    unified_items.push(Item::Const { 
                        name: unified_name, 
                        value: unified_value,
                        source_line: *source_line
                    });
                }
                Item::GlobalLet { name, value, source_line } => {
                    let unified_name = name_map.get(&(module_id.clone(), name.clone()))
                        .cloned()
                        .unwrap_or_else(|| name.clone());
                    
                    let unified_value = rewrite_expr(value, &module_id, &symbols, &name_map, options);
                    unified_items.push(Item::GlobalLet { 
                        name: unified_name, 
                        value: unified_value,
                        source_line: *source_line
                    });
                }
                Item::VectorList { name, entries } => {
                    let unified_name = name_map.get(&(module_id.clone(), name.clone()))
                        .cloned()
                        .unwrap_or_else(|| name.clone());
                    
                    unified_items.push(Item::VectorList { 
                        name: unified_name, 
                        entries: entries.clone() 
                    });
                }
                Item::ExprStatement(expr) => {
                    // Only include from entry module
                    if is_entry {
                        let unified_expr = rewrite_expr(expr, &module_id, &symbols, &name_map, options);
                        unified_items.push(Item::ExprStatement(unified_expr));
                    }
                }
                Item::Export(_) => {
                    // Export declarations are metadata, not included in output
                }
                Item::StructDef(_) => {
                    // Phase 3 - struct definitions included as-is for now
                    unified_items.push(item.clone());
                }
            }
        }
    }
    
    // Phase 4.5: Tree shaking - remove unused symbols if enabled
    if options.tree_shake {
        unified_items = shake_tree(unified_items);
    }
    
    Ok(UnifiedModule {
        module: Module {
            items: unified_items,
            meta: unified_meta,
            imports: vec![], // Imports are resolved, no longer needed
        },
        symbols,
        name_map,
    })
}

/// Generate a module ID from a file path
fn module_id_from_path(path: &Path) -> String {
    path.file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Remove unused symbols via tree shaking
fn shake_tree(items: Vec<Item>) -> Vec<Item> {
    use std::collections::HashSet;
    
    // Phase 1: Collect entrypoint functions (always kept)
    let mut used_symbols: HashSet<String> = HashSet::new();
    
    // Entry points are always used
    used_symbols.insert("main".to_string());
    used_symbols.insert("loop".to_string());
    used_symbols.insert("setup".to_string());
    
    // Phase 2: Recursively find all referenced symbols starting from entry points
    loop {
        let mut added_this_round = false;
        let items_snapshot = items.clone();
        
        for item in &items_snapshot {
            match item {
                Item::Function(f) if used_symbols.contains(&f.name) => {
                    // Scan function body for symbol references
                    for stmt in &f.body {
                        let referenced = collect_stmt_symbols(stmt);
                        for sym in referenced {
                            if !used_symbols.contains(&sym) {
                                used_symbols.insert(sym);
                                added_this_round = true;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        
        // No new symbols found, tree shaking complete
        if !added_this_round {
            break;
        }
    }
    
    // Phase 3: Filter items - keep only used symbols
    items.into_iter()
        .filter(|item| {
            match item {
                Item::Function(f) => used_symbols.contains(&f.name),
                Item::Const { name, .. } => used_symbols.contains(name),
                Item::GlobalLet { name, .. } => used_symbols.contains(name),
                Item::VectorList { name, .. } => used_symbols.contains(name),
                Item::Export(_) => false, // Exports are metadata, skip
                Item::StructDef(_) => true, // Keep struct defs (conservative)
                Item::ExprStatement(_) => true, // Keep direct expressions
            }
        })
        .collect()
}

/// Collect all symbol references in a statement (recursive)
fn collect_stmt_symbols(stmt: &Stmt) -> HashSet<String> {
    use std::collections::HashSet;
    let mut symbols = HashSet::new();
    
    match stmt {
        Stmt::Expr(expr, _) => {
            symbols.extend(collect_expr_symbols(expr));
        }
        Stmt::Assign { target, value, .. } => {
            symbols.extend(collect_target_symbols(target));
            symbols.extend(collect_expr_symbols(value));
        }
        Stmt::Let { value, .. } => {
            symbols.extend(collect_expr_symbols(value));
        }
        Stmt::If { cond, body, elifs, else_body, .. } => {
            symbols.extend(collect_expr_symbols(cond));
            for s in body {
                symbols.extend(collect_stmt_symbols(s));
            }
            for (elif_cond, elif_body) in elifs {
                symbols.extend(collect_expr_symbols(elif_cond));
                for s in elif_body {
                    symbols.extend(collect_stmt_symbols(s));
                }
            }
            if let Some(else_stmts) = else_body {
                for s in else_stmts {
                    symbols.extend(collect_stmt_symbols(s));
                }
            }
        }
        Stmt::While { cond, body, .. } => {
            symbols.extend(collect_expr_symbols(cond));
            for s in body {
                symbols.extend(collect_stmt_symbols(s));
            }
        }
        Stmt::For { start, end, step, body, .. } => {
            symbols.extend(collect_expr_symbols(start));
            symbols.extend(collect_expr_symbols(end));
            if let Some(step_expr) = step {
                symbols.extend(collect_expr_symbols(step_expr));
            }
            for s in body {
                symbols.extend(collect_stmt_symbols(s));
            }
        }
        Stmt::ForIn { iterable, body, .. } => {
            symbols.extend(collect_expr_symbols(iterable));
            for s in body {
                symbols.extend(collect_stmt_symbols(s));
            }
        }
        Stmt::Return(Some(expr), _) => {
            symbols.extend(collect_expr_symbols(expr));
        }
        Stmt::CompoundAssign { target, value, .. } => {
            symbols.extend(collect_target_symbols(target));
            symbols.extend(collect_expr_symbols(value));
        }
        Stmt::Switch { expr, cases, default, .. } => {
            symbols.extend(collect_expr_symbols(expr));
            for (case_expr, case_body) in cases {
                symbols.extend(collect_expr_symbols(case_expr));
                for s in case_body {
                    symbols.extend(collect_stmt_symbols(s));
                }
            }
            if let Some(default_body) = default {
                for s in default_body {
                    symbols.extend(collect_stmt_symbols(s));
                }
            }
        }
        _ => {}
    }
    
    symbols
}

/// Collect symbol references in expressions (recursive)
fn collect_expr_symbols(expr: &Expr) -> HashSet<String> {
    use std::collections::HashSet;
    let mut symbols = HashSet::new();
    
    match expr {
        Expr::Ident(ident_info) => {
            symbols.insert(ident_info.name.clone());
        }
        Expr::Call(call_info) => {
            symbols.insert(call_info.name.clone());
            for arg in &call_info.args {
                symbols.extend(collect_expr_symbols(arg));
            }
        }
        Expr::MethodCall(method_info) => {
            symbols.extend(collect_expr_symbols(&method_info.target));
            for arg in &method_info.args {
                symbols.extend(collect_expr_symbols(arg));
            }
        }
        Expr::Binary { left, right, .. } => {
            symbols.extend(collect_expr_symbols(left));
            symbols.extend(collect_expr_symbols(right));
        }
        Expr::Compare { left, right, .. } => {
            symbols.extend(collect_expr_symbols(left));
            symbols.extend(collect_expr_symbols(right));
        }
        Expr::Logic { left, right, .. } => {
            symbols.extend(collect_expr_symbols(left));
            symbols.extend(collect_expr_symbols(right));
        }
        Expr::Not(operand) | Expr::BitNot(operand) => {
            symbols.extend(collect_expr_symbols(operand));
        }
        Expr::List(items) => {
            for item in items {
                symbols.extend(collect_expr_symbols(item));
            }
        }
        Expr::Index { target, index } => {
            symbols.extend(collect_expr_symbols(target));
            symbols.extend(collect_expr_symbols(index));
        }
        Expr::FieldAccess { target, .. } => {
            symbols.extend(collect_expr_symbols(target));
        }
        Expr::StructInit { struct_name, .. } => {
            symbols.insert(struct_name.clone());
        }
        _ => {}
    }
    
    symbols
}

/// Collect symbol references in assignment targets
fn collect_target_symbols(target: &AssignTarget) -> HashSet<String> {
    use std::collections::HashSet;
    let mut symbols = HashSet::new();
    
    match target {
        AssignTarget::Ident { name, .. } => {
            symbols.insert(name.clone());
        }
        AssignTarget::Index { target: target_expr, index, .. } => {
            symbols.extend(collect_expr_symbols(target_expr));
            symbols.extend(collect_expr_symbols(index));
        }
        AssignTarget::FieldAccess { target: target_expr, .. } => {
            symbols.extend(collect_expr_symbols(target_expr));
        }
    }
    
    symbols
}

/// Generate a unified name for a symbol
fn generate_unified_name(
    module_id: &str,
    name: &str,
    is_entry: bool,
    options: &UnifyOptions,
) -> String {
    // Special functions in entry module keep their names
    if is_entry && (name == "main" || name == "loop" || name == "setup") {
        return name.to_string();
    }
    
    if options.prefix_symbols && !is_entry {
        format!("{}{}{}", module_id, options.prefix_separator, name)
    } else {
        name.to_string()
    }
}

/// Rewrite a function with resolved references
fn rewrite_function(
    f: &Function,
    new_name: &str,
    current_module: &str,
    symbols: &SymbolTable,
    name_map: &HashMap<(String, String), String>,
    options: &UnifyOptions,
) -> Function {
    Function {
        name: new_name.to_string(),
        line: f.line,
        params: f.params.clone(),
        body: f.body.iter()
            .map(|s| rewrite_stmt(s, current_module, symbols, name_map, options))
            .collect(),
    }
}

/// Rewrite a statement with resolved references
fn rewrite_stmt(
    stmt: &Stmt,
    current_module: &str,
    symbols: &SymbolTable,
    name_map: &HashMap<(String, String), String>,
    options: &UnifyOptions,
) -> Stmt {
    match stmt {
        Stmt::Assign { target, value, source_line } => {
            Stmt::Assign {
                target: rewrite_assign_target(target, current_module, symbols, name_map, options),
                value: rewrite_expr(value, current_module, symbols, name_map, options),
                source_line: *source_line,
            }
        }
        Stmt::Let { name, value, source_line } => {
            Stmt::Let {
                name: name.clone(),
                value: rewrite_expr(value, current_module, symbols, name_map, options),
                source_line: *source_line,
            }
        }
        Stmt::For { var, start, end, step, body, source_line } => {
            Stmt::For {
                var: var.clone(),
                start: rewrite_expr(start, current_module, symbols, name_map, options),
                end: rewrite_expr(end, current_module, symbols, name_map, options),
                step: step.as_ref().map(|e| rewrite_expr(e, current_module, symbols, name_map, options)),
                body: body.iter()
                    .map(|s| rewrite_stmt(s, current_module, symbols, name_map, options))
                    .collect(),
                source_line: *source_line,
            }
        }
        Stmt::ForIn { var, iterable, body, source_line } => {
            Stmt::ForIn {
                var: var.clone(),
                iterable: rewrite_expr(iterable, current_module, symbols, name_map, options),
                body: body.iter()
                    .map(|s| rewrite_stmt(s, current_module, symbols, name_map, options))
                    .collect(),
                source_line: *source_line,
            }
        }
        Stmt::While { cond, body, source_line } => {
            Stmt::While {
                cond: rewrite_expr(cond, current_module, symbols, name_map, options),
                body: body.iter()
                    .map(|s| rewrite_stmt(s, current_module, symbols, name_map, options))
                    .collect(),
                source_line: *source_line,
            }
        }
        Stmt::If { cond, body, elifs, else_body, source_line } => {
            Stmt::If {
                cond: rewrite_expr(cond, current_module, symbols, name_map, options),
                body: body.iter()
                    .map(|s| rewrite_stmt(s, current_module, symbols, name_map, options))
                    .collect(),
                elifs: elifs.iter()
                    .map(|(c, b)| (
                        rewrite_expr(c, current_module, symbols, name_map, options),
                        b.iter().map(|s| rewrite_stmt(s, current_module, symbols, name_map, options)).collect()
                    ))
                    .collect(),
                else_body: else_body.as_ref().map(|b| 
                    b.iter().map(|s| rewrite_stmt(s, current_module, symbols, name_map, options)).collect()
                ),
                source_line: *source_line,
            }
        }
        Stmt::Switch { expr, cases, default, source_line } => {
            Stmt::Switch {
                expr: rewrite_expr(expr, current_module, symbols, name_map, options),
                cases: cases.iter()
                    .map(|(c, b)| (
                        rewrite_expr(c, current_module, symbols, name_map, options),
                        b.iter().map(|s| rewrite_stmt(s, current_module, symbols, name_map, options)).collect()
                    ))
                    .collect(),
                default: default.as_ref().map(|b| 
                    b.iter().map(|s| rewrite_stmt(s, current_module, symbols, name_map, options)).collect()
                ),
                source_line: *source_line,
            }
        }
        Stmt::Return(expr, line) => {
            Stmt::Return(
                expr.as_ref().map(|e| rewrite_expr(e, current_module, symbols, name_map, options)),
                *line
            )
        }
        Stmt::Expr(expr, line) => {
            Stmt::Expr(rewrite_expr(expr, current_module, symbols, name_map, options), *line)
        }
        Stmt::CompoundAssign { target, op, value, source_line } => {
            Stmt::CompoundAssign {
                target: rewrite_assign_target(target, current_module, symbols, name_map, options),
                op: *op,
                value: rewrite_expr(value, current_module, symbols, name_map, options),
                source_line: *source_line,
            }
        }
        Stmt::Break { source_line } => Stmt::Break { source_line: *source_line },
        Stmt::Continue { source_line } => Stmt::Continue { source_line: *source_line },
        Stmt::Pass { source_line } => Stmt::Pass { source_line: *source_line },
    }
}

/// Rewrite an expression with resolved references
fn rewrite_expr(
    expr: &Expr,
    current_module: &str,
    symbols: &SymbolTable,
    name_map: &HashMap<(String, String), String>,
    options: &UnifyOptions,
) -> Expr {
    match expr {
        Expr::Ident(info) => {
            let resolved_name = resolve_identifier(&info.name, current_module, symbols, name_map, options);
            Expr::Ident(IdentInfo {
                name: resolved_name,
                source_line: info.source_line,
                col: info.col,
            })
        }
        Expr::List(elements) => Expr::List(
            elements.iter()
                .map(|e| rewrite_expr(e, current_module, symbols, name_map, options))
                .collect()
        ),
        Expr::Index { target, index } => Expr::Index {
            target: Box::new(rewrite_expr(target, current_module, symbols, name_map, options)),
            index: Box::new(rewrite_expr(index, current_module, symbols, name_map, options)),
        },
        Expr::Call(info) => {
            let resolved_name = resolve_identifier(&info.name, current_module, symbols, name_map, options);
            Expr::Call(CallInfo {
                name: resolved_name,
                source_line: info.source_line,
                col: info.col,
                args: info.args.iter()
                    .map(|e| rewrite_expr(e, current_module, symbols, name_map, options))
                    .collect(),
            })
        }
        Expr::MethodCall(mc) => {
            // CRITICAL: Detect module.method() pattern (input.get_input(), graphics.draw_square(), etc.)
            if let Expr::Ident(target_info) = &*mc.target {
                // Check if target is an imported module (import input)
                let module_check_key = format!("{}::{}", current_module, &target_info.name);
                if let Some((origin_module, symbol_or_marker)) = symbols.aliases.get(&module_check_key) {
                    // Check if this is a module import (marker = "*")
                    if symbol_or_marker == "*" {
                        // This is module.method() - transform to function call with unified name
                        let unified_name = name_map
                            .get(&(origin_module.clone(), mc.method_name.clone()))
                            .cloned()
                            .unwrap_or_else(|| {
                                // Fallback: prefix with module name
                                format!("{}_{}", origin_module.to_uppercase(), mc.method_name.to_uppercase())
                            });
                        
                        let rewritten_args: Vec<Expr> = mc.args.iter()
                            .map(|e| rewrite_expr(e, current_module, symbols, name_map, options))
                            .collect();
                        
                        return Expr::Call(crate::ast::CallInfo {
                            name: unified_name,
                            args: rewritten_args,
                            source_line: mc.source_line,
                            col: mc.col,
                        });
                    }
                }
            }
            
            // Not module.method() - rewrite target and args normally
            Expr::MethodCall(crate::ast::MethodCallInfo {
                target: Box::new(rewrite_expr(&mc.target, current_module, symbols, name_map, options)),
                method_name: mc.method_name.clone(),
                args: mc.args.iter()
                    .map(|e| rewrite_expr(e, current_module, symbols, name_map, options))
                    .collect(),
                source_line: mc.source_line,
                col: mc.col,
            })
        }
        Expr::Binary { op, left, right } => {
            Expr::Binary {
                op: *op,
                left: Box::new(rewrite_expr(left, current_module, symbols, name_map, options)),
                right: Box::new(rewrite_expr(right, current_module, symbols, name_map, options)),
            }
        }
        Expr::Compare { op, left, right } => {
            Expr::Compare {
                op: *op,
                left: Box::new(rewrite_expr(left, current_module, symbols, name_map, options)),
                right: Box::new(rewrite_expr(right, current_module, symbols, name_map, options)),
            }
        }
        Expr::Logic { op, left, right } => {
            Expr::Logic {
                op: *op,
                left: Box::new(rewrite_expr(left, current_module, symbols, name_map, options)),
                right: Box::new(rewrite_expr(right, current_module, symbols, name_map, options)),
            }
        }
        Expr::Not(inner) => {
            Expr::Not(Box::new(rewrite_expr(inner, current_module, symbols, name_map, options)))
        }
        Expr::BitNot(inner) => {
            Expr::BitNot(Box::new(rewrite_expr(inner, current_module, symbols, name_map, options)))
        }
        // Literals pass through unchanged
        Expr::Number(n) => Expr::Number(*n),
        Expr::StringLit(s) => Expr::StringLit(s.clone()),
        Expr::StructInit { struct_name, source_line, col } => {
            // Phase 3 - struct init passes through for now
            Expr::StructInit { 
                struct_name: struct_name.clone(), 
                source_line: *source_line, 
                col: *col 
            }
        }
        Expr::FieldAccess { target, field, source_line, col } => {
            // CRITICAL: Detect module.symbol pattern (input.get_input, input.input_result, etc.)
            if let Expr::Ident(target_info) = &**target {
                // Check if target is an imported module (import input)
                let module_check_key = format!("{}::{}", current_module, &target_info.name);
                if let Some((origin_module, symbol_or_marker)) = symbols.aliases.get(&module_check_key) {
                    // Check if this is a module import (marker = "*")
                    if symbol_or_marker == "*" {
                        // This is module.symbol - look up the symbol in the origin module
                        let unified_name = name_map
                            .get(&(origin_module.clone(), field.clone()))
                            .cloned()
                            .unwrap_or_else(|| {
                                // Fallback: prefix with module name
                                format!("{}_{}", origin_module.to_uppercase(), field.to_uppercase())
                            });
                        
                        return Expr::Ident(IdentInfo {
                            name: unified_name,
                            source_line: *source_line,
                            col: *col,
                        });
                    }
                }
            }
            
            // Not module.symbol - recurse normally
            Expr::FieldAccess {
                target: Box::new(rewrite_expr(target, current_module, symbols, name_map, options)),
                field: field.clone(),
                source_line: *source_line,
                col: *col,
            }
        }
    }
}

/// Rewrite an assignment target with resolved references
fn rewrite_assign_target(
    target: &crate::ast::AssignTarget,
    current_module: &str,
    symbols: &SymbolTable,
    name_map: &HashMap<(String, String), String>,
    options: &UnifyOptions,
) -> crate::ast::AssignTarget {
    match target {
        crate::ast::AssignTarget::Ident { name, source_line, col } => {
            let resolved_name = resolve_identifier(name, current_module, symbols, name_map, options);
            crate::ast::AssignTarget::Ident {
                name: resolved_name,
                source_line: *source_line,
                col: *col,
            }
        }
        crate::ast::AssignTarget::Index { target, index, source_line, col } => {
            crate::ast::AssignTarget::Index {
                target: Box::new(rewrite_expr(target, current_module, symbols, name_map, options)),
                index: Box::new(rewrite_expr(index, current_module, symbols, name_map, options)),
                source_line: *source_line,
                col: *col,
            }
        }
        crate::ast::AssignTarget::FieldAccess { target, field, source_line, col } => {
            // Same logic as Expr::FieldAccess - detect module.symbol pattern
            if let Expr::Ident(target_info) = &**target {
                let module_check_key = format!("{}::{}", current_module, &target_info.name);
                if let Some((origin_module, symbol_or_marker)) = symbols.aliases.get(&module_check_key) {
                    if symbol_or_marker == "*" {
                        // module.symbol in assignment - transform to Index with unified name
                        let unified_name = name_map
                            .get(&(origin_module.clone(), field.clone()))
                            .cloned()
                            .unwrap_or_else(|| {
                                format!("{}_{}", origin_module.to_uppercase(), field.to_uppercase())
                            });
                        
                        // Return as Ident target (the unified symbol name)
                        return crate::ast::AssignTarget::Ident {
                            name: unified_name,
                            source_line: *source_line,
                            col: *col,
                        };
                    }
                }
            }
            
            // Not module.symbol - recurse normally
            crate::ast::AssignTarget::FieldAccess {
                target: Box::new(rewrite_expr(target, current_module, symbols, name_map, options)),
                field: field.clone(),
                source_line: *source_line,
                col: *col,
            }
        }
    }
}

/// Resolve an identifier to its unified name
fn resolve_identifier(
    name: &str,
    current_module: &str,
    symbols: &SymbolTable,
    name_map: &HashMap<(String, String), String>,
    _options: &UnifyOptions,
) -> String {
    // Check if this is an imported symbol
    let alias_key = format!("{}::{}", current_module, name);
    if let Some((module, original_name)) = symbols.aliases.get(&alias_key) {
        // Look up the unified name for the imported symbol
        if let Some(unified) = name_map.get(&(module.clone(), original_name.clone())) {
            return unified.clone();
        }
    }
    
    // Check if defined in current module
    if let Some(unified) = name_map.get(&(current_module.to_string(), name.to_string())) {
        return unified.clone();
    }
    
    // Built-in or unknown - return as-is
    name.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_module_id_from_path() {
        let path = Path::new("/project/src/utils/math.vpy");
        assert_eq!(module_id_from_path(path), "math");
    }
    
    #[test]
    fn test_generate_unified_name_entry() {
        let options = UnifyOptions::default();
        assert_eq!(generate_unified_name("main", "main", true, &options), "main");
        assert_eq!(generate_unified_name("main", "loop", true, &options), "loop");
    }
    
    #[test]
    fn test_generate_unified_name_non_entry() {
        let options = UnifyOptions::default();
        assert_eq!(generate_unified_name("utils", "clamp", false, &options), "utils_clamp");
    }
}
