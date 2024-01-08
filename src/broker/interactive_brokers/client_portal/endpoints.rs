use super::{client::IBClientPortal, model::tickle::Tickle};
use crate::{model::common::error::Error, utils::error::reqwest_error_to_rabbit_trading_error};

impl IBClientPortal {
    pub async fn tickle(&self) -> Result<Tickle, Error> {
        let response = self
            .client
            .post(self.get_url("/tickle"))
            .header(
                reqwest::header::CONTENT_LENGTH,
                reqwest::header::HeaderValue::from_static("0"),
            )
            .body("")
            .send()
            .await
            .map_err(reqwest_error_to_rabbit_trading_error)?;

        response
            .json()
            .await
            .map_err(reqwest_error_to_rabbit_trading_error)
    }
}
