use actix_web::web;
use presenter_division::{CreateDivisionRequestJdto, DivisionResponseJdto};
use controller_utils::AppContext;
use usecase_division::DivisionUsecaseCreateDivision;
use crate::DivisionControllerError;

pub async fn create(
    ctx: web::Data<AppContext>,
    req: web::Json<CreateDivisionRequestJdto>,
) -> Result<web::Json<DivisionResponseJdto>, DivisionControllerError>
{
    controller_utils::executor(
        &ctx,
        req.into_inner(),
        |conn, req| {
            di_service::build_division_usecase().execute(conn, &req)
        }
    )
}
