// https://www.interactivebrokers.com/api/doc.html#tag/Market-Data

use reqwest_middleware::Error;

use crate::{
    client::IBClientPortal,
    model::market_data::{
        GetMarketDataHistoryBetaRequest, GetMarketDataHistoryBetaResponse,
        GetMarketDataHistoryRequest, GetMarketDataRequest, GetMarketDataResponse,
        GetMarketDataSnapshotRequest, GetMarketDataSnapshotResponse, MarketDataHistory,
        UnsubscribeAllMarketDataResponse, UnsubscribeMarketDataRequest,
        UnsubscribeMarketDataResponse,
    },
};

impl IBClientPortal {
    /// Get Market Data for the given conid(s). The endpoint will return by default bid, ask, last, change, change pct, close, listing exchange. See response fields for a list of available fields that can be request via fields argument. The endpoint /iserver/accounts must be called prior to /iserver/marketdata/snapshot. For derivative contracts the endpoint /iserver/secdef/search must be called first. First /snapshot endpoint call for given conid will initiate the market data request. To receive all available fields the /snapshot endpoint will need to be called several times. To receive streaming market data the endpoint /ws can be used. Refer to Streaming WebSocket Data for details.
    pub async fn get_market_data(
        &self,
        request: GetMarketDataRequest,
    ) -> Result<GetMarketDataResponse, Error> {
        let path = "/iserver/marketdata/snapshot";
        let conids_query = (
            "conids",
            request
                .conid_list
                .into_iter()
                .map(|conid| conid.to_string())
                .collect::<Vec<String>>()
                .join(",")
                .to_string(),
        );
        let fields_query = (
            "fields",
            request
                .fields
                .into_iter()
                .flat_map(|fields| fields.into_iter().map(|field| field.to_string()))
                .collect::<Vec<String>>()
                .join(",")
                .to_string(),
        );
        let mut query = vec![conids_query, fields_query];
        if let Some(since_query) = request.since.map(|since| ("since", since.to_string())) {
            query.push(since_query);
        }
        let response = self
            .client
            .get(self.get_url(path))
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .query(&query)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// Get historical market Data for given conid, length of data is controlled by 'period' and 'bar'. Formatted as: min=minute, h=hour, d=day, w=week, m=month, y=year e.g. period =1y with bar =1w returns 52 data points (Max of 1000 data points supported). Note: There's a limit of 5 concurrent requests. Excessive requests will return a 'Too many requests' status 429 response.
    pub async fn get_market_data_history(
        &self,
        request: GetMarketDataHistoryRequest,
    ) -> Result<MarketDataHistory, Error> {
        let path = "/iserver/marketdata/history";
        let start_time_str = request.start_time.unwrap_or("".to_owned());

        let response = self
            .client
            .get(self.get_url(path))
            .query(&[("conid", request.conid)])
            .query(&[("period", request.period)])
            .query(&[("bar", request.bar)])
            .query(&[("exchange", request.exchange.unwrap_or("".to_owned()))])
            .query(&[("outsideRth", request.outside_regular_trading_hours)])
            .query(&[("startTime", start_time_str)])
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// Cancel all market data request(s). To cancel market data for given conid, see /iserver/marketdata/{conid}/unsubscribe.
    pub async fn unsubscribe_all_market_data(
        &self,
    ) -> Result<UnsubscribeAllMarketDataResponse, Error> {
        let path = "/iserver/marketdata/unsubscribeall";
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// Cancel market data for given conid. To cancel all market data request(s), see /iserver/marketdata/unsubscribeall.
    pub async fn unsubscribe_market_data(
        &self,
        request: UnsubscribeMarketDataRequest,
    ) -> Result<UnsubscribeMarketDataResponse, Error> {
        let path = format!("/iserver/marketdata/{}/unsubscribe", request.conid);
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// Using a direct connection to the market data farm, will provide a list of historical market data for given conid.
    pub async fn get_market_data_history_beta(
        &self,
        request: GetMarketDataHistoryBetaRequest,
    ) -> Result<GetMarketDataHistoryBetaResponse, Error> {
        let path = "/hmds/history";
        let mut query = vec![
            ("conid", request.conid.to_string()),
            ("period", request.period),
        ];
        if let Some(bar) = request.bar {
            query.push(("bar", bar));
        }
        if let Some(outside_regular_trading_hours) = request.outside_regular_trading_hours {
            query.push(("outsideRth", outside_regular_trading_hours.to_string()));
        }
        let response = self
            .client
            .get(self.get_url(&path))
            .query(&query)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// Get a snapshot of Market Data for the given conid(s).See response for a list of available fields that can be requested from the fields argument. Must be connected to a brokerage session before can query snapshot data. First /snapshot endpoint call for given conid(s) will initiate the market data request, make an additional request to receive field values back. To receive all available fields the /snapshot endpoint will need to be called several times. To receive streaming market data the endpoint /ws can be used. Refer to Streaming WebSocket Data for details.
    pub async fn get_market_data_snapshot_beta(
        &self,
        request: GetMarketDataSnapshotRequest,
    ) -> Result<GetMarketDataSnapshotResponse, Error> {
        let path = "/md/snapshot";
        let query: Vec<(&str, String)> = request
            .conid_list
            .into_iter()
            .map(|conid| ("conids", conid.to_string()))
            .chain(
                request
                    .field_list
                    .into_iter()
                    .map(|field| ("fields", field.to_string())),
            )
            .collect();

        let response = self
            .client
            .get(self.get_url(&path))
            .query(&query)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }
}
