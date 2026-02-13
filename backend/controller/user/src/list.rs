use actix_web::web;
use presenter_user::UserResponseJdto;
use controller_utils::AppContext;
use usecase_user::UserUsecaseListUsers;
use crate::UserControllerError;
use log::info;

pub async fn list(
    ctx: web::Data<AppContext>,
) -> Result<web::Json<Vec<UserResponseJdto>>, UserControllerError>
{
    info!("GET /users - Listing users");
    let result = controller_utils::executor(
        &ctx,
        (),
        |conn, _| {
            di_service::build_user_usecase_list().execute(conn)
        }
    );
    if let Ok(ref users) = result {
        info!("GET /users - Found {} users", users.len());
    }
    result
}
