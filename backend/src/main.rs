// main.rs
mod api;
mod ai;
mod utils;
mod config;

use actix_web::{App, HttpServer, web};
use config::Settings;
use std::sync::Arc;
use tokio::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting AI-Driven Zero Trust Threat Intelligence System...");

    // Load configuration settings
    let settings = match Settings::new() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };
    println!("Configuration loaded: {:?}", settings);

    // Initialize logger
    env_logger::init();
    println!("Logger initialized.");

    // Wrap settings in an Arc and Mutex for shared state across threads
    let shared_settings = Arc::new(Mutex::new(settings));

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            // Inject shared settings into app data for use in handlers
            .app_data(web::Data::new(shared_settings.clone()))
            // Initialize API routes
            .configure(api::init_routes)
    })
    .bind("0.0.0.0:8080")? // Listen on all interfaces on port 8080
    .run()
    .await
}
