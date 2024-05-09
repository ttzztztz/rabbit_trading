use anyhow::{Context, Error};
use async_trait::async_trait;
use ibkr_client_portal::{
    client::IBClientPortal,
    model::{
        account::GetAccountSummaryRequest,
        order::{
            CancelOrderRequest as IBCancelOrderRequest, GetOrderStatusRequest, ModifyOrderRequest,
            PlaceOrdersRequest,
        },
        portfolio::GetPortfolioPositionsRequest,
    },
};

use super::broker::InteractiveBrokersBroker;
use crate::{
    broker::common::transaction::TransactionTrait,
    model::{
        common::types::ConfigMap,
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
    config_map: ConfigMap,
    client_portal: IBClientPortal,
}

#[async_trait]
impl TransactionTrait for InteractiveBrokersTransaction {
    fn new(config_map: ConfigMap) -> Self {
        let client_portal = InteractiveBrokersBroker::create_ib_client_portal(config_map.clone());
        InteractiveBrokersTransaction {
            config_map,
            client_portal,
        }
    }

    async fn account_balance(&self) -> Result<BalanceHashMap, Error> {
        let account_id = InteractiveBrokersBroker::get_account_id(&self.config_map);
        let account_ledger = self.client_portal.get_account_ledger().await;
        todo!()
    }

    async fn positions(&self) -> Result<PositionList, Error> {
        let account_id = InteractiveBrokersBroker::get_account_id(&self.config_map);
        let positions = self
            .client_portal
            .get_portfolio_positions(GetPortfolioPositionsRequest { page: 1 })
            .await;
        todo!()
    }

    async fn estimate_max_buying_power(
        &self,
        _request: EstimateMaxBuyingPowerRequest,
    ) -> Result<BuyingPower, Error> {
        let account_id = InteractiveBrokersBroker::get_account_id(&self.config_map);
        let account_summary = self
            .client_portal
            .get_account_summary(GetAccountSummaryRequest { account_id })
            .await;
        todo!()
    }

    async fn order_detail(&self, request: OrderDetailRequest) -> Result<OrderDetail, Error> {
        let account_id = InteractiveBrokersBroker::get_account_id(&self.config_map);
        let order_detail = self
            .client_portal
            .get_order_status(GetOrderStatusRequest {
                order_id: request.order_id,
            })
            .await;
        todo!()
    }

    async fn submit_order(
        &mut self,
        _request: SubmitOrderRequest,
    ) -> Result<SubmitOrderResponse, Error> {
        let account_id = InteractiveBrokersBroker::get_account_id(&self.config_map);
        let place_order_result = self
            .client_portal
            .place_orders(PlaceOrdersRequest {
                account_id,
                orders: vec![],
            })
            .await;
        todo!()
    }

    async fn edit_order(&mut self, _request: EditOrderRequest) -> Result<EditOrderResponse, Error> {
        let account_id = InteractiveBrokersBroker::get_account_id(&self.config_map);
        let place_order_result = self
            .client_portal
            .modify_order(ModifyOrderRequest {
                account_id_or_financial_advisors_group: todo!(),
                order_id: todo!(),
                account_id: todo!(),
                conid: todo!(),
                conidex: todo!(),
                order_type: todo!(),
                outside_regular_trading_hours: todo!(),
                price: todo!(),
                aux_price: todo!(),
                side: todo!(),
                listing_exchange: todo!(),
                ticker: todo!(),
                time_in_force: todo!(),
                quantity: todo!(),
                deactivated: todo!(),
                use_adaptive: todo!(),
            })
            .await;
        todo!()
    }

    async fn cancel_order(
        &mut self,
        request: CancelOrderRequest,
    ) -> Result<CancelOrderResponse, Error> {
        let account_id = InteractiveBrokersBroker::get_account_id(&self.config_map);
        let order_id = request.order_id.clone();
        self.client_portal
            .cancel_order(IBCancelOrderRequest {
                account_id,
                order_id,
            })
            .await
            .map(|_| CancelOrderResponse {})
            .with_context(|| format!("Error when cancelling the order {:?}", request))
    }
}
