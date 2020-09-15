use rbatis::crud::CRUDEnable;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::domain::vo::*;

/// 数据库对象
#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct DBLang {
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
    status: i32,//0 为最新， 1为更新， 2为新增
    create_time: NaiveDateTime,
    update_time: NaiveDateTime,
}

#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct DBUser {
    pub id: i32,
    pub username: String,
    pub mail: String,
    pub password: String,
}


#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct DBProject {
    pub id: i32,
    pub name: String,
}

impl DBProject {
    pub fn to_vo(&self) -> Project{
        Project{
            id: self.id,
            name: self.name.clone()
        }
    }
}

#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct DBTempLang {
    id: i32,
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
}