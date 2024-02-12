// https://www.interactivebrokers.com/api/doc.html#tag/Order

use reqwest::Error;

use crate::{
    client::IBClientPortal,
    model::order::{
        CancelOrderRequest, CancelOrderResponse, GetLiveOrderResponse, GetOrderStatusRequest,
        ModifyOrderRequest, ModifyOrderResponse, PlaceOrderForFinancialAdvisorsRequest,
        PlaceOrderForFinancialAdvisorsResponse, PlaceOrderReplyRequest, PlaceOrderReplyResponse,
        PlaceOrderRequest, PlaceOrderResponse, PlaceOrdersRequest, PlaceOrdersResponse,
        PreviewOrderRequest, PreviewOrderResponse,
    },
};

impl IBClientPortal {
    /// This endpoint is going to be deprecated, you can use /iserver/account/{accountId}/orders, just pass one order in the array, the order structure will be same. Please note here, sometimes this endpoint alone can't make sure you submit the order successfully, you could receive some questions in the response, you have to to answer them in order to submit the order successfully. You can use "/iserver/reply/{replyid}" endpoint to answer questions
    #[deprecated]
    pub async fn place_order(
        &self,
        request: PlaceOrderRequest,
    ) -> Result<PlaceOrderResponse, Error> {
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

    /// The endpoint is meant to be used in polling mode, e.g. requesting every x seconds. The response will contain two objects, one is notification, the other is orders. Orders is the list of live orders (cancelled, filled, submitted). Notifications contains information about execute orders as they happen, see status field. To receive streaming live orders the endpoint /ws can be used. Refer to Streaming WebSocket Data for details.
    pub async fn get_live_orders(&self) -> Result<GetLiveOrderResponse, Error> {
        let path = "/iserver/account/orders";
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    pub async fn get_order_status(
        &self,
        request: GetOrderStatusRequest,
    ) -> Result<GetLiveOrderResponse, Error> {
        let path = format!("/iserver/account/order/status/{}", request.order_id);
        let response = self.client.get(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// This endpoint allows you to preview order without actually submitting the order and you can get commission information in the response. Also supports bracket orders.
    pub async fn preview_order(
        &self,
        request: PreviewOrderRequest,
    ) -> Result<PreviewOrderResponse, Error> {
        let path = format!("/iserver/account/{}/orders/whatif", request.account_id);
        let response = self
            .client
            .post(self.get_url(&path))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Cancels an open order. Must call /iserver/accounts endpoint prior to cancelling an order. Use /iservers/account/orders endpoint to review open-order(s) and get latest order status.
    pub async fn cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> Result<CancelOrderResponse, Error> {
        let path = format!(
            "/iserver/account/{}/order/{}",
            request.account_id, request.order_id
        );
        let response = self.client.delete(self.get_url(&path)).send().await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// When connected to an IServer Brokerage Session, this endpoint will allow you to submit orders. CP WEB API supports various advanced orderTypes, for additional details and examples refer to IBKR Quant Blog.
    ///
    /// Bracket - Attach additional opposite-side order(s) by using a single cOID sent with the parent and set the same value for parentId in each child order(s).
    /// Cash Quantity - Send orders using monetary value by specifying cashQty instead of quantity, e.g. cashQty: 200. The endpoint /iserver/contract/rules returns list of valid orderTypes in cqtTypes.
    /// Currency Conversion - Convert cash from one currency to another by including isCcyConv = true. To specify the cash quantity use fxQTY instead of quantity, e.g. fxQTY: 100.
    /// Fractional - Contracts that support fractional shares can be traded by specifying quantity as a float, e.g. quantity: 0.001. The endpoint /iserver/contract/rules returns a list of valid orderTypes in fraqTypes.
    /// IB Algos - Attached user-defined settings to your trades by using any of IBKR's Algo Orders. Use the endpoint /iserver/contract/{conid}/algos to identify the available strategies for a contract.
    /// One-Cancels-All (OCA) - Group multiple unrelated orders by passing order request info in an array and including isSingleGroup = true for each order. All orders will be assigned the same oca_group_id.
    pub async fn place_orders(
        &self,
        request: PlaceOrdersRequest,
    ) -> Result<PlaceOrdersResponse, Error> {
        let path = format!("/iserver/account/{}/orders", request.account_id);
        let response = self
            .client
            .post(self.get_url(&path))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Financial Advisors can use this endpoint to place an order for a specified group. To place an order for a specified account use the endpoint /iserver/account/{accountId}/order. More information about groups can be found in the TWS Users' Guide:.
    pub async fn place_order_for_financial_advisors(
        &self,
        request: PlaceOrderForFinancialAdvisorsRequest,
    ) -> Result<PlaceOrderForFinancialAdvisorsResponse, Error> {
        let path = format!(
            "/iserver/account/orders/{}",
            request.financial_advisors_group
        );
        let response = self
            .client
            .post(self.get_url(&path))
            .json(&request.order)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Reply to questions when placing orders and submit orders
    pub async fn place_order_reply(
        &self,
        request: PlaceOrderReplyRequest,
    ) -> Result<PlaceOrderReplyResponse, Error> {
        let path = format!("/iserver/reply/{}", request.reply_id);
        let response = self
            .client
            .post(self.get_url(&path))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }

    /// Modifies an open order. Must call /iserver/accounts endpoint prior to modifying an order. Use /iservers/account/orders endpoint to review open-order(s).
    pub async fn modify_order(
        &self,
        request: ModifyOrderRequest,
    ) -> Result<ModifyOrderResponse, Error> {
        let path = format!(
            "/iserver/account/{}/order/{}",
            request.account_id_or_financial_advisors_group, request.order_id
        );
        let response = self
            .client
            .post(self.get_url(&path))
            .json(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;
        response.json().await
    }
}
