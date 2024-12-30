use serde_json::{json, Value};

/// Analyzes the threat data and calculates a threat score.
///
/// # Arguments
/// * `data` - A JSON-formatted string containing threat details.
///
/// # Returns
/// A JSON string with the calculated threat score or an error message.
pub async fn analyze_threat(data: &str) -> String {
    // Parse the input JSON string into a `Value` object
    let threat_data: Value = match serde_json::from_str(data) {
        Ok(data) => data,
        Err(_) => {
            return json!({ "error": "Invalid JSON data" }).to_string();
        }
    };

    // Calculate the threat score based on the parsed data
    let threat_score = calculate_threat_score(&threat_data);

    // Return the threat score as a JSON string
    json!({ "threat_score": threat_score }).to_string()
}

/// Calculates the threat score based on the provided data.
///
/// # Arguments
/// * `data` - A `serde_json::Value` object containing the threat details.
///
/// # Returns
/// A float representing the threat score (0.0 to 1.0).
fn calculate_threat_score(data: &Value) -> f32 {
    // Determine the base score based on the "type" of threat
    let base_score = match data["type"].as_str() {
        Some("malware") => 0.9,
        Some("phishing") => 0.7,
        Some("adware") => 0.4,
        _ => 0.1,
    };

    // Adjust the score based on the "severity" of the threat
    let severity_adjustment = match data["severity"].as_str() {
        Some("high") => 1.0,
        Some("medium") => 0.5,
        Some("low") => 0.2,
        _ => 0.0,
    };

    // Ensure the score does not exceed 1.0
    (base_score + severity_adjustment).min(1.0)
}

/// Entry point of the program.
#[tokio::main]
async fn main() {
    // Example threat data in JSON format
    let example_data = r#"{ "type": "malware", "severity": "high" }"#;

    // Analyze the threat and print the result
    let result = analyze_threat(example_data).await;
    println!("{}", result);
}
