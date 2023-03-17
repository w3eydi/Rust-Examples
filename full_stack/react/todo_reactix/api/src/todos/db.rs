use super::models::Todos;
use diesel::prelude::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_all(conn: &mut PgConnection) -> Result<Vec<Todos>, DbError> {
    use crate::schema::todos::dsl::todos;
    
    let take_todos = todos.load::<Todos>(conn)?;
    Ok(take_todos)
}

pub fn create_todo(conn: &mut PgConnection, _title: &str) -> Result<Todos, DbError> {
    use crate::schema::todos::dsl::*;

    let new_todo = Todos {
        id: uuid::Uuid::new_v4().to_string(),
        title: _title.to_owned(),
        completed: false,
    };

    diesel::insert_into(todos)
        .values(&new_todo)
        .execute(conn)?;

    Ok(new_todo)
}

pub fn find_todo_by_uuid(
    uuid: uuid::Uuid,
    conn: &mut PgConnection
) -> Result<Option<Todos>, DbError> {

    use crate::schema::todos::dsl::*;
    // tablodaki id'ye göre sorgu yapılıyor. todos da table_name olarak fonksiyonlar
    // onun üzerinden yazılıyor. option döndürdüğümüz için ek olarak optional()? fonksiyonu 
    // ? operatörüyle birlikte kullanılabilir.
    let find_todo = todos
        .filter(id.eq(uuid.to_string()))
        .first::<Todos>(conn)
        .optional()?;
    Ok(find_todo)
}


pub fn update_todo_by_uuid(
    _uuid: uuid::Uuid,
    _completed: &bool,
    conn: &mut PgConnection
) -> Result<Todos, DbError> {
    use crate::schema::todos::dsl::*;

    // id type must be .to_string() not .to_owned()
    let update_todo = diesel::update(todos.find(_uuid.to_string()))
        .set(completed.eq(_completed))
        .get_result::<Todos>(conn)?;

    Ok(update_todo)
}

pub fn delete_todo_by_uuid(
    _uuid: uuid::Uuid,
    conn: &mut PgConnection
) -> Result<usize, DbError> {
    use crate::schema::todos::dsl::*;
    let delete_todo = diesel::delete(todos
            .find(_uuid.to_string()))
            .execute(conn)?;
    Ok(delete_todo)
}