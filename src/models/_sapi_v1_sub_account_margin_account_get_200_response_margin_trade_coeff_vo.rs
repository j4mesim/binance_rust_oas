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
pub struct SapiV1SubAccountMarginAccountGet200ResponseMarginTradeCoeffVo {
    /// Liquidation margin ratio
    #[serde(rename = "forceLiquidationBar")]
    pub force_liquidation_bar: String,
    /// Margin call margin ratio
    #[serde(rename = "marginCallBar")]
    pub margin_call_bar: String,
    /// Initial margin ratio
    #[serde(rename = "normalBar")]
    pub normal_bar: String,
}

impl SapiV1SubAccountMarginAccountGet200ResponseMarginTradeCoeffVo {
    pub fn new(force_liquidation_bar: String, margin_call_bar: String, normal_bar: String) -> SapiV1SubAccountMarginAccountGet200ResponseMarginTradeCoeffVo {
        SapiV1SubAccountMarginAccountGet200ResponseMarginTradeCoeffVo {
            force_liquidation_bar,
            margin_call_bar,
            normal_bar,
        }
    }
}


