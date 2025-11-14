// Integration tests for Level 5 AI Scientist
use quantum_limit_graph_level5_ai_scientist::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn test_full_research_cycle() {
    let bundle = Level5Bundle::new("./test_output".to_string());
    
    let input = ResearchInput {
        question: "What are quantum algorithms?".to_string(),
        context: vec!["Quantum algorithms use quantum mechanics".to_string()],
        max_iterations: 3,
        convergence_threshold: 0.8,
        enable_quantum_walk: true,
        validation_policy: ValidationPolicy::default(),
    };

    let output = bundle.metaagent.run_research_cycle(input).await.unwrap();
    
    assert!(output.iterations_completed <= 3);
    assert!(output.global_metrics.total_nodes > 0);
}

#[tokio::test]
async fn test_validation_rails() {
    let policy = ValidationPolicy::default();
    let rails = ValidationRails::new(policy);
    
    let mut graph = WorldModelGraph::new();
    let node = GraphNode::new(NodeType::Hypothesis, "Test hypothesis".to_string());
    let node_id = graph.add_node(node.clone());
    
    // Add evidence
    for i in 0..3 {
        let evidence = GraphNode::new(NodeType::Evidence, format!("Evidence {}", i));
        let evidence_id = graph.add_node(evidence);
        let edge = GraphEdge::new(evidence_id, node_id, EdgeType::Supports, 0.8);
        graph.add_edge(edge);
    }
    
    let validation = rails.validate_node(graph.get_node(&node_id).unwrap(), &graph).unwrap();
    assert!(validation.required_supports >= 2);
}

#[tokio::test]
async fn test_quantum_walk_exploration() {
    let explorer = QuantumWalkExplorer::new(20);
    let mut graph = WorldModelGraph::new();
    
    // Create graph
    let node1 = GraphNode::new(NodeType::Hypothesis, "H1".to_string());
    let node2 = GraphNode::new(NodeType::Hypothesis, "H2".to_string());
    let id1 = graph.add_node(node1);
    let id2 = graph.add_node(node2);
    
    graph.add_edge(GraphEdge::new(id1, id2, EdgeType::Supports, 0.9));
    
    let result = explorer.explore(&graph, &[id1]).unwrap();
    assert!(result.coverage > 0.0);
}

#[test]
fn test_provenance_tracking() {
    let mut tracker = ProvenanceTracker::new();
    assert_eq!(tracker.get_provenance_info().pipeline_version, "2.4.0-NSN");
    
    let node_id = uuid::Uuid::new_v4();
    tracker.track_node_creation(node_id, "TestAgent", "Test question");
    
    let lineage = tracker.get_lineage(&node_id);
    assert!(lineage.is_some());
}

#[test]
fn test_ci_gates() {
    let mut validation = Level5Validation::new();
    assert!(!validation.passes_gates());
    
    validation.bootstrap_stability = 0.85;
    validation.holdout_consistency = 0.80;
    validation.required_supports = 3;
    validation.contradiction_rate = 0.15;
    
    assert!(validation.passes_gates());
}

#[tokio::test]
async fn test_parallel_agents() {
    let policy = ValidationPolicy::default();
    let rails = ValidationRails::new(policy);
    let explorer = QuantumWalkExplorer::new(10);
    let provenance = Arc::new(Mutex::new(ProvenanceTracker::new()));
    
    let metaagent = MetaAgent::new(rails, explorer, provenance);
    
    let input = ResearchInput {
        question: "Test parallel execution".to_string(),
        context: vec!["Context 1".to_string()],
        max_iterations: 2,
        ..Default::default()
    };
    
    let output = metaagent.run_research_cycle(input).await.unwrap();
    assert!(output.iterations_completed <= 2);
}

#[test]
fn test_validation_report() {
    let bundle = Level5Bundle::new("./test".to_string());
    let mut graph = WorldModelGraph::new();
    
    let mut node = GraphNode::new(NodeType::Hypothesis, "Test".to_string());
    node.level5_validation.bootstrap_stability = 0.85;
    node.level5_validation.holdout_consistency = 0.80;
    node.level5_validation.required_supports = 3;
    node.level5_validation.contradiction_rate = 0.15;
    
    graph.add_node(node);
    
    let report = bundle.validate_graph(&graph);
    assert!(report.validation_rate() > 0.0);
}
