use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateDivisionRequestJdto {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct DivisionResponseJdto {
    pub id: i32,
    pub name: String,
}
