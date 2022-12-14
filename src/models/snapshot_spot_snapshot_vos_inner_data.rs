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
pub struct SnapshotSpotSnapshotVosInnerData {
    #[serde(rename = "balances")]
    pub balances: Vec<crate::models::SnapshotSpotSnapshotVosInnerDataBalancesInner>,
    #[serde(rename = "totalAssetOfBtc")]
    pub total_asset_of_btc: String,
}

impl SnapshotSpotSnapshotVosInnerData {
    pub fn new(balances: Vec<crate::models::SnapshotSpotSnapshotVosInnerDataBalancesInner>, total_asset_of_btc: String) -> SnapshotSpotSnapshotVosInnerData {
        SnapshotSpotSnapshotVosInnerData {
            balances,
            total_asset_of_btc,
        }
    }
}


