//! Call graph analysis for determining function dependencies
//!
//! Builds a call graph to understand function call patterns
//! for optimal bank allocation

use std::collections::HashMap;
use vpy_parser::{Module, Item, Function, Stmt, Expr};

/// Node in the call graph (represents a function)
#[derive(Debug, Clone)]
pub struct FunctionNode {
    /// Function name
    pub name: String,
    /// Estimated size in bytes (ASM code)
    pub size_bytes: usize,
    /// Is this a critical function (main, loop, interrupt handler)?
    pub is_critical: bool,
}

/// Edge in the call graph (represents a function call)
#[derive(Debug, Clone)]
pub struct CallEdge {
    /// Caller function name
    pub from: String,
    /// Callee function name
    pub to: String,
}

/// Complete call graph
#[derive(Debug, Clone)]
pub struct CallGraph {
    /// All function nodes
    pub nodes: HashMap<String, FunctionNode>,
    /// All call edges
    pub edges: Vec<CallEdge>,
}

impl CallGraph {
    pub fn new() -> Self {
        CallGraph {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }
    
    /// Build call graph from unified module
    pub fn from_module(module: &Module) -> Self {
        let mut graph = CallGraph::new();
        
        // Add all functions as nodes
        for item in &module.items {
            if let Item::Function(func) = item {
                let size_estimate = estimate_function_size(func);
                let is_critical = func.name == "main" || func.name == "loop";
                
                graph.add_node(FunctionNode {
                    name: func.name.clone(),
                    size_bytes: size_estimate,
                    is_critical,
                });
            }
        }
        
        // Analyze function calls to build edges
        for item in &module.items {
            if let Item::Function(func) = item {
                let callees = find_function_calls(&func.body);
                for callee in callees {
                    graph.add_edge(CallEdge {
                        from: func.name.clone(),
                        to: callee,
                    });
                }
            }
        }
        
        graph
    }
    
    /// Add a function node
    pub fn add_node(&mut self, node: FunctionNode) {
        self.nodes.insert(node.name.clone(), node);
    }
    
    /// Add a call edge
    pub fn add_edge(&mut self, edge: CallEdge) {
        self.edges.push(edge);
    }
    
    /// Get functions that are called by a specific function
    pub fn callees(&self, func: &str) -> Vec<String> {
        self.edges.iter()
            .filter(|e| e.from == func)
            .map(|e| e.to.clone())
            .collect()
    }
    
    /// Get functions that call a specific function
    pub fn callers(&self, func: &str) -> Vec<String> {
        self.edges.iter()
            .filter(|e| e.to == func)
            .map(|e| e.from.clone())
            .collect()
    }
}

/// Estimate function size in bytes (ASM code)
/// 
/// Rough estimates:
/// - Statement: ~10 bytes average
/// - Expression: ~5 bytes average
/// - Function overhead: 20 bytes (label + RTS)
fn estimate_function_size(func: &Function) -> usize {
    let stmt_count = count_statements(&func.body);
    let base_size = 20; // Function overhead
    let stmt_avg = 10; // Average bytes per statement
    
    base_size + (stmt_count * stmt_avg)
}

/// Count total statements recursively
fn count_statements(body: &[Stmt]) -> usize {
    let mut count = 0;
    for stmt in body {
        count += 1; // This statement
        
        // Recurse into nested statements
        match stmt {
            Stmt::If { body, elifs, else_body, .. } => {
                count += count_statements(body);
                for (_, elif_body) in elifs {
                    count += count_statements(elif_body);
                }
                if let Some(else_b) = else_body {
                    count += count_statements(else_b);
                }
            },
            Stmt::While { body, .. } |
            Stmt::For { body, .. } => {
                count += count_statements(body);
            },
            _ => {}
        }
    }
    count
}

/// Find all function calls in a statement block
fn find_function_calls(body: &[Stmt]) -> Vec<String> {
    let mut calls = Vec::new();
    
    for stmt in body {
        find_calls_in_stmt(stmt, &mut calls);
    }
    
    calls
}

/// Recursively find function calls in a statement
fn find_calls_in_stmt(stmt: &Stmt, calls: &mut Vec<String>) {
    match stmt {
        Stmt::Expr(expr, _) => {
            find_calls_in_expr(expr, calls);
        },
        Stmt::Return(Some(expr), _) => {
            find_calls_in_expr(expr, calls);
        },
        Stmt::Assign { value, .. } => {
            find_calls_in_expr(value, calls);
        },
        Stmt::If { cond, body, elifs, else_body, .. } => {
            find_calls_in_expr(cond, calls);
            for s in body {
                find_calls_in_stmt(s, calls);
            }
            for (elif_cond, elif_body) in elifs {
                find_calls_in_expr(elif_cond, calls);
                for s in elif_body {
                    find_calls_in_stmt(s, calls);
                }
            }
            if let Some(else_b) = else_body {
                for s in else_b {
                    find_calls_in_stmt(s, calls);
                }
            }
        },
        Stmt::While { cond, body, .. } => {
            find_calls_in_expr(cond, calls);
            for s in body {
                find_calls_in_stmt(s, calls);
            }
        },
        Stmt::For { start, end, step, body, .. } => {
            find_calls_in_expr(start, calls);
            find_calls_in_expr(end, calls);
            if let Some(st) = step {
                find_calls_in_expr(st, calls);
            }
            for s in body {
                find_calls_in_stmt(s, calls);
            }
        },
        _ => {}
    }
}

/// Recursively find function calls in an expression
fn find_calls_in_expr(expr: &Expr, calls: &mut Vec<String>) {
    match expr {
        Expr::Call(call_info) => {
            calls.push(call_info.name.clone());
            for arg in &call_info.args {
                find_calls_in_expr(arg, calls);
            }
        },
        Expr::MethodCall(method_call) => {
            find_calls_in_expr(&method_call.target, calls);
            for arg in &method_call.args {
                find_calls_in_expr(arg, calls);
            }
        },
        Expr::Binary { left, right, .. } => {
            find_calls_in_expr(left, calls);
            find_calls_in_expr(right, calls);
        },
        Expr::Compare { left, right, .. } => {
            find_calls_in_expr(left, calls);
            find_calls_in_expr(right, calls);
        },
        Expr::Logic { left, right, .. } => {
            find_calls_in_expr(left, calls);
            find_calls_in_expr(right, calls);
        },
        Expr::Not(operand) | Expr::BitNot(operand) => {
            find_calls_in_expr(operand, calls);
        },
        Expr::Index { target, index } => {
            find_calls_in_expr(target, calls);
            find_calls_in_expr(index, calls);
        },
        Expr::List(elements) => {
            for elem in elements {
                find_calls_in_expr(elem, calls);
            }
        },
        Expr::FieldAccess { target, .. } => {
            find_calls_in_expr(target, calls);
        },
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vpy_parser::{Module, ModuleMeta, Item, Function};

    #[test]
    fn test_call_graph_creation() {
        let graph = CallGraph::new();
        assert!(graph.edges.is_empty());
        assert!(graph.nodes.is_empty());
    }
    
    #[test]
    fn test_add_node() {
        let mut graph = CallGraph::new();
        graph.add_node(FunctionNode {
            name: "test_func".to_string(),
            size_bytes: 100,
            is_critical: false,
        });
        
        assert_eq!(graph.nodes.len(), 1);
        assert!(graph.nodes.contains_key("test_func"));
    }
    
    #[test]
    fn test_add_edge() {
        let mut graph = CallGraph::new();
        graph.add_edge(CallEdge {
            from: "main".to_string(),
            to: "helper".to_string(),
        });
        
        assert_eq!(graph.edges.len(), 1);
        assert_eq!(graph.callees("main"), vec!["helper"]);
    }
    
    #[test]
    fn test_from_module_simple() {
        // Create minimal module with one function
        let module = Module {
            items: vec![
                Item::Function(Function {
                    name: "main".to_string(),
                    line: 0,
                    params: vec![],
                    body: vec![],
                })
            ],
            meta: ModuleMeta::default(),
            imports: vec![],
        };
        
        let graph = CallGraph::from_module(&module);
        assert_eq!(graph.nodes.len(), 1);
        assert!(graph.nodes.contains_key("main"));
    }
}