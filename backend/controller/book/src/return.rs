use actix_web::web;
use presenter_book::{ReturnBookRequestJdto, BookResponseJdto};
use controller_utils::AppContext;
use usecase_book::BookUsecaseReturnBook;
use crate::BookControllerError;
use log::info;

pub async fn r#return(
    ctx: web::Data<AppContext>,
    path: web::Path<i32>,
    _req: web::Json<ReturnBookRequestJdto>,
) -> Result<web::Json<BookResponseJdto>, BookControllerError>
{
    info!("POST /books/{}/return - Returning book", *path);
    let result = controller_utils::executor(
        &ctx,
        *path,
        |conn, path| {
            di_service::build_book_usecase().execute(conn, path)
        }
    );
    if result.is_ok() {
        info!("POST /books/{}/return - Book returned successfully", *path);
    }
    result
}
