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
pub struct SavingsFlexiblePurchaseRecordInner {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    #[serde(rename = "lendingType")]
    pub lending_type: String,
    #[serde(rename = "productName")]
    pub product_name: String,
    #[serde(rename = "purchaseId")]
    pub purchase_id: i64,
    #[serde(rename = "status")]
    pub status: String,
}

impl SavingsFlexiblePurchaseRecordInner {
    pub fn new(amount: String, asset: String, create_time: i64, lending_type: String, product_name: String, purchase_id: i64, status: String) -> SavingsFlexiblePurchaseRecordInner {
        SavingsFlexiblePurchaseRecordInner {
            amount,
            asset,
            create_time,
            lending_type,
            product_name,
            purchase_id,
            status,
        }
    }
}


