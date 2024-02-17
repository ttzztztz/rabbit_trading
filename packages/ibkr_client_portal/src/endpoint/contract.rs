// https://www.interactivebrokers.com/api/doc.html#tag/Contract

use reqwest::Error;

use crate::{
    client::IBClientPortal,
    model::contract::{
        ContractDetail, FuturesContracts, GetContractDetailRequest, GetContractRulesRequest,
        GetContractRulesResponse, GetFuturesBySymbolRequest, GetIBAlgorithmParametersRequest,
        GetIBAlgorithmParametersResponse, GetInfoAndRulesByConIdRequest,
        GetInfoAndRulesByConIdResponse, GetSecurityDefinitionByConIdRequest,
        GetSecurityStrikesRequest, GetSecurityStrikesResponse, GetSecurityTradingScheduleRequest,
        GetSecurityTradingScheduleResponse, GetStocksBySymbolRequest, SearchForSecurityRequest,
        SearchForSecurityResponse, SecurityDefinitions, SecurityDefinitionsRequest,
        SecurityDefinitionsResponse, StockContracts,
    },
};

impl IBClientPortal {
    /// Returns a list of security definitions for the given conids
    pub async fn get_security_definition_by_contract_id(
        &self,
        request: GetSecurityDefinitionByConIdRequest,
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

    /// Returns a list of non-expired future contracts for given symbol(s)
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

    /// Returns an object contains all stock contracts for given symbol(s)
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

    /// Using the Contract Identifier get contract info. You can use this to prefill your order before you submit an order
    pub async fn get_contract_detail(
        &self,
        request: GetContractDetailRequest,
    ) -> Result<ContractDetail, Error> {
        let path = format!("/iserver/contract/{}/info", request.conid);
        let response = self
            .client
            .get(self.get_url(&path))
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Search by underlying symbol or company name. Relays back what derivative contract(s) it has. This endpoint must be called before using /secdef/info. If company name is specified will only receive limited response: conid, companyName, companyHeader and symbol.
    pub async fn search_for_security(
        &self,
        request: SearchForSecurityRequest,
    ) -> Result<SearchForSecurityResponse, Error> {
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

    /// Provides Contract Details of Futures, Options, Warrants, Cash and CFDs based on conid. To get the strike price for Options/Warrants use "/iserver/secdef/strikes" endpoint. Must call /secdef/search for the underlying contract first.
    pub async fn get_contract_details_of_futures_options_warrants_cash_cfds(
        &self,
        request: SecurityDefinitionsRequest,
    ) -> Result<SecurityDefinitionsResponse, Error> {
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
        if let Some(right) = request.right {
            query.push(("right", right.to_string()));
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

    /// Returns the trading schedule up to a month for the requested contract
    pub async fn get_security_trading_schedule(
        &self,
        request: GetSecurityTradingScheduleRequest,
    ) -> Result<GetSecurityTradingScheduleResponse, Error> {
        let path = "/trsrv/secdef/schedule";
        let mut query = vec![
            ("assetClass", request.asset_class),
            ("symbol", request.symbol),
        ];
        if let Some(exchange) = request.exchange {
            query.push(("exchange", exchange));
        }
        if let Some(exchange_filter) = request.exchange_filter {
            query.push(("exchangeFilter", exchange_filter));
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

    /// Query strikes for Options/Warrants. For the conid of the underlying contract, available contract months and exchanges use "/iserver/secdef/search"
    pub async fn get_security_strikes(
        &self,
        request: GetSecurityStrikesRequest,
    ) -> Result<GetSecurityStrikesResponse, Error> {
        let path = "/iserver/secdef/strikes";
        let mut query = vec![
            ("conid", request.conid.to_string()),
            ("sectype", request.sectype),
            ("month", request.month),
        ];
        if let Some(exchange) = request.exchange {
            query.push(("exchange", exchange));
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

    /// Returns both contract info and rules from a single endpoint.
    /// For only contract rules, use the endpoint /iserver/contract/rules.
    /// For only contract info, use the endpoint /iserver/contract/{conid}/info.
    pub async fn get_info_and_rules_by_conid(
        &self,
        request: GetInfoAndRulesByConIdRequest,
    ) -> Result<GetInfoAndRulesByConIdResponse, Error> {
        let path = format!("/iserver/contract/{}/info-and-rules", request.conid);
        let response = self
            .client
            .get(self.get_url(&path))
            .query(&[("isBuy", request.is_buy.to_string().as_str())])
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Returns trading related rules for a specific contract and side. For both contract info and rules use the endpoint /iserver/contract/{conid}/info-and-rules.
    pub async fn get_contract_rules(
        &self,
        request: GetContractRulesRequest,
    ) -> Result<GetContractRulesResponse, Error> {
        let path = "/iserver/contract/rules";
        let response = self
            .client
            .post(self.get_url(path))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Returns supported IB Algos for contract. Must be called a second time to query the list of available parameters.
    pub async fn get_supported_algorithms_by_contract(
        &self,
        request: GetIBAlgorithmParametersRequest,
    ) -> Result<GetIBAlgorithmParametersResponse, Error> {
        let path = format!("/iserver/contract/{}/algos", request.conid);
        let mut query = vec![("conid", request.conid.to_string())];
        if let Some(algos) = request.algos {
            query.push(("algos", algos));
        }
        if let Some(add_description) = request.add_description {
            query.push(("addDescription", add_description));
        }
        if let Some(add_params) = request.add_params {
            query.push(("addParams", add_params));
        }
        let response = self
            .client
            .get(self.get_url(&path))
            .query(&query)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }
}
