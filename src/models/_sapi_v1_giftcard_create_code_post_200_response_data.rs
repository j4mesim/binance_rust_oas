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
pub struct SapiV1GiftcardCreateCodePost200ResponseData {
    #[serde(rename = "referenceNo")]
    pub reference_no: String,
    #[serde(rename = "code")]
    pub code: String,
}

impl SapiV1GiftcardCreateCodePost200ResponseData {
    pub fn new(reference_no: String, code: String) -> SapiV1GiftcardCreateCodePost200ResponseData {
        SapiV1GiftcardCreateCodePost200ResponseData {
            reference_no,
            code,
        }
    }
}

