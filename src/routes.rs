use actix_web::web;

// Import handler functions from handlers.rs
use crate::handlers::{index, sysInfo};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index))
       .route("/sysInfo", web::get().to(sysInfo));
}