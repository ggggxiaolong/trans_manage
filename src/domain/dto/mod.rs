use async_graphql::{Enum, SimpleObject};

// 接口传输对象
#[Enum]
pub enum LanaguageSearchType {
    All,
    New,
    Update,
    Change,
}

#[SimpleObject]
pub struct UpdateLang {
    pub id: i32,
    pub en: Option<String>,
    pub ja: Option<String>,
    pub ko: Option<String>,
    pub sk: Option<String>,
    pub cs: Option<String>,
    pub fr: Option<String>,
    pub es: Option<String>,
    pub pt: Option<String>,
    #[field(name = "not_trans")]
    pub not_trans: Option<i32>,
    pub descripe: Option<String>,
    pub label: Option<String>,
    #[field(name = "file_name")]
    pub file_name: Option<String>,
    #[field(name = "project_id")]
    pub project_id: Option<i32>,
    #[field(name = "mode_name")]
    pub mode_name: Option<String>,
}

#[SimpleObject]
pub struct AddLang {
    pub en: String,
    pub ja: Option<String>,
    pub ko: Option<String>,
    pub sk: Option<String>,
    pub cs: Option<String>,
    pub fr: Option<String>,
    pub es: Option<String>,
    pub pt: Option<String>,
    #[field(name = "not_trans")]
    pub not_trans: i32,
    pub descripe: Option<String>,
    pub label: Option<String>,
    #[field(name = "file_name")]
    pub file_name: Option<String>,
    #[field(name = "project_id")]
    pub project_id: i32,
    #[field(name = "mode_name")]
    pub mode_name: Option<String>,
}