mod oauth;
mod api;

use actix_web::{web, App, HttpServer};
use api::handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
        .route("/callback",web::get().to(handler::authorize))
        .route("/authz",web::get().to(handler::authorize))
    }).bind(("127.0.0.1", 8080))?
   .run()
   .await
}
