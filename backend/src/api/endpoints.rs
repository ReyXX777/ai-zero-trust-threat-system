use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
struct ThreatData {
    data: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

/// Handles the detection of threats by analyzing the threat score from the input data.
#[post("/detect_threats")]
pub async fn detect_threats(data: web::Json<ThreatData>) -> impl Responder {
    // Attempt to analyze the threat score asynchronously
    match analyze_threat_score(&data.data).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::BadRequest().json(ErrorResponse { error: err }),
    }
}

/// Handles behavioral analysis by performing the analysis and returning the result.
#[get("/behavioral_analysis")]
pub async fn behavioral_analysis() -> impl Responder {
    // Perform the behavioral analysis asynchronously
    match perform_behavioral_analysis().await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse { error: err }),
    }
}

/// Configures the routes for the API.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(detect_threats);
    cfg.service(behavioral_analysis);
}

/// Additional component 1: Middleware for logging incoming requests
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
pub async fn analyze_threat_score(data: &str) -> Result<serde_json::Value, String> {
    use serde_json::Value;
    let parsed_data: Value = serde_json::from_str(data).map_err(|_| "Invalid JSON data".to_string())?;
    if !parsed_data.get("type").is_some() || !parsed_data.get("severity").is_some() {
        return Err("Missing required fields: 'type' or 'severity'".to_string());
    }
    Ok(json!({ "threat_score": 0.85 })) // Mock result for demonstration
}

/// Additional component 4: Perform behavioral analysis
pub async fn perform_behavioral_analysis() -> Result<serde_json::Value, String> {
    Ok(json!({ "Behavioral analysis result": 0.75 })) // Mock result for demonstration
}

/// Additional component 5: Add a rate limiter middleware
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

/// Entry point for the Actix Web server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap_fn(log_requests)
            .configure(configure_routes)
            .service(health_check)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
