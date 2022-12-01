/*
 * Binance Public Spot API
 *
 * OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MarginOrderResponseFull {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    #[serde(rename = "transactTime")]
    pub transact_time: i64,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "origQty")]
    pub orig_qty: String,
    #[serde(rename = "executedQty")]
    pub executed_qty: String,
    #[serde(rename = "cummulativeQuoteQty")]
    pub cummulative_quote_qty: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "side")]
    pub side: String,
    /// will not return if no margin trade happens
    #[serde(rename = "marginBuyBorrowAmount")]
    pub margin_buy_borrow_amount: f64,
    /// will not return if no margin trade happens
    #[serde(rename = "marginBuyBorrowAsset")]
    pub margin_buy_borrow_asset: String,
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    #[serde(rename = "fills")]
    pub fills: Vec<crate::models::OrderResponseFullFillsInner>,
}

impl MarginOrderResponseFull {
    pub fn new(symbol: String, order_id: i64, client_order_id: String, transact_time: i64, price: String, orig_qty: String, executed_qty: String, cummulative_quote_qty: String, status: String, time_in_force: String, r#type: String, side: String, margin_buy_borrow_amount: f64, margin_buy_borrow_asset: String, is_isolated: bool, fills: Vec<crate::models::OrderResponseFullFillsInner>) -> MarginOrderResponseFull {
        MarginOrderResponseFull {
            symbol,
            order_id,
            client_order_id,
            transact_time,
            price,
            orig_qty,
            executed_qty,
            cummulative_quote_qty,
            status,
            time_in_force,
            r#type,
            side,
            margin_buy_borrow_amount,
            margin_buy_borrow_asset,
            is_isolated,
            fills,
        }
    }
}


