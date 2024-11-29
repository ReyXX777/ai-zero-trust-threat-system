use serde_json::{json, Value};

pub async fn analyze_threat(data: &str) -> String {
    let threat_data: Value = match serde_json::from_str(data) {
        Ok(data) => data,
        Err(_) => return json!({"error": "Invalid JSON data"}).to_string(),
    };
    let threat_score = calculate_threat_score(&threat_data);
    json!({"threat_score": threat_score}).to_string()
}

fn calculate_threat_score(data: &Value) -> f32 {
    let base_score = match data["type"].as_str() {
        Some("malware") => 0.9,
        Some("phishing") => 0.7,
        Some("adware") => 0.4,
        _ => 0.1,
    };
    let severity_adjustment = match data["severity"].as_str() {
        Some("high") => 1.0,
        Some("medium") => 0.5,
        Some("low") => 0.2,
        _ => 0.0,
    };
    (base_score + severity_adjustment).min(1.0)
}

#[tokio::main]
async fn main() {
    let example_data = r#"{ "type": "malware", "severity": "high" }"#;
    let result = analyze_threat(example_data).await;
    println!("{}", result);
}
