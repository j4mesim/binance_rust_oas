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
pub struct SavingsFixedActivityRedemptionRecordInner {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    #[serde(rename = "interest")]
    pub interest: String,
    #[serde(rename = "principal")]
    pub principal: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "projectName")]
    pub project_name: String,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "status")]
    pub status: String,
}

impl SavingsFixedActivityRedemptionRecordInner {
    pub fn new(amount: String, asset: String, create_time: i64, interest: String, principal: String, project_id: String, project_name: String, start_time: i64, status: String) -> SavingsFixedActivityRedemptionRecordInner {
        SavingsFixedActivityRedemptionRecordInner {
            amount,
            asset,
            create_time,
            interest,
            principal,
            project_id,
            project_name,
            start_time,
            status,
        }
    }
}

