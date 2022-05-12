use actix_web::{get, post, web::Query, HttpResponse, Responder};
use crate::oauth::authz::AuthzRequest;
use crate::oauth::client::Client;

fn get_client(client_id: String) -> Option<Client> {
    let clients: Vec<Client> = vec!{};
    clients.iter().find(|i|{
        i.client_id == client_id
    }).cloned()
}

pub async fn authorize(range: Query<AuthzRequest>) -> impl Responder {
    let client = get_client(range.client_id.clone());
    HttpResponse::Ok().body("Hey there!")
}

