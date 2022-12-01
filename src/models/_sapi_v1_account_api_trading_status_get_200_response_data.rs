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
pub struct SapiV1AccountApiTradingStatusGet200ResponseData {
    /// API trading function is locked or not
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    /// If API trading function is locked, this is the planned recover time
    #[serde(rename = "plannedRecoverTime")]
    pub planned_recover_time: i64,
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: Box<crate::models::SapiV1AccountApiTradingStatusGet200ResponseDataTriggerCondition>,
    #[serde(rename = "indicators")]
    pub indicators: Box<crate::models::SapiV1AccountApiTradingStatusGet200ResponseDataIndicators>,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
}

impl SapiV1AccountApiTradingStatusGet200ResponseData {
    pub fn new(is_locked: bool, planned_recover_time: i64, trigger_condition: crate::models::SapiV1AccountApiTradingStatusGet200ResponseDataTriggerCondition, indicators: crate::models::SapiV1AccountApiTradingStatusGet200ResponseDataIndicators, update_time: i64) -> SapiV1AccountApiTradingStatusGet200ResponseData {
        SapiV1AccountApiTradingStatusGet200ResponseData {
            is_locked,
            planned_recover_time,
            trigger_condition: Box::new(trigger_condition),
            indicators: Box::new(indicators),
            update_time,
        }
    }
}

