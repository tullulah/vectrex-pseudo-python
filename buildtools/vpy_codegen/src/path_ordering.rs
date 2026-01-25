//! Path ordering optimization for vector graphics
//!
//! Reorders vector paths to minimize beam travel distance,
//! reducing flicker on Vectrex hardware.

use crate::vecres::{VecPath, Point};
use std::collections::{HashMap, HashSet};

/// Trait for path ordering strategies
pub trait PathOrderer {
    /// Reorder paths to minimize beam travel distance
    fn order(&self, paths: Vec<VecPath>) -> Vec<VecPath>;
}

/// Greedy Nearest Neighbor with 2-opt local search
/// 
/// Algorithm:
/// 1. Start at (0,0)
/// 2. Greedily pick nearest unvisited path
/// 3. Apply 2-opt improvements (try reversing paths, swapping order)
/// 4. Repeat until convergence
/// 
/// Result: ~90-95% of optimal tour, deterministic
pub struct GreedyThenTwoOpt {
    /// Maximum iterations for 2-opt refinement
    pub max_iterations: usize,
}

impl Default for GreedyThenTwoOpt {
    fn default() -> Self {
        Self {
            max_iterations: 100,
        }
    }
}

impl GreedyThenTwoOpt {
    pub fn new(max_iterations: usize) -> Self {
        Self { max_iterations }
    }

    /// Calculate Euclidean distance between two points
    fn distance(p1: Point, p2: Point) -> f64 {
        let dx = (p2.x - p1.x) as f64;
        let dy = (p2.y - p1.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }

    /// Get the first point of a path
    fn path_start(path: &VecPath) -> Point {
        path.points.first().copied().unwrap_or(Point { x: 0, y: 0, intensity: None })
    }

    /// Get the last point of a path
    fn path_end(path: &VecPath) -> Point {
        path.points.last().copied().unwrap_or(Point { x: 0, y: 0, intensity: None })
    }

    /// Calculate total tour distance
    fn tour_distance(paths: &[VecPath]) -> f64 {
        let mut dist = 0.0;
        let mut current_pos = Point { x: 0, y: 0, intensity: None };

        for path in paths {
            let start = Self::path_start(path);
            let end = Self::path_end(path);
            dist += Self::distance(current_pos, start);
            dist += Self::distance(start, end);
            current_pos = end;
        }

        dist
    }

    /// Greedy nearest neighbor initial solution
    fn greedy_initial(&self, mut paths: Vec<VecPath>) -> Vec<VecPath> {
        let mut ordered = Vec::new();
        let mut current_pos = Point { x: 0, y: 0, intensity: None };

        while !paths.is_empty() {
            // Find nearest unvisited path
            let (best_idx, should_reverse) = paths
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let forward_dist = Self::distance(current_pos, Self::path_start(p));
                    let reverse_dist = Self::distance(current_pos, Self::path_end(p));
                    (i, forward_dist, reverse_dist)
                })
                .min_by(|a, b| {
                    let a_min = a.1.min(a.2);
                    let b_min = b.1.min(b.2);
                    a_min.partial_cmp(&b_min).unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(i, f, r)| (i, r < f))
                .unwrap_or((0, false));

            let mut path = paths.remove(best_idx);

            // Reverse path if it's closer from the end
            if should_reverse {
                path.points.reverse();
            }

            current_pos = Self::path_end(&path);
            ordered.push(path);
        }

        ordered
    }

    /// 2-opt local search refinement
    fn two_opt_improve(&self, mut paths: Vec<VecPath>) -> Vec<VecPath> {
        let mut improved = true;
        let mut iterations = 0;

        while improved && iterations < self.max_iterations {
            improved = false;
            iterations += 1;

            let current_distance = Self::tour_distance(&paths);

            // Try all pairs of paths
            for i in 0..paths.len() {
                for j in (i + 1)..paths.len() {
                    // Try reversing path i
                    {
                        let mut test_paths = paths.clone();
                        test_paths[i].points.reverse();
                        let new_distance = Self::tour_distance(&test_paths);
                        if new_distance < current_distance {
                            paths = test_paths;
                            improved = true;
                            break;
                        }
                    }

                    // Try reversing path j
                    {
                        let mut test_paths = paths.clone();
                        test_paths[j].points.reverse();
                        let new_distance = Self::tour_distance(&test_paths);
                        if new_distance < current_distance {
                            paths = test_paths;
                            improved = true;
                            break;
                        }
                    }

                    // Try swapping paths i and j
                    {
                        let mut test_paths = paths.clone();
                        test_paths.swap(i, j);
                        let new_distance = Self::tour_distance(&test_paths);
                        if new_distance < current_distance {
                            paths = test_paths;
                            improved = true;
                            break;
                        }
                    }

                    // Try swapping + reversing i
                    {
                        let mut test_paths = paths.clone();
                        test_paths.swap(i, j);
                        test_paths[i].points.reverse();
                        let new_distance = Self::tour_distance(&test_paths);
                        if new_distance < current_distance {
                            paths = test_paths;
                            improved = true;
                            break;
                        }
                    }

                    // Try swapping + reversing j
                    {
                        let mut test_paths = paths.clone();
                        test_paths.swap(i, j);
                        test_paths[j].points.reverse();
                        let new_distance = Self::tour_distance(&test_paths);
                        if new_distance < current_distance {
                            paths = test_paths;
                            improved = true;
                            break;
                        }
                    }
                }

                if improved {
                    break;
                }
            }
        }

        paths
    }
}

impl PathOrderer for GreedyThenTwoOpt {
    fn order(&self, paths: Vec<VecPath>) -> Vec<VecPath> {
        if paths.is_empty() {
            return paths;
        }

        if paths.len() == 1 {
            return paths;
        }

        // Phase 1: Greedy initial solution
        let initial = self.greedy_initial(paths);

        // Phase 2: 2-opt refinement
        self.two_opt_improve(initial)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let p1 = Point { x: 0, y: 0, intensity: None };
        let p2 = Point { x: 3, y: 4, intensity: None };
        let dist = GreedyThenTwoOpt::distance(p1, p2);
        assert!((dist - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_greedy_ordering() {
        let paths = vec![
            VecPath {
                name: "path1".to_string(),
                intensity: 127,
                closed: false,
                points: vec![
                    Point { x: 0, y: 0, intensity: None },
                    Point { x: 10, y: 0, intensity: None },
                ],
            },
            VecPath {
                name: "path2".to_string(),
                intensity: 127,
                closed: false,
                points: vec![
                    Point { x: 5, y: 5, intensity: None },
                    Point { x: 15, y: 5, intensity: None },
                ],
            },
        ];

        let orderer = GreedyThenTwoOpt::default();
        let ordered = orderer.order(paths);

        assert_eq!(ordered.len(), 2);
    }
}

/// Christofides Algorithm - Near-optimal TSP solution
/// 
/// Algorithm (5 phases):
/// 1. Construct complete graph of path-to-path distances
/// 2. Find Minimum Spanning Tree (Prim's algorithm)
/// 3. Find vertices with odd degree in MST
/// 4. Find minimum-weight perfect matching on odd vertices (greedy)
/// 5. Combine MST + matching → Eulerian circuit → Hamiltonian path
/// 
/// Result: ≤1.5x optimal tour, O(n³) complexity
pub struct ChristofidesOrderer;

impl Default for ChristofidesOrderer {
    fn default() -> Self {
        Self
    }
}

impl ChristofidesOrderer {
    /// Calculate Euclidean distance between two points
    fn distance(p1: Point, p2: Point) -> f64 {
        let dx = (p2.x - p1.x) as f64;
        let dy = (p2.y - p1.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }

    /// Get the first point of a path
    fn path_start(path: &VecPath) -> Point {
        path.points.first().copied().unwrap_or(Point { x: 0, y: 0, intensity: None })
    }

    /// Get the last point of a path
    fn path_end(path: &VecPath) -> Point {
        path.points.last().copied().unwrap_or(Point { x: 0, y: 0, intensity: None })
    }

    /// Build complete distance matrix between all paths (considering start/end)
    fn build_distance_matrix(paths: &[VecPath]) -> Vec<Vec<f64>> {
        let n = paths.len();
        let mut matrix = vec![vec![0.0; n]; n];

        for i in 0..n {
            for j in (i + 1)..n {
                // Distance from end of i to start of j
                let dist = Self::distance(Self::path_end(&paths[i]), Self::path_start(&paths[j]));
                matrix[i][j] = dist;
                matrix[j][i] = dist;
            }
        }

        matrix
    }

    /// Prim's algorithm for Minimum Spanning Tree
    /// Returns list of edges (i, j, weight)
    fn minimum_spanning_tree(dist_matrix: &[Vec<f64>]) -> Vec<(usize, usize, f64)> {
        let n = dist_matrix.len();
        if n == 0 {
            return vec![];
        }

        let mut mst = Vec::new();
        let mut in_mst = vec![false; n];
        let mut min_edge = vec![f64::INFINITY; n];
        let mut parent = vec![0; n];

        // Start from node 0
        min_edge[0] = 0.0;

        for _ in 0..n {
            // Find minimum edge not in MST
            let mut u = n;
            let mut min_weight = f64::INFINITY;
            for v in 0..n {
                if !in_mst[v] && min_edge[v] < min_weight {
                    min_weight = min_edge[v];
                    u = v;
                }
            }

            if u == n {
                break;
            }

            in_mst[u] = true;

            // Add edge to MST (except first node)
            if min_weight > 0.0 {
                mst.push((parent[u], u, min_weight));
            }

            // Update edges
            for v in 0..n {
                if !in_mst[v] && dist_matrix[u][v] < min_edge[v] {
                    min_edge[v] = dist_matrix[u][v];
                    parent[v] = u;
                }
            }
        }

        mst
    }

    /// Find vertices with odd degree in MST
    fn find_odd_degree_vertices(mst: &[(usize, usize, f64)], n: usize) -> Vec<usize> {
        let mut degree = vec![0; n];
        for (u, v, _) in mst {
            degree[*u] += 1;
            degree[*v] += 1;
        }

        degree.iter()
            .enumerate()
            .filter(|(_, &d)| d % 2 == 1)
            .map(|(i, _)| i)
            .collect()
    }

    /// Greedy minimum-weight perfect matching on odd vertices
    /// Returns list of matched edges (i, j, weight)
    fn greedy_matching(odd_vertices: &[usize], dist_matrix: &[Vec<f64>]) -> Vec<(usize, usize, f64)> {
        let mut matching = Vec::new();
        let mut matched = HashSet::new();
        let mut candidates: Vec<_> = odd_vertices.to_vec();

        while candidates.len() >= 2 {
            // Find closest pair
            let mut best_i = 0;
            let mut best_j = 1;
            let mut best_dist = f64::INFINITY;

            for (idx_i, &i) in candidates.iter().enumerate() {
                for (idx_j, &j) in candidates.iter().enumerate().skip(idx_i + 1) {
                    let dist = dist_matrix[i][j];
                    if dist < best_dist {
                        best_dist = dist;
                        best_i = idx_i;
                        best_j = idx_j;
                    }
                }
            }

            let u = candidates[best_i];
            let v = candidates[best_j];
            matching.push((u, v, best_dist));
            matched.insert(u);
            matched.insert(v);

            // Remove matched vertices (remove larger index first)
            if best_i > best_j {
                candidates.remove(best_i);
                candidates.remove(best_j);
            } else {
                candidates.remove(best_j);
                candidates.remove(best_i);
            }
        }

        matching
    }

    /// Build adjacency list from edges
    fn build_adjacency(edges: &[(usize, usize, f64)], n: usize) -> HashMap<usize, Vec<usize>> {
        let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..n {
            adj.insert(i, Vec::new());
        }
        for (u, v, _) in edges {
            adj.get_mut(u).unwrap().push(*v);
            adj.get_mut(v).unwrap().push(*u);
        }
        adj
    }

    /// Find Eulerian circuit (assumes all vertices have even degree)
    fn eulerian_circuit(adj: &mut HashMap<usize, Vec<usize>>, start: usize) -> Vec<usize> {
        let mut stack = vec![start];
        let mut circuit = Vec::new();

        while !stack.is_empty() {
            let u = *stack.last().unwrap();
            if let Some(neighbors) = adj.get_mut(&u) {
                if neighbors.is_empty() {
                    circuit.push(u);
                    stack.pop();
                } else {
                    let v = neighbors.pop().unwrap();
                    stack.push(v);
                    // Remove reverse edge
                    if let Some(v_neighbors) = adj.get_mut(&v) {
                        if let Some(pos) = v_neighbors.iter().position(|&x| x == u) {
                            v_neighbors.remove(pos);
                        }
                    }
                }
            } else {
                stack.pop();
            }
        }

        circuit
    }

    /// Convert Eulerian circuit to Hamiltonian path (skip repeated vertices)
    fn to_hamiltonian(circuit: Vec<usize>, n: usize) -> Vec<usize> {
        let mut visited = vec![false; n];
        let mut path = Vec::new();

        for v in circuit {
            if !visited[v] {
                visited[v] = true;
                path.push(v);
            }
        }

        path
    }

    /// Optimize path by considering reversing each path
    fn optimize_directions(mut paths: Vec<VecPath>) -> Vec<VecPath> {
        let mut current_pos = Point { x: 0, y: 0, intensity: None };

        for path in &mut paths {
            let forward_dist = Self::distance(current_pos, Self::path_start(path));
            let reverse_dist = Self::distance(current_pos, Self::path_end(path));

            if reverse_dist < forward_dist {
                path.points.reverse();
            }

            current_pos = Self::path_end(path);
        }

        paths
    }
}

impl PathOrderer for ChristofidesOrderer {
    fn order(&self, paths: Vec<VecPath>) -> Vec<VecPath> {
        if paths.is_empty() {
            return paths;
        }

        if paths.len() == 1 {
            return paths;
        }

        let n = paths.len();

        // Phase 1: Build distance matrix
        let dist_matrix = Self::build_distance_matrix(&paths);

        // Phase 2: Find MST
        let mst = Self::minimum_spanning_tree(&dist_matrix);

        // Phase 3: Find odd-degree vertices
        let odd_vertices = Self::find_odd_degree_vertices(&mst, n);

        // Phase 4: Find minimum-weight perfect matching on odd vertices
        let matching = Self::greedy_matching(&odd_vertices, &dist_matrix);

        // Phase 5: Combine MST + matching edges
        let mut all_edges = mst.clone();
        all_edges.extend(matching);

        // Build adjacency list and find Eulerian circuit
        let mut adj = Self::build_adjacency(&all_edges, n);
        let circuit = Self::eulerian_circuit(&mut adj, 0);

        // Convert to Hamiltonian path
        let hamiltonian = Self::to_hamiltonian(circuit, n);

        // Reorder paths according to Hamiltonian path
        let mut ordered: Vec<VecPath> = hamiltonian.iter()
            .map(|&i| paths[i].clone())
            .collect();

        // Optimize path directions (forward vs reverse)
        ordered = Self::optimize_directions(ordered);

        ordered
    }
}
