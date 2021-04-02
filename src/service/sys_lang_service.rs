use crate::service::translate::translate;
use crate::domain::domain::{sql};
use crate::domain::dto::LanguageSearchType;
use crate::domain::vo::{CustomError, VOLang, Trans};
use async_graphql::*;

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
    pub async fn trans(&self, en: String) -> Result<Trans, CustomError> {
        Ok(Trans { en: en })
    }
    pub async fn add(&self) {}
    pub async fn update(&self) {}
    pub async fn update_multiple(&self) {}
}

#[Object]
impl Trans {
    async fn ja(&self) -> String {
        translate(&self.en, "en", "ja").await
    }
    async fn ko(&self) -> String {
        translate(&self.en, "en", "ko").await
    }
    async fn sk(&self) -> String {
        translate(&self.en, "en", "sk").await
    }
    async fn cs(&self) -> String {
        translate(&self.en, "en", "cs").await
    }
    async fn fr(&self) -> String {
        translate(&self.en, "en", "fr").await
    }
    async fn es(&self) -> String {
        translate(&self.en, "en", "es").await
    }
    async fn pt(&self) -> String {
        translate(&self.en, "en", "pt").await
    }
    async fn zh(&self) -> String {
        translate(&self.en, "en", "zh").await
    }
}