use feature_division::DivisionFeatureCreateDivision;
use value_object_division::DivisionName;
use presenter_division::{CreateDivisionRequestJdto, DivisionResponseJdto};
use anyhow::Result;

pub struct DivisionUsecaseService<DivFeature> {
    division_feature: DivFeature,
}

impl<DivFeature> DivisionUsecaseService<DivFeature> {
    pub fn new(division_feature: DivFeature) -> Self {
        Self { division_feature }
    }
}

pub trait DivisionUsecaseCreateDivision<Ctx> {
    fn execute(&self, ctx: &mut Ctx, req: &CreateDivisionRequestJdto) -> Result<DivisionResponseJdto>;
}

impl<Ctx, DivFeature> DivisionUsecaseCreateDivision<Ctx> for DivisionUsecaseService<DivFeature>
where
    DivFeature: DivisionFeatureCreateDivision<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx, req: &CreateDivisionRequestJdto) -> Result<DivisionResponseJdto> {
        let name = DivisionName::new(req.name.clone())?;
        let division = self.division_feature.execute(ctx, &name)?;

        Ok(DivisionResponseJdto {
            id: division.id().value(),
            name: division.name().value().to_string(),
        })
    }
}
