use entity_object_user::User;
use collection_object_user::Users;
use value_object_user::{UserId, UserName};
use value_object_division::DivisionId;
use anyhow::Result;

pub trait UserRepoCreate<Ctx> {
    fn create_user(&self, ctx: &mut Ctx, name: &UserName, division_id: DivisionId) -> Result<User>;
}

pub trait UserRepoRead<Ctx> {
    fn find_user(&self, ctx: &mut Ctx, id: UserId) -> Result<Option<User>>;
    fn list_users(&self, ctx: &mut Ctx) -> Result<Users>;
}

pub trait UserRepoUpdate<Ctx> {
    // No update operations for now
}

pub trait UserRepoDelete<Ctx> {
    // No delete operations for now
}
