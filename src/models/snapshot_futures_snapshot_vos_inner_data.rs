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
pub struct SnapshotFuturesSnapshotVosInnerData {
    #[serde(rename = "assets")]
    pub assets: Vec<crate::models::SnapshotFuturesSnapshotVosInnerDataAssetsInner>,
    #[serde(rename = "position")]
    pub position: Vec<crate::models::SnapshotFuturesSnapshotVosInnerDataPositionInner>,
}

impl SnapshotFuturesSnapshotVosInnerData {
    pub fn new(assets: Vec<crate::models::SnapshotFuturesSnapshotVosInnerDataAssetsInner>, position: Vec<crate::models::SnapshotFuturesSnapshotVosInnerDataPositionInner>) -> SnapshotFuturesSnapshotVosInnerData {
        SnapshotFuturesSnapshotVosInnerData {
            assets,
            position,
        }
    }
}


