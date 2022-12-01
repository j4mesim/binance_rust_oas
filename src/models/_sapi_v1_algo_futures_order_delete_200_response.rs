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
pub struct SapiV1AlgoFuturesOrderDelete200Response {
    #[serde(rename = "algoId")]
    pub algo_id: i64,
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "code")]
    pub code: i64,
    #[serde(rename = "msg")]
    pub msg: String,
}

impl SapiV1AlgoFuturesOrderDelete200Response {
    pub fn new(algo_id: i64, success: bool, code: i64, msg: String) -> SapiV1AlgoFuturesOrderDelete200Response {
        SapiV1AlgoFuturesOrderDelete200Response {
            algo_id,
            success,
            code,
            msg,
        }
    }
}


