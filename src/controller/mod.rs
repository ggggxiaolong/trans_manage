use async_graphql::*;
pub mod query;
mod mutation;
use crate::controller::query::Query;

pub fn gen_schema()-> async_graphql::Schema<Query, EmptyMutation, EmptySubscription> {
    //限制查询的深度为5
    Schema::build(Query, EmptyMutation, EmptySubscription).finish()
}