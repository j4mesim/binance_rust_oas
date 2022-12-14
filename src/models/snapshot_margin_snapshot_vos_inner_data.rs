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
pub struct SnapshotMarginSnapshotVosInnerData {
    #[serde(rename = "marginLevel")]
    pub margin_level: String,
    #[serde(rename = "totalAssetOfBtc")]
    pub total_asset_of_btc: String,
    #[serde(rename = "totalLiabilityOfBtc")]
    pub total_liability_of_btc: String,
    #[serde(rename = "totalNetAssetOfBtc")]
    pub total_net_asset_of_btc: String,
    #[serde(rename = "userAssets")]
    pub user_assets: Vec<crate::models::SnapshotMarginSnapshotVosInnerDataUserAssetsInner>,
}

impl SnapshotMarginSnapshotVosInnerData {
    pub fn new(margin_level: String, total_asset_of_btc: String, total_liability_of_btc: String, total_net_asset_of_btc: String, user_assets: Vec<crate::models::SnapshotMarginSnapshotVosInnerDataUserAssetsInner>) -> SnapshotMarginSnapshotVosInnerData {
        SnapshotMarginSnapshotVosInnerData {
            margin_level,
            total_asset_of_btc,
            total_liability_of_btc,
            total_net_asset_of_btc,
            user_assets,
        }
    }
}


