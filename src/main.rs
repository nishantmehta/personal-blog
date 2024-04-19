use actix_web::{App, HttpServer};
use log::info;
use log4rs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};
use blogs::{MyObject};
use crate::blogs::{retrieve_objects, store_objects};
use uuid::Uuid;


// Import the routes module
mod routes;
mod handlers;
mod blogs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let file_path = Path::new("objects.json");

    // Create some objects
    let objects = vec![
        MyObject { date: Utc::now(), string: "Object 1".to_string(), id: Uuid::new_v4()},
        MyObject { date: Utc::now(), string: "Object 2".to_string(), id: Uuid::new_v4()},
    ];

    // Store the objects
    match store_objects(&file_path, &objects) {
        Ok(()) => println!("Objects stored successfully!"),
        Err(e) => println!("Error storing objects: {}", e),
    }

    info!("Application started");

    HttpServer::new(|| {
        App::new()
            .configure(routes::configure_routes) // Use the configure_routes function
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}