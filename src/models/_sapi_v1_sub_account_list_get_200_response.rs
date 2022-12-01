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
pub struct SapiV1SubAccountListGet200Response {
    #[serde(rename = "subAccounts")]
    pub sub_accounts: Vec<crate::models::SapiV1SubAccountListGet200ResponseSubAccountsInner>,
}

impl SapiV1SubAccountListGet200Response {
    pub fn new(sub_accounts: Vec<crate::models::SapiV1SubAccountListGet200ResponseSubAccountsInner>) -> SapiV1SubAccountListGet200Response {
        SapiV1SubAccountListGet200Response {
            sub_accounts,
        }
    }
}


