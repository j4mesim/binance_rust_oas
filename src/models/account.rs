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
pub struct Account {
    #[serde(rename = "makerCommission")]
    pub maker_commission: i64,
    #[serde(rename = "takerCommission")]
    pub taker_commission: i64,
    #[serde(rename = "buyerCommission")]
    pub buyer_commission: i64,
    #[serde(rename = "sellerCommission")]
    pub seller_commission: i64,
    #[serde(rename = "canTrade")]
    pub can_trade: bool,
    #[serde(rename = "canWithdraw")]
    pub can_withdraw: bool,
    #[serde(rename = "canDeposit")]
    pub can_deposit: bool,
    #[serde(rename = "brokered")]
    pub brokered: bool,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    #[serde(rename = "accountType")]
    pub account_type: String,
    #[serde(rename = "balances")]
    pub balances: Vec<crate::models::AccountBalancesInner>,
}

impl Account {
    pub fn new(maker_commission: i64, taker_commission: i64, buyer_commission: i64, seller_commission: i64, can_trade: bool, can_withdraw: bool, can_deposit: bool, brokered: bool, update_time: i64, account_type: String, balances: Vec<crate::models::AccountBalancesInner>) -> Account {
        Account {
            maker_commission,
            taker_commission,
            buyer_commission,
            seller_commission,
            can_trade,
            can_withdraw,
            can_deposit,
            brokered,
            update_time,
            account_type,
            balances,
        }
    }
}


