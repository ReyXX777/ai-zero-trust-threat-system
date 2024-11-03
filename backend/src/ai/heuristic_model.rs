// ai/heuristic_model.rs
use serde_json::{json, Value};

pub async fn analyze_threat(data: &str) -> String {
    // Parse JSON input into a serde_json::Value object
    let threat_data: Value = match serde_json::from_str(data) {
        Ok(data) => data,
        Err(_) => return json!({"error": "Invalid JSON data"}).to_string(),
    };

    // Calculate the threat score based on parsed data
    let threat_score = calculate_threat_score(&threat_data);

    // Return the result as a JSON-formatted string
    json!({"threat_score": threat_score}).to_string()
}

// Function to calculate the threat score based on heuristics
fn calculate_threat_score(data: &Value) -> f32 {
    // Base score is determined by the type of threat
    let base_score = match data["type"].as_str() {
        Some("malware") => 0.9,
        Some("phishing") => 0.7,
        Some("adware") => 0.4,
        _ => 0.1, // Default score for unknown or low-risk types
    };

    // Severity adjustment factor
    let severity_adjustment = match data["severity"].as_str() {
        Some("high") => 1.0,
        Some("medium") => 0.5,
        Some("low") => 0.2,
        _ => 0.0, // No adjustment if severity is undefined
    };

    // Calculate the final threat score with a weighted adjustment, capped at 1.0
    (base_score + severity_adjustment).min(1.0)
}

// Example usage of analyze_threat function
#[tokio::main]
async fn main() {
    // Example JSON input data
    let example_data = r#"
    {
        "type": "malware",
        "severity": "high"
    }
    "#;

    // Call analyze_threat and print the result
    let result = analyze_threat(example_data).await;
    println!("{}", result);
}
