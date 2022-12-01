# \SavingsApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_lending_customized_fixed_purchase_post**](SavingsApi.md#sapi_v1_lending_customized_fixed_purchase_post) | **POST** /sapi/v1/lending/customizedFixed/purchase | Purchase Fixed/Activity Project (USER_DATA)
[**sapi_v1_lending_daily_product_list_get**](SavingsApi.md#sapi_v1_lending_daily_product_list_get) | **GET** /sapi/v1/lending/daily/product/list | Get Flexible Product List (USER_DATA)
[**sapi_v1_lending_daily_purchase_post**](SavingsApi.md#sapi_v1_lending_daily_purchase_post) | **POST** /sapi/v1/lending/daily/purchase | Purchase Flexible Product (USER_DATA)
[**sapi_v1_lending_daily_redeem_post**](SavingsApi.md#sapi_v1_lending_daily_redeem_post) | **POST** /sapi/v1/lending/daily/redeem | Redeem Flexible Product (USER_DATA)
[**sapi_v1_lending_daily_token_position_get**](SavingsApi.md#sapi_v1_lending_daily_token_position_get) | **GET** /sapi/v1/lending/daily/token/position | Get Flexible Product Position (USER_DATA)
[**sapi_v1_lending_daily_user_left_quota_get**](SavingsApi.md#sapi_v1_lending_daily_user_left_quota_get) | **GET** /sapi/v1/lending/daily/userLeftQuota | Get Left Daily Purchase Quota of Flexible Product (USER_DATA)
[**sapi_v1_lending_daily_user_redemption_quota_get**](SavingsApi.md#sapi_v1_lending_daily_user_redemption_quota_get) | **GET** /sapi/v1/lending/daily/userRedemptionQuota | Get Left Daily Redemption Quota of Flexible Product (USER_DATA)
[**sapi_v1_lending_position_changed_post**](SavingsApi.md#sapi_v1_lending_position_changed_post) | **POST** /sapi/v1/lending/positionChanged | Change Fixed/Activity Position to Daily Position (USER_DATA)
[**sapi_v1_lending_project_list_get**](SavingsApi.md#sapi_v1_lending_project_list_get) | **GET** /sapi/v1/lending/project/list | Get Fixed/Activity Project List(USER_DATA)
[**sapi_v1_lending_project_position_list_get**](SavingsApi.md#sapi_v1_lending_project_position_list_get) | **GET** /sapi/v1/lending/project/position/list | Get Fixed/Activity Project Position (USER_DATA)
[**sapi_v1_lending_union_account_get**](SavingsApi.md#sapi_v1_lending_union_account_get) | **GET** /sapi/v1/lending/union/account | Lending Account (USER_DATA)
[**sapi_v1_lending_union_interest_history_get**](SavingsApi.md#sapi_v1_lending_union_interest_history_get) | **GET** /sapi/v1/lending/union/interestHistory | Get Interest History (USER_DATA)
[**sapi_v1_lending_union_purchase_record_get**](SavingsApi.md#sapi_v1_lending_union_purchase_record_get) | **GET** /sapi/v1/lending/union/purchaseRecord | Get Purchase Record (USER_DATA)
[**sapi_v1_lending_union_redemption_record_get**](SavingsApi.md#sapi_v1_lending_union_redemption_record_get) | **GET** /sapi/v1/lending/union/redemptionRecord | Get Redemption Record (USER_DATA)



## sapi_v1_lending_customized_fixed_purchase_post

> crate::models::SapiV1LendingCustomizedFixedPurchasePost200Response sapi_v1_lending_customized_fixed_purchase_post(project_id, lot, timestamp, signature, recv_window)
Purchase Fixed/Activity Project (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**lot** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1LendingCustomizedFixedPurchasePost200Response**](_sapi_v1_lending_customizedFixed_purchase_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_daily_product_list_get

> Vec<crate::models::SapiV1LendingDailyProductListGet200ResponseInner> sapi_v1_lending_daily_product_list_get(timestamp, signature, status, featured, current, size, recv_window)
Get Flexible Product List (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**status** | Option<**String**> | Default `ALL` |  |
**featured** | Option<**String**> | Default `ALL` |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1LendingDailyProductListGet200ResponseInner>**](_sapi_v1_lending_daily_product_list_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_daily_purchase_post

> crate::models::SapiV1LendingDailyPurchasePost200Response sapi_v1_lending_daily_purchase_post(product_id, amount, timestamp, signature, recv_window)
Purchase Flexible Product (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1LendingDailyPurchasePost200Response**](_sapi_v1_lending_daily_purchase_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_daily_redeem_post

> serde_json::Value sapi_v1_lending_daily_redeem_post(product_id, amount, r#type, timestamp, signature, recv_window)
Redeem Flexible Product (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**r#type** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_daily_token_position_get

> Vec<crate::models::SapiV1LendingDailyTokenPositionGet200ResponseInner> sapi_v1_lending_daily_token_position_get(asset, timestamp, signature, recv_window)
Get Flexible Product Position (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1LendingDailyTokenPositionGet200ResponseInner>**](_sapi_v1_lending_daily_token_position_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_daily_user_left_quota_get

> crate::models::SapiV1LendingDailyUserLeftQuotaGet200Response sapi_v1_lending_daily_user_left_quota_get(product_id, timestamp, signature, recv_window)
Get Left Daily Purchase Quota of Flexible Product (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1LendingDailyUserLeftQuotaGet200Response**](_sapi_v1_lending_daily_userLeftQuota_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_daily_user_redemption_quota_get

> crate::models::SapiV1LendingDailyUserRedemptionQuotaGet200Response sapi_v1_lending_daily_user_redemption_quota_get(product_id, r#type, timestamp, signature, recv_window)
Get Left Daily Redemption Quota of Flexible Product (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |
**r#type** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1LendingDailyUserRedemptionQuotaGet200Response**](_sapi_v1_lending_daily_userRedemptionQuota_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_position_changed_post

> crate::models::SapiV1LendingPositionChangedPost200Response sapi_v1_lending_position_changed_post(project_id, lot, timestamp, signature, position_id, recv_window)
Change Fixed/Activity Position to Daily Position (USER_DATA)

- PositionId is mandatory parameter for fixed position.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**lot** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**position_id** | Option<**String**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1LendingPositionChangedPost200Response**](_sapi_v1_lending_positionChanged_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_project_list_get

> Vec<crate::models::SapiV1LendingProjectListGet200ResponseInner> sapi_v1_lending_project_list_get(r#type, timestamp, signature, asset, status, is_sort_asc, sort_by, current, size, recv_window)
Get Fixed/Activity Project List(USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**status** | Option<**String**> | Default `ALL` |  |
**is_sort_asc** | Option<**bool**> | default \"true\" |  |
**sort_by** | Option<**String**> | Default `START_TIME` |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1LendingProjectListGet200ResponseInner>**](_sapi_v1_lending_project_list_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_project_position_list_get

> Vec<crate::models::SapiV1LendingProjectPositionListGet200ResponseInner> sapi_v1_lending_project_position_list_get(asset, timestamp, signature, project_id, status, recv_window)
Get Fixed/Activity Project Position (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**project_id** | Option<**String**> |  |  |
**status** | Option<**String**> | Default `ALL` |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1LendingProjectPositionListGet200ResponseInner>**](_sapi_v1_lending_project_position_list_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_union_account_get

> crate::models::SapiV1LendingUnionAccountGet200Response sapi_v1_lending_union_account_get(timestamp, signature, recv_window)
Lending Account (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1LendingUnionAccountGet200Response**](_sapi_v1_lending_union_account_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_union_interest_history_get

> Vec<crate::models::SapiV1LendingUnionInterestHistoryGet200ResponseInner> sapi_v1_lending_union_interest_history_get(lending_type, timestamp, signature, asset, start_time, end_time, current, size, recv_window)
Get Interest History (USER_DATA)

- The time between startTime and endTime cannot be longer than 30 days. - If startTime and endTime are both not sent, then the last 30 days' data will be returned.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lending_type** | **String** | * `DAILY` - for flexible * `ACTIVITY` - for activity * `CUSTOMIZED_FIXED` for fixed | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1LendingUnionInterestHistoryGet200ResponseInner>**](_sapi_v1_lending_union_interestHistory_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_union_purchase_record_get

> crate::models::SapiV1LendingUnionPurchaseRecordGet200Response sapi_v1_lending_union_purchase_record_get(lending_type, timestamp, signature, asset, start_time, end_time, current, size, recv_window)
Get Purchase Record (USER_DATA)

- The time between startTime and endTime cannot be longer than 30 days. - If startTime and endTime are both not sent, then the last 30 days' data will be returned.  Weigh(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lending_type** | **String** | * `DAILY` - for flexible * `ACTIVITY` - for activity * `CUSTOMIZED_FIXED` for fixed | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1LendingUnionPurchaseRecordGet200Response**](_sapi_v1_lending_union_purchaseRecord_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_lending_union_redemption_record_get

> crate::models::SapiV1LendingUnionRedemptionRecordGet200Response sapi_v1_lending_union_redemption_record_get(lending_type, timestamp, signature, asset, start_time, end_time, current, size, recv_window)
Get Redemption Record (USER_DATA)

- The time between startTime and endTime cannot be longer than 30 days. - If startTime and endTime are both not sent, then the last 30 days' data will be returned.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lending_type** | **String** | * `DAILY` - for flexible * `ACTIVITY` - for activity * `CUSTOMIZED_FIXED` for fixed | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1LendingUnionRedemptionRecordGet200Response**](_sapi_v1_lending_union_redemptionRecord_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

