// https://www.interactivebrokers.com/api/doc.html#tag/Market-Data

use reqwest::Error;
use time::macros::format_description;

use crate::{
    client::IBClientPortal,
    model::market_data::{
        GetMarketDataHistoryRequest, MarketDataHistory, MarketDataRequest, MarketDataResponse,
    },
};

impl IBClientPortal {
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
            .query(&query)
            .send()
            .await?;

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

    // todo
    // (beta) /hmds/history
    // (beta) /md/snapshot
    // /iserver/marketdata/unsubscribeall
    // /iserver/marketdata/unsubscribeall
}
