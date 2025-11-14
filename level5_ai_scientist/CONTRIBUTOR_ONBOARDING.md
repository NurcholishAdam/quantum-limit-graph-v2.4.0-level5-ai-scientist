# Contributor Onboarding Checklist

Welcome to the Quantum LIMIT Graph v2.4.0 Level 5 MetaAgent AI Scientist project!

## Environment Setup

### Prerequisites

- [ ] Rust stable toolchain (1.75+)
  ```bash
  rustup install stable
  rustup default stable
  ```

- [ ] Development tools
  ```bash
  rustup component add rustfmt clippy
  ```

- [ ] Git configured
  ```bash
  git config --global user.name "Your Name"
  git config --global user.email "your.email@example.com"
  ```

### Build and Test

- [ ] Clone repository
  ```bash
  git clone <repository-url>
  cd quantum-limit-graph-v2.4.0/rust/level5_ai_scientist
  ```

- [ ] Build project
  ```bash
  cargo build
  cargo build --release
  ```

- [ ] Run tests
  ```bash
  cargo test
  cargo test -- --nocapture  # With output
  ```

- [ ] Run formatting and linting
  ```bash
  cargo fmt --check
  cargo clippy -- -D warnings
  ```

## Adding Methods or Agents

### Implement New Agent

1. [ ] Create agent struct in `src/metaagent.rs`
2. [ ] Implement `Agent` trait with `role()` and `execute()` methods
3. [ ] Add validation hooks in agent execution
4. [ ] Enumerate failure modes and log into attributes
5. [ ] Add tests in `tests/integration_tests.rs`

Example:

```rust
pub struct CustomAgent {
    provenance: Arc<Mutex<ProvenanceTracker>>,
}

#[async_trait::async_trait]
impl Agent for CustomAgent {
    fn role(&self) -> AgentRole {
        AgentRole::Custom
    }

    async fn execute(&self, graph: &mut WorldModelGraph, context: &AgentContext) -> Result<AgentOutput> {
        // 1. Validation hooks
        let validation_rails = ValidationRails::new(ValidationPolicy::default());
        
        // 2. Enumerate failure modes
        let failure_modes = validation_rails.enumerate_failure_modes(node, graph);
        
        // 3. Log into attributes
        for mode in failure_modes {
            node.add_metadata("failure_mode".to_string(), mode);
        }
        
        // 4. Track provenance
        let mut prov = self.provenance.lock().await;
        prov.track_node_creation(node_id, "CustomAgent", &context.question);
        
        Ok(output)
    }
}
```

### Add Validation Method

1. [ ] Extend `ValidationRails` in `src/level5.rs`
2. [ ] Implement bootstrap, holdout split, or sensitivity toggles
3. [ ] Return validation metrics
4. [ ] Add unit tests

Example:

```rust
impl ValidationRails {
    pub fn custom_validation(&self, node: &GraphNode, graph: &WorldModelGraph) -> Result<f64> {
        // Custom validation logic
        let score = compute_custom_metric(node, graph);
        Ok(score)
    }
}
```

## Emit Level 5 Metrics

### Required Fields

- [ ] Populate `level5.validation`:
  - `bootstrap_stability`
  - `holdout_consistency`
  - `sensitivity_score`

- [ ] Populate `claim_strength_governor`:
  - `confidence`
  - `evidence_weight`
  - `coherence_score`

- [ ] Populate `risk_flags`:
  - `high_uncertainty`
  - `conflicting_evidence`
  - `insufficient_validation`
  - `ethical_concerns`
  - `reproducibility_issues`

Example:

```rust
let mut validation = Level5Validation::new();
validation.bootstrap_stability = 0.85;
validation.holdout_consistency = 0.80;
validation.sensitivity_score = 0.90;
validation.required_supports = 3;
validation.contradiction_rate = 0.15;

node.update_validation(validation);
```

## Provenance Completeness

### Required Tracking

- [ ] Git commit hash
  ```rust
  let tracker = ProvenanceTracker::new();
  // Automatically captures git commit
  ```

- [ ] Script path and execution metadata
  ```rust
  tracker.track_node_creation(node_id, "AgentName", "Question");
  ```

- [ ] Artifact URIs
  ```rust
  tracker.register_artifact("file:///path/to/artifact.json".to_string());
  ```

- [ ] Pipeline version = "2.4.0-NSN"
  ```rust
  assert_eq!(PIPELINE_VERSION, "2.4.0-NSN");
  ```

## CI Gates (GitHub Actions)

### Build Gates

- [ ] Compilation succeeds
  ```bash
  cargo build --release
  ```

- [ ] Code formatting
  ```bash
  cargo fmt --check
  ```

- [ ] Linting passes
  ```bash
  cargo clippy -- -D warnings
  ```

### Test Gates

- [ ] All tests pass
  ```bash
  cargo test
  ```

- [ ] Integration tests pass
  ```bash
  cargo test --test integration_tests
  ```

### Validation Gates

Fail CI if:

- [ ] `bootstrap_stability < 0.8`
- [ ] `holdout_consistency < 0.75`
- [ ] `required_supports < 2`
- [ ] `contradiction_rate > 0.2` without resolution task

Example test:

```rust
#[test]
fn test_validation_gates() {
    let validation = Level5Validation {
        bootstrap_stability: 0.85,
        holdout_consistency: 0.80,
        required_supports: 3,
        contradiction_rate: 0.15,
        // ...
    };
    
    assert!(validation.passes_gates());
}
```

### Schema Validation (Optional)

- [ ] JSON Schema validation with `schemars`
  ```bash
  cargo test --features schema-validation
  ```

## Dashboard Integration (Future)

### Planned Features

- [ ] Discovery map: render nodes/edges with provenance
- [ ] Reliability panel: Level 5 metrics per node
- [ ] Coherence monitor: global contradiction_rate and latency
- [ ] Exploration heatmap: quantum-walk path coverage

### Preparation

- [ ] Ensure all nodes have complete metadata
- [ ] Track timing information for latency metrics
- [ ] Export graph in visualization-friendly format

## Code Quality Checklist

### Before Submitting PR

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated
- [ ] Examples work
- [ ] Provenance tracking complete
- [ ] Level 5 validation gates pass

### Documentation

- [ ] Public functions have doc comments
- [ ] Complex algorithms explained
- [ ] Examples provided for new features
- [ ] README updated if needed

### Testing

- [ ] Unit tests for new functions
- [ ] Integration tests for new agents
- [ ] Edge cases covered
- [ ] Error handling tested

## Common Tasks

### Run Demo

```bash
cargo run --bin level5-ai-scientist demo --demo-type quick
```

### Generate Example Input

```bash
cargo run --bin level5-ai-scientist generate --output example.yaml
```

### Validate Graph

```bash
cargo run --bin level5-ai-scientist validate --graph output/graph.json
```

### Check Provenance

```bash
cargo run --bin level5-ai-scientist provenance --output prov.json
```

## Getting Help

- Check `README.md` for usage examples
- Review `examples/demo_cycle.rs` for complete example
- Read inline documentation: `cargo doc --open`
- Ask questions in issues or discussions

## Next Steps

After completing this checklist:

1. [ ] Review existing code in `src/`
2. [ ] Run all examples
3. [ ] Try modifying validation thresholds
4. [ ] Implement a custom agent
5. [ ] Submit your first PR!

Welcome aboard! 🚀
