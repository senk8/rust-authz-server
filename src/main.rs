use actix_web::{get, post, web::{self, Query, Json}, App, HttpResponse, HttpServer, Responder, http::Method };
use oauth::client::Client;
use oauth::authz::AuthzRequest;
mod oauth;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn authorize(range: Query<AuthzRequest>) -> impl Responder {
    let client = get_client(range.client_id.clone());
    HttpResponse::Ok().body("Hey there!")
}

fn get_client(client_id: String) -> Option<Client> {
    let clients: Vec<Client> = vec!{};
    clients.iter().find(|i|{
        i.client_id == client_id
    }).cloned()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
        .route("/callback",web::get().to(authorize))
        .route("/authz",web::get().to(authorize))
    }).bind(("127.0.0.1", 8080))?
   .run()
   .await
}
