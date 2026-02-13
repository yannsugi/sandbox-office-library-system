use actix_web::web;
use presenter_division::{CreateDivisionRequestJdto, DivisionResponseJdto};
use controller_utils::AppContext;
use usecase_division::DivisionUsecaseCreateDivision;
use crate::DivisionControllerError;
use log::info;

pub async fn create(
    ctx: web::Data<AppContext>,
    req: web::Json<CreateDivisionRequestJdto>,
) -> Result<web::Json<DivisionResponseJdto>, DivisionControllerError>
{
    info!("POST /divisions - Creating division: {}", req.name);
    let result = controller_utils::executor(
        &ctx,
        req.into_inner(),
        |conn, req| {
            di_service::build_division_usecase().execute(conn, &req)
        }
    );
    if result.is_ok() {
        info!("POST /divisions - Division created successfully");
    }
    result
}
