use anyhow::{anyhow, Error};

use crate::{
    client::IBClientPortal,
    model::order::{PlaceOrModifyOrderResult, PlaceOrderReplyRequest, PlaceOrdersResponse},
};

pub async fn handle_reply_order_requests(
    client: IBClientPortal,
    place_order_response: PlaceOrdersResponse,
    max_retry_count: i32,
) -> Result<String, Error> {
    if let PlaceOrdersResponse::Ok(mut arr) = place_order_response {
        let mut result_optional: Option<String> = Option::None;
        for _ in 0..max_retry_count {
            match arr[0].clone() {
                PlaceOrModifyOrderResult::Success(detail) => {
                    result_optional = Option::Some(detail.order_id);
                    break;
                }

                PlaceOrModifyOrderResult::Question(question) => {
                    let place_order_reply_response = client
                        .place_order_reply(PlaceOrderReplyRequest {
                            reply_id: question.id.clone(),
                            confirmed: true,
                        })
                        .await?;

                    if let PlaceOrdersResponse::Ok(next_arr) = place_order_reply_response {
                        if let PlaceOrModifyOrderResult::Success(detail) = next_arr[0].clone() {
                            result_optional = Option::Some(detail.order_id);
                            break;
                        } else {
                            arr = next_arr;
                        }
                    } else {
                        return Result::Err(anyhow!(
                            "Place order failed when replying {:?}",
                            question
                        ));
                    }
                }
            }
        }

        if let Some(result_order_id) = result_optional {
            return Result::Ok(result_order_id);
        } else {
            return Result::Err(anyhow!("Place order failed after 3 times retry"));
        }
    } else {
        return Result::Err(anyhow!("Place order failed {:?}", place_order_response));
    }
}
