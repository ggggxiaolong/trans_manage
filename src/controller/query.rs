use async_graphql::*;
use crate::service::SYS_PROJECT_SERVICE;
use crate::domain::vo::{Project, CustomError};

pub struct Query ;

#[GQLObject]
impl Query {
    #[field(desc = "查询所有的项目信息")]
    //-> FieldResult<Token>
    async fn projects(&self) -> FieldResult<Vec<Project>>  {
        SYS_PROJECT_SERVICE.all_project().await.map_err(|e|CustomError::Internal(e.to_string()).extend())
    }

    async fn borrow_from_context_data<'ctx>(
        &self,
        ctx: &'ctx Context<'_>
    ) -> FieldResult<&'ctx String> {
        ctx.data::<String>()
    }
}