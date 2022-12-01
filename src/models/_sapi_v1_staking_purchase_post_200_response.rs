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
pub struct SapiV1StakingPurchasePost200Response {
    #[serde(rename = "positionId")]
    pub position_id: String,
    #[serde(rename = "success")]
    pub success: bool,
}

impl SapiV1StakingPurchasePost200Response {
    pub fn new(position_id: String, success: bool) -> SapiV1StakingPurchasePost200Response {
        SapiV1StakingPurchasePost200Response {
            position_id,
            success,
        }
    }
}

