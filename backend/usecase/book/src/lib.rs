use feature_book::{
    BookFeatureCreateBook, BookFeatureFindBook, BookFeatureListBooks,
    BookFeatureBorrowBook, BookFeatureReturnBook,
};
use feature_user::UserFeatureFindUser;
use value_object_book::{BookId, BookTitle};
use value_object_division::DivisionId;
use value_object_user::UserId;
use presenter_book::{CreateBookRequestJdto, BorrowBookRequestJdto, BookResponseJdto};
use anyhow::Result;

pub struct BookUsecaseService<BookFeature, UserFeature> {
    book_feature: BookFeature,
    user_feature: UserFeature,
}

impl<BookFeature, UserFeature> BookUsecaseService<BookFeature, UserFeature> {
    pub fn new(book_feature: BookFeature, user_feature: UserFeature) -> Self {
        Self { book_feature, user_feature }
    }
}

pub trait BookUsecaseCreateBook<Ctx> {
    fn execute(&self, ctx: &mut Ctx, req: &CreateBookRequestJdto) -> Result<BookResponseJdto>;
}

impl<Ctx, BookFeature, UserFeature> BookUsecaseCreateBook<Ctx> for BookUsecaseService<BookFeature, UserFeature>
where
    BookFeature: BookFeatureCreateBook<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx, req: &CreateBookRequestJdto) -> Result<BookResponseJdto> {
        let title = BookTitle::new(req.title.clone())?;
        let division_id = DivisionId::new(req.division_id)?;
        let book = self.book_feature.execute(ctx, &title, division_id)?;

        Ok(BookResponseJdto {
            id: book.id().value(),
            title: book.title().value().to_string(),
            division_id: book.division_id().value(),
            borrowed_by_user_id: book.borrowed_by_user_id().map(|uid| uid.value()),
        })
    }
}

pub trait BookUsecaseFindBook<Ctx> {
    fn execute(&self, ctx: &mut Ctx, book_id: i32) -> Result<BookResponseJdto>;
}

impl<Ctx, BookFeature, UserFeature> BookUsecaseFindBook<Ctx> for BookUsecaseService<BookFeature, UserFeature>
where
    BookFeature: BookFeatureFindBook<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx, book_id: i32) -> Result<BookResponseJdto> {
        let id = BookId::new(book_id)?;
        let book = self.book_feature.execute(ctx, id)?;

        Ok(BookResponseJdto {
            id: book.id().value(),
            title: book.title().value().to_string(),
            division_id: book.division_id().value(),
            borrowed_by_user_id: book.borrowed_by_user_id().map(|uid| uid.value()),
        })
    }
}

pub trait BookUsecaseListBooks<Ctx> {
    fn execute(&self, ctx: &mut Ctx) -> Result<Vec<BookResponseJdto>>;
}

impl<Ctx, BookFeature, UserFeature> BookUsecaseListBooks<Ctx> for BookUsecaseService<BookFeature, UserFeature>
where
    BookFeature: BookFeatureListBooks<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx) -> Result<Vec<BookResponseJdto>> {
        let books = self.book_feature.execute(ctx)?;

        Ok(books.items().iter().map(|book| {
            BookResponseJdto {
                id: book.id().value(),
                title: book.title().value().to_string(),
                division_id: book.division_id().value(),
                borrowed_by_user_id: book.borrowed_by_user_id().map(|uid| uid.value()),
            }
        }).collect())
    }
}

pub trait BookUsecaseBorrowBook<Ctx> {
    fn execute(&self, ctx: &mut Ctx, book_id: i32, req: &BorrowBookRequestJdto) -> Result<BookResponseJdto>;
}

impl<Ctx, BookFeature, UserFeature> BookUsecaseBorrowBook<Ctx> for BookUsecaseService<BookFeature, UserFeature>
where
    BookFeature: BookFeatureBorrowBook<Ctx>,
    UserFeature: UserFeatureFindUser<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx, book_id: i32, req: &BorrowBookRequestJdto) -> Result<BookResponseJdto> {
        let book_id = BookId::new(book_id)?;
        let user_id = UserId::new(req.user_id)?;

        // Verify user exists
        self.user_feature.execute(ctx, user_id)?;

        // Borrow book
        let book = self.book_feature.execute(ctx, book_id, user_id)?;

        Ok(BookResponseJdto {
            id: book.id().value(),
            title: book.title().value().to_string(),
            division_id: book.division_id().value(),
            borrowed_by_user_id: book.borrowed_by_user_id().map(|uid| uid.value()),
        })
    }
}

pub trait BookUsecaseReturnBook<Ctx> {
    fn execute(&self, ctx: &mut Ctx, book_id: i32) -> Result<BookResponseJdto>;
}

impl<Ctx, BookFeature, UserFeature> BookUsecaseReturnBook<Ctx> for BookUsecaseService<BookFeature, UserFeature>
where
    BookFeature: BookFeatureReturnBook<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx, book_id: i32) -> Result<BookResponseJdto> {
        let book_id = BookId::new(book_id)?;
        let book = self.book_feature.execute(ctx, book_id)?;

        Ok(BookResponseJdto {
            id: book.id().value(),
            title: book.title().value().to_string(),
            division_id: book.division_id().value(),
            borrowed_by_user_id: book.borrowed_by_user_id().map(|uid| uid.value()),
        })
    }
}
