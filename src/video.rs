use crate::response::{build_response, YoutubeListResponse};
use crate::wasm_bindgen::JsValue;
use crate::ClientError;
use seed::fetch::{Method, Request};
use seed::prelude::IndexMap;
use serde::*;

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
        let request = Request::new(url).method(Method::Post).body(body);
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
    pub channel_title: String,
    pub tags: Option<Vec<String>>,
    pub category_id: String,
    pub live_broadcast_content: Option<String>,
    pub default_language: Option<String>,
    pub localized: Location,
    pub default_audio_language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Location {
    title: String,
    description: String,
}
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Part {
    /// Specifies a comma-separated list of one or more video resource properties that the API
    /// response will include
    part: String,
}

impl Part {
    /// Parse selection of parameters
    pub fn new(part: &str) -> Part {
        Part::default().parse_part(part)
    }
    /// Parse every parameter
    pub fn new_with_every_parameter() -> Part {
        Part::default().parse_part(
            "contentDetails,fileDetails,id,liveStreamingDetails,localizations,player,
            processingDetails,recordingDetails,snippet,statistics,status,suggestions,topicDetails",
        )
    }
    /// Parse part to its field
    pub fn parse_part(mut self, part: &str) -> Self {
        self.part = part.to_string();
        self
    }
    /// Returns the part
    pub fn get_part(&self) -> &str {
        &self.part
    }
}
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ListVideos {
    /// Identifies the desired chart
    chart: String,
    /// Set to 'like'/'dislike' to sort by user rating
    my_rating: String,
    /// The full request URL
    query_params: String,
}

impl ListVideos {
    pub fn create_with_my_rating_like() -> ListVideos {
        ListVideos::default().parse_my_rating("like")
    }
    pub fn create_with_chart_most_popular() -> ListVideos {
        ListVideos::default().parse_chart("mostPopular")
    }
    /// Parse chart to its field
    pub fn parse_chart(mut self, chart: &str) -> Self {
        self.chart = chart.to_string();
        self
    }
    /// Parse my_rating to its field
    pub fn parse_my_rating(mut self, my_rating: &str) -> Self {
        self.my_rating = my_rating.to_string();
        self
    }
    /// Returns the chart
    pub fn get_chart(&self) -> &str {
        &self.chart
    }
    /// Returns rating, (like/dislike)
    pub fn get_my_rating(&self) -> &str {
        &self.my_rating
    }
    /// Build and assign the query parameters
    pub fn build_query_parameters(mut self) -> Self {
        let part = Part::new(
            // Required owner of video: fileDetails, processingDetails, suggestions
            "snippet,statistics,contentDetails,id,liveStreamingDetails,localizations,player,recordingDetails,status,topicDetails",
        );
        let mut query_params = String::new();
        // Check if created with chart or my_rating
        if self.my_rating.is_empty() && !self.chart.is_empty() {
            query_params = "".to_string() + "part=" + part.get_part() + "&chart=" + &self.chart;
        } else if !self.my_rating.is_empty() && self.chart.is_empty() {
            query_params =
                "".to_string() + "part=" + part.get_part() + "&my_rating=" + &self.my_rating;
        } else {
            panic!("Expected one of: chart, my_rating")
        };

        self.query_params = query_params;
        self
    }
    /// Returns the full URL
    pub fn get_query_params(&self) -> &str {
        &self.query_params
    }
}
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct InsertVideos {
    query_params: String,
}
impl InsertVideos {
    /// Build and assign the query parameters
    pub fn build_query_parameters(mut self) -> Self {
        let part = Part::new(
            // Required owner of video: fileDetails, processingDetails, suggestions
            "snippet,statistics,contentDetails,id,liveStreamingDetails,localizations,player,recordingDetails,status,topicDetails",
        );
        let mut query_params = "".to_string() + "part=" + part.get_part();

        self.query_params = query_params;
        self
    }
    pub fn get_query_params(&self) -> &str {
        &self.query_params
    }
}
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RateVideos {
    id: String,
    rating: String,
    query_params: String,
}
impl RateVideos {
    pub fn like_video() -> RateVideos {
        RateVideos::default()
            .parse_rating("like")
            .parse_id("E6UTz_Doic8")
    }
    pub fn dislike_video() -> RateVideos {
        RateVideos::default()
            .parse_rating("dislike")
            .parse_id("E6UTz_Doic8")
    }
    /// Parse id to its field
    pub fn parse_id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }
    /// Parse rating to its field
    pub fn parse_rating(mut self, rating: &str) -> Self {
        self.rating = rating.to_string();
        self
    }
    /// Returns the id
    pub fn get_id(&self) -> &str {
        &self.id
    }
    /// Returns the rating
    pub fn get_rating(&self) -> &str {
        &self.rating
    }
    /// Build and assign the query parameters
    pub fn build_query_params(mut self) -> Self {
        let query_params = "".to_string() + "id=" + &self.id + "&rating=" + &self.rating;

        self.query_params = query_params;
        self
    }
    pub fn get_query_params(&self) -> &str {
        &self.query_params
    }
}
