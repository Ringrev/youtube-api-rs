pub struct Client {
    base_url: String,
    config: Config,
}

impl Client {
    pub fn new(client: Client) -> Self {
        Client {
            base_url: "https://youtube.googleapis.com/youtube/v3/",
            config: Config,
        }
    }
    pub fn url(&self) -> String {
        self.url
    }
    pub fn config(&self) -> Config {
        self.config
    }
    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }
    pub fn set_config(&mut self, client: Config) {
        self.config = client;
    }
}
