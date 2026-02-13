use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateBookRequestJdto {
    pub title: String,
    pub division_id: i32,
}

#[derive(Debug, Serialize)]
pub struct BookResponseJdto {
    pub id: i32,
    pub title: String,
    pub division_id: i32,
    pub borrowed_by_user_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct BorrowBookRequestJdto {
    pub user_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct ReturnBookRequestJdto {}

#[derive(Debug, Serialize)]
pub struct ErrorJdto {
    pub code: String,
    pub message: String,
}
