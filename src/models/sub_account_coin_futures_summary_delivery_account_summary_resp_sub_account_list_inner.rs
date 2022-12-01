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
pub struct SubAccountCoinFuturesSummaryDeliveryAccountSummaryRespSubAccountListInner {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "totalMarginBalance")]
    pub total_margin_balance: String,
    #[serde(rename = "totalUnrealizedProfit")]
    pub total_unrealized_profit: String,
    #[serde(rename = "totalWalletBalance")]
    pub total_wallet_balance: String,
    #[serde(rename = "asset")]
    pub asset: String,
}

impl SubAccountCoinFuturesSummaryDeliveryAccountSummaryRespSubAccountListInner {
    pub fn new(email: String, total_margin_balance: String, total_unrealized_profit: String, total_wallet_balance: String, asset: String) -> SubAccountCoinFuturesSummaryDeliveryAccountSummaryRespSubAccountListInner {
        SubAccountCoinFuturesSummaryDeliveryAccountSummaryRespSubAccountListInner {
            email,
            total_margin_balance,
            total_unrealized_profit,
            total_wallet_balance,
            asset,
        }
    }
}


