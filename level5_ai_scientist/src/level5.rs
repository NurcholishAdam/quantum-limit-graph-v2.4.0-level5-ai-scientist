// Level 5 validation rails implementation
use crate::model::{Level5Validation, GraphNode, WorldModelGraph};
use crate::io::ValidationPolicy;
use anyhow::Result;
use rand::Rng;
use statrs::distribution::{Normal, ContinuousCDF};

/// Validation rails engine for Level 5 quality gates
pub struct ValidationRails {
    policy: ValidationPolicy,
}

impl ValidationRails {
    pub fn new(policy: ValidationPolicy) -> Self {
        Self { policy }
    }

    /// Run full validation suite on a node
    pub fn validate_node(
        &self,
        node: &GraphNode,
        graph: &WorldModelGraph,
    ) -> Result<Level5Validation> {
        let mut validation = Level5Validation::new();

        // Bootstrap stability check
        validation.bootstrap_stability = self.compute_bootstrap_stability(node, graph)?;

        // Holdout consistency check
        validation.holdout_consistency = self.compute_holdout_consistency(node, graph)?;

        // Sensitivity analysis
        if self.policy.enable_sensitivity_analysis {
            validation.sensitivity_score = self.compute_sensitivity(node, graph)?;
        } else {
            validation.sensitivity_score = 1.0;
        }

        // Count supporting evidence
        validation.required_supports = graph.get_supporting_evidence(&node.id).len();

        // Compute contradiction rate
        validation.contradiction_rate = self.compute_contradiction_rate(node, graph);

        validation.validation_timestamp = chrono::Utc::now();

        Ok(validation)
    }

    /// Bootstrap stability: resample-based confidence estimation
    fn compute_bootstrap_stability(
        &self,
        node: &GraphNode,
        graph: &WorldModelGraph,
    ) -> Result<f64> {
        let evidence = graph.get_supporting_evidence(&node.id);
        
        if evidence.is_empty() {
            return Ok(0.0);
        }

        let mut rng = rand::thread_rng();
        let mut bootstrap_scores = Vec::new();

        // Perform bootstrap resampling
        for _ in 0..self.policy.bootstrap_samples {
            let mut sample_score = 0.0;
            let sample_size = evidence.len();

            // Resample with replacement
            for _ in 0..sample_size {
                let idx = rng.gen_range(0..evidence.len());
                sample_score += evidence[idx].claim_strength.confidence;
            }

            sample_score /= sample_size as f64;
            bootstrap_scores.push(sample_score);
        }

        // Compute stability as inverse of coefficient of variation
        let mean = bootstrap_scores.iter().sum::<f64>() / bootstrap_scores.len() as f64;
        let variance = bootstrap_scores
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / bootstrap_scores.len() as f64;
        let std_dev = variance.sqrt();

        if mean == 0.0 {
            Ok(0.0)
        } else {
            let cv = std_dev / mean;
            Ok((1.0 - cv).max(0.0).min(1.0))
        }
    }

    /// Holdout consistency: train/test split agreement
    fn compute_holdout_consistency(
        &self,
        node: &GraphNode,
        graph: &WorldModelGraph,
    ) -> Result<f64> {
        let evidence = graph.get_supporting_evidence(&node.id);
        
        if evidence.len() < 4 {
            // Not enough evidence for meaningful split
            return Ok(if evidence.len() >= 2 { 0.7 } else { 0.0 });
        }

        // Split evidence into train (70%) and test (30%)
        let split_point = (evidence.len() as f64 * 0.7) as usize;
        let train_evidence = &evidence[..split_point];
        let test_evidence = &evidence[split_point..];

        // Compute confidence on train set
        let train_confidence = train_evidence
            .iter()
            .map(|e| e.claim_strength.confidence)
            .sum::<f64>()
            / train_evidence.len() as f64;

        // Compute confidence on test set
        let test_confidence = test_evidence
            .iter()
            .map(|e| e.claim_strength.confidence)
            .sum::<f64>()
            / test_evidence.len() as f64;

        // Consistency is inverse of relative difference
        let diff = (train_confidence - test_confidence).abs();
        let consistency = 1.0 - (diff / train_confidence.max(0.01));

        Ok(consistency.max(0.0).min(1.0))
    }

    /// Sensitivity analysis: robustness to parameter perturbation
    fn compute_sensitivity(
        &self,
        node: &GraphNode,
        graph: &WorldModelGraph,
    ) -> Result<f64> {
        let evidence = graph.get_supporting_evidence(&node.id);
        
        if evidence.is_empty() {
            return Ok(0.0);
        }

        let base_confidence = node.claim_strength.confidence;
        let mut perturbation_scores = Vec::new();

        // Test robustness by perturbing evidence weights
        let perturbation_levels = vec![0.9, 0.95, 1.05, 1.1];
        
        for &factor in &perturbation_levels {
            let perturbed_confidence = evidence
                .iter()
                .map(|e| e.claim_strength.confidence * factor)
                .sum::<f64>()
                / evidence.len() as f64;

            let relative_change = ((perturbed_confidence - base_confidence) / base_confidence).abs();
            perturbation_scores.push(relative_change);
        }

        // Sensitivity is inverse of average relative change
        let avg_change = perturbation_scores.iter().sum::<f64>() / perturbation_scores.len() as f64;
        let sensitivity = 1.0 - avg_change.min(1.0);

        Ok(sensitivity.max(0.0).min(1.0))
    }

    /// Compute contradiction rate for a node
    fn compute_contradiction_rate(&self, node: &GraphNode, graph: &WorldModelGraph) -> f64 {
        let neighbors = graph.get_neighbors(&node.id);
        if neighbors.is_empty() {
            return 0.0;
        }

        let contradictions = graph.get_contradictions(&node.id);
        contradictions.len() as f64 / neighbors.len() as f64
    }

    /// Check if node passes all validation gates
    pub fn passes_gates(&self, validation: &Level5Validation) -> bool {
        validation.bootstrap_stability >= self.policy.min_bootstrap_stability
            && validation.holdout_consistency >= self.policy.min_holdout_consistency
            && validation.required_supports >= self.policy.min_required_supports
            && (validation.contradiction_rate <= self.policy.max_contradiction_rate
                || self.has_resolution_task(validation))
    }

    /// Check if resolution task exists for contradictions
    fn has_resolution_task(&self, _validation: &Level5Validation) -> bool {
        // Placeholder: in full implementation, check graph for ResolutionTask nodes
        false
    }

    /// Generate resolution task for contradictions
    pub fn generate_resolution_task(
        &self,
        node: &GraphNode,
        graph: &WorldModelGraph,
    ) -> Option<String> {
        let contradictions = graph.get_contradictions(&node.id);
        
        if contradictions.is_empty() {
            return None;
        }

        let task = format!(
            "Resolve {} contradictions for node '{}'. Contradicting nodes: {}",
            contradictions.len(),
            node.content,
            contradictions
                .iter()
                .map(|n| format!("'{}'", n.content))
                .collect::<Vec<_>>()
                .join(", ")
        );

        Some(task)
    }

    /// Enumerate failure modes for a node
    pub fn enumerate_failure_modes(&self, node: &GraphNode, graph: &WorldModelGraph) -> Vec<String> {
        let mut modes = Vec::new();

        let evidence = graph.get_supporting_evidence(&node.id);
        if evidence.len() < self.policy.min_required_supports {
            modes.push(format!(
                "Insufficient evidence: {} < required {}",
                evidence.len(),
                self.policy.min_required_supports
            ));
        }

        if node.claim_strength.confidence < 0.5 {
            modes.push("Low confidence score".to_string());
        }

        let contradictions = graph.get_contradictions(&node.id);
        if !contradictions.is_empty() {
            modes.push(format!("Unresolved contradictions: {}", contradictions.len()));
        }

        if node.risk_flags.has_any() {
            modes.push("Risk flags present".to_string());
        }

        modes
    }

    /// Compute global validation metrics for entire graph
    pub fn compute_global_metrics(&self, graph: &WorldModelGraph) -> GlobalValidationMetrics {
        let total_nodes = graph.nodes.len();
        let validated_nodes = graph
            .nodes
            .values()
            .filter(|n| n.level5_validation.passes_gates())
            .count();

        let avg_bootstrap = if total_nodes > 0 {
            graph
                .nodes
                .values()
                .map(|n| n.level5_validation.bootstrap_stability)
                .sum::<f64>()
                / total_nodes as f64
        } else {
            0.0
        };

        let avg_holdout = if total_nodes > 0 {
            graph
                .nodes
                .values()
                .map(|n| n.level5_validation.holdout_consistency)
                .sum::<f64>()
                / total_nodes as f64
        } else {
            0.0
        };

        GlobalValidationMetrics {
            total_nodes,
            validated_nodes,
            validation_rate: if total_nodes > 0 {
                validated_nodes as f64 / total_nodes as f64
            } else {
                0.0
            },
            avg_bootstrap_stability: avg_bootstrap,
            avg_holdout_consistency: avg_holdout,
            global_contradiction_rate: graph.compute_global_contradiction_rate(),
        }
    }
}

/// Global validation metrics
#[derive(Debug, Clone)]
pub struct GlobalValidationMetrics {
    pub total_nodes: usize,
    pub validated_nodes: usize,
    pub validation_rate: f64,
    pub avg_bootstrap_stability: f64,
    pub avg_holdout_consistency: f64,
    pub global_contradiction_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{NodeType, GraphEdge, EdgeType};
    use uuid::Uuid;

    #[test]
    fn test_validation_rails() {
        let policy = ValidationPolicy::default();
        let rails = ValidationRails::new(policy);

        let mut graph = WorldModelGraph::new();
        let node = GraphNode::new(NodeType::Hypothesis, "Test".to_string());
        let node_id = graph.add_node(node.clone());

        // Add supporting evidence
        for i in 0..3 {
            let evidence = GraphNode::new(NodeType::Evidence, format!("Evidence {}", i));
            let evidence_id = graph.add_node(evidence);
            let edge = GraphEdge::new(evidence_id, node_id, EdgeType::Supports, 0.8);
            graph.add_edge(edge);
        }

        let validation = rails.validate_node(graph.get_node(&node_id).unwrap(), &graph).unwrap();
        assert!(validation.required_supports >= 2);
    }

    #[test]
    fn test_failure_modes() {
        let policy = ValidationPolicy::default();
        let rails = ValidationRails::new(policy);

        let graph = WorldModelGraph::new();
        let node = GraphNode::new(NodeType::Hypothesis, "Test".to_string());

        let modes = rails.enumerate_failure_modes(&node, &graph);
        assert!(!modes.is_empty());
    }
}
