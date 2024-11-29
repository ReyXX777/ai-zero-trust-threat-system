use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use crate::ai::{analyze_threat_score, perform_behavioral_analysis};

#[derive(Deserialize)]
struct ThreatData {
    data: String,
}

#[post("/detect_threats")]
pub async fn detect_threats(data: web::Json<ThreatData>) -> impl Responder {
    let result = analyze_threat_score(&data.data).await;
    HttpResponse::Ok().json(result)
}

#[get("/behavioral_analysis")]
pub async fn behavioral_analysis() -> impl Responder {
    let result = perform_behavioral_analysis().await;
    HttpResponse::Ok().json(result)
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(detect_threats);
    cfg.service(behavioral_analysis);
}
