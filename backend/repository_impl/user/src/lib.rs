use diesel::prelude::*;
use repository_user::{UserRepoCreate, UserRepoRead};
use entity_object_user::User;
use value_object_user::{UserId, UserName};
use value_object_division::DivisionId;
use db_domain_user::schema::users;
use db_domain_user::models::{UserRow, NewUserRow};
use domain_context::PgConn;
use anyhow::Result;

pub struct UserRepoImpl;

impl UserRepoImpl {
    pub fn new() -> Self {
        Self
    }

    fn row_to_domain(row: UserRow) -> Result<User> {
        let id = UserId::new(row.id)?;
        let name = UserName::new(row.name)?;
        let division_id = DivisionId::new(row.division_id)?;
        Ok(User::new(id, name, division_id))
    }
}


impl<Ctx> UserRepoCreate<Ctx> for UserRepoImpl
where
    Ctx: PgConn,
{
    fn create_user(&self, ctx: &mut Ctx, name: &UserName, division_id: DivisionId) -> Result<User> {
        let new_row = NewUserRow {
            name: name.value().to_string(),
            division_id: division_id.value(),
        };

        let row: UserRow = diesel::insert_into(users::table)
            .values(&new_row)
            .returning(UserRow::as_returning())
            .get_result(ctx.conn())?;

        Self::row_to_domain(row)
    }
}

impl<Ctx> UserRepoRead<Ctx> for UserRepoImpl
where
    Ctx: PgConn,
{
    fn find_user(&self, ctx: &mut Ctx, id: UserId) -> Result<Option<User>> {
        let result = users::table
            .find(id.value())
            .select(UserRow::as_select())
            .first(ctx.conn())
            .optional()?;

        match result {
            Some(row) => Ok(Some(Self::row_to_domain(row)?)),
            None => Ok(None),
        }
    }
}
