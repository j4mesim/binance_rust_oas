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
pub struct SapiV1MiningPaymentListGet200Response {
    #[serde(rename = "code")]
    pub code: i64,
    #[serde(rename = "msg")]
    pub msg: String,
    #[serde(rename = "data")]
    pub data: Box<crate::models::SapiV1MiningPaymentListGet200ResponseData>,
}

impl SapiV1MiningPaymentListGet200Response {
    pub fn new(code: i64, msg: String, data: crate::models::SapiV1MiningPaymentListGet200ResponseData) -> SapiV1MiningPaymentListGet200Response {
        SapiV1MiningPaymentListGet200Response {
            code,
            msg,
            data: Box::new(data),
        }
    }
}


