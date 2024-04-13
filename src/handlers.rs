use actix_web::{Responder};
use chrono::{Utc};
use log::info;

pub async fn index() -> impl Responder {
    info!("accepting request on /");
    "Hello from Rust web server!"
}

pub async fn sysInfo() -> impl Responder {
    info!("accepting request on /sysInfo");
    let now = Utc::now();
    format!("Hello, world! Today is {}.", now.time().to_string())
}