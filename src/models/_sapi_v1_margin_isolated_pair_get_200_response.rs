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
pub struct SapiV1MarginIsolatedPairGet200Response {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "base")]
    pub base: String,
    #[serde(rename = "quote")]
    pub quote: String,
    #[serde(rename = "isMarginTrade")]
    pub is_margin_trade: bool,
    #[serde(rename = "isBuyAllowed")]
    pub is_buy_allowed: bool,
    #[serde(rename = "isSellAllowed")]
    pub is_sell_allowed: bool,
}

impl SapiV1MarginIsolatedPairGet200Response {
    pub fn new(symbol: String, base: String, quote: String, is_margin_trade: bool, is_buy_allowed: bool, is_sell_allowed: bool) -> SapiV1MarginIsolatedPairGet200Response {
        SapiV1MarginIsolatedPairGet200Response {
            symbol,
            base,
            quote,
            is_margin_trade,
            is_buy_allowed,
            is_sell_allowed,
        }
    }
}


