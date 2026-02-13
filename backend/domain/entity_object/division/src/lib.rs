use value_object_division::{DivisionId, DivisionName};

#[derive(Debug, Clone)]
pub struct Division {
    id: DivisionId,
    name: DivisionName,
}

impl Division {
    pub fn new(id: DivisionId, name: DivisionName) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> DivisionId {
        self.id
    }

    pub fn name(&self) -> &DivisionName {
        &self.name
    }
}
