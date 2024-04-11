use actix_web::{web, App, HttpServer, Responder};
use chrono::{Utc};
use log::info;
use log4rs;

async fn index() -> impl Responder {
    info!("accepting request on /");
    "Hello from Rust web server!"
}

async fn sysInfo() -> impl Responder {
    info!("accepting request on /sysInfo");
    let now = Utc::now();
    format!("Hello, world! Today is {}.", now.time().to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Application started");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/sysInfo", web::get().to(sysInfo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
