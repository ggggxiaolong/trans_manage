use crate::domain::{
    domain::{sql, User},
    vo::CustomError,
};
use bcrypt;

pub struct SysUserService {}

impl SysUserService {
    pub async fn login(&self, mail: String, password: String) -> Result<User, CustomError> {
        let user: Option<User> = sql::user::query_by_mail(mail).await?;
        match user {
            None => Err(CustomError::MailOrPasswordFail),
            Some(user) => match bcrypt::verify(&password, &user.password) {
                Ok(true) => Ok(user),
                _ => Err(CustomError::MailOrPasswordFail),
            },
        }
    }

    pub async fn change_pass(
        &self,
        mail: String,
        old_pass: String,
        new_pass: String,
    ) -> Result<User, CustomError> {
        let user: Option<User> = sql::user::query_by_mail(mail).await?;
        match user {
            None => Err(CustomError::MailOrPasswordFail),
            Some(user) => match bcrypt::verify(&old_pass, &user.password) {
                Ok(true) => {
                    sql::user::update_password(user.id, new_pass).await?;
                    Ok(user)
                }
                _ => Err(CustomError::MailOrPasswordFail),
            },
        }
    }

    pub async fn check_token(&self, id: i32, ticker: i64) -> Result<User, CustomError> {
        let user: Option<User> = sql::user::query_by_id(id).await?;
        match user {
            Some(user) => {
                if ticker == user.update_time.timestamp() {
                    Ok(user)
                } else {
                    Err(CustomError::TokenError)
                }
            }
            None => Err(CustomError::TokenError),
        }
    }
}

#[cfg(test)]
mod tests {

    // use bcrypt::{DEFAULT_COST, hash, verify};

    #[test]
    fn test_bcrypt() {
        let ret = bcrypt::verify(
            "UEoC?5K3*xays2/B",
            "$2b$12$dQu7D/KHYLC0GiZWLn7zV.7ceiccLA74aCd.u676aaRCUENIcLvCq",
        )
        .unwrap();
        assert_eq!(ret, true);
    }
}
