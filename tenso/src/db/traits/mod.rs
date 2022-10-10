pub mod links;
pub mod stats;
pub mod users;

use self::{links::Links, stats::Stats, users::Users};

pub trait Database: Users + Links + Stats {}
