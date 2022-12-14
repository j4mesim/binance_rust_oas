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
pub struct SapiV1LendingDailyPurchasePost200Response {
    #[serde(rename = "purchaseId")]
    pub purchase_id: i64,
}

impl SapiV1LendingDailyPurchasePost200Response {
    pub fn new(purchase_id: i64) -> SapiV1LendingDailyPurchasePost200Response {
        SapiV1LendingDailyPurchasePost200Response {
            purchase_id,
        }
    }
}


