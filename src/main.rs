use actix_web::{App, HttpServer};
use log::info;
use log4rs;

// Import the routes module
mod routes;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Application started");

    HttpServer::new(|| {
        App::new()
            .configure(routes::configure_routes) // Use the configure_routes function
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}