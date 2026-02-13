use value_object_book::{BookId, BookTitle};
use value_object_division::DivisionId;
use value_object_user::UserId;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Book {
    id: BookId,
    title: BookTitle,
    division_id: DivisionId,
    borrowed_by_user_id: Option<UserId>,
    borrowed_at: Option<DateTime<Utc>>,
}

impl Book {
    pub fn new(
        id: BookId,
        title: BookTitle,
        division_id: DivisionId,
        borrowed_by_user_id: Option<UserId>,
        borrowed_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            title,
            division_id,
            borrowed_by_user_id,
            borrowed_at,
        }
    }

    pub fn id(&self) -> BookId {
        self.id
    }

    pub fn title(&self) -> &BookTitle {
        &self.title
    }

    pub fn division_id(&self) -> DivisionId {
        self.division_id
    }

    pub fn borrowed_by_user_id(&self) -> Option<UserId> {
        self.borrowed_by_user_id
    }

    pub fn borrowed_at(&self) -> Option<DateTime<Utc>> {
        self.borrowed_at
    }

    pub fn is_borrowed(&self) -> bool {
        self.borrowed_by_user_id.is_some()
    }
}
