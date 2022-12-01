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
pub struct SapiV1MarginPriceIndexGet200Response {
    #[serde(rename = "calcTime")]
    pub calc_time: i64,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
}

impl SapiV1MarginPriceIndexGet200Response {
    pub fn new(calc_time: i64, price: String, symbol: String) -> SapiV1MarginPriceIndexGet200Response {
        SapiV1MarginPriceIndexGet200Response {
            calc_time,
            price,
            symbol,
        }
    }
}


