// Core graph types for v2.4.0 + Level 5 AI Scientist
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Node types in the world model graph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NodeType {
    Finding,
    Hypothesis,
    Evidence,
    Contradiction,
    ResolutionTask,
}

/// Edge types representing relationships
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EdgeType {
    Supports,
    Contradicts,
    DerivedFrom,
    Resolves,
    TemporalNext,
}

/// Level 5 validation metrics for a node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level5Validation {
    pub bootstrap_stability: f64,      // Resample-based confidence [0,1]
    pub holdout_consistency: f64,      // Train/test agreement [0,1]
    pub sensitivity_score: f64,        // Robustness to perturbation [0,1]
    pub required_supports: usize,      // Evidence count
    pub contradiction_rate: f64,       // Conflict ratio [0,1]
    pub validation_timestamp: DateTime<Utc>,
}

impl Level5Validation {
    pub fn new() -> Self {
        Self {
            bootstrap_stability: 0.0,
            holdout_consistency: 0.0,
            sensitivity_score: 0.0,
            required_supports: 0,
            contradiction_rate: 0.0,
            validation_timestamp: Utc::now(),
        }
    }

    /// Check if validation passes Level 5 gates
    pub fn passes_gates(&self) -> bool {
        self.bootstrap_stability >= 0.8
            && self.holdout_consistency >= 0.75
            && self.required_supports >= 2
            && (self.contradiction_rate <= 0.2 || self.has_resolution_task())
    }

    fn has_resolution_task(&self) -> bool {
        // Placeholder: check if resolution task exists
        // In full implementation, query graph for ResolutionTask nodes
        false
    }
}

impl Default for Level5Validation {
    fn default() -> Self {
        Self::new()
    }
}

/// Claim strength assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimStrength {
    pub confidence: f64,               // Overall confidence [0,1]
    pub evidence_weight: f64,          // Aggregated evidence strength
    pub coherence_score: f64,          // Internal consistency
    pub novelty_score: f64,            // Originality measure
}

impl ClaimStrength {
    pub fn new() -> Self {
        Self {
            confidence: 0.5,
            evidence_weight: 0.0,
            coherence_score: 1.0,
            novelty_score: 0.5,
        }
    }
}

impl Default for ClaimStrength {
    fn default() -> Self {
        Self::new()
    }
}

/// Risk flags for governance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFlags {
    pub high_uncertainty: bool,
    pub conflicting_evidence: bool,
    pub insufficient_validation: bool,
    pub ethical_concerns: bool,
    pub reproducibility_issues: bool,
}

impl RiskFlags {
    pub fn new() -> Self {
        Self {
            high_uncertainty: false,
            conflicting_evidence: false,
            insufficient_validation: false,
            ethical_concerns: false,
            reproducibility_issues: false,
        }
    }

    pub fn has_any(&self) -> bool {
        self.high_uncertainty
            || self.conflicting_evidence
            || self.insufficient_validation
            || self.ethical_concerns
            || self.reproducibility_issues
    }
}

impl Default for RiskFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// Node in the world model graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: Uuid,
    pub node_type: NodeType,
    pub content: String,
    pub metadata: HashMap<String, String>,
    pub level5_validation: Level5Validation,
    pub claim_strength: ClaimStrength,
    pub risk_flags: RiskFlags,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl GraphNode {
    pub fn new(node_type: NodeType, content: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            node_type,
            content,
            metadata: HashMap::new(),
            level5_validation: Level5Validation::new(),
            claim_strength: ClaimStrength::new(),
            risk_flags: RiskFlags::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_validation(&mut self, validation: Level5Validation) {
        self.level5_validation = validation;
        self.updated_at = Utc::now();
    }

    pub fn update_claim_strength(&mut self, strength: ClaimStrength) {
        self.claim_strength = strength;
        self.updated_at = Utc::now();
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.updated_at = Utc::now();
    }
}

/// Edge in the world model graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: Uuid,
    pub source: Uuid,
    pub target: Uuid,
    pub edge_type: EdgeType,
    pub weight: f64,                   // Support strength or confidence
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

impl GraphEdge {
    pub fn new(source: Uuid, target: Uuid, edge_type: EdgeType, weight: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            source,
            target,
            edge_type,
            weight,
            metadata: HashMap::new(),
            created_at: Utc::now(),
        }
    }
}

/// World model graph structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldModelGraph {
    pub nodes: HashMap<Uuid, GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub pipeline_version: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl WorldModelGraph {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            pipeline_version: "2.4.0-NSN".to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_node(&mut self, node: GraphNode) -> Uuid {
        let id = node.id;
        self.nodes.insert(id, node);
        self.updated_at = Utc::now();
        id
    }

    pub fn add_edge(&mut self, edge: GraphEdge) {
        self.edges.push(edge);
        self.updated_at = Utc::now();
    }

    pub fn get_node(&self, id: &Uuid) -> Option<&GraphNode> {
        self.nodes.get(id)
    }

    pub fn get_node_mut(&mut self, id: &Uuid) -> Option<&mut GraphNode> {
        self.nodes.get_mut(id)
    }

    pub fn get_neighbors(&self, node_id: &Uuid) -> Vec<&GraphNode> {
        self.edges
            .iter()
            .filter(|e| &e.source == node_id)
            .filter_map(|e| self.nodes.get(&e.target))
            .collect()
    }

    pub fn get_supporting_evidence(&self, node_id: &Uuid) -> Vec<&GraphNode> {
        self.edges
            .iter()
            .filter(|e| &e.target == node_id && matches!(e.edge_type, EdgeType::Supports))
            .filter_map(|e| self.nodes.get(&e.source))
            .collect()
    }

    pub fn get_contradictions(&self, node_id: &Uuid) -> Vec<&GraphNode> {
        self.edges
            .iter()
            .filter(|e| {
                (&e.source == node_id || &e.target == node_id)
                    && matches!(e.edge_type, EdgeType::Contradicts)
            })
            .filter_map(|e| {
                if &e.source == node_id {
                    self.nodes.get(&e.target)
                } else {
                    self.nodes.get(&e.source)
                }
            })
            .collect()
    }

    pub fn compute_global_contradiction_rate(&self) -> f64 {
        if self.nodes.is_empty() {
            return 0.0;
        }

        let contradiction_count = self
            .edges
            .iter()
            .filter(|e| matches!(e.edge_type, EdgeType::Contradicts))
            .count();

        let total_edges = self.edges.len();
        if total_edges == 0 {
            0.0
        } else {
            contradiction_count as f64 / total_edges as f64
        }
    }

    pub fn get_nodes_by_type(&self, node_type: &NodeType) -> Vec<&GraphNode> {
        self.nodes
            .values()
            .filter(|n| &n.node_type == node_type)
            .collect()
    }
}

impl Default for WorldModelGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = GraphNode::new(NodeType::Hypothesis, "Test hypothesis".to_string());
        assert_eq!(node.node_type, NodeType::Hypothesis);
        assert_eq!(node.content, "Test hypothesis");
    }

    #[test]
    fn test_validation_gates() {
        let mut validation = Level5Validation::new();
        assert!(!validation.passes_gates());

        validation.bootstrap_stability = 0.85;
        validation.holdout_consistency = 0.80;
        validation.required_supports = 3;
        validation.contradiction_rate = 0.15;
        assert!(validation.passes_gates());
    }

    #[test]
    fn test_graph_operations() {
        let mut graph = WorldModelGraph::new();
        
        let node1 = GraphNode::new(NodeType::Hypothesis, "H1".to_string());
        let node2 = GraphNode::new(NodeType::Evidence, "E1".to_string());
        
        let id1 = graph.add_node(node1.clone());
        let id2 = graph.add_node(node2.clone());
        
        let edge = GraphEdge::new(id2, id1, EdgeType::Supports, 0.9);
        graph.add_edge(edge);
        
        let evidence = graph.get_supporting_evidence(&id1);
        assert_eq!(evidence.len(), 1);
    }
}
