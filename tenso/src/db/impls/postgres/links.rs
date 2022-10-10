use super::Postgres;
use crate::db::{
    impls::postgres::mute_not_found,
    models::Link,
    traits::{self},
};
use anyhow::Result;
use diesel::prelude::*;

impl traits::links::Links for Postgres {
    fn list_links(&self, user_id: &str, limit: i64, offset: i64) -> Result<Vec<Link>> {
        use crate::db::schema::links::dsl;
        let mut conn = self.pool.get()?;

        let res = dsl::links
            .filter(dsl::creator_id.eq(user_id))
            .limit(limit)
            .offset(offset)
            .load(&mut conn)?;
        Ok(res)
    }

    fn get_link(&self, user_id: &str, id_or_ident: &str) -> Result<Option<Link>> {
        use crate::db::schema::links::dsl;
        let mut conn = self.pool.get()?;

        let res = dsl::links
            .filter(dsl::creator_id.eq(user_id))
            .filter(
                dsl::id
                    .eq(id_or_ident)
                    .or(dsl::ident.eq(id_or_ident)),
            )
            .first(&mut conn);
        let res = mute_not_found(res)?;
        Ok(res)
    }

    fn get_link_by_ident(&self, ident: &str) -> Result<Option<Link>> {
        use crate::db::schema::links::dsl;
        let mut conn = self.pool.get()?;

        let res = dsl::links
            .filter(dsl::ident.eq(ident))
            .first(&mut conn);
        let res = mute_not_found(res)?;
        Ok(res)
    }

    fn put_link(&self, link: &Link) -> Result<()> {
        use crate::db::schema::links::dsl;
        let mut conn = self.pool.get()?;

        let res: usize = diesel::update(dsl::links.find(&link.id))
            .set((
                dsl::destination.eq(&link.destination),
                dsl::enabled.eq(&link.enabled),
                dsl::ident.eq(&link.ident),
                dsl::permanent_redirect.eq(&link.permanent_redirect),
            ))
            .execute(&mut conn)?;

        if res == 0 {
            diesel::insert_into(dsl::links)
                .values((
                    dsl::id.eq(&link.id),
                    dsl::created_date.eq(&link.created_date),
                    dsl::creator_id.eq(&link.creator_id),
                    dsl::destination.eq(&link.destination),
                    dsl::enabled.eq(&link.enabled),
                    dsl::ident.eq(&link.ident),
                    dsl::permanent_redirect.eq(&link.permanent_redirect),
                ))
                .execute(&mut conn)?;
        }

        Ok(())
    }

    fn delete_link(&self, id: &str) -> Result<()> {
        use crate::db::schema::links::dsl;
        let mut conn = self.pool.get()?;

        diesel::delete(dsl::links.find(id)).execute(&mut conn)?;
        Ok(())
    }
}
