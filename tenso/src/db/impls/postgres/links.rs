use super::Postgres;
use crate::db::{
    impls::postgres::mute_not_found,
    models::Link,
    traits::{self},
};
use anyhow::Result;
use diesel::prelude::*;

impl traits::links::Links for Postgres {
    fn list_links(&self, limit: i64, offset: i64) -> Result<Vec<Link>> {
        use crate::db::schema::links::dsl;
        let mut conn = self.pool.get()?;

        let res = dsl::links
            .limit(limit)
            .offset(offset)
            .load(&mut conn)?;
        Ok(res)
    }

    fn get_link(&self, id_or_ident: &str) -> Result<Option<Link>> {
        use crate::db::schema::links::dsl;
        let mut conn = self.pool.get()?;

        let res = dsl::links
            .filter(
                dsl::id
                    .eq(id_or_ident)
                    .or(dsl::ident.eq(id_or_ident)),
            )
            .first(&mut conn);
        let res = mute_not_found(res)?;
        Ok(res)
    }

    fn put_link(&self, link: &Link) -> Result<()> {
        use crate::db::schema::links::dsl;
        let mut conn = self.pool.get()?;

        let res = diesel::update(dsl::links.find(&link.id))
            .set((
                dsl::destination.eq(&link.destination),
                dsl::enabled.eq(&link.enabled),
                dsl::ident.eq(&link.ident),
                dsl::permanent_redirect.eq(&link.permanent_redirect),
            ))
            .execute(&mut conn);

        if let Err(diesel::NotFound) = res {
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

    fn delete_link(&self, link: &Link) -> Result<()> {
        use crate::db::schema::links::dsl;
        let mut conn = self.pool.get()?;

        diesel::delete(dsl::links.find(&link.id)).execute(&mut conn)?;
        Ok(())
    }
}
