use actix_web::web;
use presenter_book::{BookResponseJdto, CreateBookRequestJdto};
use controller_utils::AppContext;
use usecase_book::BookUsecaseCreateBook;
use crate::BookControllerError;
use log::info;

pub async fn create(
    ctx: web::Data<AppContext>,
    req: web::Json<CreateBookRequestJdto>,
) -> Result<web::Json<BookResponseJdto>, BookControllerError> {
    info!("POST /books - Creating book: {} (division_id: {})", req.title, req.division_id);
    let result = controller_utils::executor(&ctx, req.into_inner(), |conn, req| {
        di_service::build_book_usecase().execute(conn, &req)
    });
    if result.is_ok() {
        info!("POST /books - Book created successfully");
    }
    result
}
