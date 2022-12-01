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
pub struct SapiV1LoanLtvAdjustmentHistoryGet200Response {
    #[serde(rename = "rows")]
    pub rows: Vec<crate::models::SapiV1LoanLtvAdjustmentHistoryGet200ResponseRowsInner>,
    #[serde(rename = "total")]
    pub total: i32,
}

impl SapiV1LoanLtvAdjustmentHistoryGet200Response {
    pub fn new(rows: Vec<crate::models::SapiV1LoanLtvAdjustmentHistoryGet200ResponseRowsInner>, total: i32) -> SapiV1LoanLtvAdjustmentHistoryGet200Response {
        SapiV1LoanLtvAdjustmentHistoryGet200Response {
            rows,
            total,
        }
    }
}


