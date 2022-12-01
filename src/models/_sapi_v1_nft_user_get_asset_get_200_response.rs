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
pub struct SapiV1NftUserGetAssetGet200Response {
    #[serde(rename = "total")]
    pub total: i32,
    #[serde(rename = "list")]
    pub list: Vec<crate::models::SapiV1NftUserGetAssetGet200ResponseListInner>,
}

impl SapiV1NftUserGetAssetGet200Response {
    pub fn new(total: i32, list: Vec<crate::models::SapiV1NftUserGetAssetGet200ResponseListInner>) -> SapiV1NftUserGetAssetGet200Response {
        SapiV1NftUserGetAssetGet200Response {
            total,
            list,
        }
    }
}


