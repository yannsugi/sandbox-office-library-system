use actix_web::web;
use presenter_book::{BorrowBookRequestJdto, BookResponseJdto};
use controller_utils::AppContext;
use usecase_book::BookUsecaseBorrowBook;
use crate::BookControllerError;
use log::info;

pub async fn borrow(
    ctx: web::Data<AppContext>,
    path: web::Path<i32>,
    req: web::Json<BorrowBookRequestJdto>,
) -> Result<web::Json<BookResponseJdto>, BookControllerError>
{
    info!("POST /books/{}/borrow - User {} borrowing book", *path, req.user_id);
    let result = controller_utils::executor(
        &ctx,
        (*path, req.into_inner()),
        |conn, (path, req)| {
            di_service::build_book_usecase().execute(conn, path, &req)
        }
    );
    if result.is_ok() {
        info!("POST /books/{}/borrow - Book borrowed successfully", *path);
    }
    result
}
