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
pub struct SnapshotMarginSnapshotVosInner {
    #[serde(rename = "data")]
    pub data: Box<crate::models::SnapshotMarginSnapshotVosInnerData>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
}

impl SnapshotMarginSnapshotVosInner {
    pub fn new(data: crate::models::SnapshotMarginSnapshotVosInnerData, r#type: String, update_time: i64) -> SnapshotMarginSnapshotVosInner {
        SnapshotMarginSnapshotVosInner {
            data: Box::new(data),
            r#type,
            update_time,
        }
    }
}


