use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryPolicy, RetryTransientMiddleware};
use std::time::Duration;

pub struct IBClientPortal {
    pub account: String,
    pub host: String,
    pub listen_ssl: bool,

    pub client: ClientWithMiddleware,
}

impl IBClientPortal {
    pub fn new<R>(account: String, host: String, listen_ssl: bool, retry_policy: R) -> Self
    where
        R: RetryPolicy + Sync + Send + 'static,
    {
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        default_headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("Console"),
        );
        let client = ClientBuilder::new(
            reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .default_headers(default_headers)
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
        )
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

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
