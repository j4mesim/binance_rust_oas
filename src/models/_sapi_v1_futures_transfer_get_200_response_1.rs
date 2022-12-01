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
pub struct SapiV1FuturesTransferGet200Response1 {
    #[serde(rename = "tranId")]
    pub tran_id: i64,
}

impl SapiV1FuturesTransferGet200Response1 {
    pub fn new(tran_id: i64) -> SapiV1FuturesTransferGet200Response1 {
        SapiV1FuturesTransferGet200Response1 {
            tran_id,
        }
    }
}


