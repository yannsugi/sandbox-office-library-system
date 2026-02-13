use actix_web::web;
use presenter_book::BookResponseJdto;
use controller_utils::AppContext;
use usecase_book::BookUsecaseListBooks;
use crate::BookControllerError;
use log::info;

pub async fn list(
    ctx: web::Data<AppContext>,
) -> Result<web::Json<Vec<BookResponseJdto>>, BookControllerError>
{
    info!("GET /books - Listing books");
    let result = controller_utils::executor(
        &ctx,
        (),
        |conn, _| {
            di_service::build_book_usecase().execute(conn)
        }
    );
    if let Ok(ref books) = result {
        info!("GET /books - Found {} books", books.len());
    }
    result
}
