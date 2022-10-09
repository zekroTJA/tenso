pub mod users;

use self::users::Users;

pub trait Database: Users {}
