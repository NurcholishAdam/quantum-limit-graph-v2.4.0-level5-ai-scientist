// Quantum LIMIT Graph v2.4.0 Level 5 MetaAgent AI Scientist
// Graph-first world model with validation rails and quantum-walk exploration

#![doc = include_str!("../README.md")]

pub mod model;
pub mod io;
pub mod level5;
pub mod quantum_walk;
pub mod metaagent;
pub mod bundle;
pub mod provenance;

// Re-export main types
pub use model::{
    WorldModelGraph, GraphNode, GraphEdge, NodeType, EdgeType,
    Level5Validation, ClaimStrength, RiskFlags,
};

pub use io::{
    ResearchInput, ResearchOutput, Finding, GlobalMetrics,
    ValidationPolicy, IOManager, PolicyChecker,
};

pub use level5::{
    ValidationRails, GlobalValidationMetrics,
};

pub use quantum_walk::{
    QuantumWalkExplorer, ExplorationResult, WalkStrategy,
};

pub use metaagent::{
    MetaAgent, Agent, AgentRole, AgentContext, AgentOutput,
    ResearcherAgent, CriticAgent, SynthesizerAgent,
};

pub use bundle::{
    Level5Bundle, ValidationReport, DemoPresets,
};

pub use provenance::{
    ProvenanceTracker, NodeLineage, ValidationEvent,
    ProvenanceCompleteness,
};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Pipeline version identifier
pub const PIPELINE_VERSION: &str = "2.4.0-NSN";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "2.4.0");
    }

    #[test]
    fn test_pipeline_version() {
        assert_eq!(PIPELINE_VERSION, "2.4.0-NSN");
    }
}
