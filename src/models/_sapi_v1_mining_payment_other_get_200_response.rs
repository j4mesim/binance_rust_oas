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
pub struct SapiV1MiningPaymentOtherGet200Response {
    #[serde(rename = "code")]
    pub code: i64,
    #[serde(rename = "msg")]
    pub msg: String,
    #[serde(rename = "data")]
    pub data: Box<crate::models::SapiV1MiningPaymentOtherGet200ResponseData>,
}

impl SapiV1MiningPaymentOtherGet200Response {
    pub fn new(code: i64, msg: String, data: crate::models::SapiV1MiningPaymentOtherGet200ResponseData) -> SapiV1MiningPaymentOtherGet200Response {
        SapiV1MiningPaymentOtherGet200Response {
            code,
            msg,
            data: Box::new(data),
        }
    }
}


