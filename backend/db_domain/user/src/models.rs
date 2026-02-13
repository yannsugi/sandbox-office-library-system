use diesel::prelude::*;
use super::schema::users;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct UserRow {
    pub id: i32,
    pub name: String,
    pub division_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUserRow {
    pub name: String,
    pub division_id: i32,
}
