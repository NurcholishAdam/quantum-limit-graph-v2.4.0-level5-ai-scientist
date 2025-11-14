// Provenance tracking helpers
use crate::io::ProvenanceInfo;
use chrono::{DateTime, Utc};
use git2::Repository;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use uuid::Uuid;

/// Provenance tracker for reproducibility
pub struct ProvenanceTracker {
    pipeline_version: String,
    git_commit: Option<String>,
    execution_start: DateTime<Utc>,
    artifact_uris: Vec<String>,
    node_lineage: HashMap<Uuid, NodeLineage>,
    validation_history: Vec<ValidationEvent>,
}

/// Lineage information for a node
#[derive(Debug, Clone)]
pub struct NodeLineage {
    pub node_id: Uuid,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub source_question: String,
    pub parent_nodes: Vec<Uuid>,
    pub script_path: Option<String>,
}

/// Validation event record
#[derive(Debug, Clone)]
pub struct ValidationEvent {
    pub node_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub passed: bool,
    pub metrics: HashMap<String, f64>,
}

impl ProvenanceTracker {
    pub fn new() -> Self {
        let git_commit = Self::get_git_commit();
        
        Self {
            pipeline_version: "2.4.0-NSN".to_string(),
            git_commit,
            execution_start: Utc::now(),
            artifact_uris: Vec::new(),
            node_lineage: HashMap::new(),
            validation_history: Vec::new(),
        }
    }

    /// Get current git commit hash
    fn get_git_commit() -> Option<String> {
        // Try to find git repository
        let current_dir = env::current_dir().ok()?;
        let repo = Repository::discover(current_dir).ok()?;
        
        let head = repo.head().ok()?;
        let commit = head.peel_to_commit().ok()?;
        
        Some(commit.id().to_string())
    }

    /// Track node creation
    pub fn track_node_creation(
        &mut self,
        node_id: Uuid,
        created_by: &str,
        source_question: &str,
    ) {
        let lineage = NodeLineage {
            node_id,
            created_by: created_by.to_string(),
            created_at: Utc::now(),
            source_question: source_question.to_string(),
            parent_nodes: Vec::new(),
            script_path: Self::get_script_path(),
        };

        self.node_lineage.insert(node_id, lineage);
    }

    /// Track node creation with parent relationship
    pub fn track_node_derivation(
        &mut self,
        node_id: Uuid,
        created_by: &str,
        source_question: &str,
        parent_ids: Vec<Uuid>,
    ) {
        let lineage = NodeLineage {
            node_id,
            created_by: created_by.to_string(),
            created_at: Utc::now(),
            source_question: source_question.to_string(),
            parent_nodes: parent_ids,
            script_path: Self::get_script_path(),
        };

        self.node_lineage.insert(node_id, lineage);
    }

    /// Get current script path
    fn get_script_path() -> Option<String> {
        env::current_exe()
            .ok()
            .and_then(|p| p.to_str().map(|s| s.to_string()))
    }

    /// Track validation event
    pub fn track_validation(&mut self, node_id: Uuid, passed: bool) {
        let event = ValidationEvent {
            node_id,
            timestamp: Utc::now(),
            passed,
            metrics: HashMap::new(),
        };

        self.validation_history.push(event);
    }

    /// Track validation with detailed metrics
    pub fn track_validation_detailed(
        &mut self,
        node_id: Uuid,
        passed: bool,
        metrics: HashMap<String, f64>,
    ) {
        let event = ValidationEvent {
            node_id,
            timestamp: Utc::now(),
            passed,
            metrics,
        };

        self.validation_history.push(event);
    }

    /// Register artifact URI
    pub fn register_artifact(&mut self, uri: String) {
        if !self.artifact_uris.contains(&uri) {
            self.artifact_uris.push(uri);
        }
    }

    /// Get lineage for a node
    pub fn get_lineage(&self, node_id: &Uuid) -> Option<&NodeLineage> {
        self.node_lineage.get(node_id)
    }

    /// Get validation history for a node
    pub fn get_validation_history(&self, node_id: &Uuid) -> Vec<&ValidationEvent> {
        self.validation_history
            .iter()
            .filter(|e| &e.node_id == node_id)
            .collect()
    }

    /// Get provenance info for output
    pub fn get_provenance_info(&self) -> ProvenanceInfo {
        ProvenanceInfo {
            pipeline_version: self.pipeline_version.clone(),
            git_commit: self.git_commit.clone(),
            execution_timestamp: self.execution_start.to_rfc3339(),
            artifact_uris: self.artifact_uris.clone(),
        }
    }

    /// Export provenance to JSON
    pub fn export_json(&self) -> serde_json::Value {
        serde_json::json!({
            "pipeline_version": self.pipeline_version,
            "git_commit": self.git_commit,
            "execution_start": self.execution_start.to_rfc3339(),
            "artifact_uris": self.artifact_uris,
            "node_count": self.node_lineage.len(),
            "validation_events": self.validation_history.len(),
        })
    }

    /// Generate provenance report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Provenance Report ===\n\n");
        report.push_str(&format!("Pipeline Version: {}\n", self.pipeline_version));
        
        if let Some(commit) = &self.git_commit {
            report.push_str(&format!("Git Commit: {}\n", commit));
        }
        
        report.push_str(&format!("Execution Start: {}\n", self.execution_start.to_rfc3339()));
        report.push_str(&format!("Tracked Nodes: {}\n", self.node_lineage.len()));
        report.push_str(&format!("Validation Events: {}\n", self.validation_history.len()));
        
        if !self.artifact_uris.is_empty() {
            report.push_str("\nArtifacts:\n");
            for uri in &self.artifact_uris {
                report.push_str(&format!("  - {}\n", uri));
            }
        }

        // Validation summary
        let passed = self.validation_history.iter().filter(|e| e.passed).count();
        let total = self.validation_history.len();
        
        if total > 0 {
            report.push_str(&format!(
                "\nValidation Pass Rate: {:.2}% ({}/{})\n",
                (passed as f64 / total as f64) * 100.0,
                passed,
                total
            ));
        }

        report
    }

    /// Check provenance completeness
    pub fn check_completeness(&self) -> ProvenanceCompleteness {
        ProvenanceCompleteness {
            has_pipeline_version: !self.pipeline_version.is_empty(),
            has_git_commit: self.git_commit.is_some(),
            has_execution_timestamp: true,
            has_artifacts: !self.artifact_uris.is_empty(),
            node_lineage_count: self.node_lineage.len(),
            validation_event_count: self.validation_history.len(),
        }
    }
}

impl Default for ProvenanceTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Provenance completeness check result
#[derive(Debug, Clone)]
pub struct ProvenanceCompleteness {
    pub has_pipeline_version: bool,
    pub has_git_commit: bool,
    pub has_execution_timestamp: bool,
    pub has_artifacts: bool,
    pub node_lineage_count: usize,
    pub validation_event_count: usize,
}

impl ProvenanceCompleteness {
    pub fn is_complete(&self) -> bool {
        self.has_pipeline_version
            && self.has_git_commit
            && self.has_execution_timestamp
            && self.node_lineage_count > 0
    }

    pub fn score(&self) -> f64 {
        let mut score = 0.0;
        let mut total = 0.0;

        if self.has_pipeline_version {
            score += 1.0;
        }
        total += 1.0;

        if self.has_git_commit {
            score += 1.0;
        }
        total += 1.0;

        if self.has_execution_timestamp {
            score += 1.0;
        }
        total += 1.0;

        if self.has_artifacts {
            score += 1.0;
        }
        total += 1.0;

        if self.node_lineage_count > 0 {
            score += 1.0;
        }
        total += 1.0;

        if self.validation_event_count > 0 {
            score += 1.0;
        }
        total += 1.0;

        score / total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provenance_tracker() {
        let mut tracker = ProvenanceTracker::new();
        assert_eq!(tracker.pipeline_version, "2.4.0-NSN");

        let node_id = Uuid::new_v4();
        tracker.track_node_creation(node_id, "TestAgent", "Test question");

        let lineage = tracker.get_lineage(&node_id);
        assert!(lineage.is_some());
        assert_eq!(lineage.unwrap().created_by, "TestAgent");
    }

    #[test]
    fn test_validation_tracking() {
        let mut tracker = ProvenanceTracker::new();
        let node_id = Uuid::new_v4();

        tracker.track_validation(node_id, true);
        tracker.track_validation(node_id, false);

        let history = tracker.get_validation_history(&node_id);
        assert_eq!(history.len(), 2);
    }

    #[test]
    fn test_completeness_check() {
        let tracker = ProvenanceTracker::new();
        let completeness = tracker.check_completeness();

        assert!(completeness.has_pipeline_version);
        assert!(completeness.has_execution_timestamp);
        assert!(completeness.score() > 0.0);
    }

    #[test]
    fn test_artifact_registration() {
        let mut tracker = ProvenanceTracker::new();
        
        tracker.register_artifact("file:///path/to/artifact1.json".to_string());
        tracker.register_artifact("file:///path/to/artifact2.json".to_string());
        tracker.register_artifact("file:///path/to/artifact1.json".to_string()); // Duplicate

        assert_eq!(tracker.artifact_uris.len(), 2);
    }
}
