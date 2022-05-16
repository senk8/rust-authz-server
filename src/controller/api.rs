use actix_web::{web::Query, Responder};
use crate::oauth::authz::AuthzRequest;
use crate::service::authz;

pub async fn authorize(query: Query<AuthzRequest>) -> impl Responder {
    return authz::authorize(query.into_inner());
}

pub async fn approve(query: Query<ApproveRequest>) -> impl Responder {
    return authz::approve(query.into_inner())
 
}
