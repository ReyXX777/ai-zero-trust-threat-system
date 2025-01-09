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
/// A JSON string containing the threat score or an error message.
pub async fn analyze_threat_score(data: &str) -> String {
    // Analyze threat using the heuristic model
    analyze_threat(data).await
}

/// Performs a behavioral analysis using the behavioral model.
/// 
/// # Returns
/// A JSON string containing the behavioral analysis result or an error message.
pub async fn perform_behavioral_analysis() -> String {
    // Run behavioral analysis using the behavioral model
    run_analysis().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    /// Tests the `analyze_threat_score` function with sample data.
    #[tokio::test]
    async fn test_analyze_threat_score() {
        let test_data = r#"{"type": "malware", "severity": "high"}"#;

        // Perform the analysis and get the result
        let result = analyze_threat_score(test_data).await;

        // Check if the result is a valid JSON and contains "threat_score"
        let parsed_result: Value = serde_json::from_str(&result).expect("Invalid JSON result");
        assert!(parsed_result.get("threat_score").is_some());
        
        // Validate threat score range
        let threat_score = parsed_result["threat_score"]
            .as_f64()
            .expect("threat_score should be a float");
        assert!((0.0..=1.0).contains(&threat_score));
    }

    /// Tests the `perform_behavioral_analysis` function to ensure it's producing the expected result.
    #[tokio::test]
    async fn test_perform_behavioral_analysis() {
        // Run the behavioral analysis
        let result = perform_behavioral_analysis().await;

        // Check that the result is a valid JSON and contains "Behavioral analysis result"
        let parsed_result: Value = serde_json::from_str(&result).expect("Invalid JSON result");
        assert!(parsed_result.get("Behavioral analysis result").is_some());
    }
}
