use crate::error_handler::CustomError;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use lazy_static::lazy_static;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

    
    lazy_static!(
    static ref POOL: DbPool = {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE URL err!");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        DbPool::new(manager).expect("Failed to create db pool")
    };
);

pub fn init() {
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get db connection");
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get()
        .map_err(|e| CustomError::new(500, format!("Failed getting db connection: {}", e)))
}