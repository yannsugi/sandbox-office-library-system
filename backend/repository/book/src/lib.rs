use entity_object_book::Book;
use collection_object_book::Books;
use value_object_book::{BookId, BookTitle};
use value_object_division::DivisionId;
use value_object_user::UserId;
use anyhow::Result;

pub trait BookRepoCreate<Ctx> {
    fn create_book(&self, ctx: &mut Ctx, title: &BookTitle, division_id: DivisionId) -> Result<Book>;
}

pub trait BookRepoRead<Ctx> {
    fn find_book(&self, ctx: &mut Ctx, id: BookId) -> Result<Option<Book>>;
    fn list_books(&self, ctx: &mut Ctx) -> Result<Books>;
}

pub trait BookRepoUpdate<Ctx> {
    fn borrow_book(&self, ctx: &mut Ctx, book_id: BookId, user_id: UserId) -> Result<Book>;
    fn return_book(&self, ctx: &mut Ctx, book_id: BookId) -> Result<Book>;
}

pub trait BookRepoDelete<Ctx> {
    // No delete operations for now
}
