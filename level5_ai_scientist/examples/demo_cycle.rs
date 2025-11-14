// Demo cycle example for Level 5 AI Scientist
use quantum_limit_graph_level5_ai_scientist::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=== Level 5 AI Scientist Demo Cycle ===\n");

    // Create bundle
    let bundle = Level5Bundle::new("./demo_output".to_string());

    // Use quick demo preset
    let input = DemoPresets::quick_demo();

    println!("Research Question: {}", input.question);
    println!("Max Iterations: {}", input.max_iterations);
    println!("Quantum Walk: {}\n", input.enable_quantum_walk);

    // Run research cycle
    println!("Running research cycle...\n");
    let output = bundle.run_and_save(input, "demo").await?;

    // Print results
    println!("=== Results ===");
    println!("Iterations Completed: {}", output.iterations_completed);
    println!("Convergence Achieved: {}", output.convergence_achieved);
    println!("\nGlobal Metrics:");
    println!("  Total Nodes: {}", output.global_metrics.total_nodes);
    println!("  Total Edges: {}", output.global_metrics.total_edges);
    println!("  Validated Findings: {}", output.global_metrics.validated_findings);
    println!("  Contradiction Rate: {:.2}%", output.global_metrics.contradiction_rate * 100.0);
    println!("  Average Confidence: {:.2}%", output.global_metrics.average_confidence * 100.0);

    println!("\nFindings:");
    for (i, finding) in output.findings.iter().enumerate() {
        println!("  {}. {} (confidence: {:.2}%)", 
            i + 1, finding.content, finding.confidence * 100.0);
    }

    println!("\n✓ Demo complete! Check ./demo_output for results.");

    Ok(())
}
