use serde_json::{json, Value};
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

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

    // Ensure required fields are present
    if !threat_data.get("type").is_some() || !threat_data.get("severity").is_some() {
        return json!({ "error": "Missing required fields: 'type' or 'severity'" }).to_string();
    }

    // Calculate the threat score based on the parsed data
    let threat_score = calculate_threat_score(&threat_data);

    // Return the threat score as a JSON string
    json!({
        "type": threat_data["type"],
        "severity": threat_data["severity"],
        "threat_score": threat_score
    })
    .to_string()
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
        Some("ransomware") => 0.95, // Example of extensibility
        _ => 0.1, // Default low base score for unknown types
    };

    // Adjust the score based on the "severity" of the threat
    let severity_adjustment = match data["severity"].as_str() {
        Some("high") => 1.0,
        Some("medium") => 0.5,
        Some("low") => 0.2,
        _ => 0.0, // Default adjustment for unknown severity
    };

    // Calculate and ensure the score does not exceed 1.0
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

    // Additional component 1: Log the threat analysis result to a file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("threat_analysis.log")
        .unwrap();
    writeln!(file, "{}", result).unwrap();

    // Additional component 2: Add a timestamp to the threat analysis result
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let result_with_timestamp = json!({
        "timestamp": timestamp,
        "analysis_result": result
    }).to_string();
    println!("{}", result_with_timestamp);

    // Additional component 3: Validate the input data structure
    let invalid_data = r#"{ "type": "malware" }"#; // Missing severity
    let invalid_result = analyze_threat(invalid_data).await;
    println!("{}", invalid_result);

    // Additional component 4: Add a function to categorize threats
    let threat_category = categorize_threat(&serde_json::from_str(example_data).unwrap());
    println!("Threat category: {}", threat_category);

    // Additional component 5: Add a function to generate a report summary
    let report_summary = generate_report_summary(&result);
    println!("Report summary: {}", report_summary);
}

/// Categorizes the threat based on its type.
/// 
/// # Arguments
/// * `data` - A `serde_json::Value` object containing the threat details.
/// 
/// # Returns
/// A string representing the threat category.
fn categorize_threat(data: &Value) -> String {
    match data["type"].as_str() {
        Some("malware") => "Critical",
        Some("phishing") => "High",
        Some("adware") => "Medium",
        Some("ransomware") => "Critical",
        _ => "Low",
    }.to_string()
}

/// Generates a summary of the threat analysis report.
/// 
/// # Arguments
/// * `result` - A JSON-formatted string containing the analysis result.
/// 
/// # Returns
/// A string representing the report summary.
fn generate_report_summary(result: &str) -> String {
    let result_value: Value = serde_json::from_str(result).unwrap();
    format!(
        "Threat type: {}, Severity: {}, Score: {}",
        result_value["type"], result_value["severity"], result_value["threat_score"]
    )
}
