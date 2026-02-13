use actix_web::web;
use presenter_book::{BorrowBookRequestJdto, BookResponseJdto};
use controller_utils::AppContext;
use usecase_book::BookUsecaseBorrowBook;
use crate::BookControllerError;

pub async fn borrow(
    ctx: web::Data<AppContext>,
    path: web::Path<i32>,
    req: web::Json<BorrowBookRequestJdto>,
) -> Result<web::Json<BookResponseJdto>, BookControllerError>
{
    controller_utils::executor(
        &ctx,
        (*path, req.into_inner()),
        |conn, (path, req)| {
            di_service::build_book_usecase().execute(conn, path, &req)
        }
    )
}
