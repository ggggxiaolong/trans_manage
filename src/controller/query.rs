use async_graphql::*;
use crate::{service::{SYS_PROJECT_SERVICE, SYS_LANG_SERVICE, SYS_USER_SERVICE}, utils::jwt::gen_user_token};
use crate::domain::vo::{VOProject, VOLang, Trans, Token};
use crate::domain::dto::LanguageSearchType;

pub struct Query;

#[Object]
impl Query {
    
    /// 所有的项目
    async fn projects(&self) -> FieldResult<Vec<VOProject>> {
        SYS_PROJECT_SERVICE.all_project().await.extend()
    }

    /// 查询多语言
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

    /// 翻译多语言
    async fn trans(&self, en: String) -> FieldResult<Trans> {
        SYS_LANG_SERVICE.trans(en).await.extend()
    }

    /// 登录
    async fn login(&self, mail: String, password: String) -> FieldResult<Token> {
        SYS_USER_SERVICE.login(mail, password).await.extend().map( |user|
            gen_user_token(user.into())
        )
    }
}