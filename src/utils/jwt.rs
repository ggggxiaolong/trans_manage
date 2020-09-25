
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use crate::domain::vo::Token;
use crate::domain::vo::VOUser;
use crate::config::CONFIG;
use chrono::Utc;

#[derive(Serialize, Deserialize)]
struct Claims {
    user: VOUser,
    is_refresh: bool,
    exp: i64,
}

fn gen_token(user: &VOUser, is_refresh: bool) -> String {
    let now = Utc::now().timestamp();
    let claims = Claims {
        user: user.clone(),
        is_refresh: is_refresh,
        exp: if is_refresh { now + 60 * 60 * 24 * 7 } else { now + 60 * 60 }
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(CONFIG.token_key.as_ref())).unwrap()
}

pub fn gen_user_token(user: VOUser) -> Token{
    let access_token = gen_token(&user, false);
    let refresh_token = gen_token(&user, true);
    Token::new(access_token, refresh_token)
}

pub fn validate_token(token: &str) -> Option<VOUser> {
    match decode::<Claims>(token, &DecodingKey::from_secret(CONFIG.token_key.as_ref()), &Validation::default()) {
        Ok(c) if !c.claims.is_refresh => Some(c.claims.user),
        _ => None,
    }
}

pub fn validate_refresh_token(token: &str) -> Option<VOUser> {
    match decode::<Claims>(token, &DecodingKey::from_secret(CONFIG.token_key.as_ref()), &Validation::default()) {
        Ok(c) if c.claims.is_refresh => Some(c.claims.user),
        _ => None,
    }
}