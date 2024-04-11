use actix_web::{web, App, HttpServer, Responder};
use chrono::{Utc};

async fn index() -> impl Responder {
    "Hello from Rust web server!"
}

async fn sysInfo() -> impl Responder {
    let now = Utc::now();
    format!("Hello, world! Today is {}.", now.time().to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/sysInfo", web::get().to(sysInfo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
