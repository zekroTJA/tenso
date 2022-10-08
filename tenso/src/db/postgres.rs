use super::{models::AuthUser, Database};
use anyhow::Result;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::RunQueryDsl;

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
    fn get_auth_user(&self, username: &str) -> Result<Option<AuthUser>> {
        use super::schema::auth::dsl;

        let mut conn = self.pool.get()?;

        let res = dsl::auth.find(username).first::<AuthUser>(&mut conn);
        let res = mute_not_found(res)?;

        Ok(res)
    }

    fn put_auth_user(&self, user: &AuthUser) -> Result<()> {
        use super::schema::auth::dsl;

        let mut conn = self.pool.get()?;

        let res = diesel::update(dsl::auth.find(&user.username))
            .set(dsl::password_hash.eq(&user.password_hash))
            .get_result::<(String, String)>(&mut conn);

        if let Err(diesel::NotFound) = res {
            diesel::insert_into(dsl::auth)
                .values((
                    dsl::username.eq(&user.username),
                    dsl::password_hash.eq(&user.password_hash),
                ))
                .execute(&mut conn)?;
        }

        Ok(())
    }
}

#[inline]
fn mute_not_found<T>(
    res: std::result::Result<T, diesel::result::Error>,
) -> std::result::Result<Option<T>, diesel::result::Error> {
    match res {
        Ok(v) => Ok(Some(v)),
        Err(diesel::NotFound) => Ok(None),
        Err(e) => Err(e),
    }
}
