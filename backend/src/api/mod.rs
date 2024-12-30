// api/mod.rs
pub mod endpoints;

use actix_web::web;

/// This function initializes and configures all API routes for the application.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // Register the routes defined in the endpoints module
    cfg.service(endpoints::detect_threats)
       .service(endpoints::behavioral_analysis);
}
