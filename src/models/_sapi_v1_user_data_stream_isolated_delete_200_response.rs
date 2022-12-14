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
pub struct SapiV1UserDataStreamIsolatedDelete200Response {
    #[serde(rename = "listenKey")]
    pub listen_key: String,
}

impl SapiV1UserDataStreamIsolatedDelete200Response {
    pub fn new(listen_key: String) -> SapiV1UserDataStreamIsolatedDelete200Response {
        SapiV1UserDataStreamIsolatedDelete200Response {
            listen_key,
        }
    }
}


