use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub api_key: String,
    pub client_id: String,
    pub redirect_uri: String,
}
