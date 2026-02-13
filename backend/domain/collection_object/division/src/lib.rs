use entity_object_division::Division;

#[derive(Debug, Clone)]
pub struct Divisions {
    items: Vec<Division>,
}

impl Divisions {
    pub fn new(items: Vec<Division>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &[Division] {
        &self.items
    }
}
