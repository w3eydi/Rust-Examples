use actix_web::{get, post, put, delete, web, HttpResponse, Error};
use serde_json::json;
use crate::DbPool;

use crate::employees::{Employees, Employee};

#[get("/employees")]
async fn find_all(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let employees = web::block(move || {
        let mut conn = pool.get()?;
        Employees::find_all(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(employees))
}

#[get("/employees/{id}")]
async fn find_id(
    id: web::Path<i32>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let employee = web::block(move || {
        let mut conn = pool.get()?;
        Employees::find_by_id(id.into_inner(), &mut conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().json(employee))
}

#[post("/employees")]
async fn create(
    employee: web::Json<Employee>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let employee = web::block(move || {
        let mut conn = pool.get()?;
        Employees::create(employee.into_inner(), &mut conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().json(employee))
}

#[put("/employees/{id}")]
async fn update(
    id: web::Path<i32>,
    _employee: web::Json<Employee>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let employee = web::block(move || {
        let mut conn = pool.get()?;
        Employees::update(id.into_inner(), _employee.into_inner(), &mut conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(employee))
}

#[delete("/employees/{id}")]
async fn delete(
    id: web::Path<i32>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let deleted_employee = web::block(move || {
        let mut conn = pool.get()?;
        Employees::delete(id.into_inner(), &mut conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_employee })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find_id);
    config.service(create);
    config.service(update);
    config.service(delete);
}