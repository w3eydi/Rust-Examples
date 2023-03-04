use std::env;
use actix::*;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, http, App, HttpServer, middleware};

use diesel::{prelude::*, r2d2::{self, ConnectionManager}};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    let server_address = "localhost";
    let server_port = 8080;

    // Loading .env into environment variable.
    dotenv::dotenv().ok();
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(|| async { "Actix Web!"}))
            
    })
        .workers(2)
        .bind((server_address, server_port))?
        .run();
    println!("Server running at http://{server_address}:{server_port}/");
    app.await
}
