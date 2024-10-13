// api/mod.rs
pub mod endpoints;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(endpoints::detect_threats);
    cfg.service(endpoints::behavioral_analysis);
}
