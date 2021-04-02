use async_graphql::*;
use crate::service::{SYS_PROJECT_SERVICE, SYS_LANG_SERVICE};
use crate::domain::vo::{VOProject, VOLang, Trans};
use crate::domain::dto::LanguageSearchType;

pub struct Query;

#[Object]
impl Query {
    async fn projects(&self) -> FieldResult<Vec<VOProject>> {
        SYS_PROJECT_SERVICE.all_project().await.extend()
    }

    async fn lang(&self,
                  #[graphql(default = 0)]
                  page: i32,
                  #[graphql(default = 20)]
                  page_size: i32,
                  #[graphql(default)]
                  search: Option<String>,
                  #[graphql(default = 1)]
                  project_id: i32,
                  #[graphql(default)]
                  status_type: LanguageSearchType) -> FieldResult<Vec<VOLang>> {
        SYS_LANG_SERVICE.page_language(page, page_size, project_id,search,status_type ).await.extend()
    }

    async fn trans(&self, en: String) -> FieldResult<Trans> {
        SYS_LANG_SERVICE.trans(en).await.extend()
    }
}