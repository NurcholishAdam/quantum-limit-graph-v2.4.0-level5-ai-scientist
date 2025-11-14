// Quantum-walk exploration with weight-driven selection
use crate::model::{WorldModelGraph, GraphNode};
use anyhow::Result;
use ndarray::{Array1, Array2};
use num_complex::Complex64;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

/// Quantum walk explorer for graph traversal
pub struct QuantumWalkExplorer {
    max_steps: usize,
    measurement_probability_threshold: f64,
}

impl QuantumWalkExplorer {
    pub fn new(max_steps: usize) -> Self {
        Self {
            max_steps,
            measurement_probability_threshold: 0.1,
        }
    }

    /// Perform quantum walk exploration starting from seed nodes
    pub fn explore(
        &self,
        graph: &WorldModelGraph,
        seed_nodes: &[Uuid],
    ) -> Result<ExplorationResult> {
        if seed_nodes.is_empty() {
            return Ok(ExplorationResult::default());
        }

        // Initialize quantum state
        let node_ids: Vec<Uuid> = graph.nodes.keys().cloned().collect();
        let n = node_ids.len();
        
        if n == 0 {
            return Ok(ExplorationResult::default());
        }

        let mut state = self.initialize_state(&node_ids, seed_nodes);
        let hamiltonian = self.build_hamiltonian(graph, &node_ids);

        // Perform quantum walk steps
        let mut visited_nodes = Vec::new();
        let mut path_weights = HashMap::new();

        for step in 0..self.max_steps {
            // Evolve state
            state = self.evolve_state(&state, &hamiltonian);

            // Measure with probability
            if let Some(measured_node) = self.measure_state(&state, &node_ids) {
                visited_nodes.push(measured_node);
                *path_weights.entry(measured_node).or_insert(0.0) += 1.0;
            }

            // Check for convergence
            if self.has_converged(&state) {
                break;
            }
        }

        // Compute coverage metrics
        let coverage = visited_nodes.len() as f64 / n as f64;
        let unique_visits = path_weights.len();

        Ok(ExplorationResult {
            visited_nodes,
            path_weights,
            coverage,
            unique_visits,
            total_steps: self.max_steps,
        })
    }

    /// Initialize quantum state with superposition over seed nodes
    fn initialize_state(&self, node_ids: &[Uuid], seed_nodes: &[Uuid]) -> Array1<Complex64> {
        let n = node_ids.len();
        let mut state = Array1::zeros(n);

        let amplitude = Complex64::new(1.0 / (seed_nodes.len() as f64).sqrt(), 0.0);

        for seed in seed_nodes {
            if let Some(idx) = node_ids.iter().position(|id| id == seed) {
                state[idx] = amplitude;
            }
        }

        state
    }

    /// Build Hamiltonian matrix from graph structure
    fn build_hamiltonian(&self, graph: &WorldModelGraph, node_ids: &[Uuid]) -> Array2<Complex64> {
        let n = node_ids.len();
        let mut hamiltonian = Array2::zeros((n, n));

        // Build adjacency matrix weighted by edge weights
        for edge in &graph.edges {
            if let (Some(i), Some(j)) = (
                node_ids.iter().position(|id| id == &edge.source),
                node_ids.iter().position(|id| id == &edge.target),
            ) {
                let weight = Complex64::new(edge.weight, 0.0);
                hamiltonian[[i, j]] = weight;
                hamiltonian[[j, i]] = weight; // Symmetric for undirected
            }
        }

        // Add diagonal terms (on-site energy)
        for i in 0..n {
            let degree = hamiltonian.row(i).iter().map(|c| c.norm()).sum::<f64>();
            hamiltonian[[i, i]] = Complex64::new(-degree, 0.0);
        }

        hamiltonian
    }

    /// Evolve quantum state by one time step
    fn evolve_state(&self, state: &Array1<Complex64>, hamiltonian: &Array2<Complex64>) -> Array1<Complex64> {
        // Simple evolution: |ψ(t+dt)⟩ = exp(-iHdt)|ψ(t)⟩
        // Approximation: exp(-iHdt) ≈ I - iHdt for small dt
        let dt = 0.1;
        let i = Complex64::new(0.0, 1.0);

        let mut new_state = state.clone();
        let evolution = hamiltonian.dot(state);
        
        for j in 0..state.len() {
            new_state[j] = state[j] - i * dt * evolution[j];
        }

        // Normalize
        let norm = new_state.iter().map(|c| c.norm_sqr()).sum::<f64>().sqrt();
        if norm > 0.0 {
            new_state.mapv_inplace(|c| c / norm);
        }

        new_state
    }

    /// Measure quantum state to collapse to a node
    fn measure_state(&self, state: &Array1<Complex64>, node_ids: &[Uuid]) -> Option<Uuid> {
        let mut rng = rand::thread_rng();
        
        // Compute probability distribution
        let probabilities: Vec<f64> = state.iter().map(|c| c.norm_sqr()).collect();
        
        // Sample from distribution
        let total: f64 = probabilities.iter().sum();
        if total == 0.0 {
            return None;
        }

        let mut cumulative = 0.0;
        let sample = rng.gen::<f64>() * total;

        for (i, &prob) in probabilities.iter().enumerate() {
            cumulative += prob;
            if sample <= cumulative && prob > self.measurement_probability_threshold {
                return Some(node_ids[i]);
            }
        }

        None
    }

    /// Check if quantum state has converged
    fn has_converged(&self, state: &Array1<Complex64>) -> bool {
        // Check if state is localized (high probability on few nodes)
        let probabilities: Vec<f64> = state.iter().map(|c| c.norm_sqr()).collect();
        let max_prob = probabilities.iter().cloned().fold(0.0, f64::max);
        
        max_prob > 0.8 // Converged if >80% probability on one node
    }

    /// Select next exploration target based on quantum walk results
    pub fn select_exploration_target(
        &self,
        result: &ExplorationResult,
        graph: &WorldModelGraph,
    ) -> Option<Uuid> {
        // Select node with highest path weight that hasn't been fully explored
        result
            .path_weights
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(id, _)| *id)
    }

    /// Compute exploration heatmap for visualization
    pub fn compute_heatmap(&self, result: &ExplorationResult) -> HashMap<Uuid, f64> {
        let max_weight = result
            .path_weights
            .values()
            .cloned()
            .fold(0.0, f64::max);

        if max_weight == 0.0 {
            return HashMap::new();
        }

        result
            .path_weights
            .iter()
            .map(|(id, weight)| (*id, weight / max_weight))
            .collect()
    }
}

/// Result of quantum walk exploration
#[derive(Debug, Clone, Default)]
pub struct ExplorationResult {
    pub visited_nodes: Vec<Uuid>,
    pub path_weights: HashMap<Uuid, f64>,
    pub coverage: f64,
    pub unique_visits: usize,
    pub total_steps: usize,
}

impl ExplorationResult {
    /// Get top-k most visited nodes
    pub fn get_top_nodes(&self, k: usize) -> Vec<(Uuid, f64)> {
        let mut sorted: Vec<_> = self.path_weights.iter().map(|(id, w)| (*id, *w)).collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        sorted.into_iter().take(k).collect()
    }

    /// Check if node was visited
    pub fn was_visited(&self, node_id: &Uuid) -> bool {
        self.path_weights.contains_key(node_id)
    }

    /// Get visit frequency for a node
    pub fn get_visit_frequency(&self, node_id: &Uuid) -> f64 {
        self.path_weights.get(node_id).cloned().unwrap_or(0.0)
    }
}

/// Quantum walk strategy selector
pub enum WalkStrategy {
    Continuous,      // Standard continuous-time quantum walk
    Discrete,        // Discrete-time quantum walk
    Adaptive,        // Adaptive step size based on graph structure
}

impl WalkStrategy {
    pub fn create_explorer(&self, max_steps: usize) -> QuantumWalkExplorer {
        // For now, all strategies use the same explorer
        // In full implementation, different strategies would have different evolution operators
        QuantumWalkExplorer::new(max_steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{NodeType, GraphEdge, EdgeType};

    #[test]
    fn test_quantum_walk_initialization() {
        let explorer = QuantumWalkExplorer::new(10);
        let node_ids = vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
        let seed_nodes = vec![node_ids[0]];

        let state = explorer.initialize_state(&node_ids, &seed_nodes);
        
        // Check normalization
        let norm_sqr: f64 = state.iter().map(|c| c.norm_sqr()).sum();
        assert!((norm_sqr - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_quantum_walk_exploration() {
        let mut graph = WorldModelGraph::new();
        
        // Create simple graph
        let node1 = GraphNode::new(NodeType::Hypothesis, "H1".to_string());
        let node2 = GraphNode::new(NodeType::Hypothesis, "H2".to_string());
        let node3 = GraphNode::new(NodeType::Evidence, "E1".to_string());
        
        let id1 = graph.add_node(node1);
        let id2 = graph.add_node(node2);
        let id3 = graph.add_node(node3);
        
        graph.add_edge(GraphEdge::new(id1, id2, EdgeType::Supports, 0.8));
        graph.add_edge(GraphEdge::new(id2, id3, EdgeType::Supports, 0.9));

        let explorer = QuantumWalkExplorer::new(20);
        let result = explorer.explore(&graph, &[id1]).unwrap();

        assert!(result.coverage > 0.0);
        assert!(!result.visited_nodes.is_empty());
    }

    #[test]
    fn test_exploration_result() {
        let mut result = ExplorationResult::default();
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        result.path_weights.insert(id1, 5.0);
        result.path_weights.insert(id2, 3.0);

        let top = result.get_top_nodes(1);
        assert_eq!(top.len(), 1);
        assert_eq!(top[0].0, id1);
    }
}
