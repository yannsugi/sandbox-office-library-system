use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequestJdto {
    pub name: String,
    pub division_id: i32,
}

#[derive(Debug, Serialize)]
pub struct UserResponseJdto {
    pub id: i32,
    pub name: String,
    pub division_id: i32,
}
