use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::domain::vo::*;

/// 数据库对象
#[derive(sqlx::FromRow)]
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
    status: i32,
    //0 为最新， 1为更新， 2为新增
    create_time: NaiveDateTime,
    update_time: NaiveDateTime,
}

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub mail: String,
    pub password: String,
}

#[derive(sqlx::FromRow)]
pub struct Project {
    pub id: i32,
    pub name: String,
}

pub static SQL_NAME_PROJECT: String = "project".to_string();
pub static SQL_NAME_USER: String = "user".to_string();
pub static SQL_NAME_LANG: String = "lang".to_string();
pub static SQL_QUERY_PROJECT: String = "id,name".to_string();
pub static SQL_QUERY_USER: String = "id,username,mail,password".to_string();
pub static SQL_QUERY_LANG: String = "id,user_id,en,ja,ko,sk,cs,fr,es,pt,not_trans,descripe,label,file_name,mode_name,project_id,new_user_id,new_en,new_ja,new_ko,new_sk,new_cs,new_fr,new_es,new_pt,new_not_trans,new_descripe,new_label,new_file_name,new_mode_name,new_project_id,status,create_time,update_time".to_string();