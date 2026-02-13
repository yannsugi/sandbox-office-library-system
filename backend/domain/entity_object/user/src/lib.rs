use value_object_user::{UserId, UserName};
use value_object_division::DivisionId;

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    name: UserName,
    division_id: DivisionId,
}

impl User {
    pub fn new(id: UserId, name: UserName, division_id: DivisionId) -> Self {
        Self { id, name, division_id }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn division_id(&self) -> DivisionId {
        self.division_id
    }
}
