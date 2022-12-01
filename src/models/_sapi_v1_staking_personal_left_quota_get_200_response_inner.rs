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
pub struct SapiV1StakingPersonalLeftQuotaGet200ResponseInner {
    #[serde(rename = "leftPersonalQuota")]
    pub left_personal_quota: String,
}

impl SapiV1StakingPersonalLeftQuotaGet200ResponseInner {
    pub fn new(left_personal_quota: String) -> SapiV1StakingPersonalLeftQuotaGet200ResponseInner {
        SapiV1StakingPersonalLeftQuotaGet200ResponseInner {
            left_personal_quota,
        }
    }
}


