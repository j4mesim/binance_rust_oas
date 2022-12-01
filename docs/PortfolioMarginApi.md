# \PortfolioMarginApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_portfolio_account_get**](PortfolioMarginApi.md#sapi_v1_portfolio_account_get) | **GET** /sapi/v1/portfolio/account | Get Portfolio Margin Account Info (USER_DATA)
[**sapi_v1_portfolio_collateral_rate_get**](PortfolioMarginApi.md#sapi_v1_portfolio_collateral_rate_get) | **GET** /sapi/v1/portfolio/collateralRate | Portfolio Margin Collateral Rate (MARKET_DATA)
[**sapi_v1_portfolio_pm_loan_get**](PortfolioMarginApi.md#sapi_v1_portfolio_pm_loan_get) | **GET** /sapi/v1/portfolio/pmLoan | Query Portfolio Margin Bankruptcy Loan Amount (USER_DATA)
[**sapi_v1_portfolio_repay_post**](PortfolioMarginApi.md#sapi_v1_portfolio_repay_post) | **POST** /sapi/v1/portfolio/repay | Portfolio Margin Bankruptcy Loan Repay (USER_DATA)



## sapi_v1_portfolio_account_get

> crate::models::SapiV1PortfolioAccountGet200Response sapi_v1_portfolio_account_get(timestamp, signature, recv_window)
Get Portfolio Margin Account Info (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1PortfolioAccountGet200Response**](_sapi_v1_portfolio_account_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_portfolio_collateral_rate_get

> Vec<crate::models::SapiV1PortfolioCollateralRateGet200ResponseInner> sapi_v1_portfolio_collateral_rate_get()
Portfolio Margin Collateral Rate (MARKET_DATA)

Portfolio Margin Collateral Rate.  Weight(IP): 50

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::SapiV1PortfolioCollateralRateGet200ResponseInner>**](_sapi_v1_portfolio_collateralRate_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_portfolio_pm_loan_get

> crate::models::SapiV1PortfolioPmLoanGet200Response sapi_v1_portfolio_pm_loan_get(timestamp, signature, recv_window)
Query Portfolio Margin Bankruptcy Loan Amount (USER_DATA)

Query Portfolio Margin Bankruptcy Loan Amount.  Weight(UID): 500

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1PortfolioPmLoanGet200Response**](_sapi_v1_portfolio_pmLoan_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_portfolio_repay_post

> crate::models::SapiV1PortfolioRepayPost200Response sapi_v1_portfolio_repay_post(timestamp, signature, recv_window)
Portfolio Margin Bankruptcy Loan Repay (USER_DATA)

Repay Portfolio Margin Bankruptcy Loan.  Weight(UID): 3000

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1PortfolioRepayPost200Response**](_sapi_v1_portfolio_repay_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

