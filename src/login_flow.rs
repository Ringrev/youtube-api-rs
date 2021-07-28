//! Login flow where the URLs for user redirect during login are built and response token
//! fragments are extracted

use crate::config::Config;
use crate::extract_query_parameters::extract_query_fragments;
use crate::token::AccessTokenResponse;
use seed::Url;
use serde::Deserialize;
use serde::Serialize;

/// Struct with query fragments used to build authentication redirect URL
#[derive(Default, Deserialize, Serialize, Debug)]
pub struct AuthenticationRedirectUrl {
    // The application's ID
    client_id: String,
    // Where the API redirects the user after completed authorization
    redirect_url: String,
    // Response after authorization process, set to 'token'
    response_type: String,
    // The resources granted access to for the application
    scope: String,
    // Maintains state between authorization request and authorization server's response
    state: String,
    // The full authentication redirect URL
    full_url: String,
}

impl AuthenticationRedirectUrl {
    /// Parse URL query fragments to fields
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
    /// Builds and returns a full redirect URL
    pub fn build_full_url(&mut self) -> String {
        let base_url = "https://accounts.google.com/o/oauth2/v2/auth?";
        let full_url = "".to_string()
            + base_url
            + "scope="
            + &self.scope.to_string()
            + "&state="
            + &self.state
            + "&redirect_uri="
            + &self.redirect_url.to_string()
            + "&response_type="
            + &self.response_type.to_string()
            + "&client_id="
            + &self.client_id.to_string();
        full_url
    }
    /// Parse the full redirect URL to its field
    pub fn parse_full_url(mut self) -> Self {
        self.full_url = self.build_full_url();
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

/// Extract and parse token fragments
pub fn parse_token_fragments(url: Url) {
    let query = extract_query_fragments(url);
    let iterations = query.iter();

    let mut access_token = AccessTokenResponse::default();
    // Extract URL fragments
    for e in iterations {
        match e.0.as_str() {
            "state" => {
                access_token.state = e.1.to_string();
            }
            "access_token" => {
                access_token.access_token = e.1.to_string();
            }
            "token_type" => {
                access_token.token_type = e.1.to_string();
            }
            "expires_in" => {
                access_token.expires_in = e.1.to_string();
            }
            "scope" => {
                access_token.scope = e.1.to_string();
            }
            _ => panic!("Should have token fragments, got {}",),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::config::Config;
    use crate::login_flow::AuthenticationRedirectUrl;
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
        let redir_url = AuthenticationRedirectUrl::new(config);
        redir_url.parse_full_url();
        assert_eq!(redir_url.full_url, "123");
    }
}
