use core::fmt;
use serde::Deserialize;

/// Youtube Error, see https://developers.google.com/youtube/v3/docs/errors for more information.

#[derive(Debug, Deserialize)]
pub struct YoutubeError {
    pub(crate) code: u16,
    pub(crate) message: String,
}

impl fmt::Display for YoutubeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.code, self.message)
    }
}

impl YoutubeError {
    /// Get the HTTP status code of an error response.
    pub fn code(&self) -> u16 {
        self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
