use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Trait for types that can provide a mutable reference to PgConnection
pub trait PgConn {
    fn conn(&mut self) -> &mut PgConnection;
}

// Blanket implementation for PgConnection itself
impl PgConn for PgConnection {
    fn conn(&mut self) -> &mut PgConnection {
        self
    }
}

/// Application-wide context shared across all requests
pub struct AppContext {
    pub pool: DbPool,
}

impl AppContext {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

/// Transaction context holding both app context and transaction connection
pub struct TxContext<'a> {
    pub app_ctx: &'a AppContext,
    conn: &'a mut PgConnection,
}

impl<'a> TxContext<'a> {
    pub fn new(app_ctx: &'a AppContext, conn: &'a mut PgConnection) -> Self {
        Self { app_ctx, conn }
    }
}

impl<'a> PgConn for TxContext<'a> {
    fn conn(&mut self) -> &mut PgConnection {
        self.conn
    }
}
