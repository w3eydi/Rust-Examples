mod employees;
mod error_handler;
mod schema;

use std::env;
use actix_web::{web, HttpServer, App};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Loading env file
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE URL err!");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool!");

    let mut listenfd = listenfd::ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(employees::init_routes)
    });
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env file!");
            let port = env::var("PORT").expect("Please set port in .env file!");
            server.bind(format!("{}:{}", host, port))?
        }
    };
    server.run().await
}