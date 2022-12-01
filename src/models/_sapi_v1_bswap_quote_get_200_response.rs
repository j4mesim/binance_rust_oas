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
pub struct SapiV1BswapQuoteGet200Response {
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
    #[serde(rename = "baseAsset")]
    pub base_asset: String,
    #[serde(rename = "quoteQty")]
    pub quote_qty: f64,
    #[serde(rename = "baseQty")]
    pub base_qty: f64,
    #[serde(rename = "price")]
    pub price: f64,
    #[serde(rename = "slippage")]
    pub slippage: f64,
    #[serde(rename = "fee")]
    pub fee: f64,
}

impl SapiV1BswapQuoteGet200Response {
    pub fn new(quote_asset: String, base_asset: String, quote_qty: f64, base_qty: f64, price: f64, slippage: f64, fee: f64) -> SapiV1BswapQuoteGet200Response {
        SapiV1BswapQuoteGet200Response {
            quote_asset,
            base_asset,
            quote_qty,
            base_qty,
            price,
            slippage,
            fee,
        }
    }
}

