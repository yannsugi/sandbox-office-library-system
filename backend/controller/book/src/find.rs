use actix_web::web;
use presenter_book::BookResponseJdto;
use controller_utils::AppContext;
use usecase_book::BookUsecaseFindBook;
use crate::BookControllerError;

pub async fn find(
    ctx: web::Data<AppContext>,
    path: web::Path<i32>,
) -> Result<web::Json<BookResponseJdto>, BookControllerError>
{
    controller_utils::executor(
        &ctx,
        *path,
        |conn, path| {
            di_service::build_book_usecase().execute(conn, path)
        }
    )
}
