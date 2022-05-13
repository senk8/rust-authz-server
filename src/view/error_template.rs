use askama::Template;

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {}