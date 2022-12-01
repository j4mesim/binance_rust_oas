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
pub struct SapiV1MiningPaymentOtherGet200ResponseData {
    #[serde(rename = "otherProfits")]
    pub other_profits: Vec<crate::models::SapiV1MiningPaymentOtherGet200ResponseDataOtherProfitsInner>,
    /// Total Rows
    #[serde(rename = "totalNum")]
    pub total_num: i64,
    /// Rows per page
    #[serde(rename = "pageSize")]
    pub page_size: i64,
}

impl SapiV1MiningPaymentOtherGet200ResponseData {
    pub fn new(other_profits: Vec<crate::models::SapiV1MiningPaymentOtherGet200ResponseDataOtherProfitsInner>, total_num: i64, page_size: i64) -> SapiV1MiningPaymentOtherGet200ResponseData {
        SapiV1MiningPaymentOtherGet200ResponseData {
            other_profits,
            total_num,
            page_size,
        }
    }
}


