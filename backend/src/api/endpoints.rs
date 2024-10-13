// api/endpoints.rs
use actix_web::{get, post, web, HttpResponse, Responder};
use crate::ai::{behavioral_model, heuristic_model};

#[post("/detect_threats")]
pub async fn detect_threats(data: web::Json<String>) -> impl Responder {
    let result = heuristic_model::analyze_threat(&data).await;
    HttpResponse::Ok().json(result)
}

#[get("/behavioral_analysis")]
pub async fn behavioral_analysis() -> impl Responder {
    let result = behavioral_model::run_analysis().await;
    HttpResponse::Ok().json(result)
}
