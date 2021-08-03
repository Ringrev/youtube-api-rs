use crate::response::{build_response, YoutubeListResponse};
use crate::wasm_bindgen::JsValue;
use crate::ClientError;
use seed::fetch::{Method, Request};
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
        let request = Request::new(url).method(Method::Post).body(&body);
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
            //TODO: Separate into their own function instead of string
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
    /// Sets my_rating value to 'like' to list user's liked videos
    pub fn create_with_my_rating_like() -> ListVideos {
        ListVideos::default().parse_my_rating("like")
    }
    /// Sets my_rating value to 'mostPopular' to list most popular videos on YouTube at the moment
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
    /// Query parameters to be used in URL
    query_params: String,
}
impl InsertVideos {
    /// Build and assign the query parameters
    pub fn build_query_parameters(mut self) -> Self {
        let part = Part::new(
            // Required owner of video: fileDetails, processingDetails, suggestions
            "snippet,statistics,contentDetails,id,liveStreamingDetails,localizations,player,recordingDetails,status,topicDetails",
        );
        let query_params = "".to_string() + "part=" + part.get_part();

        self.query_params = query_params;
        self
    }
    pub fn get_query_params(&self) -> &str {
        &self.query_params
    }
}
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RateVideo {
    /// ID of the YouTube video to be rated
    id: String,
    /// Rating to record
    rating: String,
    /// Query parameters to be used in URL
    query_params: String,
}
impl RateVideo {
    /// Create with value 'like' as rating
    pub fn like_video() -> RateVideo {
        RateVideo::default()
            .parse_rating("like")
            .parse_id("E6UTz_Doic8")
    }
    /// Create with value 'dislike' as rating
    pub fn dislike_video() -> RateVideo {
        RateVideo::default()
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
    /// Returns query parameters
    pub fn get_query_params(&self) -> &str {
        &self.query_params
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UpdateVideos {
    /// Query parameters to be used in URL
    query_params: String,
}
impl UpdateVideos {
    /// Build and assign the query parameters
    pub fn build_query_parameters(mut self) -> Self {
        let part = Part::new(
            "snippet,statistics,contentDetails,id,liveStreamingDetails,localizations,player,recordingDetails,status,topicDetails",
        );
        let query_params = "".to_string() + "part=" + part.get_part();

        self.query_params = query_params;
        self
    }
    /// Returns query parameters
    pub fn get_query_params(&self) -> &str {
        &self.query_params
    }
}
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GetRating {
    /// ID of the YouTube video retrieving data from
    id: String,
    /// Query parameters to be used in URL
    query_params: String,
}

impl GetRating {
    pub fn new() -> GetRating {
        GetRating::default().parse_id("E6UTz_Doic8")
    }
    /// Parse id to its field
    pub fn parse_id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }
    /// Returns the id
    pub fn id(&self) -> &str {
        &self.id
    }
    /// Build and assign the query parameters
    pub fn build_query_parameters(mut self) -> Self {
        let query_params = "".to_string() + "id=" + &self.id;
        self.query_params = query_params;
        self
    }
}
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DeleteVideo {
    /// ID of the YouTube video to be deleted
    id: String,
    /// Query parameters to be used in URL
    query_params: String,
}
impl DeleteVideo {
    pub fn new() -> DeleteVideo {
        //TODO: Make solution for gathering id
        DeleteVideo::default().parse_id("E6UTz_Doic8")
    }
    /// Parse id to its field
    pub fn parse_id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }
    /// Returns the id
    pub fn id(&self) -> &str {
        &self.id
    }
    /// Build and assign the query parameters
    pub fn build_query_parameters(mut self) -> Self {
        let query_params = "".to_string() + "id=" + &self.id;
        self.query_params = query_params;
        self
    }
}
#[cfg(test)]
mod tests {
    use crate::video::DeleteVideo;
    use crate::video::GetRating;
    use crate::video::InsertVideos;
    use crate::video::ListVideos;
    use crate::video::UpdateVideos;
    #[test]
    fn test_build_query_params_list_most_popular_videos() {
        let most_popular = ListVideos::create_with_chart_most_popular().build_query_parameters();
        assert_eq!(most_popular.query_params, "part=snippet,statistics,contentDetails,id,liveStreamingDetails,localizations,player,recordingDetails,status,topicDetails&chart=mostPopular")
    }
    #[test]
    fn test_build_query_params_list_liked_videos() {
        let liked_videos = ListVideos::create_with_my_rating_like().build_query_parameters();
        assert_eq!(liked_videos.query_params, "part=snippet,statistics,contentDetails,id,liveStreamingDetails,localizations,player,recordingDetails,status,topicDetails&my_rating=like")
    }
    #[test]
    fn test_build_query_params_insert_video() {
        let insert_video = InsertVideos::default().build_query_parameters();
        assert_eq!(insert_video.query_params, "part=snippet,statistics,contentDetails,id,liveStreamingDetails,localizations,player,recordingDetails,status,topicDetails")
    }
    #[test]
    fn test_build_query_params_update_video() {
        let update_video = UpdateVideos::default().build_query_parameters();
        assert_eq!(update_video.query_params, "part=snippet,statistics,contentDetails,id,liveStreamingDetails,localizations,player,recordingDetails,status,topicDetails")
    }
    #[test]
    fn test_build_query_params_delete_video() {
        let delete_video = DeleteVideo::new().build_query_parameters();
        assert_eq!(delete_video.query_params, "id=E6UTz_Doic8")
    }
    #[test]
    fn test_build_query_params_get_rating() {
        let get_rating = GetRating::new().build_query_parameters();
        assert_eq!(get_rating.query_params, "id=E6UTz_Doic8")
    }
    #[test]
    #[should_panic]
    fn test_panic_with_both_chart_and_my_rating_values() {
        let most_popular = ListVideos {
            chart: "123".to_string(),
            my_rating: "123".to_string(),
            query_params: "".to_string(),
        }
        .build_query_parameters();
        assert_eq!(most_popular.query_params, "part=snippet,statistics,contentDetails,id,liveStreamingDetails,localizations,player,recordingDetails,status,topicDetails&chart=mostPopular")
    }
    #[test]
    #[should_panic]
    fn test_panic_with_neither_chart_nor_my_rating_values() {
        let most_popular = ListVideos {
            chart: "".to_string(),
            my_rating: "".to_string(),
            query_params: "".to_string(),
        }
        .build_query_parameters();
        assert_eq!(most_popular.query_params, "part=snippet,statistics,contentDetails,id,liveStreamingDetails,localizations,player,recordingDetails,status,topicDetails&chart=mostPopular")
    }
}
