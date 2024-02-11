// https://www.interactivebrokers.com/api/doc.html#tag/Order

use reqwest::Error;

use crate::{
    client::IBClientPortal,
    model::order::{PlaceOrderRequest, PlaceOrderResponse},
};

impl IBClientPortal {
    /// @deprecated This endpoint is going to be deprecated, you can use /iserver/account/{accountId}/orders, just pass one order in the array, the order structure will be same. Please note here, sometimes this endpoint alone can't make sure you submit the order successfully, you could receive some questions in the response, you have to to answer them in order to submit the order successfully. You can use "/iserver/reply/{replyid}" endpoint to answer questions
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

    // todo
    // /iserver/account/orders
    // /iserver/account/{accountId}/orders
    // /iserver/account/orders/{faGroup}
    // /iserver/reply/{replyid}
    // /iserver/account/{accountId}/orders/whatif
    // /iserver/account/order/status/{orderId}
    // POST /iserver/account/{accountId}/order/{orderId}
    // DELETE /iserver/account/{accountId}/order/{orderId}
}
