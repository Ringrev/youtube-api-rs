//
#[derive(Default, Debug)]
pub struct AccessTokenResponse {
    pub state: String,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: String,
    pub scope: String,
}
impl AccessTokenResponse {
    pub fn get_access_token(&self, x: &String) -> &String {
        &self.access_token
    }
}
