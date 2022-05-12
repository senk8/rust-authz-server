use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AuthzRequest {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uris: String,
}