use actix_web::{get, post, put, delete, web, Responder, HttpResponse};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/tweets")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Tweet#index")
}

#[post("/tweets")]
async fn create() -> impl Responder {
    HttpResponse::Ok().body("Tweet#new")
}

#[get("/tweets/{id}")]
async fn show(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Tweet#show {}", id))
}

#[put("/tweets/{id}")]
async fn update(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Tweet#edit {id}"))
}

#[delete("/tweets/{id}")]
async fn destroy(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Tweet#delete {id}"))
}