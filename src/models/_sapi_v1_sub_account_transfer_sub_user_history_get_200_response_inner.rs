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
pub struct SapiV1SubAccountTransferSubUserHistoryGet200ResponseInner {
    #[serde(rename = "counterParty")]
    pub counter_party: String,
    #[serde(rename = "email")]
    pub email: String,
    /// 1 for transfer in, 2 for transfer out
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "qty")]
    pub qty: String,
    #[serde(rename = "fromAccountType")]
    pub from_account_type: String,
    #[serde(rename = "toAccountType")]
    pub to_account_type: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "tranId")]
    pub tran_id: i64,
    #[serde(rename = "time")]
    pub time: i64,
}

impl SapiV1SubAccountTransferSubUserHistoryGet200ResponseInner {
    pub fn new(counter_party: String, email: String, r#type: i32, asset: String, qty: String, from_account_type: String, to_account_type: String, status: String, tran_id: i64, time: i64) -> SapiV1SubAccountTransferSubUserHistoryGet200ResponseInner {
        SapiV1SubAccountTransferSubUserHistoryGet200ResponseInner {
            counter_party,
            email,
            r#type,
            asset,
            qty,
            from_account_type,
            to_account_type,
            status,
            tran_id,
            time,
        }
    }
}


