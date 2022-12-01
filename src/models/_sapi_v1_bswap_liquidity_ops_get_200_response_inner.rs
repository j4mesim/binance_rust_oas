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
pub struct SapiV1BswapLiquidityOpsGet200ResponseInner {
    #[serde(rename = "operationId")]
    pub operation_id: i64,
    #[serde(rename = "poolId")]
    pub pool_id: i64,
    #[serde(rename = "poolName")]
    pub pool_name: String,
    /// \"ADD\" or \"REMOVE\"
    #[serde(rename = "operation")]
    pub operation: String,
    /// 0: pending, 1: success, 2: failed
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    #[serde(rename = "shareAmount")]
    pub share_amount: String,
}

impl SapiV1BswapLiquidityOpsGet200ResponseInner {
    pub fn new(operation_id: i64, pool_id: i64, pool_name: String, operation: String, status: i32, update_time: i64, share_amount: String) -> SapiV1BswapLiquidityOpsGet200ResponseInner {
        SapiV1BswapLiquidityOpsGet200ResponseInner {
            operation_id,
            pool_id,
            pool_name,
            operation,
            status,
            update_time,
            share_amount,
        }
    }
}

