mod db;
mod ws;

use dotenvy::dotenv;
use log::info;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    let bind_addr = env::var("TENSO_WS_BINDADDR").unwrap_or_else(|_| "0.0.0.0:80".into());
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set");

    let d = db::DatabaseDriver::init(&database_url)?;

    info!("WS :: Binding to {bind_addr}");
    ws::run(&bind_addr, d).await?;

    Ok(())
}
