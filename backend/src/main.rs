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

    let settings = Settings::new().unwrap_or_else(|e| {
        eprintln!("Failed to load configuration: {}", e);
        std::process::exit(1);
    });
    println!("Configuration loaded: {:?}", settings);

    env_logger::init();
    println!("Logger initialized.");

    let shared_settings = Arc::new(Mutex::new(settings));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_settings.clone()))
            .configure(api::init_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
