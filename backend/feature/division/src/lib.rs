use repository_division::DivisionRepoCreate;
use entity_object_division::Division;
use value_object_division::DivisionName;
use anyhow::Result;

pub struct DivisionFeatureService<DivRepo> {
    division_repo: DivRepo,
}

impl<DivRepo> DivisionFeatureService<DivRepo> {
    pub fn new(division_repo: DivRepo) -> Self {
        Self { division_repo }
    }
}

pub trait DivisionFeatureCreateDivision<Ctx> {
    fn execute(&self, ctx: &mut Ctx, name: &DivisionName) -> Result<Division>;
}

impl<Ctx, DivRepo> DivisionFeatureCreateDivision<Ctx> for DivisionFeatureService<DivRepo>
where
    DivRepo: DivisionRepoCreate<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx, name: &DivisionName) -> Result<Division> {
        self.division_repo.create_division(ctx, name)
    }
}
