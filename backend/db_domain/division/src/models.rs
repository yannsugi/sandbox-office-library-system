use diesel::prelude::*;
use super::schema::divisions;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = divisions)]
pub struct DivisionRow {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = divisions)]
pub struct NewDivisionRow {
    pub name: String,
}
