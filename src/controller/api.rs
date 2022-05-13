use actix_web::{web::Query, Responder};
use crate::oauth::authz::AuthzRequest;
use crate::service::authz;

pub async fn authorize(query: Query<AuthzRequest>) -> impl Responder {
    return authz::authorize(query.into_inner());
}

/* 
pub async fn approve(query: Query<AuthzRequest>) -> impl Responder {
    let client = get_client(&query.client_id);
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
*/

