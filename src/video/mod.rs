
use crate::wasm_bindgen::JsValue;

use seed::fetch::{Method, Request};
use crate::response::{YoutubeListResponse, build_response};
use crate::video::data::YoutubeVideo;
use crate::client::ClientError;


pub  mod  query;
pub  mod  data;

pub  mod  prelude {
    pub  use crate::video::{data:: * ,VideoEndPoint , query::*};
}

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
    pub async fn insert(
        &self,
        query_search: &str,
        requested_body: &str,
    ) -> Result<YoutubeVideo, ClientError> {
        let url = format!("{}&{}", &self.url.clone(), query_search);
        let body = JsValue::from(requested_body);
        let request = Request::new(url).method(Method::Post).body(&body);
        build_response(request).await
    }
}
