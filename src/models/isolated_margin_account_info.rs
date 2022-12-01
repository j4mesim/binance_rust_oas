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
pub struct IsolatedMarginAccountInfo {
    #[serde(rename = "assets")]
    pub assets: Vec<crate::models::IsolatedMarginAccountInfoAssetsInner>,
    #[serde(rename = "totalAssetOfBtc")]
    pub total_asset_of_btc: String,
    #[serde(rename = "totalLiabilityOfBtc")]
    pub total_liability_of_btc: String,
    #[serde(rename = "totalNetAssetOfBtc")]
    pub total_net_asset_of_btc: String,
}

impl IsolatedMarginAccountInfo {
    pub fn new(assets: Vec<crate::models::IsolatedMarginAccountInfoAssetsInner>, total_asset_of_btc: String, total_liability_of_btc: String, total_net_asset_of_btc: String) -> IsolatedMarginAccountInfo {
        IsolatedMarginAccountInfo {
            assets,
            total_asset_of_btc,
            total_liability_of_btc,
            total_net_asset_of_btc,
        }
    }
}


