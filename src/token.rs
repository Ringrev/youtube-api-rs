#[derive(Default, Debug)]
pub struct AccessTokenResponse {
    pub scope: String,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: String,
}
impl AccessTokenResponse {
    pub fn get_access_token(&self) -> &String {
        &self.access_token
    }
}
