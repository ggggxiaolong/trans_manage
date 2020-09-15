use async_graphql::{ErrorExtensions, FieldError};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use serde_json::json;
use async_graphql::*;
// 前端数据
#[derive(GQLSimpleObject)]
pub struct Token {
    access_token: String,
    refresh_token: String,
}

impl Token {
    pub fn new(access_token: String, refresh_token: String) -> Token {
        Token {
            access_token: access_token,
            refresh_token: refresh_token,
        }
    }
}

#[derive(GQLSimpleObject)]
pub struct Project {
    pub id: i32,
    pub name: String,
}

#[derive(GQLSimpleObject)]
pub struct Lang {
    id: i32,
    user_id: i32,
    en: String,
    ja: Option<String>,
    ko: Option<String>,
    sk: Option<String>,
    cs: Option<String>,
    fr: Option<String>,
    es: Option<String>,
    pt: Option<String>,
    #[field(name="not_trans")]
    not_trans: i32,
    descripe: Option<String>,
    label: Option<String>,
    #[field(name="file_name")]
    file_name: Option<String>,
    #[field(name="mode_name")]
    mode_name: Option<String>,
    #[field(name="project_id")]
    project_id: i32,
    #[field(name="new_user_id")]
    new_user_id: Option<i32>,
    #[field(name="new_en")]
    new_en: Option<String>,
    #[field(name="new_ja")]
    new_ja: Option<String>,
    #[field(name="new_ko")]
    new_ko: Option<String>,
    #[field(name="new_sk")]
    new_sk: Option<String>,
    #[field(name="new_cs")]
    new_cs: Option<String>,
    #[field(name="new_fr")]
    new_fr: Option<String>,
    #[field(name="new_es")]
    new_es: Option<String>,
    #[field(name="new_pt")]
    new_pt: Option<String>,
    #[field(name="new_not_trans")]
    new_not_trans: Option<i32>,
    #[field(name="new_descripe")]
    new_descripe: Option<String>,
    #[field(name="new_label")]
    new_label: Option<String>,
    #[field(name="new_file_name")]
    new_file_name: Option<String>,
    #[field(name="new_mode_name")]
    new_mode_name: Option<String>,
    #[field(name="new_project_id")]
    new_project_id: Option<i32>,
    status: i32,//0 为最新， 1为更新， 2为新增
    #[field(name="create_time")]
    create_time: NaiveDateTime,
    #[field(name="update_time")]
    update_time: NaiveDateTime,
}

#[derive(GQLSimpleObject, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub mail: String,
}

const CODE_TOKEN: &str = "CODE_TOKEN_EXPIRE";
const CODE_INTERNAL: &str = "CODE_SERVER_INTERNAL_ERROR";
const CODE_MAIL_OR_PASSWORD_FAIL: &str = "CODE_MAIL_OR_PASSWORD_FAIL";

pub enum CustomError {
    TokenError,
    Internal(String),
    MailOrPasswordFail,
}

impl ErrorExtensions for CustomError {
    fn extend(&self) -> FieldError {
        match self {
            CustomError::TokenError => FieldError(CODE_TOKEN.to_string(), None),
            CustomError::Internal(message) => FieldError(CODE_INTERNAL.to_string(), Some(json!({"info": message}))),
            CustomError::MailOrPasswordFail => FieldError(CODE_MAIL_OR_PASSWORD_FAIL.to_string(), None),
        }
    }
}