use actix_web::{get, post, web::Query, HttpResponse, Responder};
use crate::oauth::authz::AuthzRequest;
use crate::oauth::client::{Client, self};

fn get_client(client_id: String) -> Option<Client> {
    let clients: Vec<Client> = vec!{};
    clients.iter().find(|i|{
        i.client_id == client_id
    }).cloned()
}

pub async fn authorize(query: Query<AuthzRequest>) -> impl Responder {
    let client = get_client(query.client_id.clone());
    if let Some(c) = client {

        if c.redirect_uris.contains(&query.redirect_uri) {
            return HttpResponse::BadRequest().body("Hey there!");
        }

        HttpResponse::Ok().body("Hey there!")
    } else {
        HttpResponse::InternalServerError().body("Hey there!")
    }
}

pub async fn approve(query: Query<AuthzRequest>) -> impl Responder {
    let client = get_client(query.client_id.clone());
    if let Some(c) = client {

        if c.redirect_uris.contains(&query.redirect_uri) {
            return HttpResponse::BadRequest().body("Hey there!");
        }

        HttpResponse::Found().body("Hey there!")
    } else {
        match query.response_type {
            "code" => {
                let code = "12";
                HttpResponse::Found().body("Hey there!")
            },
            "token" => (),
        }
       } 
    }
}

