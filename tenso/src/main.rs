mod db;
mod ws;

use dotenvy::dotenv;
use log::info;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    let debug_mode = env::var("DEBUG_MODE")
        .map(|v| {
            v.parse::<bool>()
                .expect("invalid value for bool env var 'DEBUG_MODE")
        })
        .unwrap_or(false);
    let bind_addr = env::var("WS_BINDADDR").unwrap_or_else(|_| "0.0.0.0:80".into());
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set");
    let jwt_signing_key =
        env::var("WS_SIGNING_KEY").unwrap_or_else(|_| "TODO: generate random signing key".into());
    let default_redirect = env::var("WS_REDIRECT_DEFAULT").ok();
    let notfound_redirect = env::var("WS_REDIRECT_NOTFOUND").ok();

    let d = db::DatabaseDriver::init(&database_url)?;

    let cfg = ws::Config {
        debug_mode,
        jwt_signing_key,
        default_redirect,
        notfound_redirect,
    };
    info!("WS :: Binding to {bind_addr}");
    ws::run(&bind_addr, cfg, d).await?;

    Ok(())
}
