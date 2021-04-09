#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate thiserror;

pub mod config;
pub mod controller;
pub mod dao;
pub mod domain;
pub mod service;
pub mod utils;

use crate::utils::session::Session;
use actix_cors::Cors;
use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use config::CONFIG;
use controller::{gen_schema, query::Query};

type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;

async fn index(schema: web::Data<MySchema>, gql_request: Request, session: Session) -> Response {
    let mut query = gql_request.into_inner();
    query = query.data(session);
    schema.execute(query).await.into()
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    println!("run on http://{}/graphql", &CONFIG.server_url);
    env_logger::init();
    dao::DBPool::init_pool(dao::DBPool::create_pool().await);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::permissive()
                    .allowed_methods(vec!["OPTION", "POST", "GET"])
                    .max_age(3600),
            )
            .data(web::JsonConfig::default().limit(1024 * 1024 * 50))
            .data(gen_schema())
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(gql_playgound),
            )
            .service(
                web::resource("/graphql")
                    .app_data(web::JsonConfig::default().limit(1024 * 1024 * 50))
                    .route(web::post().to(index)),
            )
    })
    .bind(&CONFIG.server_url)?
    .run()
    .await
}
