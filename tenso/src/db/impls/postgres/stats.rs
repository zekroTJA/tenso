use super::Postgres;
use crate::db::{
    models::StatEntry,
    traits::{self},
};
use anyhow::Result;
use diesel::{dsl::count, prelude::*};

impl traits::stats::Stats for Postgres {
    fn put_stats(&self, entry: &StatEntry) -> Result<()> {
        use crate::db::schema::stats::dsl;
        let mut conn = self.pool.get()?;

        diesel::insert_into(dsl::stats)
            .values((
                dsl::id.eq(&entry.id),
                dsl::created_date.eq(&entry.created_date),
                dsl::link_id.eq(&entry.link_id),
                dsl::user_agent.eq(&entry.user_agent),
            ))
            .execute(&mut conn)?;
        Ok(())
    }

    fn get_count(&self, user_id: Option<&str>, link_id: &str) -> Result<usize> {
        use crate::db::schema::links;
        use crate::db::schema::stats;
        let mut conn = self.pool.get()?;

        let mut query = stats::table.inner_join(links::table).into_boxed();

        if let Some(user_id) = user_id {
            query = query.filter(links::creator_id.eq(user_id));
        }

        let count: i64 = query
            .filter(stats::link_id.eq(link_id))
            .select(count(links::id))
            .first(&mut conn)?;

        Ok(count as usize)
    }

    fn query_stats(
        &self,
        user_id: Option<&str>,
        link_id: &str,
        from: Option<chrono::NaiveDateTime>,
        to: Option<chrono::NaiveDateTime>,
    ) -> Result<Vec<StatEntry>> {
        use crate::db::schema::links;
        use crate::db::schema::stats;
        let mut conn = self.pool.get()?;

        let mut query = stats::table.inner_join(links::table).into_boxed();

        query = query.filter(stats::link_id.eq(link_id));

        if let Some(user_id) = user_id {
            query = query.filter(links::creator_id.eq(user_id));
        }

        if let Some(from) = from {
            query = query.filter(stats::created_date.ge(from))
        }

        if let Some(to) = to {
            query = query.filter(stats::created_date.le(to))
        }

        let res = query
            .select((
                stats::id,
                stats::link_id,
                stats::created_date,
                stats::user_agent,
            ))
            .load(&mut conn)?;

        Ok(res)
    }
}
