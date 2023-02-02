mod hello_world;
mod mirror_body_string;

use axum::{Router, routing::{patch, post} };
use mirror_body_string::mirror_body_string;

pub fn creating_routes() -> Router {
    Router::new()
        .route("/", patch(hello_world::hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
}