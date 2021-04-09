use crate::domain::dto::LanguageSearchType;
use crate::domain::vo::{Token, Trans, VOLang, VOProject};
use crate::{
    domain::vo::CustomError,
    service::{SYS_LANG_SERVICE, SYS_PROJECT_SERVICE, SYS_USER_SERVICE},
    utils::{
        jwt::{gen_user_token, validate_refresh_token},
        session::Session,
    },
};
use async_graphql::Context;
use async_graphql::*;

use super::get_user;

pub struct Query;

#[Object]
impl Query {
    /// 所有的项目
    async fn projects(&self) -> FieldResult<Vec<VOProject>> {
        SYS_PROJECT_SERVICE.all_project().await.extend()
    }

    /// 查询多语言
    async fn lang(
        &self,
        #[graphql(default = 0)] page: i32,
        #[graphql(default = 20)] page_size: i32,
        #[graphql(default)] search: Option<String>,
        #[graphql(default = 1)] project_id: i32,
        #[graphql(default)] status_type: LanguageSearchType,
    ) -> FieldResult<Vec<VOLang>> {
        SYS_LANG_SERVICE
            .page_language(page, page_size, project_id, search, status_type)
            .await
            .extend()
    }

    /// 翻译多语言
    async fn trans(&self, en: String) -> FieldResult<Trans> {
        SYS_LANG_SERVICE.trans(en).await.extend()
    }

    /// 登录
    async fn login(&self, mail: String, password: String) -> FieldResult<Token> {
        SYS_USER_SERVICE
            .login(mail, password)
            .await
            .extend()
            .map(|user| gen_user_token(user.into()))
    }

    /// 刷新token
    async fn refresh_token(&self, refresh_token: String) -> FieldResult<Token> {
        let user = validate_refresh_token(&refresh_token);
        if let Some(user) = user {
            SYS_USER_SERVICE
                .check_token(user.id, user.ticker)
                .await
                .extend()
                .map(|user| gen_user_token(user.into()))
        } else {
            Err(CustomError::TokenError.extend())
        }
    }

    /// 添加多语言
    /// 更新多语言
    /// 批量更新多语言
    /// 合并更新
    async fn merge_update<'a>(&self, ctx: &Context<'_>, project_id: i32) -> FieldResult<i32> {
        let user_id = get_user(ctx)?;
        SYS_PROJECT_SERVICE
            .merge_update(project_id, user_id)
            .await
            .extend()
    }
}
