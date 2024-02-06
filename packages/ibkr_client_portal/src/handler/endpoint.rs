use reqwest::Error;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use std::collections::HashMap;
use time::{macros::format_description, OffsetDateTime};

use crate::{
    client::IBClientPortal,
    model::{
        account::AccountLedger,
        contract::SecurityDefinitions,
        contract_detail::ContractDetail,
        definition::AssetClass,
        futures::FuturesContracts,
        history::MarketDataHistory,
        market_data::MarketDataRequest,
        order_ticket::OrderTicket,
        position::Position,
        stock_contract::StockContracts,
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
        symbols: Vec<String>,
    ) -> Result<StockContracts, Error> {
        let path = "/trsrv/stocks";
        let request = self
            .client
            .get(self.get_url(path))
            .query(&[("symbols", symbols.join(","))]);
        let response = request.send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn market_data(
        &self,
        request: MarketDataRequest,
    ) -> Result<Vec<HashMap<String, Value>>, Error> {
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

        let request = self.client.get(self.get_url(path)).query(&query);
        let response = request.send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_positions(&self, page: i32) -> Result<Vec<Position>, Error> {
        let path = format!("/portfolio/{}/positions/{}", self.account, page);
        let response = self.client.get(self.get_url(&path)).body("").send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_security_definition_by_contract_id(
        &self,
        contract_ids: Vec<i64>,
    ) -> Result<SecurityDefinitions, Error> {
        let path = "/trsrv/secdef";
        let payload = json!({
            "conids" : contract_ids,
        });
        let request = self.client.post(self.get_url(path));
        let response = request.body(payload.to_string()).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_futures_by_symbol(
        &self,
        symbols: Vec<&str>,
    ) -> Result<FuturesContracts, Error> {
        let path = "/trsrv/futures";
        let request = self
            .client
            .get(self.get_url(path))
            .query(&[("symbols", symbols.join(","))]);
        let response = request.send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn search_for_security(
        &self,
        symbol_or_name: &str,
        is_name: bool,
        sec_type: AssetClass,
    ) -> Result<Value, Error> {
        let path = "/iserver/secdef/search";
        let body = json!( {
            "symbol": symbol_or_name,
            "name": is_name,
            "secType": sec_type,
        });
        let request = self.client.post(self.get_url(path)).body(body.to_string());
        let response = request.send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_options(
        &self,
        underlying_con_id: i64,
        sectype: AssetClass,
        month: Option<String>,
        exchange: Option<String>,
        strike: Option<Decimal>,
    ) -> Result<Value, Error> {
        let path = "/iserver/secdef/info";
        let mut query = vec![
            ("conid", underlying_con_id.to_string()),
            ("sectype", sectype.to_string()),
        ];
        if let Some(month) = month {
            query.push(("month", month));
        }
        if let Some(exchange) = exchange {
            query.push(("exchange", exchange));
        }
        if let Some(strike) = strike {
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

    pub async fn get_account_ledger(&self) -> Result<HashMap<String, AccountLedger>, Error> {
        let path = format!("/portfolio/{}/ledger", self.account);
        let response = self.client.get(self.get_url(&path)).body("").send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn place_order(&self, orders: Vec<OrderTicket>) -> Result<Value, Error> {
        let path = format!("/iserver/account/{}/order", self.account);
        let payload = json!({"orders":orders});
        let request = self.client.post(self.get_url(&path));
        let response = request.body(payload.to_string()).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_contract_detail(&self, conid: i64) -> Result<ContractDetail, Error> {
        let path = format!("/iserver/contract/{}/info", conid);
        let response = self.client.get(self.get_url(&path)).body("").send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_market_data_history(
        &self,
        conid: i64,
        exchange: Option<&str>,
        period: &str,
        bar: &str,
        outside_rth: bool,
        start_time: Option<OffsetDateTime>,
    ) -> Result<MarketDataHistory, Error> {
        let format_description =
            format_description!("[year][month][day]-[offset_hour]:[offset_minute]:[offset_second]");
        let path = "/iserver/marketdata/history";
        let start_time_str = match start_time {
            Some(start_time) => start_time
                .format(format_description)
                .unwrap() // todo: eliminate this unwrap
                .to_string(),
            None => "".to_string(),
        };

        let request = self
            .client
            .get(self.get_url(path))
            .query(&[("conid", conid)])
            .query(&[("period", period)])
            .query(&[("bar", bar)])
            .query(&[("exchange", exchange.unwrap_or(""))])
            .query(&[("outsideRth", outside_rth)])
            .query(&[("startTime", start_time_str)]);
        let response = request.send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }
}
