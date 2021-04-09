use async_graphql::*;
mod mutation;
pub mod query;
use crate::{controller::query::Query, domain::vo::CustomError, utils::session::Session};
use async_graphql::Context;

pub fn gen_schema() -> async_graphql::Schema<Query, EmptyMutation, EmptySubscription> {
    //限制查询的深度为5
    Schema::build(Query, EmptyMutation, EmptySubscription).finish()
}

pub fn get_user(ctx: &Context<'_>) -> FieldResult<i32> {
    let id = ctx
        .data_opt::<Session>()
        .map_or(-1, |session| session.user.id);
    if id != -1 {
        Ok(id)
    } else {
        Err(CustomError::TokenError.extend())
    }
}
