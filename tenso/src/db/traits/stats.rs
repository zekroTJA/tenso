use anyhow::Result;
use chrono::NaiveDateTime;

use crate::db::models::StatEntry;

pub trait Stats {
    fn put_stats(&self, entry: &StatEntry) -> Result<()>;
    fn query_stats(
        &self,
        user_id: Option<&str>,
        link_id: Option<&str>,
        from: Option<NaiveDateTime>,
        to: Option<NaiveDateTime>,
    ) -> Result<Vec<StatEntry>>;
}
