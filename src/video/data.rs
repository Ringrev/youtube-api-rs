use serde::{Deserialize,Serialize};

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
