use actix_web::web;
use presenter_division::DivisionResponseJdto;
use controller_utils::AppContext;
use usecase_division::DivisionUsecaseListDivisions;
use crate::DivisionControllerError;
use log::info;

pub async fn list(
    ctx: web::Data<AppContext>,
) -> Result<web::Json<Vec<DivisionResponseJdto>>, DivisionControllerError>
{
    info!("GET /divisions - Listing divisions");
    let result = controller_utils::executor(
        &ctx,
        (),
        |conn, _| {
            di_service::build_division_usecase_list().execute(conn)
        }
    );
    if let Ok(ref divisions) = result {
        info!("GET /divisions - Found {} divisions", divisions.len());
    }
    result
}
