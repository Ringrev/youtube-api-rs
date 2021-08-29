use crate::video::VideoEndPoint;
use crate::error::YoutubeError;
use seed::fetch::FetchError;

/// Api object that contains basic info for querying.
pub struct Client {
    base_url: String,
    token: String,
    api_key: String,
}

impl Client {
    pub fn new(token: &str, api_key: &str) -> Self {
        Client {
            base_url: "https://www.googleapis.com/youtube/v3".to_string(),
            api_key:api_key.to_string(),
            token: token.to_string(),
        }
    }

    /// Get the video Api.
    pub fn video(&mut self) -> VideoEndPoint {
        VideoEndPoint::new(
            format!(
                "{}/API?access_token={}&api={}",
                self.base_url, self.token, self.api_key
            )
                ,
        )
    }
}

#[derive(Debug)]
pub enum ClientError {
    Youtube(YoutubeError),
    Client(FetchError),
}

impl From<YoutubeError> for ClientError {
    fn from(e: YoutubeError) -> Self {
        ClientError::Youtube(e)
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(e: serde_json::Error) -> Self {
        ClientError::Client(FetchError::SerdeError(e))
    }
}

impl From<FetchError> for ClientError {
    fn from(e: FetchError) -> Self {
        ClientError::Client(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    #[test]
    fn check_config_loaded() {
        let config = Config {
            api_key: "ADF32723289FWY".to_string(),
            client_id: "myClientID".to_string(),
            redirect_uri: "myRedirectURI".to_string(),
        };
        let client = Client::new("123", &config.api_key);
        assert_eq!(client.api_key, "ADF32723289FWY");
        assert_eq!(client.token, "123");

    }
}
