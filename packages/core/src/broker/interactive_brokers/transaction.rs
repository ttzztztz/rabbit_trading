use async_trait::async_trait;
use ibkr_client_portal::client::IBClientPortal;

use crate::{
    broker::common::transaction::TransactionTrait,
    model::{
        common::{error::Error, types::ConfigMap},
        trading::{
            balance::BalanceHashMap,
            position::PositionList,
            transaction::{
                BuyingPower, CancelOrderRequest, CancelOrderResponse, EditOrderRequest,
                EditOrderResponse, EstimateMaxBuyingPowerRequest, OrderDetail, OrderDetailRequest,
                SubmitOrderRequest, SubmitOrderResponse,
            },
        },
    },
};

pub struct InteractiveBrokersTransaction {
    client_portal: IBClientPortal,
}

#[async_trait]
impl TransactionTrait for InteractiveBrokersTransaction {
    async fn new(config_map: ConfigMap) -> Self {
        todo!()
    }

    async fn account_balance(&self) -> Result<BalanceHashMap, Error> {
        todo!()
    }

    async fn positions(&self) -> Result<PositionList, Error> {
        todo!()
    }

    async fn estimate_max_buying_power(
        &self,
        request: EstimateMaxBuyingPowerRequest,
    ) -> Result<BuyingPower, Error> {
        todo!()
    }

    async fn order_detail(&self, request: OrderDetailRequest) -> Result<OrderDetail, Error> {
        todo!()
    }

    async fn submit_order(
        &self,
        request: SubmitOrderRequest,
    ) -> Result<SubmitOrderResponse, Error> {
        todo!()
    }

    async fn edit_order(&self, request: EditOrderRequest) -> Result<EditOrderResponse, Error> {
        todo!()
    }

    async fn cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> Result<CancelOrderResponse, Error> {
        todo!()
    }
}
