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

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_settings.clone()))  // Make settings available to handlers
            .configure(api::init_routes)  // Configure routes
    })
    .bind("0.0.0.0:8080")?  // Bind to address and port
    .run()  // Run the server
    .await
}
