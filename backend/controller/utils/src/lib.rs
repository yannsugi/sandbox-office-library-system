use diesel::Connection;
pub use domain_context::{AppContext, DbPool, TxContext};

pub fn with_tx<F, R>(app_ctx: &AppContext, f: F) -> Result<R, anyhow::Error>
where
    F: for<'a> FnOnce(&mut TxContext<'a>) -> Result<R, anyhow::Error>,
{
    let mut conn = app_ctx.pool.get()?;
    conn.transaction(|tx_conn| {
        let mut tx_ctx = TxContext::new(app_ctx, tx_conn);
        f(&mut tx_ctx)
    })
}

pub fn executor<Req, Res, E, F>(
    ctx: &AppContext,
    req: Req,
    f: F,
) -> Result<actix_web::web::Json<Res>, E>
where
    F: for<'a> FnOnce(&mut TxContext<'a>, Req) -> Result<Res, anyhow::Error>,
    E: From<anyhow::Error>,
{
    let response = with_tx(ctx, |tx_ctx| f(tx_ctx, req))?;
    Ok(actix_web::web::Json(response))
}
