use actix_web::web;
use presenter_user::{CreateUserRequestJdto, UserResponseJdto};
use controller_utils::AppContext;
use usecase_user::UserUsecaseCreateUser;
use crate::UserControllerError;

pub async fn create(
    ctx: web::Data<AppContext>,
    req: web::Json<CreateUserRequestJdto>,
) -> Result<web::Json<UserResponseJdto>, UserControllerError>
{
    controller_utils::executor(
        &ctx,
        req.into_inner(),
        |conn, req| {
            di_service::build_user_usecase().execute(conn, &req)
        }
    )
}
