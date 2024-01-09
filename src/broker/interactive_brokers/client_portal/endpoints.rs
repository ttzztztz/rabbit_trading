use super::{
    client::IBClientPortal,
    model::{stock_contract::StockContracts, tickle::Tickle},
};
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

    pub async fn get_stocks_by_symbol(
        &self,
        symbols: Vec<String>,
    ) -> Result<StockContracts, reqwest::Error> {
        let path = "/trsrv/stocks";
        let request = self
            .client
            .get(self.get_url(path))
            .query(&[("symbols", symbols.join(","))]);
        let response = request.send().await?;
        response.json().await
    }
}

#[cfg(test)]
mod test_ib_cp_client {
    use crate::broker::interactive_brokers::client_portal::client::IBClientPortal;

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_tickle() {
        let ib_cp_client = IBClientPortal::new(Option::None);
        let response_result = ib_cp_client.tickle().await;
        assert!(response_result.is_ok());
    }

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_get_stocks_by_symbol() {
        let ib_cp_client = IBClientPortal::new(Option::None);
        let response_result = ib_cp_client
            .get_stocks_by_symbol(vec!["QQQ".to_owned()])
            .await;
        assert!(response_result.is_ok());
    }
}
