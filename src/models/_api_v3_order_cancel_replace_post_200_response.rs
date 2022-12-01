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
pub struct ApiV3OrderCancelReplacePost200Response {
    #[serde(rename = "cancelResult")]
    pub cancel_result: String,
    #[serde(rename = "newOrderResult")]
    pub new_order_result: String,
    #[serde(rename = "cancelResponse")]
    pub cancel_response: Box<crate::models::ApiV3OrderCancelReplacePost200ResponseCancelResponse>,
    #[serde(rename = "newOrderResponse")]
    pub new_order_response: Box<crate::models::ApiV3OrderCancelReplacePost200ResponseNewOrderResponse>,
}

impl ApiV3OrderCancelReplacePost200Response {
    pub fn new(cancel_result: String, new_order_result: String, cancel_response: crate::models::ApiV3OrderCancelReplacePost200ResponseCancelResponse, new_order_response: crate::models::ApiV3OrderCancelReplacePost200ResponseNewOrderResponse) -> ApiV3OrderCancelReplacePost200Response {
        ApiV3OrderCancelReplacePost200Response {
            cancel_result,
            new_order_result,
            cancel_response: Box::new(cancel_response),
            new_order_response: Box::new(new_order_response),
        }
    }
}


