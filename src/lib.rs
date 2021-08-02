use crate::error::YoutubeError;
use crate::video::VideoEndPoint;
use seed::prelude::*;

pub mod client;
pub mod config;
mod error;
pub mod login_flow;
mod response;
pub mod token;
pub mod video;

/// Api object that contains basic info for querying.
pub struct YoutubeApi {
    base_url: String,
    token: String,
    api_key: String,
}

impl YoutubeApi {
    pub fn new(token: &str, api_key: &str) -> Self {
        YoutubeApi {
            base_url: "https://www.googleapis.com/youtube/v3".to_string(),
            api_key: api_key.to_string(),
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
            .to_string(),
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
