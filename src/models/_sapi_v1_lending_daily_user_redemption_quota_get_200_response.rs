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
pub struct SapiV1LendingDailyUserRedemptionQuotaGet200Response {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "dailyQuota")]
    pub daily_quota: String,
    #[serde(rename = "leftQuota")]
    pub left_quota: String,
    #[serde(rename = "minRedemptionAmount")]
    pub min_redemption_amount: String,
}

impl SapiV1LendingDailyUserRedemptionQuotaGet200Response {
    pub fn new(asset: String, daily_quota: String, left_quota: String, min_redemption_amount: String) -> SapiV1LendingDailyUserRedemptionQuotaGet200Response {
        SapiV1LendingDailyUserRedemptionQuotaGet200Response {
            asset,
            daily_quota,
            left_quota,
            min_redemption_amount,
        }
    }
}

