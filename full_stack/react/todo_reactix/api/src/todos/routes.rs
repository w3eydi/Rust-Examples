use actix_web::{web, get, post, HttpResponse, Error, put};
use crate::DbPool;
use uuid::Uuid;
use super::{models::{Title, BoolLogic}, db};

#[get("/todos")]
async fn get_all(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let find_all = web::block(move || {
        let mut conn = pool.get()?;
        db::find_all(&mut conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().json(find_all))
}

#[post("/todos")]
async fn create_a_todo(
    payload: web::Json<Title>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let todo = web::block(move || {
        let mut conn = pool.get()?;
        db::create_todo(&mut conn, &payload.title)
    })
    .await?
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;

    Ok(HttpResponse::Ok().json(todo))
}

#[get("/todos/{todo_uuid}")]
async fn get_by_uuid(
    id: web::Path<Uuid>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let todo_uuid = id.to_owned();
    let todo = web::block(move || {
        let mut conn = pool.get()?;
        db::find_todo_by_uuid(todo_uuid, &mut conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(todo) = todo {
        Ok(HttpResponse::Ok().json(todo))
    } else {
        let res = HttpResponse::NotFound().body(
            serde_json::json!({
                "error": 404,
                "message": format!("No user found with phone: {id}")
            })
            .to_string(),
        );
        print!("test");
        Ok(res)
    }
}

#[put("/todos/{todo_uuid}")]
async fn update_by_uuid(
    id: web::Path<Uuid>,
    payload: web::Json<BoolLogic>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let todo_uuid = id.to_owned();
    let update_todo = web::block(move || {
        let mut conn = pool.get()?;
        db::update_todo_by_uuid(todo_uuid, &payload.completed, &mut conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(update_todo))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create_a_todo);
    config.service(get_all);
    config.service(get_by_uuid);
    config.service(update_by_uuid);
}