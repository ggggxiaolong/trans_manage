use crate::domain::domain::{Lang, Project, User};
use async_graphql::*;
use async_graphql::{ErrorExtensions};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Error as SqlxError;

// 前端数据
#[derive(SimpleObject)]
pub struct Token {
    access_token: String,
    refresh_token: String,
}

impl Token {
    pub fn new(access_token: String, refresh_token: String) -> Token {
        Token {
            access_token,
            refresh_token,
        }
    }
}

pub struct Trans {
    pub en: String,
}

#[derive(SimpleObject)]
pub struct VOProject {
    pub id: i32,
    pub name: String,
}

impl From<Project> for VOProject {
    fn from(p: Project) -> Self {
        VOProject {
            id: p.id,
            name: p.name,
        }
    }
}

#[derive(SimpleObject)]
pub struct VOLang {
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
    not_trans: i32,
    descripe: Option<String>,
    label: Option<String>,
    file_name: Option<String>,
    mode_name: Option<String>,
    project_id: i32,
    new_user_id: Option<i32>,
    new_en: Option<String>,
    new_ja: Option<String>,
    new_ko: Option<String>,
    new_sk: Option<String>,
    new_cs: Option<String>,
    new_fr: Option<String>,
    new_es: Option<String>,
    new_pt: Option<String>,
    new_not_trans: Option<i32>,
    new_descripe: Option<String>,
    new_label: Option<String>,
    new_file_name: Option<String>,
    new_mode_name: Option<String>,
    new_project_id: Option<i32>,
    status: i32, //0 为最新， 1为更新， 2为新增
    create_time: NaiveDateTime,
    update_time: NaiveDateTime,
}

impl From<Lang> for VOLang {
    fn from(l: Lang) -> Self {
        VOLang {
            id: l.id,
            user_id: l.user_id,
            en: l.en,
            ja: l.ja,
            ko: l.ko,
            sk: l.sk,
            cs: l.cs,
            fr: l.fr,
            es: l.es,
            pt: l.pt,
            not_trans: l.not_trans,
            descripe: l.descripe,
            label: l.label,
            file_name: l.file_name,
            mode_name: l.mode_name,
            project_id: l.project_id,
            new_user_id: l.new_user_id,
            new_en: l.new_en,
            new_ja: l.new_ja,
            new_ko: l.new_ko,
            new_sk: l.new_sk,
            new_cs: l.new_cs,
            new_fr: l.new_fr,
            new_es: l.new_es,
            new_pt: l.new_pt,
            new_not_trans: l.new_not_trans,
            new_descripe: l.new_descripe,
            new_label: l.new_label,
            new_file_name: l.new_file_name,
            new_mode_name: l.new_mode_name,
            new_project_id: l.new_project_id,
            status: l.status,
            create_time: l.create_time,
            update_time: l.update_time,
        }
    }
}

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
pub struct VOUser {
    pub id: i32,
    pub username: String,
    pub mail: String,
    pub ticker: i64,
}

impl From<User> for VOUser {
    fn from(p: User) -> Self {
        VOUser {
            id: p.id,
            username: p.username,
            mail: p.mail,
            ticker: p.update_time.timestamp(),
        }
    }
}

const CODE_TOKEN: &str = "CODE_TOKEN_EXPIRE";
const CODE_INTERNAL: &str = "CODE_SERVER_INTERNAL_ERROR";
const CODE_MAIL_OR_PASSWORD_FAIL: &str = "CODE_MAIL_OR_PASSWORD_FAIL";

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("CODE_TOKEN_EXPIRE")]
    TokenError,
    #[error("CODE_SERVER_INTERNAL_ERROR")]
    Internal(String),
    #[error("CODE_MAIL_OR_PASSWORD_FAIL")]
    MailOrPasswordFail,
}

impl From<SqlxError> for CustomError {
    fn from(e: SqlxError) -> Self {
        CustomError::Internal(format!("{:?}", e))
    }
}

impl ErrorExtensions for CustomError {
    fn extend(&self) -> Error {
        Error::new(format!("{}", self)).extend_with(|_err, e| match self {
            CustomError::TokenError => e.set("code", CODE_TOKEN.to_string()),
            CustomError::Internal(message) => {
                e.set("code", CODE_INTERNAL.to_string());
                e.set("info", message.to_string());
            }
            CustomError::MailOrPasswordFail => e.set("code", CODE_MAIL_OR_PASSWORD_FAIL.to_string()),
        })
    }
}
