use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct AuthzTemplate {
    pub client_id: String
}
