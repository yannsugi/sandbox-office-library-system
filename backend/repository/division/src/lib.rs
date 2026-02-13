use entity_object_division::Division;
use value_object_division::{DivisionId, DivisionName};
use anyhow::Result;

pub trait DivisionRepoCreate<Ctx> {
    fn create_division(&self, ctx: &mut Ctx, name: &DivisionName) -> Result<Division>;
}

pub trait DivisionRepoRead<Ctx> {
    fn find_division(&self, ctx: &mut Ctx, id: DivisionId) -> Result<Option<Division>>;
}

pub trait DivisionRepoUpdate<Ctx> {
    // No update operations for now
}

pub trait DivisionRepoDelete<Ctx> {
    // No delete operations for now
}
