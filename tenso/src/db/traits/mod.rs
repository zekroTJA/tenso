pub mod links;
pub mod stats;
pub mod users;

use self::{links::Links, stats::Stats, users::Users};
use anyhow::Result;

pub trait Database: Users + Links + Stats {
    fn apply_migrations(&self) -> Result<()>;
}
