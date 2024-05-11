use anyhow::{Context, Error};
use async_trait::async_trait;
use ibkr_client_portal::{
    client::IBClientPortal,
    model::{
        account::{GetAccountSummaryRequest, GetAccountSummaryResponse},
        order::{
            CancelOrderRequest as IBCancelOrderRequest, GetOrderStatusRequest, ModifyOrderRequest,
            OrderRequest, OrderStatus, PlaceOrdersRequest,
        },
        portfolio::GetPortfolioPositionsRequest,
    },
    utils::reply::handle_reply_order_requests,
};
use rust_decimal_macros::dec;

use super::broker::InteractiveBrokersBroker;
use crate::{
    broker::common::transaction::TransactionTrait,
    model::{
        common::types::ConfigMap,
        trading::{
            balance::{BalanceDetail, BalanceHashMap},
            currency::Currency,
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

impl InteractiveBrokersTransaction {
    fn get_account_summary_response_to_balance_hashmap(
        account_summary: GetAccountSummaryResponse,
    ) -> BalanceHashMap {
        let total_cash = account_summary
            .full_available_funds
            .and_then(|funds| funds.amount)
            .unwrap_or(dec!(0.0));
        let init_margin = account_summary
            .init_margin_req
            .and_then(|funds| funds.amount)
            .unwrap_or(dec!(0.0));
        let maintenance_margin = account_summary
            .maintenance_marginreq
            .and_then(|funds| funds.amount)
            .unwrap_or(dec!(0.0));
        let net_assets = account_summary
            .net_liquidation
            .and_then(|funds| funds.amount)
            .unwrap_or(dec!(0.0));

        let balance_detail = BalanceDetail {
            total_cash,
            net_assets,
            margin_call: dec!(0.0), // TODO
            init_margin,
            maintenance_margin,
        };
        BalanceHashMap::from([(Currency::USD, balance_detail)]) // TODO: support other currencies
    }

    fn get_account_summary_response_to_buying_power(
        account_summary: GetAccountSummaryResponse,
    ) -> BuyingPower {
        let cash_max_quantity = account_summary
            .full_available_funds
            .and_then(|funds| funds.amount)
            .unwrap_or(dec!(0.0));
        let margin_max_quantity = account_summary
            .buying_power
            .and_then(|funds| funds.amount)
            .unwrap_or(dec!(0.0));

        BuyingPower {
            cash_max_quantity,
            margin_max_quantity,
        }
    }

    fn ib_position_to_core_position(
        position: ibkr_client_portal::model::portfolio::Position,
    ) -> crate::model::trading::position::Position {
        crate::model::trading::position::Position {
            symbol: todo!(),
            currency: todo!(),
            cost_price: todo!(),
            total_quantity: todo!(),
            available_quantity: todo!(),
        }
    }

    fn ib_order_status_to_core_order_detail(order_status: OrderStatus) -> OrderDetail {
        OrderDetail {
            order_id: todo!(),
            symbol: todo!(),
            currency: todo!(),
            quantity: todo!(),
            executed_quantity: todo!(),
            price: todo!(),
            executed_price: todo!(),
            direction: todo!(),
            regular_trading_time: todo!(),
            expire: todo!(),
            created_timestamp: todo!(),
            updated_timestamp: todo!(),
            triggered_timestamp: todo!(),
        }
    }

    fn core_edit_order_request_to_ib_modify_order_request(
        request: EditOrderRequest,
    ) -> ModifyOrderRequest {
        ModifyOrderRequest {
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
        }
    }

    fn core_submit_order_request_to_ib_order(request: SubmitOrderRequest) -> OrderRequest {
        OrderRequest {
            account_id: todo!(),
            conid: todo!(),
            conidex: todo!(),
            sec_type: todo!(),
            c_oid: todo!(),
            parent_id: todo!(),
            order_type: todo!(),
            listing_exchange: todo!(),
            is_single_group: todo!(),
            outside_regular_trading_hours: todo!(),
            price: todo!(),
            aux_price: todo!(),
            side: todo!(),
            ticker: todo!(),
            time_in_force: todo!(),
            trailing_amount: todo!(),
            trailing_type: todo!(),
            referrer: todo!(),
            quantity: todo!(),
            cash_quantity: todo!(),
            fx_quantity: todo!(),
            use_adaptive: todo!(),
            is_currency_conv: todo!(),
            allocation_method: todo!(),
            strategy: todo!(),
            strategy_parameters: todo!(),
            originator: todo!(),
        }
    }
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
        let account_summary = self
            .client_portal
            .get_account_summary(GetAccountSummaryRequest {
                account_id: account_id.clone(),
            })
            .await
            .with_context(|| format!("Error when account_balance, account_id={}", account_id))?;

        Result::Ok(Self::get_account_summary_response_to_balance_hashmap(
            account_summary,
        ))
    }

    async fn positions(&self) -> Result<PositionList, Error> {
        let account_id = InteractiveBrokersBroker::get_account_id(&self.config_map);
        let positions = self
            .client_portal
            .get_portfolio_positions(GetPortfolioPositionsRequest { page: 1 }) // TODO: support pagination
            .await
            .with_context(|| format!("Error when retrieve position data {}", account_id))?;

        Result::Ok(
            positions
                .into_iter()
                .map(Self::ib_position_to_core_position)
                .collect(),
        )
    }

    async fn estimate_max_buying_power(
        &self,
        request: EstimateMaxBuyingPowerRequest,
    ) -> Result<BuyingPower, Error> {
        let account_id = InteractiveBrokersBroker::get_account_id(&self.config_map);
        let account_summary = self
            .client_portal
            .get_account_summary(GetAccountSummaryRequest { account_id })
            .await
            .with_context(|| format!("Error when estimate_max_buying_power {:?}", request))?;

        Result::Ok(Self::get_account_summary_response_to_buying_power(
            account_summary,
        ))
    }

    async fn order_detail(&self, request: OrderDetailRequest) -> Result<OrderDetail, Error> {
        let order_detail = self
            .client_portal
            .get_order_status(GetOrderStatusRequest {
                order_id: request.order_id.clone(),
            })
            .await
            .with_context(|| format!("Error when order_detail {:?}", request))?;

        Result::Ok(Self::ib_order_status_to_core_order_detail(order_detail))
    }

    async fn submit_order(
        &mut self,
        request: SubmitOrderRequest,
    ) -> Result<SubmitOrderResponse, Error> {
        let account_id = InteractiveBrokersBroker::get_account_id(&self.config_map);
        let max_retry_count =
            InteractiveBrokersBroker::get_place_order_max_reply_count(&self.config_map);
        let place_order_response = self
            .client_portal
            .place_orders(PlaceOrdersRequest {
                account_id,
                orders: vec![Self::core_submit_order_request_to_ib_order(request)], // TODO
            })
            .await?;
        let order_id = handle_reply_order_requests(
            self.client_portal.clone(),
            place_order_response,
            max_retry_count,
        )
        .await?;
        Result::Ok(SubmitOrderResponse { order_id })
    }

    async fn edit_order(&mut self, request: EditOrderRequest) -> Result<EditOrderResponse, Error> {
        let max_retry_count =
            InteractiveBrokersBroker::get_place_order_max_reply_count(&self.config_map);

        let place_order_response = self
            .client_portal
            .modify_order(Self::core_edit_order_request_to_ib_modify_order_request(
                request,
            ))
            .await?;
        handle_reply_order_requests(
            self.client_portal.clone(),
            place_order_response,
            max_retry_count,
        )
        .await
        .map(|_| EditOrderResponse {})
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
