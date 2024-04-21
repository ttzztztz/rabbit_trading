// https://www.interactivebrokers.com/api/doc.html#tag/CCP-(Beta)

use reqwest_middleware::Error;

use crate::{
    client::IBClientPortal,
    model::ccp::{
        CompleteCCPSessionRequest, CompleteCCPSessionResponse, DeleteCCPOrderRequest,
        DeleteCCPOrderResponse, GetCCPAccountListResponse, GetCCPOrderStatusRequest,
        GetCCPOrderStatusResponse, GetCCPPositionResponse, GetCCPStatusResponse,
        GetCCPTradesRequest, GetCCPTradesResponse, StartCCPSessionRequest, StartCCPSessionResponse,
        SubmitCCPOrderRequest, SubmitCCPOrderResponse, UpdateCCPOrderRequest,
        UpdateCCPOrderResponse,
    },
};

impl IBClientPortal {
    /// Initiate a brokerage session to CCP. Only one brokerage session type can run at a time. If an existing brokerage session to iServer is running then call the endpoint /logout first. Note at this time only order management is possible from CCP session, market data and scanner endpoints can't be used since they are only available from iServer session. Work is in progress to provide new CCP endpoints for market data and scanners.
    pub async fn start_ccp_session(
        &self,
        request: StartCCPSessionRequest,
    ) -> Result<StartCCPSessionResponse, Error> {
        let path = "/ccp/auth/init";
        let response = self
            .client
            .post(self.get_url(&path))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// Session Token Authentication
    pub async fn complete_ccp_session(
        &self,
        request: CompleteCCPSessionRequest,
    ) -> Result<CompleteCCPSessionResponse, Error> {
        let path = "/ccp/auth/response";
        let response = self
            .client
            .post(self.get_url(&path))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// Provide the current CCP session status. When using the Gateway this endpoint will also initiate a brokerage session to CCP by sending /auth/init and response.
    pub async fn get_ccp_status(&self) -> Result<GetCCPStatusResponse, Error> {
        let path = "/ccp/status";
        let response = self
            .client
            .get(self.get_url(&path))
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// Provides the list of tradeable accounts
    pub async fn get_ccp_account_list(&self) -> Result<GetCCPAccountListResponse, Error> {
        let path = "/ccp/account";
        let response = self
            .client
            .get(self.get_url(&path))
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// List of positions
    pub async fn get_ccp_position(&self) -> Result<GetCCPPositionResponse, Error> {
        let path = "/ccp/positions";
        let response = self
            .client
            .get(self.get_url(&path))
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// Get status for all orders
    pub async fn get_ccp_order_status(
        &self,
        request: GetCCPOrderStatusRequest,
    ) -> Result<GetCCPOrderStatusResponse, Error> {
        let path = "/ccp/orders";
        let response = self
            .client
            .get(self.get_url(&path))
            .query(&[
                ("acct", request.account),
                ("cancelled", request.cancelled.to_string()),
            ])
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// Get a list of Trades, by default, the list is from today midnight to Date.now().
    pub async fn get_ccp_trades(
        &self,
        request: GetCCPTradesRequest,
    ) -> Result<GetCCPTradesResponse, Error> {
        let path = "/ccp/trades";
        let mut query = vec![];
        if let Some(from) = request.from {
            query.push(("from", from));
        }
        if let Some(to) = request.to {
            query.push(("to", to));
        }

        let response = self
            .client
            .get(self.get_url(&path))
            .query(&query)
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// Submits an Order.
    pub async fn submit_ccp_order(
        &self,
        request: SubmitCCPOrderRequest,
    ) -> Result<SubmitCCPOrderResponse, Error> {
        let path = "/ccp/order";
        let mut query = vec![
            ("acct", request.account),
            ("conid", request.conid.to_string()),
            ("ccy", request.contract_currency),
            ("exchange", request.exchange),
            ("qty", request.quantity.to_string()),
        ];
        if let Some(_type) = request._type {
            query.push(("type", _type));
        }
        if let Some(side) = request.side {
            query.push(("side", side));
        }
        if let Some(price) = request.price {
            query.push(("price", price.to_string()));
        }
        if let Some(time_in_force) = request.time_in_force {
            query.push(("tif", time_in_force));
        }

        let response = self
            .client
            .post(self.get_url(&path))
            .query(&query)
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// Sends an Order cancellation request. The status of the order can be queried through /ccp/order. Passing arguments as GET is also supported (requires passing action=delete) (GET is meant for development only)
    pub async fn delete_ccp_order(
        &self,
        request: DeleteCCPOrderRequest,
    ) -> Result<DeleteCCPOrderResponse, Error> {
        let path = "/ccp/order";
        let query = vec![("acct", request.account), ("id", request.id.to_string())];

        let response = self
            .client
            .delete(self.get_url(&path))
            .query(&query)
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }

    /// Updates an Order. Updating an order requires the same arguments as placing an order besides the conid. Note: The status of the order can be queried through GET /ccp/order.
    pub async fn update_ccp_order(
        &self,
        request: UpdateCCPOrderRequest,
    ) -> Result<UpdateCCPOrderResponse, Error> {
        let path = "/ccp/order";
        let mut query = vec![("acct", request.account), ("id", request.id.to_string())];
        if let Some(quantity) = request.quantity {
            query.push(("qty", quantity.to_string()));
        }
        if let Some(price) = request.price {
            query.push(("price", price.to_string()));
        }

        let response = self
            .client
            .put(self.get_url(&path))
            .query(&query)
            .header(reqwest::header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        response.error_for_status_ref()?;
        response
            .json()
            .await
            .map_err(reqwest_middleware::Error::from)
    }
}
