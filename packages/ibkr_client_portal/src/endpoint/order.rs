// https://www.interactivebrokers.com/api/doc.html#tag/Order

use reqwest::Error;
use serde_json::Value;

use crate::{client::IBClientPortal, model::order::PlaceOrderRequest};

impl IBClientPortal {
    // deprecated
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
