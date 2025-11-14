# Quantum LIMIT Graph v2.4.0 Level 5 MetaAgent AI Scientist

A Rust implementation of the Level 5 MetaAgent AI Scientist with graph-first world model, validation rails, quantum-walk exploration, and reproducible provenance.

## Features

### Core Principles
- **Iterative Cycles**: Research hypothesis generation → validation → refinement loop
- **Parallel Agents**: Researcher, Critic, Synthesizer working in parallel
- **Traceable Reasoning**: Full provenance tracking with git commits and artifact URIs
- **World-Model Context**: Graph-first knowledge representation

### Level 5 Extensions
- **Validation Rails**: Bootstrap stability, holdout consistency, sensitivity analysis
- **Quantum-Walk Exploration**: Weight-driven node selection for discovery
- **Reproducible Provenance**: Complete lineage tracking (commit, script, artifacts)
- **Quality Gates**: CI-integrated validation thresholds

## Installation

```bash
cargo build --release
```

## Quick Start

### Run a Demo

```bash
cargo run --bin level5-ai-scientist demo --demo-type quick
```

### Run from Input File

```bash
# Generate example input
cargo run --bin level5-ai-scientist generate --output input.yaml

# Run research cycle
cargo run --bin level5-ai-scientist run --input input.yaml --output ./results
```

### Validate a Graph

```bash
cargo run --bin level5-ai-scientist validate --graph results/research_graph.json
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    CLI Interface (cli.rs)                    │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│              MetaAgent Orchestrator (metaagent.rs)           │
│    Parallel Agent Coordination & Iterative Cycle Control    │
└─────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
┌───────▼────────┐  ┌────────▼────────┐  ┌────────▼────────┐
│   Researcher   │  │     Critic      │  │  Synthesizer    │
└────────────────┘  └─────────────────┘  └─────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│           Quantum-Walk Explorer (quantum_walk.rs)            │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│            World Model Graph (model.rs)                      │
└─────────────────────────────────────────────────────────────┘
```

## Usage Examples

### Programmatic Usage

```rust
use quantum_limit_graph_level5_ai_scientist::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create bundle
    let bundle = Level5Bundle::new("./output".to_string());

    // Configure research input
    let input = ResearchInput {
        question: "How do quantum algorithms work?".to_string(),
        context: vec!["Quantum computing uses superposition".to_string()],
        max_iterations: 10,
        convergence_threshold: 0.95,
        enable_quantum_walk: true,
        validation_policy: ValidationPolicy::default(),
    };

    // Run research cycle
    let output = bundle.run_and_save(input, "research").await?;

    println!("Validated findings: {}", output.global_metrics.validated_findings);

    Ok(())
}
```

### Custom Validation Policy

```rust
let policy = ValidationPolicy {
    min_bootstrap_stability: 0.85,
    min_holdout_consistency: 0.80,
    min_required_supports: 3,
    max_contradiction_rate: 0.15,
    enable_sensitivity_analysis: true,
    bootstrap_samples: 2000,
};

let bundle = Level5Bundle::with_config("./output".to_string(), policy, 100);
```

## Level 5 Validation Gates

### CI Integration

The following thresholds must be met for CI to pass:

- **Bootstrap Stability**: ≥ 0.8
- **Holdout Consistency**: ≥ 0.75
- **Required Supports**: ≥ 2
- **Contradiction Rate**: ≤ 0.2 (or resolution task exists)

### Running Tests

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration_tests

# With output
cargo test -- --nocapture
```

## Contributor Onboarding

### Environment Setup

1. Install Rust stable toolchain (1.75+)
2. Install development tools:
   ```bash
   rustup component add rustfmt clippy
   ```

### Adding New Agents

Implement the `Agent` trait in `metaagent.rs`:

```rust
pub struct MyAgent {
    // fields
}

#[async_trait::async_trait]
impl Agent for MyAgent {
    fn role(&self) -> AgentRole {
        AgentRole::Custom
    }

    async fn execute(&self, graph: &mut WorldModelGraph, context: &AgentContext) -> Result<AgentOutput> {
        // implementation
    }
}
```

### Adding Validation Hooks

Extend `ValidationRails` in `level5.rs`:

```rust
impl ValidationRails {
    pub fn custom_validation(&self, node: &GraphNode) -> Result<f64> {
        // custom validation logic
    }
}
```

### CI Gates

GitHub Actions workflow checks:

```yaml
- name: Build
  run: cargo build --release

- name: Format
  run: cargo fmt --check

- name: Clippy
  run: cargo clippy -- -D warnings

- name: Tests
  run: cargo test

- name: Level 5 Gates
  run: cargo test --test integration_tests -- --nocapture
```

## Output Files

After running a research cycle:

- `{prefix}_output.json`: Complete research output with findings
- `{prefix}_graph.json`: World model graph structure
- `{prefix}_report.md`: Human-readable markdown report

## Provenance Tracking

All operations are tracked with:

- Pipeline version: `2.4.0-NSN`
- Git commit hash (if available)
- Execution timestamp
- Script path and parameters
- Artifact URIs

Access provenance:

```bash
cargo run --bin level5-ai-scientist provenance --output provenance.json
```

## Performance

- Parallel agent execution with Tokio async runtime
- Efficient graph operations with petgraph
- Quantum walk simulation with ndarray
- Release builds optimized with LTO

## License

CC BY-NC-SA 4.0

## Contributing

See `CONTRIBUTING.md` for guidelines.

## References

- Quantum LIMIT Graph v2.4.0 specification
- Level 5 maturity model documentation
- NSN (Neural Semantic Network) integration guide
