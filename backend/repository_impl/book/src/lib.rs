use diesel::prelude::*;
use repository_book::{BookRepoCreate, BookRepoRead, BookRepoUpdate};
use entity_object_book::Book;
use collection_object_book::Books;
use value_object_book::{BookId, BookTitle};
use value_object_division::DivisionId;
use value_object_user::UserId;
use db_domain_book::schema::books;
use db_domain_book::models::{BookRow, NewBookRow, UpdateBookRow};
use domain_context::PgConn;
use chrono::Utc;
use anyhow::{Result, anyhow};

pub struct BookRepoImpl;

impl BookRepoImpl {
    pub fn new() -> Self {
        Self
    }

    fn row_to_domain(row: BookRow) -> Result<Book> {
        let id = BookId::new(row.id)?;
        let title = BookTitle::new(row.title)?;
        let division_id = DivisionId::new(row.division_id)?;
        let borrowed_by_user_id = row.borrowed_by_user_id
            .map(|uid| UserId::new(uid))
            .transpose()?;
        Ok(Book::new(id, title, division_id, borrowed_by_user_id, row.borrowed_at))
    }
}


impl<Ctx> BookRepoCreate<Ctx> for BookRepoImpl
where
    Ctx: PgConn,
{
    fn create_book(&self, ctx: &mut Ctx, title: &BookTitle, division_id: DivisionId) -> Result<Book> {
        let new_row = NewBookRow {
            title: title.value().to_string(),
            division_id: division_id.value(),
        };

        let row: BookRow = diesel::insert_into(books::table)
            .values(&new_row)
            .returning(BookRow::as_returning())
            .get_result(ctx.conn())?;

        Self::row_to_domain(row)
    }
}

impl<Ctx> BookRepoRead<Ctx> for BookRepoImpl
where
    Ctx: PgConn,
{
    fn find_book(&self, ctx: &mut Ctx, id: BookId) -> Result<Option<Book>> {
        let result = books::table
            .find(id.value())
            .select(BookRow::as_select())
            .first(ctx.conn())
            .optional()?;

        match result {
            Some(row) => Ok(Some(Self::row_to_domain(row)?)),
            None => Ok(None),
        }
    }

    fn list_books(&self, ctx: &mut Ctx) -> Result<Books> {
        let rows: Vec<BookRow> = books::table
            .select(BookRow::as_select())
            .load(ctx.conn())?;

        let items: Result<Vec<Book>> = rows.into_iter()
            .map(Self::row_to_domain)
            .collect();

        Ok(Books::new(items?))
    }
}

impl<Ctx> BookRepoUpdate<Ctx> for BookRepoImpl
where
    Ctx: PgConn,
{
    fn borrow_book(&self, ctx: &mut Ctx, book_id: BookId, user_id: UserId) -> Result<Book> {
        let update = UpdateBookRow {
            borrowed_by_user_id: Some(Some(user_id.value())),
            borrowed_at: Some(Some(Utc::now())),
        };

        let row: BookRow = diesel::update(books::table.find(book_id.value()))
            .set(&update)
            .returning(BookRow::as_returning())
            .get_result(ctx.conn())
            .map_err(|e| anyhow!("Failed to borrow book: {}", e))?;

        Self::row_to_domain(row)
    }

    fn return_book(&self, ctx: &mut Ctx, book_id: BookId) -> Result<Book> {
        let update = UpdateBookRow {
            borrowed_by_user_id: Some(None),
            borrowed_at: Some(None),
        };

        let row: BookRow = diesel::update(books::table.find(book_id.value()))
            .set(&update)
            .returning(BookRow::as_returning())
            .get_result(ctx.conn())
            .map_err(|e| anyhow!("Failed to return book: {}", e))?;

        Self::row_to_domain(row)
    }
}
