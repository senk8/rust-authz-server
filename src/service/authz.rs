use actix_web::{Responder, HttpResponse};
use crate::oauth::authz::AuthzRequest;
use crate::oauth::client::Client;
use crate::view::authz_template::{AuthzTemplate};
use crate::view::error_template::{ErrorTemplate};
use askama::Template;

fn get_client(client_id: &str) -> Option<Client> {
    Some(Client{
        client_id: "123456".to_string(),
        client_secret: "1234567".to_string(),
        redirect_uris: vec!(),
    })
    /*
    let clients: Vec<Client> = vec!{};
    clients.iter().find(|i|{
        i.client_id == client_id
    }).cloned()
    */
}

pub fn authorize(authz_request: AuthzRequest) -> impl Responder {
    let client = get_client(&authz_request.client_id);
    if let Some(c) = client {

        if c.redirect_uris.contains(&authz_request.redirect_uri) {
            return HttpResponse::BadRequest().body("Hey there!");
        }

        let html = AuthzTemplate {
            client_id: c.client_id
        };
        let response_body = html.render().unwrap();

        HttpResponse::Ok()
            .content_type("text/html")
            .body(response_body)
    } else {
        let html = ErrorTemplate {};
        let response_body = html.render().unwrap();
        HttpResponse::BadRequest()
            .content_type("text/html")
            .body(response_body)
    }
}

pub fn approve(approve_request: ApproveRequest) -> impl Responder {
    let client = get_client(&approve_request.client_id);
    if let Some(c) = client {

        if c.redirect_uris.contains(&approve_request.redirect_uri) {
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
