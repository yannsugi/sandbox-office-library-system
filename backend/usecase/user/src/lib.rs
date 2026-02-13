use feature_user::{UserFeatureCreateUser, UserFeatureListUsers};
use value_object_user::UserName;
use value_object_division::DivisionId;
use presenter_user::{CreateUserRequestJdto, UserResponseJdto};
use anyhow::Result;

pub struct UserUsecaseService<UserFeature> {
    user_feature: UserFeature,
}

impl<UserFeature> UserUsecaseService<UserFeature> {
    pub fn new(user_feature: UserFeature) -> Self {
        Self { user_feature }
    }
}

pub trait UserUsecaseCreateUser<Ctx> {
    fn execute(&self, ctx: &mut Ctx, req: &CreateUserRequestJdto) -> Result<UserResponseJdto>;
}

impl<Ctx, UserFeature> UserUsecaseCreateUser<Ctx> for UserUsecaseService<UserFeature>
where
    UserFeature: UserFeatureCreateUser<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx, req: &CreateUserRequestJdto) -> Result<UserResponseJdto> {
        let name = UserName::new(req.name.clone())?;
        let division_id = DivisionId::new(req.division_id)?;
        let user = self.user_feature.execute(ctx, &name, division_id)?;

        Ok(UserResponseJdto {
            id: user.id().value(),
            name: user.name().value().to_string(),
            division_id: user.division_id().value(),
        })
    }
}

pub trait UserUsecaseListUsers<Ctx> {
    fn execute(&self, ctx: &mut Ctx) -> Result<Vec<UserResponseJdto>>;
}

impl<Ctx, UserFeature> UserUsecaseListUsers<Ctx> for UserUsecaseService<UserFeature>
where
    UserFeature: UserFeatureListUsers<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx) -> Result<Vec<UserResponseJdto>> {
        let users = self.user_feature.execute(ctx)?;

        Ok(users.items().iter().map(|user| {
            UserResponseJdto {
                id: user.id().value(),
                name: user.name().value().to_string(),
                division_id: user.division_id().value(),
            }
        }).collect())
    }
}
