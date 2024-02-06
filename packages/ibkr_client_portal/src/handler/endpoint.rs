use reqwest::Error;
use serde_json::Value;
use time::macros::format_description;

use crate::{
    client::IBClientPortal,
    model::{
        account::{
            GetAccountLedgerResponse, GetAccountsResponse, SwitchAccountRequest,
            SwitchAccountResponse,
        },
        contract::SecurityDefinitions,
        contract_detail::{ContractDetail, GetContractDetailRequest},
        futures::{FuturesContracts, GetFuturesBySymbolRequest},
        history::{GetMarketDataHistoryRequest, MarketDataHistory},
        market_data::{MarketDataRequest, MarketDataResponse},
        options::GetOptionsRequest,
        order_ticket::PlaceOrderRequest,
        position::{GetPositionsRequest, GetPositionsResponse},
        security::{GetSecurityDefinitionByContractIdRequest, SearchForSecurityRequest},
        stock_contract::{GetStocksBySymbolRequest, StockContracts},
        tickle::{AuthStatus, Tickle},
    },
};

impl IBClientPortal {
    pub async fn check_auth_status(&self) -> Result<AuthStatus, Error> {
        let response = self
            .client
            .post(self.get_url("/iserver/auth/status"))
            .header(
                reqwest::header::CONTENT_LENGTH,
                reqwest::header::HeaderValue::from_static("0"),
            )
            .body("")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

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
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_stocks_by_symbol(
        &self,
        request: GetStocksBySymbolRequest,
    ) -> Result<StockContracts, Error> {
        let path = "/trsrv/stocks";
        let response = self
            .client
            .get(self.get_url(path))
            .query(&[("symbols", request.symbols.join(","))])
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn market_data(
        &self,
        request: MarketDataRequest,
    ) -> Result<MarketDataResponse, Error> {
        let path = "/iserver/marketdata/snapshot";
        let conids_query = ("conids", request.conids.join(",").to_string());
        let fields_query = (
            "fields",
            request
                .fields
                .into_iter()
                .map(|field| field.to_string())
                .collect::<Vec<String>>()
                .join(",")
                .to_string(),
        );
        let since_query = request.since.map(|since| ("since", since.to_string()));
        let mut query = vec![conids_query, fields_query];
        if let Some(since_query) = since_query {
            query.push(since_query);
        }
        let response = self
            .client
            .get(self.get_url(path))
            .query(&query)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_positions(
        &self,
        request: GetPositionsRequest,
    ) -> Result<GetPositionsResponse, Error> {
        let path = format!("/portfolio/{}/positions/{}", self.account, request.page);
        let response = self.client.get(self.get_url(&path)).body("").send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_security_definition_by_contract_id(
        &self,
        request: GetSecurityDefinitionByContractIdRequest,
    ) -> Result<SecurityDefinitions, Error> {
        let path = "/trsrv/secdef";
        let response = self
            .client
            .post(self.get_url(path))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_futures_by_symbol(
        &self,
        request: GetFuturesBySymbolRequest,
    ) -> Result<FuturesContracts, Error> {
        let path = "/trsrv/futures";
        let response = self
            .client
            .get(self.get_url(path))
            .query(&[("symbols", request.symbols.join(","))])
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn search_for_security(
        &self,
        request: SearchForSecurityRequest,
    ) -> Result<Value, Error> {
        let path = "/iserver/secdef/search";
        let response = self
            .client
            .post(self.get_url(path))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_options(&self, request: GetOptionsRequest) -> Result<Value, Error> {
        let path = "/iserver/secdef/info";
        let mut query = vec![
            ("conid", request.underlying_con_id.to_string()),
            ("sectype", request.sectype.to_string()),
        ];
        if let Some(month) = request.month {
            query.push(("month", month));
        }
        if let Some(exchange) = request.exchange {
            query.push(("exchange", exchange));
        }
        if let Some(strike) = request.strike {
            query.push(("strike", strike.to_string()));
        }
        let response = self
            .client
            .get(self.get_url(path))
            .query(&query)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn logout(&self) -> Result<Value, Error> {
        let response = self
            .client
            .post(self.get_url("/logout"))
            .header(
                reqwest::header::CONTENT_LENGTH,
                reqwest::header::HeaderValue::from_static("0"),
            )
            .body("")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_account_ledger(&self) -> Result<GetAccountLedgerResponse, Error> {
        let path = format!("/portfolio/{}/ledger", self.account);
        let response = self.client.get(self.get_url(&path)).body("").send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn place_order(&self, request: PlaceOrderRequest) -> Result<Value, Error> {
        let path = format!("/iserver/account/{}/order", self.account);
        let response = self
            .client
            .post(self.get_url(&path))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_contract_detail(
        &self,
        request: GetContractDetailRequest,
    ) -> Result<ContractDetail, Error> {
        let path = format!("/iserver/contract/{}/info", request.conid);
        let response = self.client.get(self.get_url(&path)).body("").send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_market_data_history(
        &self,
        request: GetMarketDataHistoryRequest,
    ) -> Result<MarketDataHistory, Error> {
        let format_description =
            format_description!("[year][month][day]-[offset_hour]:[offset_minute]:[offset_second]");
        let path = "/iserver/marketdata/history";
        let start_time_str = match request.start_time {
            Some(start_time) => start_time
                .format(format_description)
                .unwrap() // todo: eliminate this unwrap
                .to_string(),
            None => "".to_string(),
        };

        let response = self
            .client
            .get(self.get_url(path))
            .query(&[("conid", request.conid)])
            .query(&[("period", request.period)])
            .query(&[("bar", request.bar)])
            .query(&[("exchange", request.exchange.unwrap_or("".to_owned()))])
            .query(&[("outsideRth", request.outside_rth)])
            .query(&[("startTime", start_time_str)])
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_accounts(&self) -> Result<GetAccountsResponse, Error> {
        let path = "/iserver/accounts";
        let response = self.client.get(self.get_url(&path)).body("").send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn switch_account(
        &self,
        request: SwitchAccountRequest,
    ) -> Result<SwitchAccountResponse, Error> {
        let response = self
            .client
            .post(self.get_url("/iserver/account"))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }
}
