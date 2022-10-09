pub mod impls;
pub mod models;
pub mod schema;
pub mod traits;

use self::{impls::postgres::Postgres, traits::Database};
use anyhow::{bail, Result};
use std::ops::{Deref, DerefMut};

pub enum DatabaseDriver {
    Postgres(Postgres),
}

impl DatabaseDriver {
    pub fn init(url: &str) -> Result<DatabaseDriver> {
        if url.starts_with("postgres://") {
            let pg = Postgres::new(url)?;
            Ok(DatabaseDriver::Postgres(pg))
        } else {
            bail!("Invalid database URL or unsupported driver")
        }
    }
}

impl Deref for DatabaseDriver {
    type Target = dyn Database;

    fn deref(&self) -> &Self::Target {
        match self {
            DatabaseDriver::Postgres(postgres) => postgres,
        }
    }
}

impl DerefMut for DatabaseDriver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            DatabaseDriver::Postgres(postgres) => postgres,
        }
    }
}
