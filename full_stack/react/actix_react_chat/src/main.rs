use std::env;
use actix::*;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web::{self, get}, http, App, HttpServer, middleware};

use diesel::{prelude::*, r2d2::{self, ConnectionManager}};

mod routes;
mod db;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    let server_address = "localhost";
    let server_port = 8080;

    // Loading .env into environment variable.
    dotenv::dotenv().ok();
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE URL err!");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool!");

    let app = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8080")
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    //.supports_credentials()
                    .max_age(3600)
            )
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(routes::index))
            .service(Files::new("/", "./static"))
    })
        .workers(2)
        .bind(format!("{server_address}:{server_port}"))?
        .run();
    println!("Server running at http://{server_address}:{server_port}/");
    app.await
}
