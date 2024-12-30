pub mod heuristic_model;
pub mod behavioral_model;

use heuristic_model::analyze_threat;
use behavioral_model::run_analysis;

/// Analyzes the threat score based on the input data.
/// 
/// # Arguments
/// * `data` - JSON-formatted string representing threat details.
/// 
/// # Returns
/// A JSON string containing the threat score.
pub async fn analyze_threat_score(data: &str) -> String {
    // Analyze threat using the heuristic model
    analyze_threat(data).await
}

/// Performs a behavioral analysis using the behavioral model.
/// 
/// # Returns
/// A JSON string containing the behavioral analysis result.
pub async fn perform_behavioral_analysis() -> String {
    // Run behavioral analysis using the behavioral model
    run_analysis().await
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the `analyze_threat_score` function with sample data.
    #[tokio::test]
    async fn test_analyze_threat_score() {
        let test_data = r#"{"type": "malware", "severity": "high"}"#;
        
        // Perform the analysis and get the result
        let result = analyze_threat_score(test_data).await;

        // Check if the threat score is within the expected range
        assert!(result.contains("\"threat_score\":"));
        
        // Additional validation could be done here, such as parsing the result and ensuring
        // it falls within the expected range (0.0 to 1.0).
    }

    /// Tests the `perform_behavioral_analysis` function to ensure it's producing the expected result.
    #[tokio::test]
    async fn test_perform_behavioral_analysis() {
        // Run the behavioral analysis
        let result = perform_behavioral_analysis().await;
        
        // Check that the result contains "Behavioral analysis result" string
        assert!(result.contains("Behavioral analysis result"));
        
        // Additional validation could be done to ensure the output is a valid JSON structure.
    }
}
