// CLI to run demo cycle
use clap::{Parser, Subcommand};
use colored::*;
use quantum_limit_graph_level5_ai_scientist::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "level5-ai-scientist")]
#[command(about = "Quantum LIMIT Graph v2.4.0 Level 5 MetaAgent AI Scientist", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a research cycle from input file
    Run {
        /// Input file path (JSON or YAML)
        #[arg(short, long)]
        input: PathBuf,

        /// Output directory
        #[arg(short, long, default_value = "./output")]
        output: PathBuf,

        /// Output file prefix
        #[arg(short, long, default_value = "research")]
        prefix: String,
    },

    /// Run a quick demo
    Demo {
        /// Demo type: quick, full, or stress
        #[arg(short, long, default_value = "quick")]
        demo_type: String,

        /// Output directory
        #[arg(short, long, default_value = "./demo_output")]
        output: PathBuf,
    },

    /// Validate an existing graph
    Validate {
        /// Graph file path (JSON)
        #[arg(short, long)]
        graph: PathBuf,
    },

    /// Generate example input files
    Generate {
        /// Output path for example input
        #[arg(short, long, default_value = "./example_input.yaml")]
        output: PathBuf,

        /// Format: json or yaml
        #[arg(short, long, default_value = "yaml")]
        format: String,
    },

    /// Show provenance information
    Provenance {
        /// Output file path (JSON)
        #[arg(short, long)]
        output: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Run { input, output, prefix } => {
            run_research_cycle(input, output, prefix).await?;
        }
        Commands::Demo { demo_type, output } => {
            run_demo(demo_type, output).await?;
        }
        Commands::Validate { graph } => {
            validate_graph(graph)?;
        }
        Commands::Generate { output, format } => {
            generate_example(output, format)?;
        }
        Commands::Provenance { output } => {
            show_provenance(output)?;
        }
    }

    Ok(())
}

async fn run_research_cycle(
    input_path: PathBuf,
    output_dir: PathBuf,
    prefix: String,
) -> anyhow::Result<()> {
    println!("{}", "=== Level 5 AI Scientist Research Cycle ===".bold().green());
    println!("Input: {}", input_path.display());
    println!("Output: {}\n", output_dir.display());

    // Create bundle
    let bundle = bundle::Level5Bundle::new(output_dir.to_string_lossy().to_string());

    // Run from file
    println!("{}", "Loading input...".cyan());
    let output = bundle.run_from_file(input_path.to_str().unwrap()).await?;

    // Save outputs
    println!("{}", "Saving outputs...".cyan());
    let output_prefix = output_dir.join(&prefix);
    bundle.run_and_save(
        io::ResearchInput::default(), // Dummy, already ran
        output_prefix.to_str().unwrap(),
    ).await?;

    // Print summary
    print_output_summary(&output);

    println!("\n{}", "✓ Research cycle complete!".bold().green());

    Ok(())
}

async fn run_demo(demo_type: String, output_dir: PathBuf) -> anyhow::Result<()> {
    println!("{}", "=== Level 5 AI Scientist Demo ===".bold().green());
    println!("Demo Type: {}\n", demo_type);

    // Create bundle
    let bundle = bundle::Level5Bundle::new(output_dir.to_string_lossy().to_string());

    // Select demo preset
    let input = match demo_type.as_str() {
        "quick" => bundle::DemoPresets::quick_demo(),
        "full" => bundle::DemoPresets::full_research_demo(),
        "stress" => bundle::DemoPresets::stress_test(),
        _ => {
            eprintln!("{}", "Unknown demo type. Using 'quick'.".yellow());
            bundle::DemoPresets::quick_demo()
        }
    };

    println!("{}", "Running research cycle...".cyan());
    let output = bundle.run_and_save(input, "demo").await?;

    // Print summary
    print_output_summary(&output);

    println!("\n{}", "✓ Demo complete!".bold().green());

    Ok(())
}

fn validate_graph(graph_path: PathBuf) -> anyhow::Result<()> {
    println!("{}", "=== Graph Validation ===".bold().green());
    println!("Graph: {}\n", graph_path.display());

    // Load graph
    let io_manager = io::IOManager::new("./".to_string());
    let graph = io_manager.load_graph(&graph_path)?;

    // Create bundle and validate
    let bundle = bundle::Level5Bundle::new("./".to_string());
    let report = bundle.validate_graph(&graph);

    // Print report
    report.print_summary();

    if report.is_valid() {
        println!("\n{}", "✓ Graph validation passed!".bold().green());
    } else {
        println!("\n{}", "✗ Graph validation failed!".bold().red());
    }

    Ok(())
}

fn generate_example(output_path: PathBuf, format: String) -> anyhow::Result<()> {
    println!("{}", "=== Generate Example Input ===".bold().green());

    let input = bundle::DemoPresets::full_research_demo();

    match format.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&input)?;
            std::fs::write(&output_path, json)?;
        }
        "yaml" => {
            let yaml = serde_yaml::to_string(&input)?;
            std::fs::write(&output_path, yaml)?;
        }
        _ => {
            eprintln!("{}", "Unknown format. Using YAML.".yellow());
            let yaml = serde_yaml::to_string(&input)?;
            std::fs::write(&output_path, yaml)?;
        }
    }

    println!("Generated: {}", output_path.display());
    println!("{}", "✓ Example generated!".bold().green());

    Ok(())
}

fn show_provenance(output_path: PathBuf) -> anyhow::Result<()> {
    println!("{}", "=== Provenance Information ===".bold().green());

    let tracker = provenance::ProvenanceTracker::new();
    let report = tracker.generate_report();

    println!("{}", report);

    // Save to file
    let json = tracker.export_json();
    std::fs::write(&output_path, serde_json::to_string_pretty(&json)?)?;

    println!("\nSaved to: {}", output_path.display());
    println!("{}", "✓ Provenance exported!".bold().green());

    Ok(())
}

fn print_output_summary(output: &io::ResearchOutput) {
    println!("\n{}", "=== Research Output Summary ===".bold().cyan());
    println!("Question: {}", output.question);
    println!("Iterations: {}", output.iterations_completed);
    println!("Converged: {}", if output.convergence_achieved { "Yes".green() } else { "No".red() });
    
    println!("\n{}", "Global Metrics:".bold());
    println!("  Total Nodes: {}", output.global_metrics.total_nodes);
    println!("  Total Edges: {}", output.global_metrics.total_edges);
    println!("  Hypotheses: {}", output.global_metrics.hypothesis_count);
    println!("  Validated Findings: {}", output.global_metrics.validated_findings);
    println!("  Contradiction Rate: {:.2}%", output.global_metrics.contradiction_rate * 100.0);
    println!("  Average Confidence: {:.2}%", output.global_metrics.average_confidence * 100.0);

    if !output.findings.is_empty() {
        println!("\n{}", "Findings:".bold());
        for (i, finding) in output.findings.iter().enumerate().take(5) {
            let status = if finding.validation_passed {
                "✓".green()
            } else {
                "✗".red()
            };
            println!("  {} [{}] {} (confidence: {:.2}%)", 
                i + 1, status, finding.content, finding.confidence * 100.0);
        }
        
        if output.findings.len() > 5 {
            println!("  ... and {} more", output.findings.len() - 5);
        }
    }

    println!("\n{}", "Provenance:".bold());
    println!("  Pipeline Version: {}", output.provenance.pipeline_version);
    if let Some(commit) = &output.provenance.git_commit {
        println!("  Git Commit: {}", commit);
    }
    println!("  Execution Time: {}", output.provenance.execution_timestamp);
}
