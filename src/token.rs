use seed::prelude::IndexMap;

//
#[derive(Default, Debug)]
pub struct AccessTokenResponse {
    pub state: String,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: String,
    pub scope: String,
}
impl AccessTokenResponse {
    /// Extract and parse token fragments
    /// Returns response token
    pub fn get_token(hash: String) -> AccessTokenResponse {
        let query = extract_query_fragments(hash.clone());
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
                _ => panic!("Should have token fragments, got {}", hash),
            }
        }
        access_token
    }
}
/// Extract data from  from the url fragment and return an IndexMap
/// for the Enum Variant.
/// # Panics
/// The function will panic a key that has no value.
/// # Warns
/// with no query. Theses choices are opinionated for now.
pub fn extract_query_fragments(hash: String) -> IndexMap<String, String> {
    let mut query: IndexMap<String, String> = IndexMap::new();
    let key_value: Vec<&str> = hash.split('&').collect();

    for pair in key_value {
        let mut sub = pair.split('=');
        let key = sub.next().unwrap_or_else(|| {
            panic!(
                "we should have a key for the parameter key but got {}",
                hash
            )
        });
        let value = sub
            .next()
            .unwrap_or_else(|| panic!("we should have a value for the key but got {}", hash));
        query.insert(key.to_string(), value.to_string());
    }
    query
}

#[cfg(test)]
mod tests {
    use crate::token::{extract_query_fragments, AccessTokenResponse};
    #[test]
    fn test_extract_state() {
        let token = AccessTokenResponse {
            state: "testState".to_string(),
            access_token: "testAccessToken".to_string(),
            token_type: "testTokenType".to_string(),
            expires_in: "testExpiresIn".to_string(),
            scope: "testScope".to_string(),
        };
        let url = "https://oauth2.example.com/#state=test&access_token=4/P7q7W91&token_type=Bearer&expires_in=3600&scope=test";
        extract_query_fragments(url.to_string());
        assert_eq!(token.state, "testState")
    }
    #[test]
    fn test_extract_access_token() {
        let token = AccessTokenResponse {
            state: "testState".to_string(),
            access_token: "testAccessToken".to_string(),
            token_type: "testTokenType".to_string(),
            expires_in: "testExpiresIn".to_string(),
            scope: "testScope".to_string(),
        };
        let url = "https://oauth2.example.com/#state=test&access_token=4/P7q7W91&token_type=Bearer&expires_in=3600&scope=test";
        extract_query_fragments(url.to_string());
        assert_eq!(token.access_token, "testAccessToken")
    }
    #[test]
    fn test_extract_token_type() {
        let token = AccessTokenResponse {
            state: "testState".to_string(),
            access_token: "testAccessToken".to_string(),
            token_type: "testTokenType".to_string(),
            expires_in: "testExpiresIn".to_string(),
            scope: Default::default(),
        };
        let url = "https://oauth2.example.com/#state=test&access_token=4/P7q7W91&token_type=Bearer&expires_in=3600&scope=test";
        extract_query_fragments(url.to_string());
        assert_eq!(token.token_type, "testTokenType")
    }
    #[test]
    fn test_extract_expires_in() {
        let token = AccessTokenResponse {
            state: "testState".to_string(),
            access_token: "testAccessToken".to_string(),
            token_type: "testTokenType".to_string(),
            expires_in: "testExpiresIn".to_string(),
            scope: "testScope".to_string(),
        };
        let url = "https://oauth2.example.com/#state=test&access_token=4/P7q7W91&token_type=Bearer&expires_in=3600&scope=test";
        extract_query_fragments(url.to_string());
        assert_eq!(token.expires_in, "testExpiresIn")
    }
    #[test]
    fn test_extract_scope() {
        let token = AccessTokenResponse {
            state: "testState".to_string(),
            access_token: "testAccessToken".to_string(),
            token_type: "testTokenType".to_string(),
            expires_in: "testExpiresIn".to_string(),
            scope: "testScope".to_string(),
        };
        let url = "https://oauth2.example.com/#state=test&access_token=4/P7q7W91&token_type=Bearer&expires_in=3600&scope=test";
        extract_query_fragments(url.to_string());
        assert_eq!(token.scope, "testScope")
    }
}
