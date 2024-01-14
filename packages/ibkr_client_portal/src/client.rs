pub struct IBClientPortal {
    pub account: String,
    pub host: String,
    pub listen_ssl: bool,

    pub client: reqwest::Client,
}

impl IBClientPortal {
    pub fn new(account: String, host: String, listen_ssl: bool) -> Self {
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        default_headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("Console"),
        );
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .default_headers(default_headers)
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .unwrap();

        IBClientPortal {
            account,
            host,
            listen_ssl,
            client,
        }
    }

    pub fn get_url(&self, path: &str) -> String {
        let protocol = if self.listen_ssl { "https" } else { "http" };
        format!("{protocol}://{}/v1/api{path}", self.host)
    }
}
