//! VPy Unifier: Phase 3 of buildtools compiler pipeline
//!
//! Resolves imports, validates symbols, and merges multiple modules into one
//!
//! # Module Structure
//!
//! - `error.rs`: Error types (UnifierError, UnifierResult)
//! - `graph.rs`: Module dependency graph with cycle detection
//! - `resolver.rs`: Symbol resolver for unified naming
//! - `visitor.rs`: AST visitor pattern for custom passes
//! - `scope.rs`: Scope management (variables, functions, imports)
//!
//! # Input
//! `Vec<Module>` (parsed AST from Phase 2)
//!
//! # Output
//! `Module` (single merged module with resolved imports)
//!
//! # Phases
//!
//! 1. **Load & Graph**: Load all .vpy files, build dependency graph
//! 2. **Cycle Detection**: Reject circular imports
//! 3. **Topological Sort**: Determine merge order (dependencies first)
//! 4. **Symbol Renaming**: Prefix imported symbols (INPUT_*, GRAPHICS_*, etc.)
//! 5. **Merge**: Combine items into single module
//! 6. **Reference Fixing**: Update all calls/accesses to use renamed symbols
//! 7. **Validation**: Ensure all symbols are defined

pub mod error;
pub mod graph;
pub mod resolver;
pub mod scope;
pub mod visitor;

pub use error::{UnifierError, UnifierResult};
pub use graph::ModuleGraph;
pub use resolver::SymbolResolver;
pub use scope::Scope;
pub use visitor::AstVisitor;
pub use vpy_parser::ast::{Module, Item, ImportDecl};

/// Unify multiple modules into a single resolved module
///
/// # Arguments
/// * `modules` - Map of module names to parsed Modules from Phase 2
///
/// # Returns
/// * `UnifierResult<Module>` - Unified module or error
///
/// # Process
/// 1. Build dependency graph
/// 2. Check for circular imports
/// 3. Topologically sort modules
/// 4. Rename symbols based on module prefix
/// 5. Merge into single module
/// 6. Validate all references
pub fn unify_modules(
    modules: std::collections::HashMap<String, Module>,
    entry_module: &str,
) -> UnifierResult<Module> {
    // Phase 3.1: Build graph
    let mut graph = ModuleGraph::new();
    for (name, module) in modules {
        graph.add_module(name, module);
    }

    // Phase 3.2: Detect cycles
    if let Some(cycle) = graph.detect_cycles() {
        return Err(UnifierError::CircularDependency(format!("{:?}", cycle)));
    }

    // Phase 3.3: Topologically sort
    let sort_order = graph.topological_sort()?;

    // Phase 3.4-3.5: Symbol resolution and merge
    let mut resolver = SymbolResolver::new();
    let mut merged_items = Vec::new();
    let mut merged_meta = Default::default();

    for module_name in sort_order {
        let _prefix = resolver.register_module(&module_name);
        if let Some(module) = graph.get_module(&module_name) {
            // For entry module, use its metadata
            if module_name == entry_module {
                merged_meta = module.meta.clone();
            }

            // Add items (TODO: rename symbols during addition)
            for item in &module.items {
                merged_items.push(item.clone());
            }
        }
    }

    Ok(Module {
        items: merged_items,
        meta: merged_meta,
        imports: vec![],  // All imports resolved, empty in unified module
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use vpy_parser::ast::ModuleMeta;

    #[test]
    fn test_unify_single_module() {
        let mut modules = HashMap::new();
        let module = Module {
            items: vec![],
            meta: ModuleMeta::default(),
            imports: vec![],
        };
        modules.insert("main".to_string(), module);

        let result = unify_modules(modules, "main");
        assert!(result.is_ok());
        let unified = result.unwrap();
        assert_eq!(unified.items.len(), 0);
    }

    #[test]
    fn test_unify_no_circular_import() {
        let mut modules = HashMap::new();
        let module = Module {
            items: vec![],
            meta: ModuleMeta::default(),
            imports: vec![],
        };
        modules.insert("main".to_string(), module.clone());
        modules.insert("util".to_string(), module);

        let result = unify_modules(modules, "main");
        assert!(result.is_ok());
    }

    #[test]
    fn test_symbol_resolver_basic() {
        let mut resolver = SymbolResolver::new();
        
        resolver.register_module("main");
        resolver.register_module("input");
        
        assert_eq!(resolver.resolve_symbol("func", "main"), "func");
        assert_eq!(resolver.resolve_symbol("func", "input"), "INPUT_func");
    }

    #[test]
    fn test_graph_creation() {
        let mut graph = ModuleGraph::new();
        let module = Module {
            items: vec![],
            meta: ModuleMeta::default(),
            imports: vec![],
        };
        
        graph.add_module("test".to_string(), module);
        assert_eq!(graph.modules().len(), 1);
    }

    #[test]
    fn test_cycle_detection() {
        let mut graph = ModuleGraph::new();
        let module = Module {
            items: vec![],
            meta: ModuleMeta::default(),
            imports: vec![],
        };
        
        graph.add_module("a".to_string(), module.clone());
        graph.add_module("b".to_string(), module);
        
        graph.add_dependency("a".to_string(), "b".to_string()).unwrap();
        graph.add_dependency("b".to_string(), "a".to_string()).unwrap();
        
        assert!(graph.detect_cycles().is_some());
    }

    #[test]
    fn test_topological_sort() {
        let mut graph = ModuleGraph::new();
        let module = Module {
            items: vec![],
            meta: ModuleMeta::default(),
            imports: vec![],
        };
        
        graph.add_module("util".to_string(), module.clone());
        graph.add_module("input".to_string(), module.clone());
        graph.add_module("main".to_string(), module);
        
        graph.add_dependency("input".to_string(), "util".to_string()).unwrap();
        graph.add_dependency("main".to_string(), "input".to_string()).unwrap();
        
        let order = graph.topological_sort().unwrap();
        
        // Check correct order
        assert_eq!(order.len(), 3);
        let util_idx = order.iter().position(|x| x == "util").unwrap();
        let input_idx = order.iter().position(|x| x == "input").unwrap();
        let main_idx = order.iter().position(|x| x == "main").unwrap();
        
        assert!(util_idx < input_idx);
        assert!(input_idx < main_idx);
    }
}
