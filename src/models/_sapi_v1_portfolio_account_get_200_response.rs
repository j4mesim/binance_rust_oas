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
pub struct SapiV1PortfolioAccountGet200Response {
    #[serde(rename = "uniMMR")]
    pub uni_mmr: String,
    #[serde(rename = "accountEquity")]
    pub account_equity: String,
    #[serde(rename = "accountMaintMargin")]
    pub account_maint_margin: String,
    #[serde(rename = "accountStatus")]
    pub account_status: String,
}

impl SapiV1PortfolioAccountGet200Response {
    pub fn new(uni_mmr: String, account_equity: String, account_maint_margin: String, account_status: String) -> SapiV1PortfolioAccountGet200Response {
        SapiV1PortfolioAccountGet200Response {
            uni_mmr,
            account_equity,
            account_maint_margin,
            account_status,
        }
    }
}

