use super::Postgres;
use crate::db::{
    models::StatEntry,
    traits::{self},
};
use anyhow::Result;
use diesel::prelude::*;

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

    fn query_stats(
        &self,
        link_id: Option<&str>,
        from: Option<chrono::NaiveDateTime>,
        to: Option<chrono::NaiveDateTime>,
    ) -> Result<Vec<StatEntry>> {
        use crate::db::schema::stats;
        let mut conn = self.pool.get()?;

        let mut query = stats::table.into_boxed();

        if let Some(link_id) = link_id {
            query = query.filter(stats::link_id.eq(link_id));
        }

        if let Some(from) = from {
            query = query.filter(stats::created_date.ge(from))
        }

        if let Some(to) = to {
            query = query.filter(stats::created_date.le(to))
        }

        let res = query.load(&mut conn)?;

        Ok(res)
    }
}
