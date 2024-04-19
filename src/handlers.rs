use actix_web::{Responder};
use chrono::{Utc};
use log::info;
use actix_web::{get, HttpResponse}; 
use crate::blogs::{retrieve_objects, store_objects};
use std::path::Path;

pub async fn index() -> impl Responder {
    info!("accepting request on /");
    "Hello from Rust web server!"
}

pub async fn sysInfo() -> impl Responder {
    info!("accepting request on /sysInfo");
    let now = Utc::now();
    format!("Hello, world! Today is {}.", now.time().to_string())
}

pub async fn list_objects() -> impl Responder {
    let file_path = Path::new("objects.json");
    let html_result = retrieve_objects(file_path);

    match html_result {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}