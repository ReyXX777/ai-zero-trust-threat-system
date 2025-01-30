// main.rs
mod api;
mod ai;
mod utils;
mod config;

use actix_web::{App, HttpServer, web};
use config::Settings;
use std::sync::Arc;
use tokio::sync::Mutex;
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting AI-Driven Zero Trust Threat Intelligence System...");

    // Load configuration with proper error handling
    let settings = Settings::new().unwrap_or_else(|e| {
        eprintln!("Failed to load configuration: {}", e);
        std::process::exit(1);
    });

    println!("Configuration loaded: {:?}", settings);

    // Initialize the logger (based on environment variables)
    env_logger::init();
    println!("Logger initialized.");

    // Wrap settings in Arc and Mutex for shared access across async tasks
    let shared_settings = Arc::new(Mutex::new(settings));

    // Additional component 1: Health check endpoint
    async fn health_check() -> impl actix_web::Responder {
        actix_web::HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
    }

    // Additional component 2: Middleware for logging incoming requests
    async fn log_requests(req: actix_web::HttpRequest, payload: actix_web::web::Payload) -> impl actix_web::Responder {
        use chrono::Local;
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        println!("[{}] {} {}", timestamp, req.method(), req.uri());
        actix_web::web::Json(serde_json::json!({ "status": "Request logged" }))
    }

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_settings.clone()))  // Make settings available to handlers
            .wrap_fn(log_requests)  // Add request logging middleware
            .route("/health", web::get().to(health_check))  // Add health check endpoint
            .configure(api::init_routes)  // Configure routes
    })
    .bind("0.0.0.0:8080")?  // Bind to address and port
    .run()  // Run the server
    .await
}
