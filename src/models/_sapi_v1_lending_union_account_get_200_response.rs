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
pub struct SapiV1LendingUnionAccountGet200Response {
    #[serde(rename = "positionAmountVos")]
    pub position_amount_vos: Vec<crate::models::SapiV1LendingUnionAccountGet200ResponsePositionAmountVosInner>,
    #[serde(rename = "totalAmountInBTC")]
    pub total_amount_in_btc: String,
    #[serde(rename = "totalAmountInUSDT")]
    pub total_amount_in_usdt: String,
    #[serde(rename = "totalFixedAmountInBTC")]
    pub total_fixed_amount_in_btc: String,
    #[serde(rename = "totalFixedAmountInUSDT")]
    pub total_fixed_amount_in_usdt: String,
    #[serde(rename = "totalFlexibleInBTC")]
    pub total_flexible_in_btc: String,
    #[serde(rename = "totalFlexibleInUSDT")]
    pub total_flexible_in_usdt: String,
}

impl SapiV1LendingUnionAccountGet200Response {
    pub fn new(position_amount_vos: Vec<crate::models::SapiV1LendingUnionAccountGet200ResponsePositionAmountVosInner>, total_amount_in_btc: String, total_amount_in_usdt: String, total_fixed_amount_in_btc: String, total_fixed_amount_in_usdt: String, total_flexible_in_btc: String, total_flexible_in_usdt: String) -> SapiV1LendingUnionAccountGet200Response {
        SapiV1LendingUnionAccountGet200Response {
            position_amount_vos,
            total_amount_in_btc,
            total_amount_in_usdt,
            total_fixed_amount_in_btc,
            total_fixed_amount_in_usdt,
            total_flexible_in_btc,
            total_flexible_in_usdt,
        }
    }
}


