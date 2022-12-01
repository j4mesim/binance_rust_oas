# \MarginApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_bnb_burn_get**](MarginApi.md#sapi_v1_bnb_burn_get) | **GET** /sapi/v1/bnbBurn | Get All Isolated Margin Symbol(USER_DATA)
[**sapi_v1_bnb_burn_post**](MarginApi.md#sapi_v1_bnb_burn_post) | **POST** /sapi/v1/bnbBurn | Toggle BNB Burn On Spot Trade And Margin Interest (USER_DATA)
[**sapi_v1_margin_account_get**](MarginApi.md#sapi_v1_margin_account_get) | **GET** /sapi/v1/margin/account | Query Cross Margin Account Details (USER_DATA)
[**sapi_v1_margin_all_assets_get**](MarginApi.md#sapi_v1_margin_all_assets_get) | **GET** /sapi/v1/margin/allAssets | Get All Margin Assets (MARKET_DATA)
[**sapi_v1_margin_all_order_list_get**](MarginApi.md#sapi_v1_margin_all_order_list_get) | **GET** /sapi/v1/margin/allOrderList | Query Margin Account's all OCO (USER_DATA)
[**sapi_v1_margin_all_orders_get**](MarginApi.md#sapi_v1_margin_all_orders_get) | **GET** /sapi/v1/margin/allOrders | Query Margin Account's All Orders (USER_DATA)
[**sapi_v1_margin_all_pairs_get**](MarginApi.md#sapi_v1_margin_all_pairs_get) | **GET** /sapi/v1/margin/allPairs | Get All Cross Margin Pairs (MARKET_DATA)
[**sapi_v1_margin_asset_get**](MarginApi.md#sapi_v1_margin_asset_get) | **GET** /sapi/v1/margin/asset | Query Margin Asset (MARKET_DATA)
[**sapi_v1_margin_cross_margin_data_get**](MarginApi.md#sapi_v1_margin_cross_margin_data_get) | **GET** /sapi/v1/margin/crossMarginData | Query Cross Margin Fee Data (USER_DATA)
[**sapi_v1_margin_dribblet_get**](MarginApi.md#sapi_v1_margin_dribblet_get) | **GET** /sapi/v1/margin/dribblet | Margin Dustlog (USER_DATA)
[**sapi_v1_margin_force_liquidation_rec_get**](MarginApi.md#sapi_v1_margin_force_liquidation_rec_get) | **GET** /sapi/v1/margin/forceLiquidationRec | Get Force Liquidation Record (USER_DATA)
[**sapi_v1_margin_interest_history_get**](MarginApi.md#sapi_v1_margin_interest_history_get) | **GET** /sapi/v1/margin/interestHistory | Get Interest History (USER_DATA)
[**sapi_v1_margin_interest_rate_history_get**](MarginApi.md#sapi_v1_margin_interest_rate_history_get) | **GET** /sapi/v1/margin/interestRateHistory | Margin Interest Rate History (USER_DATA)
[**sapi_v1_margin_isolated_account_delete**](MarginApi.md#sapi_v1_margin_isolated_account_delete) | **DELETE** /sapi/v1/margin/isolated/account | Disable Isolated Margin Account (TRADE)
[**sapi_v1_margin_isolated_account_get**](MarginApi.md#sapi_v1_margin_isolated_account_get) | **GET** /sapi/v1/margin/isolated/account | Query Isolated Margin Account Info (USER_DATA)
[**sapi_v1_margin_isolated_account_limit_get**](MarginApi.md#sapi_v1_margin_isolated_account_limit_get) | **GET** /sapi/v1/margin/isolated/accountLimit | Query Enabled Isolated Margin Account Limit (USER_DATA)
[**sapi_v1_margin_isolated_account_post**](MarginApi.md#sapi_v1_margin_isolated_account_post) | **POST** /sapi/v1/margin/isolated/account | Enable Isolated Margin Account (TRADE)
[**sapi_v1_margin_isolated_all_pairs_get**](MarginApi.md#sapi_v1_margin_isolated_all_pairs_get) | **GET** /sapi/v1/margin/isolated/allPairs | Get All Isolated Margin Symbol(USER_DATA)
[**sapi_v1_margin_isolated_margin_data_get**](MarginApi.md#sapi_v1_margin_isolated_margin_data_get) | **GET** /sapi/v1/margin/isolatedMarginData | Query Isolated Margin Fee Data (USER_DATA)
[**sapi_v1_margin_isolated_margin_tier_get**](MarginApi.md#sapi_v1_margin_isolated_margin_tier_get) | **GET** /sapi/v1/margin/isolatedMarginTier | Query Isolated Margin Tier Data (USER_DATA)
[**sapi_v1_margin_isolated_pair_get**](MarginApi.md#sapi_v1_margin_isolated_pair_get) | **GET** /sapi/v1/margin/isolated/pair | Query Isolated Margin Symbol (USER_DATA)
[**sapi_v1_margin_isolated_transfer_get**](MarginApi.md#sapi_v1_margin_isolated_transfer_get) | **GET** /sapi/v1/margin/isolated/transfer | Get Isolated Margin Transfer History (USER_DATA)
[**sapi_v1_margin_isolated_transfer_post**](MarginApi.md#sapi_v1_margin_isolated_transfer_post) | **POST** /sapi/v1/margin/isolated/transfer | Isolated Margin Account Transfer (MARGIN)
[**sapi_v1_margin_loan_get**](MarginApi.md#sapi_v1_margin_loan_get) | **GET** /sapi/v1/margin/loan | Query Loan Record (USER_DATA)
[**sapi_v1_margin_loan_post**](MarginApi.md#sapi_v1_margin_loan_post) | **POST** /sapi/v1/margin/loan | Margin Account Borrow (MARGIN)
[**sapi_v1_margin_max_borrowable_get**](MarginApi.md#sapi_v1_margin_max_borrowable_get) | **GET** /sapi/v1/margin/maxBorrowable | Query Max Borrow (USER_DATA)
[**sapi_v1_margin_max_transferable_get**](MarginApi.md#sapi_v1_margin_max_transferable_get) | **GET** /sapi/v1/margin/maxTransferable | Query Max Transfer-Out Amount (USER_DATA)
[**sapi_v1_margin_my_trades_get**](MarginApi.md#sapi_v1_margin_my_trades_get) | **GET** /sapi/v1/margin/myTrades | Query Margin Account's Trade List (USER_DATA)
[**sapi_v1_margin_open_order_list_get**](MarginApi.md#sapi_v1_margin_open_order_list_get) | **GET** /sapi/v1/margin/openOrderList | Query Margin Account's Open OCO (USER_DATA)
[**sapi_v1_margin_open_orders_delete**](MarginApi.md#sapi_v1_margin_open_orders_delete) | **DELETE** /sapi/v1/margin/openOrders | Margin Account Cancel all Open Orders on a Symbol (TRADE)
[**sapi_v1_margin_open_orders_get**](MarginApi.md#sapi_v1_margin_open_orders_get) | **GET** /sapi/v1/margin/openOrders | Query Margin Account's Open Orders (USER_DATA)
[**sapi_v1_margin_order_delete**](MarginApi.md#sapi_v1_margin_order_delete) | **DELETE** /sapi/v1/margin/order | Margin Account Cancel Order (TRADE)
[**sapi_v1_margin_order_get**](MarginApi.md#sapi_v1_margin_order_get) | **GET** /sapi/v1/margin/order | Query Margin Account's Order (USER_DATA)
[**sapi_v1_margin_order_list_delete**](MarginApi.md#sapi_v1_margin_order_list_delete) | **DELETE** /sapi/v1/margin/orderList | Margin Account Cancel OCO (TRADE)
[**sapi_v1_margin_order_list_get**](MarginApi.md#sapi_v1_margin_order_list_get) | **GET** /sapi/v1/margin/orderList | Query Margin Account's OCO (USER_DATA)
[**sapi_v1_margin_order_oco_post**](MarginApi.md#sapi_v1_margin_order_oco_post) | **POST** /sapi/v1/margin/order/oco | Margin Account New OCO (TRADE)
[**sapi_v1_margin_order_post**](MarginApi.md#sapi_v1_margin_order_post) | **POST** /sapi/v1/margin/order | Margin Account New Order (TRADE)
[**sapi_v1_margin_pair_get**](MarginApi.md#sapi_v1_margin_pair_get) | **GET** /sapi/v1/margin/pair | Query Cross Margin Pair (MARKET_DATA)
[**sapi_v1_margin_price_index_get**](MarginApi.md#sapi_v1_margin_price_index_get) | **GET** /sapi/v1/margin/priceIndex | Query Margin PriceIndex (MARKET_DATA)
[**sapi_v1_margin_rate_limit_order_get**](MarginApi.md#sapi_v1_margin_rate_limit_order_get) | **GET** /sapi/v1/margin/rateLimit/order | Query Current Margin Order Count Usage (TRADE)
[**sapi_v1_margin_repay_get**](MarginApi.md#sapi_v1_margin_repay_get) | **GET** /sapi/v1/margin/repay | Query Repay Record (USER_DATA)
[**sapi_v1_margin_repay_post**](MarginApi.md#sapi_v1_margin_repay_post) | **POST** /sapi/v1/margin/repay | Margin Account Repay (MARGIN)
[**sapi_v1_margin_trade_coeff_get**](MarginApi.md#sapi_v1_margin_trade_coeff_get) | **GET** /sapi/v1/margin/tradeCoeff | Get Summary of Margin account (USER_DATA)
[**sapi_v1_margin_transfer_get**](MarginApi.md#sapi_v1_margin_transfer_get) | **GET** /sapi/v1/margin/transfer | Get Cross Margin Transfer History (USER_DATA)
[**sapi_v1_margin_transfer_post**](MarginApi.md#sapi_v1_margin_transfer_post) | **POST** /sapi/v1/margin/transfer | Cross Margin Account Transfer (MARGIN)



## sapi_v1_bnb_burn_get

> crate::models::BnbBurnStatus sapi_v1_bnb_burn_get(timestamp, signature, recv_window)
Get All Isolated Margin Symbol(USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::BnbBurnStatus**](bnbBurnStatus.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_bnb_burn_post

> crate::models::BnbBurnStatus sapi_v1_bnb_burn_post(timestamp, signature, spot_bnb_burn, interest_bnb_burn, recv_window)
Toggle BNB Burn On Spot Trade And Margin Interest (USER_DATA)

- \"spotBNBBurn\" and \"interestBNBBurn\" should be sent at least one.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**spot_bnb_burn** | Option<**String**> | Determines whether to use BNB to pay for trading fees on SPOT |  |
**interest_bnb_burn** | Option<**String**> | Determines whether to use BNB to pay for margin loan's interest |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::BnbBurnStatus**](bnbBurnStatus.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_account_get

> crate::models::SapiV1MarginAccountGet200Response sapi_v1_margin_account_get(timestamp, signature, recv_window)
Query Cross Margin Account Details (USER_DATA)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1MarginAccountGet200Response**](_sapi_v1_margin_account_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_all_assets_get

> Vec<crate::models::SapiV1MarginAllAssetsGet200ResponseInner> sapi_v1_margin_all_assets_get()
Get All Margin Assets (MARKET_DATA)

Weight(IP): 1

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::SapiV1MarginAllAssetsGet200ResponseInner>**](_sapi_v1_margin_allAssets_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_all_order_list_get

> Vec<crate::models::ApiV3AllOrderListGet200ResponseInner> sapi_v1_margin_all_order_list_get(timestamp, signature, is_isolated, symbol, from_id, start_time, end_time, limit, recv_window)
Query Margin Account's all OCO (USER_DATA)

Retrieves all OCO for a specific margin account based on provided optional parameters  Weight(IP): 200

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**symbol** | Option<**String**> | Mandatory for isolated margin, not supported for cross margin |  |
**from_id** | Option<**String**> | If supplied, neither `startTime` or `endTime` can be provided |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | Default Value: 500; Max Value: 1000 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::ApiV3AllOrderListGet200ResponseInner>**](_api_v3_allOrderList_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_all_orders_get

> Vec<crate::models::MarginOrderDetail> sapi_v1_margin_all_orders_get(symbol, timestamp, signature, is_isolated, order_id, start_time, end_time, limit, recv_window)
Query Margin Account's All Orders (USER_DATA)

- If `orderId` is set, it will get orders >= that orderId. Otherwise most recent orders are returned. - For some historical orders `cummulativeQuoteQty` will be < 0, meaning the data is not available at this time.  Weight(IP): 200  Request Limit: 60 times/min per IP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**order_id** | Option<**i64**> | Order id |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::MarginOrderDetail>**](marginOrderDetail.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_all_pairs_get

> Vec<crate::models::SapiV1MarginAllPairsGet200ResponseInner> sapi_v1_margin_all_pairs_get()
Get All Cross Margin Pairs (MARKET_DATA)

Weight(IP): 1

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::SapiV1MarginAllPairsGet200ResponseInner>**](_sapi_v1_margin_allPairs_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_asset_get

> crate::models::SapiV1MarginAssetGet200Response sapi_v1_margin_asset_get(asset)
Query Margin Asset (MARKET_DATA)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |

### Return type

[**crate::models::SapiV1MarginAssetGet200Response**](_sapi_v1_margin_asset_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_cross_margin_data_get

> Vec<crate::models::SapiV1MarginCrossMarginDataGet200ResponseInner> sapi_v1_margin_cross_margin_data_get(timestamp, signature, vip_level, coin, recv_window)
Query Cross Margin Fee Data (USER_DATA)

Get cross margin fee data collection with any vip level or user's current specific data as https://www.binance.com/en/margin-fee  Weight(IP): 1 when coin is specified; 5 when the coin parameter is omitted

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**vip_level** | Option<**i32**> | Defaults to user's vip level |  |
**coin** | Option<**String**> | Coin name |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1MarginCrossMarginDataGet200ResponseInner>**](_sapi_v1_margin_crossMarginData_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_dribblet_get

> crate::models::SapiV1MarginDribbletGet200Response sapi_v1_margin_dribblet_get(timestamp, signature, start_time, end_time, recv_window)
Margin Dustlog (USER_DATA)

Query the historical information of user's margin account small-value asset conversion BNB.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1MarginDribbletGet200Response**](_sapi_v1_margin_dribblet_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_force_liquidation_rec_get

> crate::models::SapiV1MarginForceLiquidationRecGet200Response sapi_v1_margin_force_liquidation_rec_get(timestamp, signature, start_time, end_time, isolated_symbol, current, size, recv_window)
Get Force Liquidation Record (USER_DATA)

- Response in descending order  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**isolated_symbol** | Option<**String**> | Isolated symbol |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1MarginForceLiquidationRecGet200Response**](_sapi_v1_margin_forceLiquidationRec_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_interest_history_get

> crate::models::SapiV1MarginInterestHistoryGet200Response sapi_v1_margin_interest_history_get(timestamp, signature, asset, isolated_symbol, start_time, end_time, current, size, archived, recv_window)
Get Interest History (USER_DATA)

- Response in descending order - If `isolatedSymbol` is not sent, crossed margin data will be returned - Set `archived` to `true` to query data from 6 months ago - `type` in response has 4 enums:   - `PERIODIC` interest charged per hour   - `ON_BORROW` first interest charged on borrow   - `PERIODIC_CONVERTED` interest charged per hour converted into BNB   - `ON_BORROW_CONVERTED` first interest charged on borrow converted into BNB  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**isolated_symbol** | Option<**String**> | Isolated symbol |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**archived** | Option<**String**> | Default: false. Set to true for archived data from 6 months ago |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1MarginInterestHistoryGet200Response**](_sapi_v1_margin_interestHistory_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_interest_rate_history_get

> Vec<crate::models::SapiV1MarginInterestRateHistoryGet200ResponseInner> sapi_v1_margin_interest_rate_history_get(asset, timestamp, signature, vip_level, start_time, end_time, recv_window)
Margin Interest Rate History (USER_DATA)

The max interval between startTime and endTime is 30 days.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**vip_level** | Option<**i32**> | Defaults to user's vip level |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1MarginInterestRateHistoryGet200ResponseInner>**](_sapi_v1_margin_interestRateHistory_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_isolated_account_delete

> crate::models::SapiV1MarginIsolatedAccountDelete200Response sapi_v1_margin_isolated_account_delete(symbol, timestamp, signature, recv_window)
Disable Isolated Margin Account (TRADE)

Disable isolated margin account for a specific symbol. Each trading pair can only be deactivated once every 24 hours .  Weight(UID): 300

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1MarginIsolatedAccountDelete200Response**](_sapi_v1_margin_isolated_account_delete_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_isolated_account_get

> crate::models::IsolatedMarginAccountInfo sapi_v1_margin_isolated_account_get(timestamp, signature, symbols, recv_window)
Query Isolated Margin Account Info (USER_DATA)

- If \"symbols\" is not sent, all isolated assets will be returned. - If \"symbols\" is sent, only the isolated assets of the sent symbols will be returned.  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**symbols** | Option<**String**> | Max 5 symbols can be sent; separated by ',' |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::IsolatedMarginAccountInfo**](isolatedMarginAccountInfo.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_isolated_account_limit_get

> crate::models::SapiV1MarginIsolatedAccountLimitGet200Response sapi_v1_margin_isolated_account_limit_get(timestamp, signature, recv_window)
Query Enabled Isolated Margin Account Limit (USER_DATA)

Query enabled isolated margin account limit.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1MarginIsolatedAccountLimitGet200Response**](_sapi_v1_margin_isolated_accountLimit_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_isolated_account_post

> crate::models::SapiV1MarginIsolatedAccountDelete200Response sapi_v1_margin_isolated_account_post(symbol, timestamp, signature, recv_window)
Enable Isolated Margin Account (TRADE)

Enable isolated margin account for a specific symbol.  Weight(UID): 300

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1MarginIsolatedAccountDelete200Response**](_sapi_v1_margin_isolated_account_delete_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_isolated_all_pairs_get

> Vec<crate::models::SapiV1MarginIsolatedPairGet200Response> sapi_v1_margin_isolated_all_pairs_get(timestamp, signature, recv_window)
Get All Isolated Margin Symbol(USER_DATA)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1MarginIsolatedPairGet200Response>**](_sapi_v1_margin_isolated_pair_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_isolated_margin_data_get

> Vec<crate::models::SapiV1MarginIsolatedMarginDataGet200ResponseInner> sapi_v1_margin_isolated_margin_data_get(timestamp, signature, vip_level, symbol, recv_window)
Query Isolated Margin Fee Data (USER_DATA)

Get isolated margin fee data collection with any vip level or user's current specific data as https://www.binance.com/en/margin-fee  Weight(IP): 1 when a single is specified; 10 when the symbol parameter is omitted

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**vip_level** | Option<**i32**> | Defaults to user's vip level |  |
**symbol** | Option<**String**> | Trading symbol, e.g. BNBUSDT |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1MarginIsolatedMarginDataGet200ResponseInner>**](_sapi_v1_margin_isolatedMarginData_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_isolated_margin_tier_get

> Vec<crate::models::SapiV1MarginIsolatedMarginTierGet200ResponseInner> sapi_v1_margin_isolated_margin_tier_get(symbol, timestamp, signature, tier, recv_window)
Query Isolated Margin Tier Data (USER_DATA)

Get isolated margin tier data collection with any tier as https://www.binance.com/en/margin-data  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**tier** | Option<**String**> | All margin tier data will be returned if tier is omitted |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1MarginIsolatedMarginTierGet200ResponseInner>**](_sapi_v1_margin_isolatedMarginTier_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_isolated_pair_get

> crate::models::SapiV1MarginIsolatedPairGet200Response sapi_v1_margin_isolated_pair_get(symbol, timestamp, signature, recv_window)
Query Isolated Margin Symbol (USER_DATA)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1MarginIsolatedPairGet200Response**](_sapi_v1_margin_isolated_pair_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_isolated_transfer_get

> crate::models::MarginTransferDetails sapi_v1_margin_isolated_transfer_get(symbol, timestamp, signature, asset, trans_from, trans_to, start_time, end_time, current, size, recv_window)
Get Isolated Margin Transfer History (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**trans_from** | Option<**String**> |  |  |
**trans_to** | Option<**String**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::MarginTransferDetails**](marginTransferDetails.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_isolated_transfer_post

> serde_json::Value sapi_v1_margin_isolated_transfer_post(asset, symbol, trans_from, trans_to, amount, timestamp, signature, recv_window)
Isolated Margin Account Transfer (MARGIN)

Weight(UID): 600

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**trans_from** | **String** |  | [required] |
**trans_to** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
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


## sapi_v1_margin_loan_get

> crate::models::SapiV1MarginLoanGet200Response sapi_v1_margin_loan_get(asset, timestamp, signature, isolated_symbol, tx_id, start_time, end_time, current, size, archived, recv_window)
Query Loan Record (USER_DATA)

- `txId` or `startTime` must be sent. `txId` takes precedence. - Response in descending order - If `isolatedSymbol` is not sent, crossed margin data will be returned - Set `archived` to `true` to query data from 6 months ago  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**isolated_symbol** | Option<**String**> | Isolated symbol |  |
**tx_id** | Option<**i64**> | the tranId in  `POST /sapi/v1/margin/loan` |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**archived** | Option<**String**> | Default: false. Set to true for archived data from 6 months ago |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1MarginLoanGet200Response**](_sapi_v1_margin_loan_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_loan_post

> crate::models::Transaction sapi_v1_margin_loan_post(asset, amount, timestamp, signature, is_isolated, symbol, recv_window)
Margin Account Borrow (MARGIN)

Apply for a loan.  - If \"isIsolated\" = \"TRUE\", \"symbol\" must be sent - \"isIsolated\" = \"FALSE\" for crossed margin loan  Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**symbol** | Option<**String**> | Trading symbol, e.g. BNBUSDT |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::Transaction**](transaction.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_max_borrowable_get

> crate::models::SapiV1MarginMaxBorrowableGet200Response sapi_v1_margin_max_borrowable_get(asset, timestamp, signature, isolated_symbol, recv_window)
Query Max Borrow (USER_DATA)

- If `isolatedSymbol` is not sent, crossed margin data will be sent. - `borrowLimit` is also available from https://www.binance.com/en/margin-fee  Weight(IP): 50

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**isolated_symbol** | Option<**String**> | Isolated symbol |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1MarginMaxBorrowableGet200Response**](_sapi_v1_margin_maxBorrowable_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_max_transferable_get

> crate::models::SapiV1MarginMaxTransferableGet200Response sapi_v1_margin_max_transferable_get(asset, timestamp, signature, isolated_symbol, recv_window)
Query Max Transfer-Out Amount (USER_DATA)

- If `isolatedSymbol` is not sent, crossed margin data will be sent.  Weight(IP): 50

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**isolated_symbol** | Option<**String**> | Isolated symbol |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1MarginMaxTransferableGet200Response**](_sapi_v1_margin_maxTransferable_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_my_trades_get

> Vec<crate::models::MarginTrade> sapi_v1_margin_my_trades_get(symbol, timestamp, signature, is_isolated, start_time, end_time, from_id, limit, recv_window)
Query Margin Account's Trade List (USER_DATA)

- If `fromId` is set, it will get orders >= that `fromId`. Otherwise most recent trades are returned.  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**from_id** | Option<**i64**> | Trade id to fetch from. Default gets most recent trades. |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::MarginTrade>**](marginTrade.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_open_order_list_get

> Vec<crate::models::SapiV1MarginOpenOrderListGet200ResponseInner> sapi_v1_margin_open_order_list_get(timestamp, signature, is_isolated, symbol, recv_window)
Query Margin Account's Open OCO (USER_DATA)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**symbol** | Option<**String**> | Mandatory for isolated margin, not supported for cross margin |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1MarginOpenOrderListGet200ResponseInner>**](_sapi_v1_margin_openOrderList_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_open_orders_delete

> Vec<crate::models::SapiV1MarginOpenOrdersDelete200ResponseInner> sapi_v1_margin_open_orders_delete(symbol, timestamp, signature, is_isolated, recv_window)
Margin Account Cancel all Open Orders on a Symbol (TRADE)

- Cancels all active orders on a symbol for margin account. - This includes OCO orders.  Weight(IP): 1 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1MarginOpenOrdersDelete200ResponseInner>**](_sapi_v1_margin_openOrders_delete_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_open_orders_get

> Vec<crate::models::MarginOrderDetail> sapi_v1_margin_open_orders_get(timestamp, signature, symbol, is_isolated, recv_window)
Query Margin Account's Open Orders (USER_DATA)

- If the `symbol` is not sent, orders for all symbols will be returned in an array. - When all symbols are returned, the number of requests counted against the rate limiter is equal to the number of symbols currently trading on the exchange - If isIsolated =\"TRUE\", symbol must be sent.  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**symbol** | Option<**String**> | Trading symbol, e.g. BNBUSDT |  |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::MarginOrderDetail>**](marginOrderDetail.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_order_delete

> crate::models::MarginOrder sapi_v1_margin_order_delete(symbol, timestamp, signature, is_isolated, order_id, orig_client_order_id, new_client_order_id, recv_window)
Margin Account Cancel Order (TRADE)

Cancel an active order for margin account.  Either `orderId` or `origClientOrderId` must be sent.  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**order_id** | Option<**i64**> | Order id |  |
**orig_client_order_id** | Option<**String**> | Order id from client |  |
**new_client_order_id** | Option<**String**> | Used to uniquely identify this cancel. Automatically generated by default |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::MarginOrder**](marginOrder.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_order_get

> crate::models::MarginOrderDetail sapi_v1_margin_order_get(symbol, timestamp, signature, is_isolated, order_id, orig_client_order_id, recv_window)
Query Margin Account's Order (USER_DATA)

- Either `orderId` or `origClientOrderId` must be sent. - For some historical orders `cummulativeQuoteQty` will be < 0, meaning the data is not available at this time.  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**order_id** | Option<**i64**> | Order id |  |
**orig_client_order_id** | Option<**String**> | Order id from client |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::MarginOrderDetail**](marginOrderDetail.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_order_list_delete

> crate::models::MarginOcoOrder sapi_v1_margin_order_list_delete(symbol, timestamp, signature, is_isolated, order_list_id, list_client_order_id, new_client_order_id, recv_window)
Margin Account Cancel OCO (TRADE)

Cancel an entire Order List for a margin account  - Canceling an individual leg will cancel the entire OCO - Either `orderListId` or `listClientOrderId` must be provided  Weight(UID): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**order_list_id** | Option<**i64**> | Order list id |  |
**list_client_order_id** | Option<**String**> | A unique Id for the entire orderList |  |
**new_client_order_id** | Option<**String**> | Used to uniquely identify this cancel. Automatically generated by default |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::MarginOcoOrder**](marginOcoOrder.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_order_list_get

> crate::models::SapiV1MarginOrderListDelete200Response sapi_v1_margin_order_list_get(timestamp, signature, is_isolated, symbol, order_list_id, orig_client_order_id, recv_window)
Query Margin Account's OCO (USER_DATA)

Retrieves a specific OCO based on provided optional parameters  - Either `orderListId` or `origClientOrderId` must be provided  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**symbol** | Option<**String**> | Mandatory for isolated margin, not supported for cross margin |  |
**order_list_id** | Option<**i64**> | Order list id |  |
**orig_client_order_id** | Option<**String**> | Order id from client |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1MarginOrderListDelete200Response**](_sapi_v1_margin_orderList_delete_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_order_oco_post

> crate::models::SapiV1MarginOrderOcoPost200Response sapi_v1_margin_order_oco_post(symbol, side, quantity, price, stop_price, timestamp, signature, is_isolated, list_client_order_id, limit_client_order_id, limit_iceberg_qty, stop_client_order_id, stop_limit_price, stop_iceberg_qty, stop_limit_time_in_force, new_order_resp_type, side_effect_type, recv_window)
Margin Account New OCO (TRADE)

Send in a new OCO for a margin account  - Price Restrictions:   - SELL: Limit Price > Last Price > Stop Price   - BUY: Limit Price < Last Price < Stop Price - Quantity Restrictions:   - Both legs must have the same quantity   - ICEBERG quantities however do not have to be the same. - Order Rate Limit   - OCO counts as 2 orders against the order rate limit.  Weight(UID): 6

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**side** | **String** |  | [required] |
**quantity** | **f64** |  | [required] |
**price** | **f64** | Order price | [required] |
**stop_price** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**list_client_order_id** | Option<**String**> | A unique Id for the entire orderList |  |
**limit_client_order_id** | Option<**String**> | A unique Id for the limit order |  |
**limit_iceberg_qty** | Option<**f64**> |  |  |
**stop_client_order_id** | Option<**String**> | A unique Id for the stop loss/stop loss limit leg |  |
**stop_limit_price** | Option<**f64**> | If provided, stopLimitTimeInForce is required. |  |
**stop_iceberg_qty** | Option<**f64**> |  |  |
**stop_limit_time_in_force** | Option<**String**> |  |  |
**new_order_resp_type** | Option<**String**> | Set the response JSON. |  |
**side_effect_type** | Option<**String**> | Default `NO_SIDE_EFFECT` |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1MarginOrderOcoPost200Response**](_sapi_v1_margin_order_oco_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_order_post

> crate::models::SapiV1MarginOrderDelete200Response sapi_v1_margin_order_post(symbol, side, r#type, quantity, timestamp, signature, is_isolated, quote_order_qty, price, stop_price, new_client_order_id, iceberg_qty, new_order_resp_type, side_effect_type, time_in_force, recv_window)
Margin Account New Order (TRADE)

Post a new order for margin account.  Weight(UID): 6

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**side** | **String** |  | [required] |
**r#type** | **String** | Order type | [required] |
**quantity** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**quote_order_qty** | Option<**f64**> | Quote quantity |  |
**price** | Option<**f64**> | Order price |  |
**stop_price** | Option<**f64**> | Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders. |  |
**new_client_order_id** | Option<**String**> | Used to uniquely identify this cancel. Automatically generated by default |  |
**iceberg_qty** | Option<**f64**> | Used with LIMIT, STOP_LOSS_LIMIT, and TAKE_PROFIT_LIMIT to create an iceberg order. |  |
**new_order_resp_type** | Option<**String**> | Set the response JSON. |  |
**side_effect_type** | Option<**String**> | Default `NO_SIDE_EFFECT` |  |
**time_in_force** | Option<**String**> | Order time in force |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1MarginOrderDelete200Response**](_sapi_v1_margin_order_delete_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_pair_get

> crate::models::SapiV1MarginPairGet200Response sapi_v1_margin_pair_get(symbol)
Query Cross Margin Pair (MARKET_DATA)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |

### Return type

[**crate::models::SapiV1MarginPairGet200Response**](_sapi_v1_margin_pair_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_price_index_get

> crate::models::SapiV1MarginPriceIndexGet200Response sapi_v1_margin_price_index_get(symbol)
Query Margin PriceIndex (MARKET_DATA)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |

### Return type

[**crate::models::SapiV1MarginPriceIndexGet200Response**](_sapi_v1_margin_priceIndex_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_rate_limit_order_get

> Vec<crate::models::SapiV1MarginRateLimitOrderGet200ResponseInner> sapi_v1_margin_rate_limit_order_get(timestamp, signature, is_isolated, symbol, recv_window)
Query Current Margin Order Count Usage (TRADE)

Displays the user's current margin order count usage for all intervals.  Weight(IP): 20

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**symbol** | Option<**String**> | isolated symbol, mandatory for isolated margin |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1MarginRateLimitOrderGet200ResponseInner>**](_sapi_v1_margin_rateLimit_order_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_repay_get

> crate::models::SapiV1MarginRepayGet200Response sapi_v1_margin_repay_get(asset, timestamp, signature, isolated_symbol, tx_id, start_time, end_time, current, size, archived, recv_window)
Query Repay Record (USER_DATA)

- `txId` or `startTime` must be sent. `txId` takes precedence. - Response in descending order - If `isolatedSymbol` is not sent, crossed margin data will be returned - Set `archived` to `true` to query data from 6 months ago  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**isolated_symbol** | Option<**String**> | Isolated symbol |  |
**tx_id** | Option<**i64**> | the tranId in  `POST /sapi/v1/margin/repay` |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**archived** | Option<**String**> | Default: false. Set to true for archived data from 6 months ago |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1MarginRepayGet200Response**](_sapi_v1_margin_repay_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_repay_post

> crate::models::Transaction sapi_v1_margin_repay_post(asset, amount, timestamp, signature, is_isolated, symbol, recv_window)
Margin Account Repay (MARGIN)

Repay loan for margin account.  - If \"isIsolated\" = \"TRUE\", \"symbol\" must be sent - \"isIsolated\" = \"FALSE\" for crossed margin repay  Weight(IP): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**is_isolated** | Option<**String**> | * `TRUE` - For isolated margin * `FALSE` - Default, not for isolated margin |  |
**symbol** | Option<**String**> | Trading symbol, e.g. BNBUSDT |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::Transaction**](transaction.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_trade_coeff_get

> crate::models::SapiV1MarginTradeCoeffGet200Response sapi_v1_margin_trade_coeff_get(email, timestamp, signature, recv_window)
Get Summary of Margin account (USER_DATA)

Get personal margin level information  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email Address | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1MarginTradeCoeffGet200Response**](_sapi_v1_margin_tradeCoeff_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_transfer_get

> crate::models::SapiV1MarginTransferGet200Response sapi_v1_margin_transfer_get(timestamp, signature, asset, r#type, start_time, end_time, current, size, archived, recv_window)
Get Cross Margin Transfer History (USER_DATA)

- Response in descending order - Returns data for last 7 days by default - Set `archived` to `true` to query data from 6 months ago  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**r#type** | Option<**String**> | Transfer Type |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**archived** | Option<**String**> | Default: false. Set to true for archived data from 6 months ago |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1MarginTransferGet200Response**](_sapi_v1_margin_transfer_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_margin_transfer_post

> crate::models::Transaction sapi_v1_margin_transfer_post(asset, amount, r#type, timestamp, signature, recv_window)
Cross Margin Account Transfer (MARGIN)

Execute transfer between spot account and cross margin account.  Weight(IP): 600

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**r#type** | **i32** | * `1` - transfer from main account to margin account * `2` - transfer from margin account to main account | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::Transaction**](transaction.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

