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
pub struct SubAccountUsdtFuturesDetails {
    #[serde(rename = "futureAccountResp")]
    pub future_account_resp: Box<crate::models::SubAccountUsdtFuturesDetailsFutureAccountResp>,
}

impl SubAccountUsdtFuturesDetails {
    pub fn new(future_account_resp: crate::models::SubAccountUsdtFuturesDetailsFutureAccountResp) -> SubAccountUsdtFuturesDetails {
        SubAccountUsdtFuturesDetails {
            future_account_resp: Box::new(future_account_resp),
        }
    }
}

