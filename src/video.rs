
use seed::fetch::{Method, Request};
use serde::*;
use crate::response::{YoutubeListResponse, build_response};
use crate::ClientError;

pub struct VideoEndPoint {
    url: String,
}

impl VideoEndPoint {
    /// Complete the url.s
    pub fn new(url: String) -> Self {
        let url = url.replace("API", "videos");
        VideoEndPoint { url }
    }
    /// Returns a list of videos that match the API request parameters.
    /// Get the list with additional parameter using format -> key=value&;
    /// More information on the official documentation https://developers.google.com/youtube/v3/docs/videos/list .
    pub async fn list(
        &self,
        query_search: &str,
    ) -> Result<YoutubeListResponse<YoutubeVideo>, ClientError> {
        let url = format!("{}&{}", &self.url.clone(), query_search);
        let request = Request::new(url).method(Method::Get);
        build_response(request).await
    }


    /// Uploads a video to YouTube and optionally sets the video's metadata.
    /// This method supports media upload. Uploaded files must conform to these constraints
    /// https://developers.google.com/youtube/v3/docs/videos/insert
    pub async fn insert(&self, query_search: &str , requested_body:&str) -> Result<YoutubeVideo, ClientError> {
        let url = format!("{}&{}", &self.url.clone(), query_search);
        let request = Request::new(url).method(Method::Post).body(JsValue::from(requested_body));
        build_response(request).await
    }
}

/// The video data model https://developers.google.com/youtube/v3/docs/videos#resource
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YoutubeVideo {
    pub kind: String,
    pub etag: String,
    pub id: String,
    pub snippet: Option<VideoSnippet>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VideoSnippet {
    pub published_at: String,
    pub channel_id: String,
    pub title: String,
    pub description: String,
}
