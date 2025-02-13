// api/mod.rs
pub mod endpoints; // Import the endpoints module where your route handlers are defined

use actix_web::web;
use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

/// This function initializes and configures all API routes for the application.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // Register the routes defined in the endpoints module
    cfg.service(endpoints::detect_threats) // Registers the "detect_threats" POST route
       .service(endpoints::behavioral_analysis); // Registers the "behavioral_analysis" GET route
}

/// Additional component 1: Middleware for request logging
pub async fn log_requests(req: actix_web::HttpRequest) -> impl Responder {
    use chrono::Local;
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!("[{}] {} {}", timestamp, req.method(), req.uri());
    actix_web::web::Json(json!({ "status": "Request logged" }))
}

/// Additional component 2: Health check endpoint
#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({ "status": "ok" }))
}

/// Additional component 3: Validate input data for threat analysis
pub async fn validate_input_data(data: &str) -> Result<(), String> {
    use serde_json::Value;
    let parsed_data: Value = serde_json::from_str(data).map_err(|_| "Invalid JSON data".to_string())?;
    if !parsed_data.get("type").is_some() || !parsed_data.get("severity").is_some() {
        return Err("Missing required fields: 'type' or 'severity'".to_string());
    }
    Ok(())
}

/// Additional component 4: Rate limiter middleware
pub async fn rate_limiter(req: actix_web::HttpRequest) -> impl Responder {
    use std::collections::HashMap;
    use std::sync::Mutex;
    use actix_web::web::Data;
    use std::time::{SystemTime, UNIX_EPOCH};

    // Simulate rate limiting logic
    let client_ip = req.connection_info().peer_addr().unwrap_or("unknown").to_string();
    let mut rate_map = Data::<Mutex<HashMap<String, u64>>>::from(req.app_data().unwrap()).lock().unwrap();

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    if let Some(last_request_time) = rate_map.get(&client_ip) {
        if current_time - last_request_time < 5 {
            return HttpResponse::TooManyRequests().json(json!({ "error": "Rate limit exceeded" }));
        }
    }

    rate_map.insert(client_ip, current_time);
    HttpResponse::Ok().json(json!({ "status": "Request allowed" }))
}

/// Additional component 5: Mock threat analysis function
pub async fn analyze_threat_score(data: &str) -> Result<serde_json::Value, String> {
    use serde_json::Value;
    let parsed_data: Value = serde_json::from_str(data).map_err(|_| "Invalid JSON data".to_string())?;
    if !parsed_data.get("type").is_some() || !parsed_data.get("severity").is_some() {
        return Err("Missing required fields: 'type' or 'severity'".to_string());
    }
    Ok(json!({ "threat_score": 0.85 })) // Mock result for demonstration
}

/// Entry point for the Actix Web server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap_fn(log_requests)
            .configure(init_routes)
            .service(health_check)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
