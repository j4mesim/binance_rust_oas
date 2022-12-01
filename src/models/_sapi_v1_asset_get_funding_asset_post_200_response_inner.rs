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
pub struct SapiV1AssetGetFundingAssetPost200ResponseInner {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "free")]
    pub free: String,
    #[serde(rename = "locked")]
    pub locked: String,
    #[serde(rename = "freeze")]
    pub freeze: String,
    #[serde(rename = "withdrawing")]
    pub withdrawing: String,
    #[serde(rename = "btcValuation")]
    pub btc_valuation: String,
}

impl SapiV1AssetGetFundingAssetPost200ResponseInner {
    pub fn new(asset: String, free: String, locked: String, freeze: String, withdrawing: String, btc_valuation: String) -> SapiV1AssetGetFundingAssetPost200ResponseInner {
        SapiV1AssetGetFundingAssetPost200ResponseInner {
            asset,
            free,
            locked,
            freeze,
            withdrawing,
            btc_valuation,
        }
    }
}

