pub mod links;
pub mod users;

use self::{links::Links, users::Users};

pub trait Database: Users + Links {}
