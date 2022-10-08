pub mod models;
pub mod postgres;
pub mod schema;

use self::{models::AuthUser, postgres::Postgres};
use anyhow::{bail, Result};
use std::ops::{Deref, DerefMut};

pub trait Database {
    fn get_auth_user(&self, username: &str) -> Result<Option<AuthUser>>;
    fn put_auth_user(&self, user: &AuthUser) -> Result<()>;
}

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
