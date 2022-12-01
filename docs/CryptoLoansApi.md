# \CryptoLoansApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_loan_adjust_ltv_post**](CryptoLoansApi.md#sapi_v1_loan_adjust_ltv_post) | **POST** /sapi/v1/loan/adjust/ltv | Crypto Loan Adjust LTV (TRADE)
[**sapi_v1_loan_borrow_history_get**](CryptoLoansApi.md#sapi_v1_loan_borrow_history_get) | **GET** /sapi/v1/loan/borrow/history | Get Crypto Loans Borrow History (USER_DATA)
[**sapi_v1_loan_borrow_post**](CryptoLoansApi.md#sapi_v1_loan_borrow_post) | **POST** /sapi/v1/loan/borrow | Crypto Loan Borrow (TRADE)
[**sapi_v1_loan_income_get**](CryptoLoansApi.md#sapi_v1_loan_income_get) | **GET** /sapi/v1/loan/income | Get Crypto Loans Income History (USER_DATA)
[**sapi_v1_loan_ltv_adjustment_history_get**](CryptoLoansApi.md#sapi_v1_loan_ltv_adjustment_history_get) | **GET** /sapi/v1/loan/ltv/adjustment/history | Get Loan LTV Adjustment History (USER_DATA)
[**sapi_v1_loan_ongoing_orders_get**](CryptoLoansApi.md#sapi_v1_loan_ongoing_orders_get) | **GET** /sapi/v1/loan/ongoing/orders | Get Loan Ongoing Orders (USER_DATA)
[**sapi_v1_loan_repay_history_get**](CryptoLoansApi.md#sapi_v1_loan_repay_history_get) | **GET** /sapi/v1/loan/repay/history | Get Loan Repayment History (USER_DATA)
[**sapi_v1_loan_repay_post**](CryptoLoansApi.md#sapi_v1_loan_repay_post) | **POST** /sapi/v1/loan/repay | Crypto Loan Repay (TRADE)



## sapi_v1_loan_adjust_ltv_post

> crate::models::SapiV1LoanAdjustLtvPost200Response sapi_v1_loan_adjust_ltv_post(order_id, amount, direction, timestamp, signature, recv_window)
Crypto Loan Adjust LTV (TRADE)

Weight(UID): 6000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **i64** | Order ID | [required] |
**amount** | **f64** | Amount | [required] |
**direction** | **String** | 'ADDITIONAL', 'REDUCED' | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1LoanAdjustLtvPost200Response**](_sapi_v1_loan_adjust_ltv_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_borrow_history_get

> crate::models::SapiV1LoanBorrowHistoryGet200Response sapi_v1_loan_borrow_history_get(timestamp, signature, order_id, loan_coin, collateral_coin, start_time, end_time, current, limit, recv_window)
Get Crypto Loans Borrow History (USER_DATA)

- If startTime and endTime are not sent, the recent 90-day data will be returned. - The max interval between startTime and endTime is 180 days.  Weight(IP): 400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_id** | Option<**i64**> | orderId in POST /sapi/v1/loan/borrow |  |
**loan_coin** | Option<**String**> | Coin loaned |  |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**limit** | Option<**i64**> | default 10, max 100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1LoanBorrowHistoryGet200Response**](_sapi_v1_loan_borrow_history_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_borrow_post

> crate::models::SapiV1LoanBorrowPost200Response sapi_v1_loan_borrow_post(loan_coin, collateral_coin, loan_term, timestamp, signature, loan_amount, collateral_amount, recv_window)
Crypto Loan Borrow (TRADE)

Weight(UID): 6000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**loan_coin** | **String** | Coin loaned | [required] |
**collateral_coin** | **String** | Coin used as collateral | [required] |
**loan_term** | **i32** | 7/14/30/90/180 days | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**loan_amount** | Option<**f64**> | Mandatory when collateralAmount is empty |  |
**collateral_amount** | Option<**f64**> | Mandatory when loanAmount is empty |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1LoanBorrowPost200Response**](_sapi_v1_loan_borrow_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_income_get

> Vec<crate::models::SapiV1LoanIncomeGet200ResponseInner> sapi_v1_loan_income_get(timestamp, signature, asset, r#type, start_time, end_time, limit, recv_window)
Get Crypto Loans Income History (USER_DATA)

- If startTime and endTime are not sent, the recent 7-day data will be returned. - The max interval between startTime and endTime is 30 days.  Weight(UID): 6000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**r#type** | Option<**String**> | All types will be returned by default. * `borrowIn` * `collateralSpent` * `repayAmount` * `collateralReturn` - Collateral return after repayment * `addCollateral` * `removeCollateral` * `collateralReturnAfterLiquidation` |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | default 20, max 100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1LoanIncomeGet200ResponseInner>**](_sapi_v1_loan_income_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_ltv_adjustment_history_get

> crate::models::SapiV1LoanLtvAdjustmentHistoryGet200Response sapi_v1_loan_ltv_adjustment_history_get(timestamp, signature, order_id, loan_coin, collateral_coin, start_time, end_time, current, limit, recv_window)
Get Loan LTV Adjustment History (USER_DATA)

If startTime and endTime are not sent, the recent 90-day data will be returned. The max interval between startTime and endTime is 180 days.  Weight(IP): 400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_id** | Option<**i64**> | Order ID |  |
**loan_coin** | Option<**String**> | Coin loaned |  |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**limit** | Option<**i64**> | default 10, max 100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1LoanLtvAdjustmentHistoryGet200Response**](_sapi_v1_loan_ltv_adjustment_history_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_ongoing_orders_get

> crate::models::SapiV1LoanOngoingOrdersGet200Response sapi_v1_loan_ongoing_orders_get(timestamp, signature, order_id, loan_coin, collateral_coin, current, limit, recv_window)
Get Loan Ongoing Orders (USER_DATA)

Weight(IP): 400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_id** | Option<**i64**> | orderId in POST /sapi/v1/loan/borrow |  |
**loan_coin** | Option<**String**> | Coin loaned |  |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**current** | Option<**i32**> | Current querying page. Start from 1; default:1, max:1000 |  |
**limit** | Option<**i64**> | default 10, max 100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1LoanOngoingOrdersGet200Response**](_sapi_v1_loan_ongoing_orders_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_repay_history_get

> crate::models::SapiV1LoanRepayHistoryGet200Response sapi_v1_loan_repay_history_get(timestamp, signature, order_id, loan_coin, collateral_coin, start_time, end_time, current, limit, recv_window)
Get Loan Repayment History (USER_DATA)

If startTime and endTime are not sent, the recent 90-day data will be returned. The max interval between startTime and endTime is 180 days.  Weight(IP): 400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_id** | Option<**i64**> | Order ID |  |
**loan_coin** | Option<**String**> | Coin loaned |  |
**collateral_coin** | Option<**String**> | Coin used as collateral |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**limit** | Option<**i64**> | default 10, max 100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1LoanRepayHistoryGet200Response**](_sapi_v1_loan_repay_history_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_loan_repay_post

> crate::models::SapiV1LoanRepayPost200Response sapi_v1_loan_repay_post(order_id, amount, timestamp, signature, r#type, collateral_return, recv_window)
Crypto Loan Repay (TRADE)

Weight(UID): 6000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **i64** | Order ID | [required] |
**amount** | **f64** | Repayment Amount | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**r#type** | Option<**i32**> | Default: 1. 1 for 'repay with borrowed coin'; 2 for 'repay with collateral'. |  |
**collateral_return** | Option<**bool**> | Default: TRUE. TRUE: Return extra collateral to spot account; FALSE: Keep extra collateral in the order. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1LoanRepayPost200Response**](_sapi_v1_loan_repay_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

