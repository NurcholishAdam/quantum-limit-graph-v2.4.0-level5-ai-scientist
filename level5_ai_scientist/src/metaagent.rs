// MetaAgent orchestration with parallel agents
use crate::model::*;
use crate::io::{ResearchInput, ResearchOutput, Finding, GlobalMetrics, ProvenanceInfo};
use crate::level5::ValidationRails;
use crate::quantum_walk::{QuantumWalkExplorer, WalkStrategy};
use crate::provenance::ProvenanceTracker;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

/// Agent role in the research process
#[derive(Debug, Clone, PartialEq)]
pub enum AgentRole {
    Researcher,   // Hypothesis generation and exploration
    Critic,       // Validation and contradiction detection
    Synthesizer,  // Evidence aggregation and claim strength
    Orchestrator, // Meta-level coordination
}

/// Agent trait for parallel execution
#[async_trait::async_trait]
pub trait Agent: Send + Sync {
    fn role(&self) -> AgentRole;
    async fn execute(&self, graph: &mut WorldModelGraph, context: &AgentContext) -> Result<AgentOutput>;
}

/// Context passed to agents
#[derive(Debug, Clone)]
pub struct AgentContext {
    pub question: String,
    pub iteration: usize,
    pub max_iterations: usize,
    pub exploration_targets: Vec<Uuid>,
}

/// Output from agent execution
#[derive(Debug, Clone)]
pub struct AgentOutput {
    pub nodes_created: Vec<Uuid>,
    pub nodes_updated: Vec<Uuid>,
    pub messages: Vec<String>,
}

/// Researcher agent: generates hypotheses
pub struct ResearcherAgent {
    provenance: Arc<Mutex<ProvenanceTracker>>,
}

impl ResearcherAgent {
    pub fn new(provenance: Arc<Mutex<ProvenanceTracker>>) -> Self {
        Self { provenance }
    }
}

#[async_trait::async_trait]
impl Agent for ResearcherAgent {
    fn role(&self) -> AgentRole {
        AgentRole::Researcher
    }

    async fn execute(&self, graph: &mut WorldModelGraph, context: &AgentContext) -> Result<AgentOutput> {
        let mut output = AgentOutput {
            nodes_created: Vec::new(),
            nodes_updated: Vec::new(),
            messages: Vec::new(),
        };

        // Generate hypotheses based on exploration targets
        for target_id in &context.exploration_targets {
            if let Some(target) = graph.get_node(target_id) {
                // Generate related hypothesis
                let hypothesis_content = format!(
                    "Hypothesis derived from '{}' at iteration {}",
                    target.content, context.iteration
                );

                let mut hypothesis = GraphNode::new(NodeType::Hypothesis, hypothesis_content);
                hypothesis.claim_strength.novelty_score = 0.7;

                let hyp_id = graph.add_node(hypothesis);
                output.nodes_created.push(hyp_id);

                // Link to source
                let edge = GraphEdge::new(*target_id, hyp_id, EdgeType::DerivedFrom, 0.8);
                graph.add_edge(edge);

                // Track provenance
                let mut prov = self.provenance.lock().await;
                prov.track_node_creation(hyp_id, "ResearcherAgent", &context.question);

                output.messages.push(format!("Generated hypothesis: {}", hyp_id));
            }
        }

        Ok(output)
    }
}

/// Critic agent: validates and detects contradictions
pub struct CriticAgent {
    validation_rails: ValidationRails,
    provenance: Arc<Mutex<ProvenanceTracker>>,
}

impl CriticAgent {
    pub fn new(validation_rails: ValidationRails, provenance: Arc<Mutex<ProvenanceTracker>>) -> Self {
        Self {
            validation_rails,
            provenance,
        }
    }
}

#[async_trait::async_trait]
impl Agent for CriticAgent {
    fn role(&self) -> AgentRole {
        AgentRole::Critic
    }

    async fn execute(&self, graph: &mut WorldModelGraph, context: &AgentContext) -> Result<AgentOutput> {
        let mut output = AgentOutput {
            nodes_created: Vec::new(),
            nodes_updated: Vec::new(),
            messages: Vec::new(),
        };

        // Validate all hypotheses
        let hypothesis_ids: Vec<Uuid> = graph
            .get_nodes_by_type(&NodeType::Hypothesis)
            .iter()
            .map(|n| n.id)
            .collect();

        for hyp_id in hypothesis_ids {
            if let Some(node) = graph.get_node(&hyp_id).cloned() {
                // Run validation
                let validation = self.validation_rails.validate_node(&node, graph)?;
                
                if let Some(node_mut) = graph.get_node_mut(&hyp_id) {
                    node_mut.update_validation(validation.clone());
                    output.nodes_updated.push(hyp_id);

                    // Check for contradictions
                    if validation.contradiction_rate > 0.2 {
                        // Generate resolution task
                        if let Some(task_content) = self.validation_rails.generate_resolution_task(&node, graph) {
                            let task = GraphNode::new(NodeType::ResolutionTask, task_content);
                            let task_id = graph.add_node(task);
                            
                            let edge = GraphEdge::new(hyp_id, task_id, EdgeType::Resolves, 1.0);
                            graph.add_edge(edge);
                            
                            output.nodes_created.push(task_id);
                            output.messages.push(format!("Created resolution task for {}", hyp_id));
                        }
                    }

                    // Track provenance
                    let mut prov = self.provenance.lock().await;
                    prov.track_validation(hyp_id, validation.passes_gates());
                }
            }
        }

        Ok(output)
    }
}

/// Synthesizer agent: aggregates evidence and computes claim strength
pub struct SynthesizerAgent {
    provenance: Arc<Mutex<ProvenanceTracker>>,
}

impl SynthesizerAgent {
    pub fn new(provenance: Arc<Mutex<ProvenanceTracker>>) -> Self {
        Self { provenance }
    }
}

#[async_trait::async_trait]
impl Agent for SynthesizerAgent {
    fn role(&self) -> AgentRole {
        AgentRole::Synthesizer
    }

    async fn execute(&self, graph: &mut WorldModelGraph, context: &AgentContext) -> Result<AgentOutput> {
        let mut output = AgentOutput {
            nodes_created: Vec::new(),
            nodes_updated: Vec::new(),
            messages: Vec::new(),
        };

        // Compute claim strength for all hypotheses
        let hypothesis_ids: Vec<Uuid> = graph
            .get_nodes_by_type(&NodeType::Hypothesis)
            .iter()
            .map(|n| n.id)
            .collect();

        for hyp_id in hypothesis_ids {
            let evidence = graph.get_supporting_evidence(&hyp_id);
            
            if !evidence.is_empty() {
                let evidence_weight = evidence
                    .iter()
                    .map(|e| e.claim_strength.confidence)
                    .sum::<f64>()
                    / evidence.len() as f64;

                let contradictions = graph.get_contradictions(&hyp_id);
                let coherence_score = 1.0 - (contradictions.len() as f64 / (evidence.len() + 1) as f64);

                let confidence = (evidence_weight * 0.6 + coherence_score * 0.4).min(1.0);

                let claim_strength = ClaimStrength {
                    confidence,
                    evidence_weight,
                    coherence_score,
                    novelty_score: 0.5,
                };

                if let Some(node) = graph.get_node_mut(&hyp_id) {
                    node.update_claim_strength(claim_strength);
                    output.nodes_updated.push(hyp_id);
                }
            }
        }

        Ok(output)
    }
}

/// MetaAgent orchestrator
pub struct MetaAgent {
    researcher: ResearcherAgent,
    critic: CriticAgent,
    synthesizer: SynthesizerAgent,
    quantum_explorer: QuantumWalkExplorer,
    provenance: Arc<Mutex<ProvenanceTracker>>,
}

impl MetaAgent {
    pub fn new(
        validation_rails: ValidationRails,
        quantum_explorer: QuantumWalkExplorer,
        provenance: Arc<Mutex<ProvenanceTracker>>,
    ) -> Self {
        Self {
            researcher: ResearcherAgent::new(provenance.clone()),
            critic: CriticAgent::new(validation_rails, provenance.clone()),
            synthesizer: SynthesizerAgent::new(provenance.clone()),
            quantum_explorer,
            provenance,
        }
    }

    /// Run complete research cycle
    pub async fn run_research_cycle(&self, input: ResearchInput) -> Result<ResearchOutput> {
        let mut graph = WorldModelGraph::new();
        
        // Initialize with seed nodes from context
        let mut seed_nodes = Vec::new();
        for ctx in &input.context {
            let node = GraphNode::new(NodeType::Finding, ctx.clone());
            let id = graph.add_node(node);
            seed_nodes.push(id);
        }

        let mut converged = false;
        let mut iteration = 0;

        // Iterative research cycle
        while iteration < input.max_iterations && !converged {
            iteration += 1;

            // Quantum walk exploration
            let exploration_result = if input.enable_quantum_walk {
                self.quantum_explorer.explore(&graph, &seed_nodes)?
            } else {
                Default::default()
            };

            let exploration_targets = exploration_result.get_top_nodes(5)
                .into_iter()
                .map(|(id, _)| id)
                .collect();

            let context = AgentContext {
                question: input.question.clone(),
                iteration,
                max_iterations: input.max_iterations,
                exploration_targets,
            };

            // Execute agents in parallel
            let (researcher_out, critic_out, synthesizer_out) = tokio::join!(
                self.researcher.execute(&mut graph, &context),
                self.critic.execute(&mut graph, &context),
                self.synthesizer.execute(&mut graph, &context),
            );

            // Check convergence
            let validated_count = graph
                .get_nodes_by_type(&NodeType::Hypothesis)
                .iter()
                .filter(|n| n.level5_validation.passes_gates())
                .count();

            let total_hypotheses = graph.get_nodes_by_type(&NodeType::Hypothesis).len();
            
            if total_hypotheses > 0 {
                let validation_rate = validated_count as f64 / total_hypotheses as f64;
                converged = validation_rate >= input.convergence_threshold;
            }
        }

        // Generate output
        self.generate_output(input, graph, iteration, converged).await
    }

    async fn generate_output(
        &self,
        input: ResearchInput,
        graph: WorldModelGraph,
        iterations: usize,
        converged: bool,
    ) -> Result<ResearchOutput> {
        let findings = self.extract_findings(&graph);
        let global_metrics = self.compute_global_metrics(&graph);
        
        let prov = self.provenance.lock().await;
        let provenance_info = prov.get_provenance_info();

        Ok(ResearchOutput {
            question: input.question,
            findings,
            world_model: graph,
            iterations_completed: iterations,
            convergence_achieved: converged,
            global_metrics,
            provenance: provenance_info,
        })
    }

    fn extract_findings(&self, graph: &WorldModelGraph) -> Vec<Finding> {
        graph
            .get_nodes_by_type(&NodeType::Hypothesis)
            .iter()
            .filter(|n| n.level5_validation.passes_gates())
            .map(|n| {
                let evidence_count = graph.get_supporting_evidence(&n.id).len();
                let risk_flags = self.get_risk_flag_names(&n.risk_flags);

                Finding {
                    content: n.content.clone(),
                    confidence: n.claim_strength.confidence,
                    evidence_count,
                    validation_passed: n.level5_validation.passes_gates(),
                    risk_flags,
                }
            })
            .collect()
    }

    fn compute_global_metrics(&self, graph: &WorldModelGraph) -> GlobalMetrics {
        let hypotheses = graph.get_nodes_by_type(&NodeType::Hypothesis);
        let validated = hypotheses
            .iter()
            .filter(|n| n.level5_validation.passes_gates())
            .count();

        let avg_confidence = if !hypotheses.is_empty() {
            hypotheses
                .iter()
                .map(|n| n.claim_strength.confidence)
                .sum::<f64>()
                / hypotheses.len() as f64
        } else {
            0.0
        };

        GlobalMetrics {
            total_nodes: graph.nodes.len(),
            total_edges: graph.edges.len(),
            hypothesis_count: hypotheses.len(),
            validated_findings: validated,
            contradiction_rate: graph.compute_global_contradiction_rate(),
            average_confidence: avg_confidence,
            quantum_walk_coverage: 0.0, // Computed separately
        }
    }

    fn get_risk_flag_names(&self, flags: &RiskFlags) -> Vec<String> {
        let mut names = Vec::new();
        if flags.high_uncertainty {
            names.push("High Uncertainty".to_string());
        }
        if flags.conflicting_evidence {
            names.push("Conflicting Evidence".to_string());
        }
        if flags.insufficient_validation {
            names.push("Insufficient Validation".to_string());
        }
        if flags.ethical_concerns {
            names.push("Ethical Concerns".to_string());
        }
        if flags.reproducibility_issues {
            names.push("Reproducibility Issues".to_string());
        }
        names
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::ValidationPolicy;

    #[tokio::test]
    async fn test_metaagent_cycle() {
        let policy = ValidationPolicy::default();
        let rails = ValidationRails::new(policy);
        let explorer = QuantumWalkExplorer::new(10);
        let provenance = Arc::new(Mutex::new(ProvenanceTracker::new()));

        let metaagent = MetaAgent::new(rails, explorer, provenance);

        let input = ResearchInput {
            question: "Test question".to_string(),
            context: vec!["Initial context".to_string()],
            max_iterations: 2,
            ..Default::default()
        };

        let output = metaagent.run_research_cycle(input).await.unwrap();
        assert!(output.iterations_completed <= 2);
    }
}
