use std::collections::HashMap;

use crate::{
    db::{models::StatEntry, DatabaseDriver},
    ws::{middleware::auth::AuthService, models::StatsQuery},
};
use actix_web::{
    error, get,
    web::{self, Data, Path, Query, ServiceConfig},
    Error, HttpResponse,
};
use chrono::{Duration, Local, NaiveDateTime};
use log::debug;

#[get("/{id}")]
async fn get_stats(
    auth: AuthService,
    db: Data<DatabaseDriver>,
    id: Path<String>,
    query: Query<StatsQuery>,
) -> Result<HttpResponse, Error> {
    let to = query
        .to
        .unwrap_or_else(|| Local::now().naive_local());
    let from = query
        .from
        .unwrap_or_else(|| to - Duration::days(7));

    let delta = to - from;
    if delta < Duration::hours(1) {
        return Err(error::ErrorBadRequest(
            "invalid time span - must be larger than 1h",
        ));
    }

    let entries = web::block(move || {
        db.query_stats(Some(&auth.claims().sub), Some(&id), Some(from), Some(to))
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let bucket_size = get_bucket_size(delta);
    let n_buckets = delta.num_seconds() / bucket_size.num_seconds();
    debug!(
        "bucket_size: {}m; n_buckets: {}",
        bucket_size.num_minutes(),
        n_buckets
    );

    let buckets = (0..n_buckets)
        .map(|i| from + Duration::seconds(i * bucket_size.num_seconds()))
        .map(|d| (d, 0usize))
        .collect::<HashMap<_, _>>();

    let mut bucket_map = HashMap::<String, HashMap<NaiveDateTime, usize>>::new();

    for entry in entries {
        if !bucket_map.contains_key(&entry.link_id) {
            bucket_map.insert(entry.link_id.clone(), buckets.clone());
        }
        let entry_buckets = bucket_map
            .get_mut(&entry.link_id)
            .unwrap();
        let d = from
            + Duration::seconds(
                ((entry.created_date - from).num_seconds() / bucket_size.num_seconds())
                    * bucket_size.num_seconds(),
            );
        let count = entry_buckets.get(&d);
        let count = count.unwrap_or(&0) + 1;
        entry_buckets.insert(d, count);
    }

    Ok(HttpResponse::Ok().json(bucket_map))
}

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(get_stats);
}

// --- helpers ---

fn get_bucket_size(d: Duration) -> Duration {
    match d {
        d if d <= Duration::hours(1) => Duration::minutes(10),
        d if d <= Duration::hours(12) => Duration::minutes(30),
        d if d <= Duration::days(1) => Duration::hours(1),
        d if d <= Duration::days(7) => Duration::hours(6),
        _ => Duration::days(1),
    }
}
