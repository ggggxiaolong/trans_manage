use crate::dao::DBPool;
use sqlx::{Error};
use crate::domain::domain::User;

pub struct SysUserService {}

impl SysUserService {
    pub async fn login(&self, mail: String, pass: String) -> Result<i32, Error>{
        let user: Option<User> = sqlx::query_as("SELECT FROM ").fetch_optional(DBPool::get_pool()).await?;
    }
    pub async fn change_pass(&self, mail: String, old_pass: String, new_pass: String){}
    pub async fn refresh_token(&self, token: String){}
}