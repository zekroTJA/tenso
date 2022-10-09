use super::Postgres;
use crate::db::{impls::postgres::mute_not_found, models::AuthUser, traits};
use anyhow::Result;
use diesel::prelude::*;

impl traits::users::Users for Postgres {
    fn get_auth_user(&self, username: &str) -> Result<Option<AuthUser>> {
        use crate::db::schema::auth::dsl;
        let mut conn = self.pool.get()?;

        let res = dsl::auth
            .find(username)
            .first::<AuthUser>(&mut conn);
        let res = mute_not_found(res)?;

        Ok(res)
    }

    fn get_users_count(&self) -> Result<i64> {
        use crate::db::schema::auth::dsl;
        let mut conn = self.pool.get()?;

        let res = dsl::auth
            .count()
            .get_result(&mut conn)?;
        Ok(res)
    }

    fn list_users(&self) -> Result<Vec<AuthUser>> {
        use crate::db::schema::auth::dsl;
        let mut conn = self.pool.get()?;

        let res = dsl::auth.load(&mut conn)?;
        Ok(res)
    }

    fn put_auth_user(&self, user: &AuthUser) -> Result<()> {
        use crate::db::schema::auth::dsl;
        let mut conn = self.pool.get()?;

        let res = diesel::update(dsl::auth.find(&user.username))
            .set(dsl::password_hash.eq(&user.password_hash))
            .execute(&mut conn);

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
