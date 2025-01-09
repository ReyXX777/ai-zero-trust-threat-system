// api/mod.rs
pub mod endpoints; // Import the endpoints module where your route handlers are defined

use actix_web::web;

/// This function initializes and configures all API routes for the application.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // Register the routes defined in the endpoints module
    cfg.service(endpoints::detect_threats) // Registers the "detect_threats" POST route
       .service(endpoints::behavioral_analysis); // Registers the "behavioral_analysis" GET route
}
