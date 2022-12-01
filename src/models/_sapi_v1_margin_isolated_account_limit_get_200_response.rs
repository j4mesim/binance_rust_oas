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
pub struct SapiV1MarginIsolatedAccountLimitGet200Response {
    #[serde(rename = "enabledAccount")]
    pub enabled_account: i64,
    #[serde(rename = "maxAccount")]
    pub max_account: i64,
}

impl SapiV1MarginIsolatedAccountLimitGet200Response {
    pub fn new(enabled_account: i64, max_account: i64) -> SapiV1MarginIsolatedAccountLimitGet200Response {
        SapiV1MarginIsolatedAccountLimitGet200Response {
            enabled_account,
            max_account,
        }
    }
}


