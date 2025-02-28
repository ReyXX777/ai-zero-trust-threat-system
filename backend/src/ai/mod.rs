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

/// Additional component 1: Logging functionality for threat analysis
pub async fn log_threat_analysis(data: &str) {
    use std::fs::OpenOptions;
    use std::io::Write;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("threat_analysis.log")
        .unwrap();
    writeln!(file, "{}", data).unwrap();
}

/// Additional component 2: Timestamp generation for behavioral analysis
pub fn generate_timestamp() -> String {
    use chrono::Local;
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Additional component 3: Validate input data for threat analysis
pub fn validate_input_data(data: &str) -> Result<(), String> {
    use serde_json::Value;
    let parsed_data: Value = serde_json::from_str(data).map_err(|_| "Invalid JSON data".to_string())?;
    if !parsed_data.get("type").is_some() || !parsed_data.get("severity").is_some() {
        return Err("Missing required fields: 'type' or 'severity'".to_string());
    }
    Ok(())
}

/// Additional component 4: Categorize threat based on severity
pub fn categorize_threat(severity: &str) -> String {
    match severity {
        "high" => "Critical",
        "medium" => "High",
        "low" => "Medium",
        _ => "Unknown",
    }.to_string()
}

/// Additional component 5: Generate a summary report for threat analysis
pub fn generate_summary_report(threat_score: f64, severity: &str) -> String {
    format!(
        "Threat Score: {:.2}, Severity: {}, Category: {}",
        threat_score,
        severity,
        categorize_threat(severity)
    )
}

/// Entry point for the program.
#[tokio::main]
async fn main() {
    // Example threat data in JSON format
    let example_data = r#"{ "type": "malware", "severity": "high" }"#;

    // Validate input data
    if let Err(err) = validate_input_data(example_data) {
        println!("Validation error: {}", err);
        return;
    }

    // Analyze the threat and print the result
    let threat_result = analyze_threat_score(example_data).await;
    println!("{}", threat_result);

    // Log the threat analysis result
    log_threat_analysis(&threat_result).await;

    // Perform behavioral analysis and print the result
    let behavioral_result = perform_behavioral_analysis().await;
    println!("{}", behavioral_result);

    // Add a timestamp to the behavioral analysis result
    let timestamp = generate_timestamp();
    let result_with_timestamp = format!("{} - {}", timestamp, behavioral_result);
    println!("{}", result_with_timestamp);

    // Generate and print a summary report
    let parsed_result: serde_json::Value = serde_json::from_str(&threat_result).unwrap();
    let threat_score = parsed_result["threat_score"].as_f64().unwrap();
    let severity = parsed_result["severity"].as_str().unwrap();
    let summary_report = generate_summary_report(threat_score, severity);
    println!("Summary Report: {}", summary_report);
}
