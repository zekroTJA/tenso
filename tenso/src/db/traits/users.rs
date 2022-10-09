use crate::db::models::AuthUser;
use anyhow::Result;

pub trait Users {
    fn get_auth_user(&self, username: &str) -> Result<Option<AuthUser>>;
    fn get_users_count(&self) -> Result<i64>;
    fn list_users(&self) -> Result<Vec<AuthUser>>;
    fn put_auth_user(&self, user: &AuthUser) -> Result<()>;
}
