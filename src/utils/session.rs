use super::jwt::validate_token;
use crate::domain::vo::VOUser;
use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::future::{ok, Ready};

pub struct Session {
    pub user: VOUser,
}

impl FromRequest for Session {
    type Error = Error;
    type Future = Ready<Result<Session, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let user = req
            .headers()
            .get("token")
            .map(|v| validate_token(v.to_str().unwrap_or("")).unwrap_or(VOUser::default()))
            .unwrap_or(VOUser::default());
        ok(Session { user })
    }
}
