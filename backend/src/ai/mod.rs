// ai-zero-trust-threat-system/backend/src/ai/mod.rs

// Import sub-modules
pub mod heuristic_model;
pub mod behavioral_model;

// Publicly expose key functions for use in other parts of the system
use heuristic_model::analyze_threat;
use behavioral_model::run_analysis;

/// Analyze a threat based on provided JSON data
/// # Arguments
/// * `data` - A JSON string containing threat information
///
/// # Returns
/// * A JSON string with a threat score
pub async fn analyze_threat_score(data: &str) -> String {
    analyze_threat(data).await
}

/// Perform behavioral analysis for an input tensor
///
/// # Returns
/// * A JSON-formatted string with the behavioral analysis result
pub async fn perform_behavioral_analysis() -> String {
    run_analysis().await
}
