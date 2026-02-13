use diesel::prelude::*;
use repository_division::{DivisionRepoCreate, DivisionRepoRead};
use entity_object_division::Division;
use value_object_division::{DivisionId, DivisionName};
use db_domain_division::schema::divisions;
use db_domain_division::models::{DivisionRow, NewDivisionRow};
use domain_context::PgConn;
use anyhow::Result;

pub struct DivisionRepoImpl;

impl DivisionRepoImpl {
    pub fn new() -> Self {
        Self
    }

    fn row_to_domain(row: DivisionRow) -> Result<Division> {
        let id = DivisionId::new(row.id)?;
        let name = DivisionName::new(row.name)?;
        Ok(Division::new(id, name))
    }
}


impl<Ctx> DivisionRepoCreate<Ctx> for DivisionRepoImpl
where
    Ctx: PgConn,
{
    fn create_division(&self, ctx: &mut Ctx, name: &DivisionName) -> Result<Division> {
        let new_row = NewDivisionRow {
            name: name.value().to_string(),
        };

        let row: DivisionRow = diesel::insert_into(divisions::table)
            .values(&new_row)
            .returning(DivisionRow::as_returning())
            .get_result(ctx.conn())?;

        Self::row_to_domain(row)
    }
}

impl<Ctx> DivisionRepoRead<Ctx> for DivisionRepoImpl
where
    Ctx: PgConn,
{
    fn find_division(&self, ctx: &mut Ctx, id: DivisionId) -> Result<Option<Division>> {
        let result = divisions::table
            .find(id.value())
            .select(DivisionRow::as_select())
            .first(ctx.conn())
            .optional()?;

        match result {
            Some(row) => Ok(Some(Self::row_to_domain(row)?)),
            None => Ok(None),
        }
    }
}
