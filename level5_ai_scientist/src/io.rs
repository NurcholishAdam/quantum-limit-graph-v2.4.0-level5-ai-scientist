// I/O and policies for Level 5 AI Scientist
use crate::model::{WorldModelGraph, GraphNode, GraphEdge};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Input configuration for research cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchInput {
    pub question: String,
    pub context: Vec<String>,
    pub max_iterations: usize,
    pub convergence_threshold: f64,
    pub enable_quantum_walk: bool,
    pub validation_policy: ValidationPolicy,
}

impl Default for ResearchInput {
    fn default() -> Self {
        Self {
            question: String::new(),
            context: Vec::new(),
            max_iterations: 10,
            convergence_threshold: 0.95,
            enable_quantum_walk: true,
            validation_policy: ValidationPolicy::default(),
        }
    }
}

/// Validation policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationPolicy {
    pub min_bootstrap_stability: f64,
    pub min_holdout_consistency: f64,
    pub min_required_supports: usize,
    pub max_contradiction_rate: f64,
    pub enable_sensitivity_analysis: bool,
    pub bootstrap_samples: usize,
}

impl Default for ValidationPolicy {
    fn default() -> Self {
        Self {
            min_bootstrap_stability: 0.8,
            min_holdout_consistency: 0.75,
            min_required_supports: 2,
            max_contradiction_rate: 0.2,
            enable_sensitivity_analysis: true,
            bootstrap_samples: 1000,
        }
    }
}

/// Output from research cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchOutput {
    pub question: String,
    pub findings: Vec<Finding>,
    pub world_model: WorldModelGraph,
    pub iterations_completed: usize,
    pub convergence_achieved: bool,
    pub global_metrics: GlobalMetrics,
    pub provenance: ProvenanceInfo,
}

/// Individual finding with validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub content: String,
    pub confidence: f64,
    pub evidence_count: usize,
    pub validation_passed: bool,
    pub risk_flags: Vec<String>,
}

/// Global metrics for the research cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalMetrics {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub hypothesis_count: usize,
    pub validated_findings: usize,
    pub contradiction_rate: f64,
    pub average_confidence: f64,
    pub quantum_walk_coverage: f64,
}

/// Provenance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceInfo {
    pub pipeline_version: String,
    pub git_commit: Option<String>,
    pub execution_timestamp: String,
    pub artifact_uris: Vec<String>,
}

/// I/O manager for reading/writing research data
pub struct IOManager {
    output_dir: String,
}

impl IOManager {
    pub fn new(output_dir: String) -> Self {
        Self { output_dir }
    }

    /// Load research input from JSON file
    pub fn load_input<P: AsRef<Path>>(&self, path: P) -> Result<ResearchInput> {
        let content = fs::read_to_string(path.as_ref())
            .context("Failed to read input file")?;
        let input: ResearchInput = serde_json::from_str(&content)
            .context("Failed to parse input JSON")?;
        Ok(input)
    }

    /// Load research input from YAML file
    pub fn load_input_yaml<P: AsRef<Path>>(&self, path: P) -> Result<ResearchInput> {
        let content = fs::read_to_string(path.as_ref())
            .context("Failed to read input file")?;
        let input: ResearchInput = serde_yaml::from_str(&content)
            .context("Failed to parse input YAML")?;
        Ok(input)
    }

    /// Save research output to JSON file
    pub fn save_output<P: AsRef<Path>>(&self, output: &ResearchOutput, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(output)
            .context("Failed to serialize output")?;
        fs::write(path.as_ref(), json)
            .context("Failed to write output file")?;
        Ok(())
    }

    /// Save world model graph separately
    pub fn save_graph<P: AsRef<Path>>(&self, graph: &WorldModelGraph, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(graph)
            .context("Failed to serialize graph")?;
        fs::write(path.as_ref(), json)
            .context("Failed to write graph file")?;
        Ok(())
    }

    /// Load world model graph from file
    pub fn load_graph<P: AsRef<Path>>(&self, path: P) -> Result<WorldModelGraph> {
        let content = fs::read_to_string(path.as_ref())
            .context("Failed to read graph file")?;
        let graph: WorldModelGraph = serde_json::from_str(&content)
            .context("Failed to parse graph JSON")?;
        Ok(graph)
    }

    /// Export findings to markdown report
    pub fn export_markdown<P: AsRef<Path>>(&self, output: &ResearchOutput, path: P) -> Result<()> {
        let mut md = String::new();
        
        md.push_str(&format!("# Research Report\n\n"));
        md.push_str(&format!("**Question:** {}\n\n", output.question));
        md.push_str(&format!("**Pipeline Version:** {}\n\n", output.provenance.pipeline_version));
        md.push_str(&format!("**Iterations:** {}\n\n", output.iterations_completed));
        md.push_str(&format!("**Convergence:** {}\n\n", output.convergence_achieved));
        
        md.push_str("## Global Metrics\n\n");
        md.push_str(&format!("- Total Nodes: {}\n", output.global_metrics.total_nodes));
        md.push_str(&format!("- Total Edges: {}\n", output.global_metrics.total_edges));
        md.push_str(&format!("- Validated Findings: {}\n", output.global_metrics.validated_findings));
        md.push_str(&format!("- Contradiction Rate: {:.2}%\n", output.global_metrics.contradiction_rate * 100.0));
        md.push_str(&format!("- Average Confidence: {:.2}%\n\n", output.global_metrics.average_confidence * 100.0));
        
        md.push_str("## Findings\n\n");
        for (i, finding) in output.findings.iter().enumerate() {
            md.push_str(&format!("### Finding {}\n\n", i + 1));
            md.push_str(&format!("{}\n\n", finding.content));
            md.push_str(&format!("- **Confidence:** {:.2}%\n", finding.confidence * 100.0));
            md.push_str(&format!("- **Evidence Count:** {}\n", finding.evidence_count));
            md.push_str(&format!("- **Validation:** {}\n", if finding.validation_passed { "✓ Passed" } else { "✗ Failed" }));
            
            if !finding.risk_flags.is_empty() {
                md.push_str(&format!("- **Risk Flags:** {}\n", finding.risk_flags.join(", ")));
            }
            md.push_str("\n");
        }
        
        md.push_str("## Provenance\n\n");
        if let Some(commit) = &output.provenance.git_commit {
            md.push_str(&format!("- **Git Commit:** `{}`\n", commit));
        }
        md.push_str(&format!("- **Execution Time:** {}\n", output.provenance.execution_timestamp));
        
        if !output.provenance.artifact_uris.is_empty() {
            md.push_str("\n### Artifacts\n\n");
            for uri in &output.provenance.artifact_uris {
                md.push_str(&format!("- {}\n", uri));
            }
        }
        
        fs::write(path.as_ref(), md)
            .context("Failed to write markdown file")?;
        Ok(())
    }

    /// Create output directory if it doesn't exist
    pub fn ensure_output_dir(&self) -> Result<()> {
        fs::create_dir_all(&self.output_dir)
            .context("Failed to create output directory")?;
        Ok(())
    }
}

/// Policy checker for validation gates
pub struct PolicyChecker {
    policy: ValidationPolicy,
}

impl PolicyChecker {
    pub fn new(policy: ValidationPolicy) -> Self {
        Self { policy }
    }

    /// Check if validation metrics pass policy gates
    pub fn check_validation(&self, validation: &crate::model::Level5Validation) -> bool {
        validation.bootstrap_stability >= self.policy.min_bootstrap_stability
            && validation.holdout_consistency >= self.policy.min_holdout_consistency
            && validation.required_supports >= self.policy.min_required_supports
            && validation.contradiction_rate <= self.policy.max_contradiction_rate
    }

    /// Check if global metrics pass policy gates
    pub fn check_global_metrics(&self, metrics: &GlobalMetrics) -> bool {
        metrics.contradiction_rate <= self.policy.max_contradiction_rate
    }

    /// Get policy violations as human-readable strings
    pub fn get_violations(&self, validation: &crate::model::Level5Validation) -> Vec<String> {
        let mut violations = Vec::new();

        if validation.bootstrap_stability < self.policy.min_bootstrap_stability {
            violations.push(format!(
                "Bootstrap stability {:.2} < required {:.2}",
                validation.bootstrap_stability, self.policy.min_bootstrap_stability
            ));
        }

        if validation.holdout_consistency < self.policy.min_holdout_consistency {
            violations.push(format!(
                "Holdout consistency {:.2} < required {:.2}",
                validation.holdout_consistency, self.policy.min_holdout_consistency
            ));
        }

        if validation.required_supports < self.policy.min_required_supports {
            violations.push(format!(
                "Required supports {} < minimum {}",
                validation.required_supports, self.policy.min_required_supports
            ));
        }

        if validation.contradiction_rate > self.policy.max_contradiction_rate {
            violations.push(format!(
                "Contradiction rate {:.2} > maximum {:.2}",
                validation.contradiction_rate, self.policy.max_contradiction_rate
            ));
        }

        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_input() {
        let input = ResearchInput::default();
        assert_eq!(input.max_iterations, 10);
        assert!(input.enable_quantum_walk);
    }

    #[test]
    fn test_policy_checker() {
        let policy = ValidationPolicy::default();
        let checker = PolicyChecker::new(policy);

        let mut validation = crate::model::Level5Validation::new();
        assert!(!checker.check_validation(&validation));

        validation.bootstrap_stability = 0.85;
        validation.holdout_consistency = 0.80;
        validation.required_supports = 3;
        validation.contradiction_rate = 0.15;
        assert!(checker.check_validation(&validation));
    }
}
