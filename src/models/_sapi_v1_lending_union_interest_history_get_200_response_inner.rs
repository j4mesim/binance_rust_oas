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
pub struct SapiV1LendingUnionInterestHistoryGet200ResponseInner {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "interest")]
    pub interest: String,
    #[serde(rename = "lendingType")]
    pub lending_type: String,
    #[serde(rename = "productName")]
    pub product_name: String,
    #[serde(rename = "time")]
    pub time: i64,
}

impl SapiV1LendingUnionInterestHistoryGet200ResponseInner {
    pub fn new(asset: String, interest: String, lending_type: String, product_name: String, time: i64) -> SapiV1LendingUnionInterestHistoryGet200ResponseInner {
        SapiV1LendingUnionInterestHistoryGet200ResponseInner {
            asset,
            interest,
            lending_type,
            product_name,
            time,
        }
    }
}


