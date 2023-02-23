use diesel::prelude::*;
use crate::schema::employees;
use serde::{Serialize, Deserialize};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = employees)]
pub struct Employee {
    pub first_name: String,
    pub last_name: String,
    pub department: String,
    pub salary: i32,
    pub age: i32
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Employees {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub department: String,
    pub salary: i32,
    pub age: i32
}

impl Employees {
    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<Employees>, DbError> {
        use crate::schema::employees::dsl::*;
        let items = employees.load::<Employees>(conn)?;
        Ok(items)
    }

    pub fn find_by_id(employee_id: i32, conn: &mut PgConnection) -> Result<Option<Employees>, DbError> {
        use crate::schema::employees::dsl::*;
        let employee = employees
            .filter(id.eq(employee_id))
            .first::<Employees>(conn)
            .optional()?;
        Ok(employee)
    }

    pub fn create(employee: Employee, conn: &mut PgConnection) -> Result<Self, DbError> {
        use crate::schema::employees::dsl::*;
        let employee = Employee::from(employee);
        let employee = diesel::insert_into(employees)
            .values(employee)
            .get_result(conn)?;
        Ok(employee)
    }

    pub fn update(_id: i32, _employee: Employee, conn: &mut PgConnection) -> Result<Self, DbError> {
        use crate::schema::employees::dsl::*;

        let employee = diesel::update(employees.find(_id))
            .set(_employee)
            .get_result::<Employees>(conn)?;

        Ok(employee)
    }

    pub fn delete(id: i32, conn: &mut PgConnection) -> Result<usize, DbError> {
        use crate::schema::employees::dsl::employees;

        let count = diesel::delete(employees.find(id)).execute(conn)?;
        Ok(count)
    }

    
}

impl Employee {
    fn from(employee: Employee) -> Self {
        Employee {
            first_name: employee.first_name,
            last_name: employee.last_name,
            department: employee.department,
            salary: employee.salary,
            age: employee.age
        }
    }
}