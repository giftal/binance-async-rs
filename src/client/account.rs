use crate::{
    client::Binance,
    model::{
        AccountInformation, AssetDetail, DepositAddressData, DepositHistory, Order, OrderCanceled,
        TradeHistory,
    },
};
use chrono::prelude::*;
use failure::Fallible;
use futures::prelude::*;
use serde_json::json;
// use std::collections::HashMap;

// const ORDER_TYPE_LIMIT: &str = "LIMIT";
// const ORDER_TYPE_MARKET: &str = "MARKET";
// const ORDER_SIDE_BUY: &str = "BUY";
// const ORDER_SIDE_SELL: &str = "SELL";
// const TIME_IN_FORCE_GTC: &str = "GTC";

const FAPI_V1_ORDER: &str = "/fapi/v1/order";
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    Buy,
    Sell,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionSide {
    Short,
    Long,
    Both,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TrailingStopMarket,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Fok,
    Gtx,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NewOrderRespType {
    Ack,
    Result,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    pub symbol: String,
    pub side: Side,
    pub position_side: Option<PositionSide>,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: Option<String>,
    pub reduce_only: Option<bool>,
    pub price: Option<String>,
    pub new_client_order_id: Option<String>,
    pub stop_price: Option<String>,
    pub close_position: Option<bool>,
    pub activation_price: Option<String>,
    pub callback_rate: Option<String>,
    pub working_type: Option<WorkingType>,
    pub new_order_resp_type: Option<NewOrderRespType>,
    pub recv_window: Option<u64>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub order_id: i64,
    pub symbol: String,
    pub status: OrderStatus,
    pub client_order_id: String,
    pub price: String,
    pub avg_price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cum_qty: String,
    pub cum_quote: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub reduce_only: bool,
    pub close_position: bool,
    pub side: String,
    pub position_side: String,
    pub stop_price: String,
    pub working_type: String,
    pub price_protect: String,
    pub orig_type: String,
    pub update_time: i64,
}

impl Binance {
    // Account Information
    pub fn get_account(&self) -> Fallible<impl Future<Output = Fallible<AccountInformation>>> {
        let account_info = self
            .transport
            .signed_get::<_, ()>("/fapi/v1/account", None)?;
        Ok(account_info)
    }

    // // Balance for ONE Asset
    // pub fn get_balance(&self, asset: &str) -> Fallible<impl Future<Output = Fallible<Balance>>> {
    //     let asset = asset.to_string();
    //     let search = move |account: AccountInformation| {
    //         let balance = account
    //             .balances
    //             .into_iter()
    //             .find(|balance| balance.asset == asset);
    //         future::ready(balance.ok_or_else(|| Error::AssetsNotFound.into()))
    //     };

    //     let balance = self.get_account()?.and_then(search);
    //     Ok(balance)
    // }

    // Current open orders for ONE symbol
    pub fn get_open_orders(
        &self,
        symbol: &str,
    ) -> Fallible<impl Future<Output = Fallible<Vec<Order>>>> {
        let params = json! {{"symbol": symbol}};
        let orders = self
            .transport
            .signed_get("/fapi/v1/openOrders", Some(params))?;
        Ok(orders)
    }

    // All current open orders
    pub fn get_all_open_orders(&self) -> Fallible<impl Future<Output = Fallible<Vec<Order>>>> {
        let orders = self
            .transport
            .signed_get::<_, ()>("/fapi/v1/openOrders", None)?;
        Ok(orders)
    }

    // Check an order's status
    pub fn order_status(
        &self,
        symbol: &str,
        order_id: u64,
    ) -> Fallible<impl Future<Output = Fallible<Order>>> {
        let params = json! {{"symbol": symbol, "orderId": order_id}};

        let order = self.transport.signed_get(FAPI_V1_ORDER, Some(params))?;
        Ok(order)
    }

    pub fn place_order(
        &self,
        order_request: OrderRequest,
    ) -> Fallible<impl Future<Output = Fallible<OrderResponse>>> {
        let transaction = self
            .transport
            .signed_post(FAPI_V1_ORDER, Some(order_request))?;

        Ok(transaction)
    }

    // Check an order's status
    pub fn cancel_order(
        &self,
        symbol: &str,
        order_id: u64,
    ) -> Fallible<impl Future<Output = Fallible<OrderCanceled>>> {
        let params = json! {{"symbol":symbol, "orderId":order_id}};
        let order_canceled = self.transport.signed_delete(FAPI_V1_ORDER, Some(params))?;
        Ok(order_canceled)
    }

    // Trade history
    pub fn trade_history(
        &self,
        symbol: &str,
    ) -> Fallible<impl Future<Output = Fallible<Vec<TradeHistory>>>> {
        let params = json! {{"symbol":symbol}};
        let trade_history = self
            .transport
            .signed_get("/fapi/v1/myTrades", Some(params))?;

        Ok(trade_history)
    }

    pub fn get_deposit_address(
        &self,
        symbol: &str,
    ) -> Fallible<impl Future<Output = Fallible<DepositAddressData>>> {
        let params = json! {{"asset":symbol}};
        let deposit_address = self
            .transport
            .signed_get("/wapi/v3/depositAddress.html", Some(params))?;

        Ok(deposit_address)
    }

    pub fn get_deposit_history(
        &self,
        symbol: Option<&str>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Fallible<impl Future<Output = Fallible<DepositHistory>>> {
        let params = json! {{"asset":symbol, "startTime":start_time.map(|t| t.timestamp_millis()), "endTime":end_time.map(|t| t.timestamp_millis())}};
        let deposit_history = self
            .transport
            .signed_get("/wapi/v3/depositHistory.html", Some(params))?;

        Ok(deposit_history)
    }

    pub fn asset_detail(&self) -> Fallible<impl Future<Output = Fallible<AssetDetail>>> {
        let asset_detail = self
            .transport
            .signed_get::<_, ()>("/wapi/v3/assetDetail.html", None)?;

        Ok(asset_detail)
    }
}
