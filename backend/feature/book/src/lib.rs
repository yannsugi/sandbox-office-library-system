use repository_book::{BookRepoCreate, BookRepoRead, BookRepoUpdate};
use entity_object_book::Book;
use collection_object_book::Books;
use value_object_book::{BookId, BookTitle};
use value_object_division::DivisionId;
use value_object_user::UserId;
use anyhow::{Result, anyhow};

pub struct BookFeatureService<BookRepo> {
    book_repo: BookRepo,
}

impl<BookRepo> BookFeatureService<BookRepo> {
    pub fn new(book_repo: BookRepo) -> Self {
        Self { book_repo }
    }
}

pub trait BookFeatureCreateBook<Ctx> {
    fn execute(&self, ctx: &mut Ctx, title: &BookTitle, division_id: DivisionId) -> Result<Book>;
}

impl<Ctx, BookRepo> BookFeatureCreateBook<Ctx> for BookFeatureService<BookRepo>
where
    BookRepo: BookRepoCreate<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx, title: &BookTitle, division_id: DivisionId) -> Result<Book> {
        self.book_repo.create_book(ctx, title, division_id)
    }
}

pub trait BookFeatureFindBook<Ctx> {
    fn execute(&self, ctx: &mut Ctx, id: BookId) -> Result<Book>;
}

impl<Ctx, BookRepo> BookFeatureFindBook<Ctx> for BookFeatureService<BookRepo>
where
    BookRepo: BookRepoRead<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx, id: BookId) -> Result<Book> {
        self.book_repo.find_book(ctx, id)?
            .ok_or_else(|| anyhow!("Book not found"))
    }
}

pub trait BookFeatureListBooks<Ctx> {
    fn execute(&self, ctx: &mut Ctx) -> Result<Books>;
}

impl<Ctx, BookRepo> BookFeatureListBooks<Ctx> for BookFeatureService<BookRepo>
where
    BookRepo: BookRepoRead<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx) -> Result<Books> {
        self.book_repo.list_books(ctx)
    }
}

pub trait BookFeatureBorrowBook<Ctx> {
    fn execute(&self, ctx: &mut Ctx, book_id: BookId, user_id: UserId) -> Result<Book>;
}

impl<Ctx, BookRepo> BookFeatureBorrowBook<Ctx> for BookFeatureService<BookRepo>
where
    BookRepo: BookRepoUpdate<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx, book_id: BookId, user_id: UserId) -> Result<Book> {
        self.book_repo.borrow_book(ctx, book_id, user_id)
    }
}

pub trait BookFeatureReturnBook<Ctx> {
    fn execute(&self, ctx: &mut Ctx, book_id: BookId) -> Result<Book>;
}

impl<Ctx, BookRepo> BookFeatureReturnBook<Ctx> for BookFeatureService<BookRepo>
where
    BookRepo: BookRepoUpdate<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx, book_id: BookId) -> Result<Book> {
        self.book_repo.return_book(ctx, book_id)
    }
}
