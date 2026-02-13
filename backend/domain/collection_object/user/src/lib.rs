use entity_object_user::User;

#[derive(Debug, Clone)]
pub struct Users {
    items: Vec<User>,
}

impl Users {
    pub fn new(items: Vec<User>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &[User] {
        &self.items
    }
}
