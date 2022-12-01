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
pub struct ApiV3OrderListDelete200ResponseOrdersInner {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
}

impl ApiV3OrderListDelete200ResponseOrdersInner {
    pub fn new(symbol: String, order_id: i64, client_order_id: String) -> ApiV3OrderListDelete200ResponseOrdersInner {
        ApiV3OrderListDelete200ResponseOrdersInner {
            symbol,
            order_id,
            client_order_id,
        }
    }
}


