

use seed::log;
use seed::prelude::*;
use serde::{de, de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use serde_json::Value;
use crate::error::YoutubeError;

use crate::client::ClientError;

#[derive(Debug)]
pub(crate) enum Response<T> {
    Ok(T),
    Err(YoutubeError),
}

impl<T> Into<Result<T, YoutubeError>> for Response<T> {
    fn into(self) -> Result<T, YoutubeError> {
        match self {
            Response::Ok(success) => Ok(success),
            Response::Err(err) => Err(err),
        }
    }
}


/// We need this since error response is contained inside the "error" key.
impl<'de, T> Deserialize<'de> for Response<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = serde_json::Map::deserialize(deserializer)?;
        log!("Deserialize normal Response: {:?}", map.clone());
        let rest = Value::Object(map.clone());
        let error = map.get("error");

        match error {
            Some(e) => {
                YoutubeError::deserialize(e)
                    .map(Response::Err)
                    .map_err(de::Error::custom)
            }
            None => T::deserialize(rest)
                .map(Response::Ok)
                .map_err(de::Error::custom),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YoutubeListResponse<T> {
    pub kind: String,
    pub etag: String,
    #[serde(default)]
    pub next_page_token: String,
    #[serde(default)]
    pub prev_page_token: String,
    pub page_info: YoutubePageInfo,
    pub items: Vec<T>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YoutubePageInfo {
    total_results: u16,
    results_per_page: u16,
}

pub async fn build_response<T: 'static + DeserializeOwned + Serialize>(
    request: Request<'_>,
) -> Result<T, ClientError> {
    let response = fetch(request).await;
    match response {
        Ok(res) => {
            let response = res.text().await?;
            deserialize_response(response.as_str())
        }
        Err(err) => Err(ClientError::Client(err)),
    }
}

fn deserialize_response<T>(text: &str) -> Result<T, ClientError>
where
    T: DeserializeOwned,
{
    let response: Response<T> = serde_json::from_str(text)?;
    Ok(Into::<Result<T, YoutubeError>>::into(response)?)
}
