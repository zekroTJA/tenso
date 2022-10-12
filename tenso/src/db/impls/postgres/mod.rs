mod links;
mod stats;
mod users;

use crate::db::traits::Database;
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

pub struct Postgres {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Postgres {
    pub fn new(url: &str) -> Result<Self> {
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = Pool::builder().build(manager)?;

        Ok(Self { pool })
    }
}

impl Database for Postgres {
    fn apply_migrations(&self) -> Result<()> {
        let mut conn = self.pool.get()?;
        conn.run_pending_migrations(MIGRATIONS).map_err(|e| anyhow!(e))?;
        Ok(())
    }
}

#[inline]
pub fn mute_not_found<T>(
    res: std::result::Result<T, diesel::result::Error>,
) -> std::result::Result<Option<T>, diesel::result::Error> {
    match res {
        Ok(v) => Ok(Some(v)),
        Err(diesel::NotFound) => Ok(None),
        Err(e) => Err(e),
    }
}
