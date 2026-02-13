use actix_web::web;
use presenter_book::BookResponseJdto;
use controller_utils::AppContext;
use usecase_book::BookUsecaseListBooks;
use crate::BookControllerError;

pub async fn list(
    ctx: web::Data<AppContext>,
) -> Result<web::Json<Vec<BookResponseJdto>>, BookControllerError>
{
    controller_utils::executor(
        &ctx,
        (),
        |conn, _| {
            di_service::build_book_usecase().execute(conn)
        }
    )
}
