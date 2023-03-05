use std::time::Instant;

use actix::*;
use actix_files::NamedFile;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;

use diesel::{prelude::*, r2d2::{self, ConnectionManager}};
use serde_json::json;
use uuid::Uuid;

pub async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}