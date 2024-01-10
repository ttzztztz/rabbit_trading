use serde_json::{json, Value};

use super::{
    client::IBClientPortal,
    model::{
        contract_detail::ContractDetail, order_ticket::OrderTicket, position::Position,
        stock_contract::StockContracts, tickle::Tickle,
    },
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
    ) -> Result<StockContracts, Error> {
        let path = "/trsrv/stocks";
        let request = self
            .client
            .get(self.get_url(path))
            .query(&[("symbols", symbols.join(","))]);
        let response = request
            .send()
            .await
            .map_err(reqwest_error_to_rabbit_trading_error)?;

        response
            .json()
            .await
            .map_err(reqwest_error_to_rabbit_trading_error)
    }

    pub async fn get_positions(&self, page: i32) -> Result<Vec<Position>, Error> {
        let path = format!("/portfolio/{}/positions/{}", self.account, page);
        let response = self
            .client
            .get(self.get_url(&path))
            .body("")
            .send()
            .await
            .map_err(reqwest_error_to_rabbit_trading_error)?;

        response
            .json()
            .await
            .map_err(reqwest_error_to_rabbit_trading_error)
    }

    pub async fn place_order(&self, orders: Vec<OrderTicket>) -> Result<Value, Error> {
        let path = format!("/iserver/account/{}/order", self.account);
        let payload = json!({"orders":orders});
        let request = self.client.post(self.get_url(&path));
        let response = request
            .body(payload.to_string())
            .send()
            .await
            .map_err(reqwest_error_to_rabbit_trading_error)?;

        response
            .json()
            .await
            .map_err(reqwest_error_to_rabbit_trading_error)
    }

    pub async fn get_contract_detail(&self, conid: i64) -> Result<ContractDetail, Error> {
        let path = format!("/iserver/contract/{}/info", conid);
        let response = self
            .client
            .get(self.get_url(&path))
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

#[cfg(test)]
mod test_ib_cp_client {
    use crate::broker::interactive_brokers::client_portal::client::IBClientPortal;

    const TEST_ACCOUNT: &'static str = "0";
    const TEST_HOST: &'static str = "localhost";

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_tickle() {
        let ib_cp_client =
            IBClientPortal::new(TEST_ACCOUNT.to_owned(), TEST_HOST.to_owned(), false);
        let response_result = ib_cp_client.tickle().await;
        assert!(response_result.is_ok());
    }

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_get_stocks_by_symbol() {
        let ib_cp_client =
            IBClientPortal::new(TEST_ACCOUNT.to_owned(), TEST_HOST.to_owned(), false);
        let response_result = ib_cp_client
            .get_stocks_by_symbol(vec!["QQQ".to_owned()])
            .await;
        assert!(response_result.is_ok());
    }
}
