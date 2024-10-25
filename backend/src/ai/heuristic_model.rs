// ai/heuristic_model.rs
use serde_json::json;

pub async fn analyze_threat(data: &str) -> String {
    let threat_data: serde_json::Value = match serde_json::from_str(data) {
        Ok(data) => data,
        Err(_) => return json!({"error": "Invalid JSON data"}).to_string(),
    };

    let threat_score = calculate_threat_score(&threat_data);
    json!({"threat_score": threat_score}).to_string()
}

fn calculate_threat_score(data: &serde_json::Value) -> f32 {
    // Heuristic scoring based on threat type and severity
    let base_score = match data["type"].as_str() {
        Some("malware") => 0.9,
        Some("phishing") => 0.7,
        Some("adware") => 0.4,
        _ => 0.1, // Default score for unknown or low-risk types
    };

    // Adjust score based on severity level if available
    let severity_adjustment = match data["severity"].as_str() {
        Some("high") => 1.0,
        Some("medium") => 0.5,
        Some("low") => 0.2,
        _ => 0.0, // No adjustment if severity is not defined
    };

    // Calculate final threat score with a weighted adjustment
    (base_score + severity_adjustment).min(1.0)
}
