use sqlx::{Error};
use crate::domain::domain::{User, sql};
use bcrypt;

pub struct SysUserService {}

impl SysUserService {
    pub async fn login(&self, mail: String, password: String) -> Result<Option<User>, Error> {
        let user: Option<User> = sql::user::query_by_mail(mail).await?;
        match user {
            None => Ok(None),
            Some(user) => {
                match bcrypt::verify(&password, &user.password) {
                    Ok(true) => Ok(Some(user)),
                    _ => Ok(None)
                }
            }
        }
    }

    pub async fn change_pass(&self, mail: String, old_pass: String, new_pass: String)-> Result<Option<User>, Error> {
        let user: Option<User> = sql::user::query_by_mail(mail).await?;
        match user {
            None => Ok(None),
            Some(user) => {
                match bcrypt::verify(&old_pass, &user.password) {
                    Ok(true) => {
                        sql::user::update_password(user.id, new_pass).await?;
                        Ok(Some(user))
                    },
                    _ => Ok(None)
                }
            }
        }
    }

    pub async fn check_token(&self, id: i32, ticker: i64)-> Result<Option<User>, Error> {
        let user: Option<User> = sql::user::query_by_id(id).await?;
        match user {
            Some(user) => {
                if ticker == user.update_time.timestamp() {
                    Ok(Some(user))
                } else {
                    Ok(None)
                }
            },
            None => Ok(None)
        }
    }
}