use actix_web::web;
use presenter_book::{BookResponseJdto, CreateBookRequestJdto};
use controller_utils::AppContext;
use usecase_book::BookUsecaseCreateBook;
use crate::BookControllerError;

pub async fn create(
    ctx: web::Data<AppContext>,
    req: web::Json<CreateBookRequestJdto>,
) -> Result<web::Json<BookResponseJdto>, BookControllerError> {
    controller_utils::executor(&ctx, req.into_inner(), |conn, req| {
        di_service::build_book_usecase().execute(conn, &req)
    })
}
