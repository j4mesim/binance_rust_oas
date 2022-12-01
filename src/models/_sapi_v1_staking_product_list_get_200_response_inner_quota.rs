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
pub struct SapiV1StakingProductListGet200ResponseInnerQuota {
    #[serde(rename = "totalPersonalQuota")]
    pub total_personal_quota: String,
    #[serde(rename = "minimum")]
    pub minimum: String,
}

impl SapiV1StakingProductListGet200ResponseInnerQuota {
    pub fn new(total_personal_quota: String, minimum: String) -> SapiV1StakingProductListGet200ResponseInnerQuota {
        SapiV1StakingProductListGet200ResponseInnerQuota {
            total_personal_quota,
            minimum,
        }
    }
}

