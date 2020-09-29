use crate::domain::domain::{sql};
use crate::domain::dto::LanguageSearchType;
use crate::domain::vo::{CustomError, VOLang};

pub struct SysLangService {}

impl SysLangService {
    pub async fn page_language(
        &self,
        page: i32,
        size: i32,
        project_id: i32,
        search: Option<String>,
        status: LanguageSearchType,
    ) -> Result<Vec<VOLang>, CustomError> {
        let lang = sql::lang::page_fetch(page, size, search, project_id, status).await?;
        Ok(lang.into_iter().map(|l| l.into()).collect())
    }
    pub async fn trans(&self, en: &str, to: &str) {}
    pub async fn add(&self) {}
    pub async fn update(&self) {}
    pub async fn update_multiple(&self) {}
}