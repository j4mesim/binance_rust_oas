# \ConvertApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_convert_trade_flow_get**](ConvertApi.md#sapi_v1_convert_trade_flow_get) | **GET** /sapi/v1/convert/tradeFlow | Get Convert Trade History (USER_DATA)



## sapi_v1_convert_trade_flow_get

> crate::models::SapiV1ConvertTradeFlowGet200Response sapi_v1_convert_trade_flow_get(start_time, end_time, timestamp, signature, limit, recv_window)
Get Convert Trade History (USER_DATA)

- The max interval between startTime and endTime is 30 days.  Weight(UID): 100

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_time** | **i64** | UTC timestamp in ms | [required] |
**end_time** | **i64** | UTC timestamp in ms | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**limit** | Option<**i32**> | default 100, max 1000 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1ConvertTradeFlowGet200Response**](_sapi_v1_convert_tradeFlow_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

