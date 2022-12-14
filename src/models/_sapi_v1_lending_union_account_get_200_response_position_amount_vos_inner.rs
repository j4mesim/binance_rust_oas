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
pub struct SapiV1LendingUnionAccountGet200ResponsePositionAmountVosInner {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "amountInBTC")]
    pub amount_in_btc: String,
    #[serde(rename = "amountInUSDT")]
    pub amount_in_usdt: String,
    #[serde(rename = "asset")]
    pub asset: String,
}

impl SapiV1LendingUnionAccountGet200ResponsePositionAmountVosInner {
    pub fn new(amount: String, amount_in_btc: String, amount_in_usdt: String, asset: String) -> SapiV1LendingUnionAccountGet200ResponsePositionAmountVosInner {
        SapiV1LendingUnionAccountGet200ResponsePositionAmountVosInner {
            amount,
            amount_in_btc,
            amount_in_usdt,
            asset,
        }
    }
}


