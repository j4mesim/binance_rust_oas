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
pub struct SapiV1BswapUnclaimedRewardsGet200ResponseDetailsBnbBtc {
    #[serde(rename = "BNB")]
    pub bnb: f64,
}

impl SapiV1BswapUnclaimedRewardsGet200ResponseDetailsBnbBtc {
    pub fn new(bnb: f64) -> SapiV1BswapUnclaimedRewardsGet200ResponseDetailsBnbBtc {
        SapiV1BswapUnclaimedRewardsGet200ResponseDetailsBnbBtc {
            bnb,
        }
    }
}


