mod links;
mod users;

use crate::db::traits::Database;
use anyhow::Result;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

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

impl Database for Postgres {}

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
