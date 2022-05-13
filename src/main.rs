mod oauth;
mod controller;
mod service;
mod view;

use actix_web::{web, App, HttpServer};
use controller::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
        .route("/callback",web::get().to(api::authorize))
        .route("/authz",web::get().to(api::authorize))
    }).bind(("127.0.0.1", 8080))?
   .run()
   .await
}
