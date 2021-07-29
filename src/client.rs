use crate::config::Config;

pub struct Client {
    base_url: String,
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Client {
            base_url: "https://youtube.googleapis.com/youtube/v3/".to_string(),
            config,
        }
    }
    pub fn url(&self) -> &str {
        &self.base_url
    }
    pub fn config(&self) -> &Config {
        &self.config
    }
    pub fn set_url(&mut self, url: String) {
        self.base_url = url;
    }
    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }
}
#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::config::Config;
    #[test]
    fn it_works() {
        let config = Config {
            api_key: "ADF32723289FWY".to_string(),
            client_id: "myClientID".to_string(),
            redirect_uri: "myRedirectURI".to_string(),
        };
        let client = Client::new(config);
        assert_eq!(client.config().api_key, "ADF32723289FWY");
    }
}
