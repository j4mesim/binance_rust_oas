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
pub struct SapiV1BswapPoolConfigureGet200ResponseInnerAssetConfigureBusd {
    #[serde(rename = "minAdd")]
    pub min_add: i64,
    #[serde(rename = "maxAdd")]
    pub max_add: i64,
    #[serde(rename = "minSwap")]
    pub min_swap: i64,
    #[serde(rename = "maxSwap")]
    pub max_swap: i64,
}

impl SapiV1BswapPoolConfigureGet200ResponseInnerAssetConfigureBusd {
    pub fn new(min_add: i64, max_add: i64, min_swap: i64, max_swap: i64) -> SapiV1BswapPoolConfigureGet200ResponseInnerAssetConfigureBusd {
        SapiV1BswapPoolConfigureGet200ResponseInnerAssetConfigureBusd {
            min_add,
            max_add,
            min_swap,
            max_swap,
        }
    }
}


