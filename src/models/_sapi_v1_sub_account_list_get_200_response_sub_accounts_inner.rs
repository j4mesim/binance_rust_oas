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
pub struct SapiV1SubAccountListGet200ResponseSubAccountsInner {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "isFreeze")]
    pub is_freeze: bool,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    #[serde(rename = "isManagedSubAccount")]
    pub is_managed_sub_account: bool,
    #[serde(rename = "isAssetManagementSubAccount")]
    pub is_asset_management_sub_account: bool,
}

impl SapiV1SubAccountListGet200ResponseSubAccountsInner {
    pub fn new(email: String, is_freeze: bool, create_time: i64, is_managed_sub_account: bool, is_asset_management_sub_account: bool) -> SapiV1SubAccountListGet200ResponseSubAccountsInner {
        SapiV1SubAccountListGet200ResponseSubAccountsInner {
            email,
            is_freeze,
            create_time,
            is_managed_sub_account,
            is_asset_management_sub_account,
        }
    }
}


