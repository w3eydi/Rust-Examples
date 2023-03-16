use serde::{Serialize, Deserialize};
use diesel::{Insertable, Queryable};

use crate::schema::todos;

#[derive(Clone, Deserialize, Serialize, Insertable, Queryable, diesel::AsChangeset)]
#[diesel(table_name = todos)]
pub struct Todos {
    pub id: String,
    pub title: String,
    pub completed: bool
}

#[derive(Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = todos)]
pub struct Title {
    pub title: String,
}

#[derive(Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = todos)]
pub struct BoolLogic {
    pub completed: bool,
}