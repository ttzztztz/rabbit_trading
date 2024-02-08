// https://www.interactivebrokers.com/api/doc.html#tag/Contract

use reqwest::Error;
use serde_json::Value;

use crate::{
    client::IBClientPortal,
    model::{
        contract::{
            ContractDetail, GetContractDetailRequest, GetSecurityDefinitionByContractIdRequest,
            GetStocksBySymbolRequest, SearchForSecurityRequest, SecurityDefinitions,
            StockContracts,
        },
        futures::{FuturesContracts, GetFuturesBySymbolRequest},
        options::GetOptionsRequest,
    },
};

impl IBClientPortal {
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

    pub async fn get_contract_detail(
        &self,
        request: GetContractDetailRequest,
    ) -> Result<ContractDetail, Error> {
        let path = format!("/iserver/contract/{}/info", request.conid);
        let response = self.client.get(self.get_url(&path)).body("").send().await?;

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

    // todo
    // /trsrv/secdef/schedule
    // /iserver/secdef/strikes
    // /iserver/contract/{conid}/algos
    // /iserver/contract/rules
    // /iserver/contract/{conid}/info-and-rules
}
