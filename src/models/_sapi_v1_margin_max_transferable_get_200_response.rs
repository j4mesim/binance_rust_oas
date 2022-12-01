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
pub struct SapiV1MarginMaxTransferableGet200Response {
    /// Account's currently max borrowable amount with sufficient system availability
    #[serde(rename = "amount")]
    pub amount: String,
    /// Max borrowable amount limited by the account level
    #[serde(rename = "borrowLimit")]
    pub borrow_limit: String,
}

impl SapiV1MarginMaxTransferableGet200Response {
    pub fn new(amount: String, borrow_limit: String) -> SapiV1MarginMaxTransferableGet200Response {
        SapiV1MarginMaxTransferableGet200Response {
            amount,
            borrow_limit,
        }
    }
}


