use entity_object_book::Book;

#[derive(Debug, Clone)]
pub struct Books {
    items: Vec<Book>,
}

impl Books {
    pub fn new(items: Vec<Book>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &[Book] {
        &self.items
    }
}
