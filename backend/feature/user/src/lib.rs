use repository_user::{UserRepoCreate, UserRepoRead};
use entity_object_user::User;
use collection_object_user::Users;
use value_object_user::{UserName, UserId};
use value_object_division::DivisionId;
use anyhow::{Result, anyhow};

pub struct UserFeatureService<UserRepo> {
    user_repo: UserRepo,
}

impl<UserRepo> UserFeatureService<UserRepo> {
    pub fn new(user_repo: UserRepo) -> Self {
        Self { user_repo }
    }
}

pub trait UserFeatureCreateUser<Ctx> {
    fn execute(&self, ctx: &mut Ctx, name: &UserName, division_id: DivisionId) -> Result<User>;
}

impl<Ctx, UserRepo> UserFeatureCreateUser<Ctx> for UserFeatureService<UserRepo>
where
    UserRepo: UserRepoCreate<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx, name: &UserName, division_id: DivisionId) -> Result<User> {
        self.user_repo.create_user(ctx, name, division_id)
    }
}

pub trait UserFeatureFindUser<Ctx> {
    fn execute(&self, ctx: &mut Ctx, id: UserId) -> Result<User>;
}

impl<Ctx, UserRepo> UserFeatureFindUser<Ctx> for UserFeatureService<UserRepo>
where
    UserRepo: UserRepoRead<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx, id: UserId) -> Result<User> {
        self.user_repo.find_user(ctx, id)?
            .ok_or_else(|| anyhow!("User not found"))
    }
}

pub trait UserFeatureListUsers<Ctx> {
    fn execute(&self, ctx: &mut Ctx) -> Result<Users>;
}

impl<Ctx, UserRepo> UserFeatureListUsers<Ctx> for UserFeatureService<UserRepo>
where
    UserRepo: UserRepoRead<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx) -> Result<Users> {
        self.user_repo.list_users(ctx)
    }
}
