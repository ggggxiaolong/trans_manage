#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rbatis_macro_driver;

pub mod config;
pub mod dao;
pub mod service;
pub mod domain;

use actix_web::{middleware, App, HttpResponse, HttpServer, Responder, web};
use actix_cors::Cors;
use config::CONFIG;
use dao::RB;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    RB.link(&CONFIG.db_url).await.unwrap();
    // Start http server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["OPTION", "POST", "GET"])
                    .max_age(3600)
                    .finish(),
            )
            // .data(web::PayloadConfig::new(1024 * 1024 * 50))
            .data(web::JsonConfig::default().limit(1024 * 1024 * 50))
            // .data(app_state.clone())
            // .service(
            //     web::resource("/graphql")
            //         .app_data(web::JsonConfig::default().limit(1024 * 1024 * 50))
            //         .route(web::post().to(api_graphql)),
            // )
        // .service(test_api)
        // .service(test_api2)
    })
    .bind(&CONFIG.server_url)?
    .run()
    .await
}


