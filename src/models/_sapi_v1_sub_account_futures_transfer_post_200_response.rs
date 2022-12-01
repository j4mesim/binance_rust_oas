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
pub struct SapiV1SubAccountFuturesTransferPost200Response {
    #[serde(rename = "txnId")]
    pub txn_id: String,
}

impl SapiV1SubAccountFuturesTransferPost200Response {
    pub fn new(txn_id: String) -> SapiV1SubAccountFuturesTransferPost200Response {
        SapiV1SubAccountFuturesTransferPost200Response {
            txn_id,
        }
    }
}


