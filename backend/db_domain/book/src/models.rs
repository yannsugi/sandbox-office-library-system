use diesel::prelude::*;
use chrono::{DateTime, Utc};
use super::schema::books;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = books)]
pub struct BookRow {
    pub id: i32,
    pub title: String,
    pub division_id: i32,
    pub borrowed_by_user_id: Option<i32>,
    pub borrowed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = books)]
pub struct NewBookRow {
    pub title: String,
    pub division_id: i32,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = books)]
pub struct UpdateBookRow {
    pub borrowed_by_user_id: Option<Option<i32>>,
    pub borrowed_at: Option<Option<DateTime<Utc>>>,
}
