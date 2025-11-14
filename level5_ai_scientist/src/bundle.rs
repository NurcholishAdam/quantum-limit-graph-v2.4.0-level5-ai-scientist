// First-cycle bundles: structs and demo
use crate::model::*;
use crate::io::*;
use crate::level5::ValidationRails;
use crate::quantum_walk::{QuantumWalkExplorer, WalkStrategy};
use crate::metaagent::MetaAgent;
use crate::provenance::ProvenanceTracker;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Complete bundle for Level 5 AI Scientist
pub struct Level5Bundle {
    pub validation_rails: ValidationRails,
    pub quantum_explorer: QuantumWalkExplorer,
    pub metaagent: MetaAgent,
    pub io_manager: IOManager,
    pub provenance: Arc<Mutex<ProvenanceTracker>>,
}

impl Level5Bundle {
    /// Create new bundle with default configuration
    pub fn new(output_dir: String) -> Self {
        let policy = ValidationPolicy::default();
        let validation_rails = ValidationRails::new(policy);
        let quantum_explorer = WalkStrategy::Continuous.create_explorer(50);
        let provenance = Arc::new(Mutex::new(ProvenanceTracker::new()));
        let metaagent = MetaAgent::new(
            validation_rails.clone(),
            quantum_explorer.clone(),
            provenance.clone(),
        );
        let io_manager = IOManager::new(output_dir);

        Self {
            validation_rails,
            quantum_explorer,
            metaagent,
            io_manager,
            provenance,
        }
    }

    /// Create bundle with custom configuration
    pub fn with_config(output_dir: String, policy: ValidationPolicy, max_walk_steps: usize) -> Self {
        let validation_rails = ValidationRails::new(policy);
        let quantum_explorer = WalkStrategy::Continuous.create_explorer(max_walk_steps);
        let provenance = Arc::new(Mutex::new(ProvenanceTracker::new()));
        let metaagent = MetaAgent::new(
            validation_rails.clone(),
            quantum_explorer.clone(),
            provenance.clone(),
        );
        let io_manager = IOManager::new(output_dir);

        Self {
            validation_rails,
            quantum_explorer,
            metaagent,
            io_manager,
            provenance,
        }
    }

    /// Run complete research cycle from input file
    pub async fn run_from_file(&self, input_path: &str) -> Result<ResearchOutput> {
        // Load input
        let input = if input_path.ends_with(".yaml") || input_path.ends_with(".yml") {
            self.io_manager.load_input_yaml(input_path)?
        } else {
            self.io_manager.load_input(input_path)?
        };

        // Run research cycle
        let output = self.metaagent.run_research_cycle(input).await?;

        Ok(output)
    }

    /// Run research cycle and save outputs
    pub async fn run_and_save(
        &self,
        input: ResearchInput,
        output_prefix: &str,
    ) -> Result<ResearchOutput> {
        // Ensure output directory exists
        self.io_manager.ensure_output_dir()?;

        // Run research cycle
        let output = self.metaagent.run_research_cycle(input).await?;

        // Save outputs
        let json_path = format!("{}_output.json", output_prefix);
        let graph_path = format!("{}_graph.json", output_prefix);
        let md_path = format!("{}_report.md", output_prefix);

        self.io_manager.save_output(&output, &json_path)?;
        self.io_manager.save_graph(&output.world_model, &graph_path)?;
        self.io_manager.export_markdown(&output, &md_path)?;

        Ok(output)
    }

    /// Validate existing graph against Level 5 gates
    pub fn validate_graph(&self, graph: &WorldModelGraph) -> ValidationReport {
        let mut report = ValidationReport::default();

        for node in graph.nodes.values() {
            let passes = self.validation_rails.passes_gates(&node.level5_validation);
            
            if passes {
                report.passed_nodes.push(node.id);
            } else {
                report.failed_nodes.push(node.id);
                let violations = self.get_violations(node, graph);
                report.violations.extend(violations);
            }
        }

        let global_metrics = self.validation_rails.compute_global_metrics(graph);
        report.global_metrics = Some(global_metrics);

        report
    }

    fn get_violations(&self, node: &GraphNode, graph: &WorldModelGraph) -> Vec<String> {
        let mut violations = Vec::new();

        if node.level5_validation.bootstrap_stability < 0.8 {
            violations.push(format!(
                "Node {}: Bootstrap stability {:.2} < 0.8",
                node.id, node.level5_validation.bootstrap_stability
            ));
        }

        if node.level5_validation.holdout_consistency < 0.75 {
            violations.push(format!(
                "Node {}: Holdout consistency {:.2} < 0.75",
                node.id, node.level5_validation.holdout_consistency
            ));
        }

        if node.level5_validation.required_supports < 2 {
            violations.push(format!(
                "Node {}: Required supports {} < 2",
                node.id, node.level5_validation.required_supports
            ));
        }

        if node.level5_validation.contradiction_rate > 0.2 {
            violations.push(format!(
                "Node {}: Contradiction rate {:.2} > 0.2",
                node.id, node.level5_validation.contradiction_rate
            ));
        }

        violations
    }
}

/// Validation report for graph
#[derive(Debug, Clone, Default)]
pub struct ValidationReport {
    pub passed_nodes: Vec<uuid::Uuid>,
    pub failed_nodes: Vec<uuid::Uuid>,
    pub violations: Vec<String>,
    pub global_metrics: Option<crate::level5::GlobalValidationMetrics>,
}

impl ValidationReport {
    pub fn is_valid(&self) -> bool {
        self.failed_nodes.is_empty()
    }

    pub fn validation_rate(&self) -> f64 {
        let total = self.passed_nodes.len() + self.failed_nodes.len();
        if total == 0 {
            0.0
        } else {
            self.passed_nodes.len() as f64 / total as f64
        }
    }

    pub fn print_summary(&self) {
        println!("=== Validation Report ===");
        println!("Passed: {}", self.passed_nodes.len());
        println!("Failed: {}", self.failed_nodes.len());
        println!("Validation Rate: {:.2}%", self.validation_rate() * 100.0);
        
        if !self.violations.is_empty() {
            println!("\nViolations:");
            for violation in &self.violations {
                println!("  - {}", violation);
            }
        }

        if let Some(metrics) = &self.global_metrics {
            println!("\nGlobal Metrics:");
            println!("  Total Nodes: {}", metrics.total_nodes);
            println!("  Validated Nodes: {}", metrics.validated_nodes);
            println!("  Avg Bootstrap Stability: {:.2}", metrics.avg_bootstrap_stability);
            println!("  Avg Holdout Consistency: {:.2}", metrics.avg_holdout_consistency);
            println!("  Global Contradiction Rate: {:.2}", metrics.global_contradiction_rate);
        }
    }
}

/// Demo configuration presets
pub struct DemoPresets;

impl DemoPresets {
    /// Quick demo with minimal iterations
    pub fn quick_demo() -> ResearchInput {
        ResearchInput {
            question: "What are the key principles of quantum computing?".to_string(),
            context: vec![
                "Quantum computing uses quantum mechanics principles".to_string(),
                "Qubits can exist in superposition states".to_string(),
            ],
            max_iterations: 3,
            convergence_threshold: 0.8,
            enable_quantum_walk: true,
            validation_policy: ValidationPolicy {
                min_bootstrap_stability: 0.7,
                min_holdout_consistency: 0.65,
                min_required_supports: 1,
                max_contradiction_rate: 0.3,
                enable_sensitivity_analysis: false,
                bootstrap_samples: 100,
            },
        }
    }

    /// Full research demo with strict validation
    pub fn full_research_demo() -> ResearchInput {
        ResearchInput {
            question: "How can quantum algorithms improve machine learning?".to_string(),
            context: vec![
                "Quantum algorithms can solve certain problems exponentially faster".to_string(),
                "Machine learning requires optimization and pattern recognition".to_string(),
                "Quantum machine learning is an emerging field".to_string(),
            ],
            max_iterations: 10,
            convergence_threshold: 0.95,
            enable_quantum_walk: true,
            validation_policy: ValidationPolicy::default(),
        }
    }

    /// Stress test with many iterations
    pub fn stress_test() -> ResearchInput {
        ResearchInput {
            question: "Complex multi-faceted research question".to_string(),
            context: vec!["Context 1".to_string(), "Context 2".to_string()],
            max_iterations: 50,
            convergence_threshold: 0.99,
            enable_quantum_walk: true,
            validation_policy: ValidationPolicy::default(),
        }
    }
}

/// Example usage and integration tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bundle_creation() {
        let bundle = Level5Bundle::new("./output".to_string());
        assert!(bundle.provenance.lock().await.get_provenance_info().pipeline_version.contains("2.4.0"));
    }

    #[tokio::test]
    async fn test_quick_demo() {
        let bundle = Level5Bundle::new("./test_output".to_string());
        let input = DemoPresets::quick_demo();
        
        let output = bundle.metaagent.run_research_cycle(input).await.unwrap();
        assert!(output.iterations_completed <= 3);
    }

    #[test]
    fn test_validation_report() {
        let mut report = ValidationReport::default();
        report.passed_nodes.push(uuid::Uuid::new_v4());
        report.failed_nodes.push(uuid::Uuid::new_v4());
        
        assert_eq!(report.validation_rate(), 0.5);
        assert!(!report.is_valid());
    }
}
