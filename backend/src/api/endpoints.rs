use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::ai::{analyze_threat_score, perform_behavioral_analysis};

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
