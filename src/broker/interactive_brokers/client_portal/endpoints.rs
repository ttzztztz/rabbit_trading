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
    use dotenv::dotenv;
    use std::env;

    use crate::broker::interactive_brokers::client_portal::client::IBClientPortal;

    const ENV_KEY_TEST_ACCOUNT: &'static str = "IBKR_TEST_ACCOUNT";
    const TEST_ACCOUNT: &'static str = "0";
    const TEST_HOST: &'static str = "localhost:5000";
    const CONID_QQQ: i64 = 320227571;

    fn get_test_account() -> String {
        dotenv().unwrap();
        env::var(ENV_KEY_TEST_ACCOUNT).unwrap_or(TEST_ACCOUNT.to_owned())
    }

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_tickle() {
        let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
        let response_result = ib_cp_client.tickle().await;
        assert!(response_result.is_ok());
        let response = response_result.unwrap();
        assert!(response.session.len() > 0);
        assert!(response.user_id > 0);
    }

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_get_stocks_by_symbol() {
        let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
        let response_result = ib_cp_client
            .get_stocks_by_symbol(vec!["QQQ".to_owned()])
            .await;
        assert!(response_result.is_ok());
        let response = response_result.unwrap();
        let response_stock_contract_info_option = response.get("QQQ");
        assert!(response_stock_contract_info_option.is_some());
        let response_stock_contract_info = response_stock_contract_info_option.unwrap();
        assert!(response_stock_contract_info.len() > 0);
        let contract_info = &response_stock_contract_info[0];
        assert!(contract_info.contracts.len() > 0);
        let contract = &contract_info.contracts[0];
        assert!(contract.conid == CONID_QQQ);
        assert!(contract_info.name.starts_with("INVESCO QQQ"));
    }

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_get_contract_detail() {
        let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
        let response_result = ib_cp_client.get_contract_detail(CONID_QQQ).await;
        assert!(response_result.is_ok());
        let response = response_result.unwrap();
        assert_eq!("QQQ", response.symbol);
        assert!(response.valid_exchanges.len() > 0);
    }

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_get_positions() {
        let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
        let response_result = ib_cp_client.get_positions(1).await;
        assert!(response_result.is_ok());
        let response = response_result.unwrap();
        response.into_iter().for_each(|position| {
            assert!(position.conid > 0);
        });
    }
}
