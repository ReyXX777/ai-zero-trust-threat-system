use actix_web::{get, post, web, HttpResponse, Responder, Result};
use serde::Deserialize;
use crate::ai::{analyze_threat_score, perform_behavioral_analysis};

#[derive(Deserialize)]
struct ThreatData {
    data: String,
}

/// Handles the detection of threats by analyzing the threat score from the input data.
#[post("/detect_threats")]
pub async fn detect_threats(data: web::Json<ThreatData>) -> impl Responder {
    // Attempt to analyze the threat score asynchronously
    match analyze_threat_score(&data.data).await {
        // If analysis is successful, return the result as JSON with a 200 OK status
        result => HttpResponse::Ok().json(result),
    }
}

/// Handles behavioral analysis by performing the analysis and returning the result.
#[get("/behavioral_analysis")]
pub async fn behavioral_analysis() -> impl Responder {
    // Perform the behavioral analysis asynchronously
    match perform_behavioral_analysis().await {
        // If successful, return the result as JSON with a 200 OK status
        result => HttpResponse::Ok().json(result),
    }
}

/// Configures the routes for the API.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(detect_threats);
    cfg.service(behavioral_analysis);
}
