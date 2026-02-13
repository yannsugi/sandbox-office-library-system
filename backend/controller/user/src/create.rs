use actix_web::web;
use presenter_user::{CreateUserRequestJdto, UserResponseJdto};
use controller_utils::AppContext;
use usecase_user::UserUsecaseCreateUser;
use crate::UserControllerError;
use log::info;

pub async fn create(
    ctx: web::Data<AppContext>,
    req: web::Json<CreateUserRequestJdto>,
) -> Result<web::Json<UserResponseJdto>, UserControllerError>
{
    info!("POST /users - Creating user: {} (division_id: {})", req.name, req.division_id);
    let result = controller_utils::executor(
        &ctx,
        req.into_inner(),
        |conn, req| {
            di_service::build_user_usecase().execute(conn, &req)
        }
    );
    if result.is_ok() {
        info!("POST /users - User created successfully");
    }
    result
}
