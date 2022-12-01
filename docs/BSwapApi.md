# \BSwapApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_bswap_add_liquidity_preview_get**](BSwapApi.md#sapi_v1_bswap_add_liquidity_preview_get) | **GET** /sapi/v1/bswap/addLiquidityPreview | Add Liquidity Preview (USER_DATA)
[**sapi_v1_bswap_claim_rewards_post**](BSwapApi.md#sapi_v1_bswap_claim_rewards_post) | **POST** /sapi/v1/bswap/claimRewards | Claim rewards (TRADE)
[**sapi_v1_bswap_claimed_history_get**](BSwapApi.md#sapi_v1_bswap_claimed_history_get) | **GET** /sapi/v1/bswap/claimedHistory | Get Claimed History (USER_DATA)
[**sapi_v1_bswap_liquidity_add_post**](BSwapApi.md#sapi_v1_bswap_liquidity_add_post) | **POST** /sapi/v1/bswap/liquidityAdd | Add Liquidity (TRADE)
[**sapi_v1_bswap_liquidity_get**](BSwapApi.md#sapi_v1_bswap_liquidity_get) | **GET** /sapi/v1/bswap/liquidity | Liquidity information of a pool (USER_DATA)
[**sapi_v1_bswap_liquidity_ops_get**](BSwapApi.md#sapi_v1_bswap_liquidity_ops_get) | **GET** /sapi/v1/bswap/liquidityOps | Liquidity Operation Record (USER_DATA)
[**sapi_v1_bswap_liquidity_remove_post**](BSwapApi.md#sapi_v1_bswap_liquidity_remove_post) | **POST** /sapi/v1/bswap/liquidityRemove | Remove Liquidity (TRADE)
[**sapi_v1_bswap_pool_configure_get**](BSwapApi.md#sapi_v1_bswap_pool_configure_get) | **GET** /sapi/v1/bswap/poolConfigure | Pool Configure (USER_DATA)
[**sapi_v1_bswap_pools_get**](BSwapApi.md#sapi_v1_bswap_pools_get) | **GET** /sapi/v1/bswap/pools | List All Swap Pools (MARKET_DATA)
[**sapi_v1_bswap_quote_get**](BSwapApi.md#sapi_v1_bswap_quote_get) | **GET** /sapi/v1/bswap/quote | Request Quote (USER_DATA)
[**sapi_v1_bswap_remove_liquidity_preview_get**](BSwapApi.md#sapi_v1_bswap_remove_liquidity_preview_get) | **GET** /sapi/v1/bswap/removeLiquidityPreview | Remove Liquidity Preview (USER_DATA)
[**sapi_v1_bswap_swap_get**](BSwapApi.md#sapi_v1_bswap_swap_get) | **GET** /sapi/v1/bswap/swap | Swap History (USER_DATA)
[**sapi_v1_bswap_swap_post**](BSwapApi.md#sapi_v1_bswap_swap_post) | **POST** /sapi/v1/bswap/swap | Swap (TRADE)
[**sapi_v1_bswap_unclaimed_rewards_get**](BSwapApi.md#sapi_v1_bswap_unclaimed_rewards_get) | **GET** /sapi/v1/bswap/unclaimedRewards | Get Unclaimed Rewards Record (USER_DATA)



## sapi_v1_bswap_add_liquidity_preview_get

> crate::models::SapiV1BswapAddLiquidityPreviewGet200Response sapi_v1_bswap_add_liquidity_preview_get(pool_id, r#type, quote_asset, quote_qty, timestamp, signature, recv_window)
Add Liquidity Preview (USER_DATA)

Calculate expected share amount for adding liquidity in single or dual token.  Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **i64** |  | [required] |
**r#type** | **String** | * `SINGLE` - for adding a single token * `COMBINATION` - for adding dual tokens | [required] |
**quote_asset** | **String** |  | [required] |
**quote_qty** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1BswapAddLiquidityPreviewGet200Response**](_sapi_v1_bswap_addLiquidityPreview_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_bswap_claim_rewards_post

> crate::models::SapiV1BswapClaimRewardsPost200Response sapi_v1_bswap_claim_rewards_post(timestamp, signature, r#type, recv_window)
Claim rewards (TRADE)

Claim swap rewards or liquidity rewards   Weight(UID): 1000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**r#type** | Option<**i32**> | 0: Swap rewards, 1: Liquidity rewards, default to 0 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1BswapClaimRewardsPost200Response**](_sapi_v1_bswap_claimRewards_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_bswap_claimed_history_get

> Vec<crate::models::SapiV1BswapClaimedHistoryGet200ResponseInner> sapi_v1_bswap_claimed_history_get(timestamp, signature, pool_id, asset_rewards, r#type, start_time, end_time, limit, recv_window)
Get Claimed History (USER_DATA)

Get history of claimed rewards.   Weight(UID): 1000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**pool_id** | Option<**i64**> |  |  |
**asset_rewards** | Option<**String**> |  |  |
**r#type** | Option<**i32**> | 0: Swap rewards, 1: Liquidity rewards, default to 0 |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | Default 3, max 100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1BswapClaimedHistoryGet200ResponseInner>**](_sapi_v1_bswap_claimedHistory_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_bswap_liquidity_add_post

> crate::models::SapiV1BswapLiquidityAddPost200Response sapi_v1_bswap_liquidity_add_post(pool_id, asset, quantity, timestamp, signature, r#type, recv_window)
Add Liquidity (TRADE)

Add liquidity to a pool.  Weight(UID): 1000 (Additional: 3 times one second)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **i64** |  | [required] |
**asset** | **String** |  | [required] |
**quantity** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**r#type** | Option<**String**> | * `Single` - to add a single token * `Combination` - to add dual tokens |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1BswapLiquidityAddPost200Response**](_sapi_v1_bswap_liquidityAdd_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_bswap_liquidity_get

> Vec<crate::models::SapiV1BswapLiquidityGet200ResponseInner> sapi_v1_bswap_liquidity_get(timestamp, signature, pool_id, recv_window)
Liquidity information of a pool (USER_DATA)

Get liquidity information and user share of a pool.  Weight(IP): - `1` for one pool; - `10` when the poolId parameter is omitted;

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**pool_id** | Option<**i64**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1BswapLiquidityGet200ResponseInner>**](_sapi_v1_bswap_liquidity_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_bswap_liquidity_ops_get

> Vec<crate::models::SapiV1BswapLiquidityOpsGet200ResponseInner> sapi_v1_bswap_liquidity_ops_get(timestamp, signature, operation_id, pool_id, operation, start_time, end_time, limit, recv_window)
Liquidity Operation Record (USER_DATA)

Get liquidity operation (add/remove) records.  Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**operation_id** | Option<**i64**> |  |  |
**pool_id** | Option<**i64**> |  |  |
**operation** | Option<**String**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1BswapLiquidityOpsGet200ResponseInner>**](_sapi_v1_bswap_liquidityOps_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_bswap_liquidity_remove_post

> crate::models::SapiV1BswapLiquidityAddPost200Response sapi_v1_bswap_liquidity_remove_post(pool_id, r#type, share_amount, timestamp, signature, asset, recv_window)
Remove Liquidity (TRADE)

Remove liquidity from a pool, `type` include `SINGLE` and `COMBINATION`, asset is mandatory for single asset removal  Weight(UID): 1000 (Additional: 3 times one second)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **i64** |  | [required] |
**r#type** | **String** | * `SINGLE` - for single asset removal * `COMBINATION` - for combination of all coins removal | [required] |
**share_amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> | Mandatory for single asset removal |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1BswapLiquidityAddPost200Response**](_sapi_v1_bswap_liquidityAdd_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_bswap_pool_configure_get

> Vec<crate::models::SapiV1BswapPoolConfigureGet200ResponseInner> sapi_v1_bswap_pool_configure_get(timestamp, signature, pool_id, recv_window)
Pool Configure (USER_DATA)

Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**pool_id** | Option<**i64**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1BswapPoolConfigureGet200ResponseInner>**](_sapi_v1_bswap_poolConfigure_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_bswap_pools_get

> Vec<crate::models::SapiV1BswapPoolsGet200ResponseInner> sapi_v1_bswap_pools_get()
List All Swap Pools (MARKET_DATA)

Get metadata about all swap pools.  Weight(IP): 1

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::SapiV1BswapPoolsGet200ResponseInner>**](_sapi_v1_bswap_pools_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_bswap_quote_get

> crate::models::SapiV1BswapQuoteGet200Response sapi_v1_bswap_quote_get(quote_asset, base_asset, quote_qty, timestamp, signature, recv_window)
Request Quote (USER_DATA)

Request a quote for swap quote asset (selling asset) for base asset (buying asset), essentially price/exchange rates.  quoteQty is quantity of quote asset (to sell).  Please be noted the quote is for reference only, the actual price will change as the liquidity changes, it's recommended to swap immediate after request a quote for slippage prevention.  Weight(UID): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quote_asset** | **String** |  | [required] |
**base_asset** | **String** |  | [required] |
**quote_qty** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1BswapQuoteGet200Response**](_sapi_v1_bswap_quote_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_bswap_remove_liquidity_preview_get

> crate::models::SapiV1BswapRemoveLiquidityPreviewGet200Response sapi_v1_bswap_remove_liquidity_preview_get(pool_id, r#type, quote_asset, share_amount, timestamp, signature, recv_window)
Remove Liquidity Preview (USER_DATA)

Calculate the expected asset amount of single token redemption or dual token redemption.  Weight(IP): 150

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **i64** |  | [required] |
**r#type** | **String** | * `SINGLE` - remove and obtain a single token * `COMBINATION` - remove and obtain dual token | [required] |
**quote_asset** | **String** |  | [required] |
**share_amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1BswapRemoveLiquidityPreviewGet200Response**](_sapi_v1_bswap_removeLiquidityPreview_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_bswap_swap_get

> Vec<crate::models::SapiV1BswapSwapGet200ResponseInner> sapi_v1_bswap_swap_get(timestamp, signature, swap_id, start_time, end_time, status, quote_asset, base_asset, limit, recv_window)
Swap History (USER_DATA)

Get swap history.  Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**swap_id** | Option<**i64**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**status** | Option<**i32**> | * `0` - pending for swap * `1` - success * `2` - failed |  |
**quote_asset** | Option<**String**> |  |  |
**base_asset** | Option<**String**> |  |  |
**limit** | Option<**i32**> | default 3, max 100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1BswapSwapGet200ResponseInner>**](_sapi_v1_bswap_swap_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_bswap_swap_post

> crate::models::SapiV1BswapSwapGet200Response sapi_v1_bswap_swap_post(quote_asset, base_asset, quote_qty, timestamp, signature, recv_window)
Swap (TRADE)

Swap `quoteAsset` for `baseAsset`.  Weight(UID): 1000 (Additional: 3 times one second)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quote_asset** | **String** |  | [required] |
**base_asset** | **String** |  | [required] |
**quote_qty** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1BswapSwapGet200Response**](_sapi_v1_bswap_swap_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_bswap_unclaimed_rewards_get

> crate::models::SapiV1BswapUnclaimedRewardsGet200Response sapi_v1_bswap_unclaimed_rewards_get(timestamp, signature, r#type, recv_window)
Get Unclaimed Rewards Record (USER_DATA)

Get unclaimed rewards record.   Weight(UID): 1000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**r#type** | Option<**i32**> | 0: Swap rewards, 1: Liquidity rewards, default to 0 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1BswapUnclaimedRewardsGet200Response**](_sapi_v1_bswap_unclaimedRewards_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

