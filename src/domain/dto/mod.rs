use async_graphql::*;

// 接口传输对象
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum LanguageSearchType {
    All,
    New,
    Update,
    Change,
}

impl Default for LanguageSearchType {
    fn default() -> Self {
        LanguageSearchType::All
    }
}

#[derive(InputObject)]
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
    pub not_trans: Option<i32>,
    pub descripe: Option<String>,
    pub label: Option<String>,
    pub file_name: Option<String>,
    pub project_id: Option<i32>,
    pub mode_name: Option<String>,
}

#[derive(InputObject)]
pub struct AddLang {
    pub en: String,
    pub ja: Option<String>,
    pub ko: Option<String>,
    pub sk: Option<String>,
    pub cs: Option<String>,
    pub fr: Option<String>,
    pub es: Option<String>,
    pub pt: Option<String>,
    pub not_trans: i32,
    pub descripe: Option<String>,
    pub label: Option<String>,
    pub file_name: Option<String>,
    pub project_id: i32,
    pub mode_name: Option<String>,
}
