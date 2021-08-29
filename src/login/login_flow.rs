//! Login flow where the URLs for user redirect during login are built and response token
//! fragments are extracted

use crate::config::Config;
use serde::Deserialize;
use serde::Serialize;

/// Struct with query fragments used to build authentication redirect URL
#[derive(Default, Deserialize, Serialize, Debug)]
pub struct AuthenticationRedirectUrl {
    /// The application's ID
    client_id: String,
    /// Where the API redirects the user after completed authorization
    redirect_url: String,
    /// Response after authorization process, set to 'token'
    response_type: String,
    /// The resources granted access to for the application
    scope: String,
    /// Maintains state between authorization request and authorization server's response
    state: String,
    /// The full authentication redirect URL
    full_url: String,
}

impl AuthenticationRedirectUrl {
    /// Read config to build URL
    pub fn new(config: Config) -> AuthenticationRedirectUrl {
        AuthenticationRedirectUrl::default()
            .parse_client_id(&config.client_id)
            .parse_redirect_url(&config.redirect_uri)
            .parse_response_type("token")
            .parse_scope("https://www.googleapis.com/auth/youtube.readonly")
            .parse_state("")
    }
    /// Parse client ID to its field
    pub fn parse_client_id(mut self, client_id: &str) -> Self {
        self.client_id = client_id.to_string();
        self
    }
    /// Parse redirect URL to its field
    pub fn parse_redirect_url(mut self, redirect_url: &str) -> Self {
        self.redirect_url = redirect_url.to_string();
        self
    }
    /// Parse response type to its field
    pub fn parse_response_type(mut self, response_type: &str) -> Self {
        self.response_type = response_type.to_string();
        self
    }
    /// Parse scope to its field
    pub fn parse_scope(mut self, scope: &str) -> Self {
        self.scope = scope.to_string();
        self
    }
    /// State not required, but recommended
    /// Parse state to its field
    pub fn parse_state(mut self, state: &str) -> Self {
        self.state = state.to_string();
        self
    }

    /// Build and assign the full redirect URL
    pub fn build_full_url(mut self) -> Self {
        let base_url = "https://accounts.google.com/o/oauth2/v2/auth?";
        let full_url = "".to_string()
            + base_url
            + "scope="
            + &self.scope
            + "&state="
            + &self.state
            + "&redirect_uri="
            + &self.redirect_url.to_string()
            + "&response_type="
            + &self.response_type.to_string()
            + "&client_id="
            + &self.client_id.to_string();

        self.full_url = full_url;
        self
    }
    /// Returns the full redirect URL
    pub fn get_full_url(&self) -> &String {
        &self.full_url
    }
    /// Returns the state fragment
    pub fn get_state(&self) -> &String {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_client_id() {
        let config = Config {
            api_key: "testApiKey".to_string(),
            client_id: "testClientID".to_string(),
            redirect_uri: "testRedirectURI".to_string(),
        };
        let auth_url = AuthenticationRedirectUrl::new(config);
        assert_eq!(auth_url.client_id, "testClientID");
    }
    #[test]
    fn test_parse_redirect_url() {
        let config = Config {
            api_key: "testApiKey".to_string(),
            client_id: "testClientID".to_string(),
            redirect_uri: "testRedirectURI".to_string(),
        };
        let redir_url = AuthenticationRedirectUrl::new(config);
        assert_eq!(redir_url.client_id, "testClientID");
    }
    #[test]
    fn test_build_full_url() {
        let config = Config {
            api_key: "testApiKey".to_string(),
            client_id: "testClientID".to_string(),
            redirect_uri: "testRedirectURI".to_string(),
        };
        let full_url = "https://accounts.google.com/o/oauth2/v2/auth?scope=https://www.googleapis.com/auth/youtube.readonly&state=&redirect_uri=testRedirectURI&response_type=token&client_id=testClientID";
        let redirect_url = AuthenticationRedirectUrl::new(config).build_full_url();
        assert_eq!(full_url, redirect_url.get_full_url());
        //TODO: fix the unit test
    }
}
