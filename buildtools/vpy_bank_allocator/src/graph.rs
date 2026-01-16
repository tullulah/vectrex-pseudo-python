//! Call graph analysis for determining function dependencies
//!
//! Builds a call graph to understand function call patterns
//! for optimal bank allocation

#[derive(Debug, Clone)]
pub struct CallGraph {
    /// Adjacency list: function -> functions it calls
    pub edges: std::collections::HashMap<String, Vec<String>>,
}

impl CallGraph {
    pub fn new() -> Self {
        CallGraph {
            edges: std::collections::HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: String, to: String) {
        self.edges
            .entry(from)
            .or_insert_with(Vec::new)
            .push(to);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_graph_creation() {
        let graph = CallGraph::new();
        assert!(graph.edges.is_empty());
    }
}
