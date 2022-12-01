# \FuturesApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_futures_loan_adjust_collateral_history_get**](FuturesApi.md#sapi_v1_futures_loan_adjust_collateral_history_get) | **GET** /sapi/v1/futures/loan/adjustCollateral/history | Adjust Cross-Collateral LTV History (USER_DATA)
[**sapi_v1_futures_loan_borrow_history_get**](FuturesApi.md#sapi_v1_futures_loan_borrow_history_get) | **GET** /sapi/v1/futures/loan/borrow/history | Cross-Collateral Borrow History (USER_DATA)
[**sapi_v1_futures_loan_interest_history_get**](FuturesApi.md#sapi_v1_futures_loan_interest_history_get) | **GET** /sapi/v1/futures/loan/interestHistory | Cross-Collateral Interest History (USER_DATA)
[**sapi_v1_futures_loan_liquidation_history_get**](FuturesApi.md#sapi_v1_futures_loan_liquidation_history_get) | **GET** /sapi/v1/futures/loan/liquidationHistory | Cross-Collateral Liquidation History (USER_DATA)
[**sapi_v1_futures_loan_repay_history_get**](FuturesApi.md#sapi_v1_futures_loan_repay_history_get) | **GET** /sapi/v1/futures/loan/repay/history | Cross-Collateral Repayment History (USER_DATA)
[**sapi_v1_futures_transfer_get**](FuturesApi.md#sapi_v1_futures_transfer_get) | **GET** /sapi/v1/futures/transfer | Get Future Account Transaction History List (USER_DATA)
[**sapi_v1_futures_transfer_post**](FuturesApi.md#sapi_v1_futures_transfer_post) | **POST** /sapi/v1/futures/transfer | New Future Account Transfer (USER_DATA)
[**sapi_v2_futures_loan_wallet_get**](FuturesApi.md#sapi_v2_futures_loan_wallet_get) | **GET** /sapi/v2/futures/loan/wallet | Cross-Collateral Wallet V2 (USER_DATA)



## sapi_v1_futures_loan_adjust_collateral_history_get

> crate::models::SapiV1FuturesLoanAdjustCollateralHistoryGet200Response sapi_v1_futures_loan_adjust_collateral_history_get(timestamp, signature, loan_coin, collateral_coin, start_time, end_time, limit, recv_window)
Adjust Cross-Collateral LTV History (USER_DATA)

All data will be returned if loanCoin or collateralCoin is not sent  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**loan_coin** | Option<**String**> | Coin loaned |  |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1FuturesLoanAdjustCollateralHistoryGet200Response**](_sapi_v1_futures_loan_adjustCollateral_history_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_futures_loan_borrow_history_get

> crate::models::SapiV1FuturesLoanBorrowHistoryGet200Response sapi_v1_futures_loan_borrow_history_get(timestamp, signature, coin, start_time, end_time, limit, recv_window)
Cross-Collateral Borrow History (USER_DATA)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**coin** | Option<**String**> | Coin name |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1FuturesLoanBorrowHistoryGet200Response**](_sapi_v1_futures_loan_borrow_history_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_futures_loan_interest_history_get

> crate::models::SapiV1FuturesLoanInterestHistoryGet200Response sapi_v1_futures_loan_interest_history_get(timestamp, signature, collateral_coin, start_time, end_time, current, limit, recv_window)
Cross-Collateral Interest History (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1FuturesLoanInterestHistoryGet200Response**](_sapi_v1_futures_loan_interestHistory_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_futures_loan_liquidation_history_get

> crate::models::SapiV1FuturesLoanLiquidationHistoryGet200Response sapi_v1_futures_loan_liquidation_history_get(timestamp, signature, loan_coin, collateral_coin, start_time, end_time, limit, recv_window)
Cross-Collateral Liquidation History (USER_DATA)

All data will be returned if loanCoin or collateralCoin is not sent  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**loan_coin** | Option<**String**> | Coin loaned |  |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1FuturesLoanLiquidationHistoryGet200Response**](_sapi_v1_futures_loan_liquidationHistory_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_futures_loan_repay_history_get

> crate::models::SapiV1FuturesLoanRepayHistoryGet200Response sapi_v1_futures_loan_repay_history_get(timestamp, signature, coin, start_time, end_time, limit, recv_window)
Cross-Collateral Repayment History (USER_DATA)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**coin** | Option<**String**> | Coin name |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1FuturesLoanRepayHistoryGet200Response**](_sapi_v1_futures_loan_repay_history_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_futures_transfer_get

> crate::models::SapiV1FuturesTransferGet200Response sapi_v1_futures_transfer_get(asset, start_time, timestamp, signature, end_time, current, size, recv_window)
Get Future Account Transaction History List (USER_DATA)

Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**start_time** | **i64** | UTC timestamp in ms | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1FuturesTransferGet200Response**](_sapi_v1_futures_transfer_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_futures_transfer_post

> crate::models::SapiV1FuturesTransferGet200Response1 sapi_v1_futures_transfer_post(asset, amount, r#type, timestamp, signature, recv_window)
New Future Account Transfer (USER_DATA)

Execute transfer between spot account and futures account.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**r#type** | **i64** | 1: transfer from spot account to USDT-Ⓜ futures account. 2: transfer from USDT-Ⓜ futures account to spot account. 3: transfer from spot account to COIN-Ⓜ futures account. 4: transfer from COIN-Ⓜ futures account to spot account. | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1FuturesTransferGet200Response1**](_sapi_v1_futures_transfer_get_200_response_1.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v2_futures_loan_wallet_get

> crate::models::SapiV2FuturesLoanWalletGet200Response sapi_v2_futures_loan_wallet_get(timestamp, signature, recv_window)
Cross-Collateral Wallet V2 (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV2FuturesLoanWalletGet200Response**](_sapi_v2_futures_loan_wallet_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

