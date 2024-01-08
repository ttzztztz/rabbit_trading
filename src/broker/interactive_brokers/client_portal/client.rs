pub struct IBClientPortal {
    pub(super) client: reqwest::Client,
    pub(super) host: String,
}

impl IBClientPortal {
    pub fn new(host: Option<String>) -> Self {
        const DEFAULT_HOST: &'static str = "http://localhost:5000";

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

        let host = host.unwrap_or(DEFAULT_HOST.to_owned());
        IBClientPortal { client, host }
    }

    pub(crate) fn get_url(&self, path: &str) -> String {
        format!("{}/v1/api{path}", self.host)
    }
}
