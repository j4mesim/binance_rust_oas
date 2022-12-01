# \StakingApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_staking_personal_left_quota_get**](StakingApi.md#sapi_v1_staking_personal_left_quota_get) | **GET** /sapi/v1/staking/personalLeftQuota | Get Personal Left Quota of Staking Product (USER_DATA)
[**sapi_v1_staking_position_get**](StakingApi.md#sapi_v1_staking_position_get) | **GET** /sapi/v1/staking/position | Get Staking Product Position (USER_DATA)
[**sapi_v1_staking_product_list_get**](StakingApi.md#sapi_v1_staking_product_list_get) | **GET** /sapi/v1/staking/productList | Get Staking Product List (USER_DATA)
[**sapi_v1_staking_purchase_post**](StakingApi.md#sapi_v1_staking_purchase_post) | **POST** /sapi/v1/staking/purchase | Purchase Staking Product (USER_DATA)
[**sapi_v1_staking_redeem_post**](StakingApi.md#sapi_v1_staking_redeem_post) | **POST** /sapi/v1/staking/redeem | Redeem Staking Product (USER_DATA)
[**sapi_v1_staking_set_auto_staking_post**](StakingApi.md#sapi_v1_staking_set_auto_staking_post) | **POST** /sapi/v1/staking/setAutoStaking | Set Auto Staking (USER_DATA)
[**sapi_v1_staking_staking_record_get**](StakingApi.md#sapi_v1_staking_staking_record_get) | **GET** /sapi/v1/staking/stakingRecord | Get Staking History (USER_DATA)



## sapi_v1_staking_personal_left_quota_get

> Vec<crate::models::SapiV1StakingPersonalLeftQuotaGet200ResponseInner> sapi_v1_staking_personal_left_quota_get(product, product_id, timestamp, signature, recv_window)
Get Personal Left Quota of Staking Product (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product** | **String** | * `STAKING` - for Locked Staking * `F_DEFI` - for flexible DeFi Staking * `L_DEFI` - for locked DeFi Staking | [required] |
**product_id** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1StakingPersonalLeftQuotaGet200ResponseInner>**](_sapi_v1_staking_personalLeftQuota_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_staking_position_get

> Vec<crate::models::SapiV1StakingPositionGet200ResponseInner> sapi_v1_staking_position_get(product, timestamp, signature, product_id, asset, current, size, recv_window)
Get Staking Product Position (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product** | **String** | * `STAKING` - for Locked Staking * `F_DEFI` - for flexible DeFi Staking * `L_DEFI` - for locked DeFi Staking | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**product_id** | Option<**String**> |  |  |
**asset** | Option<**String**> |  |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1StakingPositionGet200ResponseInner>**](_sapi_v1_staking_position_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_staking_product_list_get

> Vec<crate::models::SapiV1StakingProductListGet200ResponseInner> sapi_v1_staking_product_list_get(product, timestamp, signature, asset, current, size, recv_window)
Get Staking Product List (USER_DATA)

Get available Staking product list.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product** | **String** | * `STAKING` - for Locked Staking * `F_DEFI` - for flexible DeFi Staking * `L_DEFI` - for locked DeFi Staking | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1StakingProductListGet200ResponseInner>**](_sapi_v1_staking_productList_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_staking_purchase_post

> crate::models::SapiV1StakingPurchasePost200Response sapi_v1_staking_purchase_post(product, product_id, amount, timestamp, signature, renewable, recv_window)
Purchase Staking Product (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product** | **String** | * `STAKING` - for Locked Staking * `F_DEFI` - for flexible DeFi Staking * `L_DEFI` - for locked DeFi Staking | [required] |
**product_id** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**renewable** | Option<**String**> | true or false, default false. Active if product is `STAKING` or `L_DEFI` |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1StakingPurchasePost200Response**](_sapi_v1_staking_purchase_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_staking_redeem_post

> crate::models::SapiV1StakingRedeemPost200Response sapi_v1_staking_redeem_post(product, product_id, timestamp, signature, position_id, amount, recv_window)
Redeem Staking Product (USER_DATA)

Redeem Staking product. Locked staking and Locked DeFI staking belong to early redemption, redeeming in advance will result in loss of interest that you have earned.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product** | **String** | * `STAKING` - for Locked Staking * `F_DEFI` - for flexible DeFi Staking * `L_DEFI` - for locked DeFi Staking | [required] |
**product_id** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**position_id** | Option<**String**> | Mandatory if product is `STAKING` or `L_DEFI` |  |
**amount** | Option<**f64**> | Mandatory if product is `F_DEFI` |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1StakingRedeemPost200Response**](_sapi_v1_staking_redeem_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_staking_set_auto_staking_post

> crate::models::SapiV1StakingRedeemPost200Response sapi_v1_staking_set_auto_staking_post(product, position_id, renewable, timestamp, signature, recv_window)
Set Auto Staking (USER_DATA)

Set auto staking on Locked Staking or Locked DeFi Staking  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product** | **String** | * `STAKING` - for Locked Staking * `L_DEFI` - for locked DeFi Staking | [required] |
**position_id** | **String** |  | [required] |
**renewable** | **String** | true or false | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1StakingRedeemPost200Response**](_sapi_v1_staking_redeem_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_staking_staking_record_get

> Vec<crate::models::SapiV1StakingStakingRecordGet200ResponseInner> sapi_v1_staking_staking_record_get(product, txn_type, timestamp, signature, asset, start_time, end_time, current, size, recv_window)
Get Staking History (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product** | **String** | * `STAKING` - for Locked Staking * `F_DEFI` - for flexible DeFi Staking * `L_DEFI` - for locked DeFi Staking | [required] |
**txn_type** | **String** | `SUBSCRIPTION`, `REDEMPTION`, `INTEREST` | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1StakingStakingRecordGet200ResponseInner>**](_sapi_v1_staking_stakingRecord_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

