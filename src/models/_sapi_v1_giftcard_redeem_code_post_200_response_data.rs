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
pub struct SapiV1GiftcardRedeemCodePost200ResponseData {
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "referenceNo")]
    pub reference_no: String,
    #[serde(rename = "identityNo")]
    pub identity_no: String,
}

impl SapiV1GiftcardRedeemCodePost200ResponseData {
    pub fn new(token: String, amount: String, reference_no: String, identity_no: String) -> SapiV1GiftcardRedeemCodePost200ResponseData {
        SapiV1GiftcardRedeemCodePost200ResponseData {
            token,
            amount,
            reference_no,
            identity_no,
        }
    }
}


