use async_graphql::*;
pub mod query;
mod mutation;
use crate::controller::query::Query;

pub fn gen_schema()-> async_graphql::Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::new(Query, EmptyMutation, EmptySubscription)
}