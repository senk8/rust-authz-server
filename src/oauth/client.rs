#[derive(Clone,Debug)]
pub struct Client {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uris: Vec<String>    
}