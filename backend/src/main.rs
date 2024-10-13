// main.rs
mod api;
mod ai;
mod utils;
mod config;

use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting AI-Driven Zero Trust Threat Intelligence System...");

    HttpServer::new(|| {
        App::new()
            .configure(api::init_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
