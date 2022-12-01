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
pub struct ApiV3OrderCancelReplacePost200ResponseNewOrderResponse {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
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
    #[serde(rename = "fills")]
    pub fills: Vec<String>,
}

impl ApiV3OrderCancelReplacePost200ResponseNewOrderResponse {
    pub fn new(symbol: String, order_id: i64, order_list_id: i64, client_order_id: String, transact_time: i64, price: String, orig_qty: String, executed_qty: String, cummulative_quote_qty: String, status: String, time_in_force: String, r#type: String, side: String, fills: Vec<String>) -> ApiV3OrderCancelReplacePost200ResponseNewOrderResponse {
        ApiV3OrderCancelReplacePost200ResponseNewOrderResponse {
            symbol,
            order_id,
            order_list_id,
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
            fills,
        }
    }
}


