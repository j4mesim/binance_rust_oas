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
pub struct MarginTransferDetailsRowsInner {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "txId")]
    pub tx_id: i64,
    #[serde(rename = "transFrom")]
    pub trans_from: String,
    #[serde(rename = "transTo")]
    pub trans_to: String,
}

impl MarginTransferDetailsRowsInner {
    pub fn new(amount: String, asset: String, status: String, timestamp: i64, tx_id: i64, trans_from: String, trans_to: String) -> MarginTransferDetailsRowsInner {
        MarginTransferDetailsRowsInner {
            amount,
            asset,
            status,
            timestamp,
            tx_id,
            trans_from,
            trans_to,
        }
    }
}


