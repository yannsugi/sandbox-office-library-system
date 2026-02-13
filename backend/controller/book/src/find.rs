use actix_web::web;
use presenter_book::BookResponseJdto;
use controller_utils::AppContext;
use usecase_book::BookUsecaseFindBook;
use crate::BookControllerError;
use log::info;

pub async fn find(
    ctx: web::Data<AppContext>,
    path: web::Path<i32>,
) -> Result<web::Json<BookResponseJdto>, BookControllerError>
{
    info!("GET /books/{} - Finding book", *path);
    let result = controller_utils::executor(
        &ctx,
        *path,
        |conn, path| {
            di_service::build_book_usecase().execute(conn, path)
        }
    );
    if result.is_ok() {
        info!("GET /books/{} - Book found", *path);
    }
    result
}
