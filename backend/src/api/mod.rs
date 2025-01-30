// api/mod.rs
pub mod endpoints; // Import the endpoints module where your route handlers are defined

use actix_web::web;

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
