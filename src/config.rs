#[derive(Debug, Default)]
pub struct Config {
    pub base_url: http::Uri,
    pub token: String,
}

impl Config {
    pub fn new(url: http::Uri, token: &str) -> Self {
        Self {
            base_url: url,
            token: token.to_string(),
        }
    }
}
