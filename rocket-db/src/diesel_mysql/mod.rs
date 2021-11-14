
// extern crate dotenv;

pub mod models;
pub use crate::schema::*;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{Post, NewPost};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// main page post function
pub fn create_post(conn: &MysqlConnection, title: &str, body: &str) -> Post {
    //use super::schema::posts;

    let new_post = NewPost {title, body};

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");
    posts::table.order(posts::id.desc()).first(conn).unwrap()
}
