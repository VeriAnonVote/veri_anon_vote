use crate::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::backend::Backend;
// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../election_shared/migrations");
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../election_shared/migrations");


pub fn run<DB>(
    connection: &mut impl MigrationHarness<DB>,
) -> AResult<()>
where DB: Backend,
{

    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)
        .map_err(msg)?;

    Ok(())
}
