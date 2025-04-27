use common_core::prelude::*;

pub use diesel::result::Error as DslError;
pub use diesel::{
    upsert::*,
    prelude::*,
    SqliteConnection,
    r2d2::ConnectionManager,
    r2d2::ManageConnection,
    r2d2::PooledConnection,
    r2d2::R2D2Connection,
    r2d2::Pool,
};
pub type SqliteDbPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DbPool = SqliteDbPool;
pub type DbConn = PooledConnection<ConnectionManager<SqliteConnection>>;


pub trait ConnectionProvider
{
    fn conn(&self) -> AResult<DbConn>;
}

impl ConnectionProvider for SqliteDbPool
{
    fn conn(&self) -> AResult<DbConn> {
        let conn = self.get()
            .map_err(msg)?;

        Ok(conn)
    }
}

// impl ConnectionProvider for web::Data<SqliteDbPool>
// {
//     fn conn(&self) -> AResult<DbConn> {
//         let conn = self.get()
//             .map_err(msg)?;

//         Ok(conn)
//     }
// }

