use anyhow::Result;

use crate::db::models::Link;

pub trait Links {
    fn list_links(&self, user_id: &str, limit: i64, offset: i64) -> Result<Vec<Link>>;
    fn get_link(&self, user_id: &str, id_or_ident: &str) -> Result<Option<Link>>;
    fn get_link_by_ident(&self, ident: &str) -> Result<Option<Link>>;
    fn put_link(&self, link: &Link) -> Result<()>;
    fn delete_link(&self, id: &str) -> Result<()>;
}
