# \WalletApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_account_api_restrictions_get**](WalletApi.md#sapi_v1_account_api_restrictions_get) | **GET** /sapi/v1/account/apiRestrictions | Get API Key Permission (USER_DATA)
[**sapi_v1_account_api_trading_status_get**](WalletApi.md#sapi_v1_account_api_trading_status_get) | **GET** /sapi/v1/account/apiTradingStatus | Account API Trading Status (USER_DATA)
[**sapi_v1_account_disable_fast_withdraw_switch_post**](WalletApi.md#sapi_v1_account_disable_fast_withdraw_switch_post) | **POST** /sapi/v1/account/disableFastWithdrawSwitch | Disable Fast Withdraw Switch (USER_DATA)
[**sapi_v1_account_enable_fast_withdraw_switch_post**](WalletApi.md#sapi_v1_account_enable_fast_withdraw_switch_post) | **POST** /sapi/v1/account/enableFastWithdrawSwitch | Enable Fast Withdraw Switch (USER_DATA)
[**sapi_v1_account_snapshot_get**](WalletApi.md#sapi_v1_account_snapshot_get) | **GET** /sapi/v1/accountSnapshot | Daily Account Snapshot (USER_DATA)
[**sapi_v1_account_status_get**](WalletApi.md#sapi_v1_account_status_get) | **GET** /sapi/v1/account/status | Account Status (USER_DATA)
[**sapi_v1_asset_asset_detail_get**](WalletApi.md#sapi_v1_asset_asset_detail_get) | **GET** /sapi/v1/asset/assetDetail | Asset Detail (USER_DATA)
[**sapi_v1_asset_asset_dividend_get**](WalletApi.md#sapi_v1_asset_asset_dividend_get) | **GET** /sapi/v1/asset/assetDividend | Asset Dividend Record (USER_DATA)
[**sapi_v1_asset_convert_transfer_post**](WalletApi.md#sapi_v1_asset_convert_transfer_post) | **POST** /sapi/v1/asset/convert-transfer | Convert Transfer (USER_DATA)
[**sapi_v1_asset_convert_transfer_query_by_page_post**](WalletApi.md#sapi_v1_asset_convert_transfer_query_by_page_post) | **POST** /sapi/v1/asset/convert-transfer/queryByPage | Query Convert Transfer (USER_DATA)
[**sapi_v1_asset_dribblet_get**](WalletApi.md#sapi_v1_asset_dribblet_get) | **GET** /sapi/v1/asset/dribblet | DustLog(USER_DATA)
[**sapi_v1_asset_dust_btc_post**](WalletApi.md#sapi_v1_asset_dust_btc_post) | **POST** /sapi/v1/asset/dust-btc | Get Assets That Can Be Converted Into BNB (USER_DATA)
[**sapi_v1_asset_dust_post**](WalletApi.md#sapi_v1_asset_dust_post) | **POST** /sapi/v1/asset/dust | Dust Transfer (USER_DATA)
[**sapi_v1_asset_get_funding_asset_post**](WalletApi.md#sapi_v1_asset_get_funding_asset_post) | **POST** /sapi/v1/asset/get-funding-asset | Funding Wallet (USER_DATA)
[**sapi_v1_asset_trade_fee_get**](WalletApi.md#sapi_v1_asset_trade_fee_get) | **GET** /sapi/v1/asset/tradeFee | Trade Fee (USER_DATA)
[**sapi_v1_asset_transfer_get**](WalletApi.md#sapi_v1_asset_transfer_get) | **GET** /sapi/v1/asset/transfer | Query User Universal Transfer History (USER_DATA)
[**sapi_v1_asset_transfer_post**](WalletApi.md#sapi_v1_asset_transfer_post) | **POST** /sapi/v1/asset/transfer | User Universal Transfer (USER_DATA)
[**sapi_v1_capital_config_getall_get**](WalletApi.md#sapi_v1_capital_config_getall_get) | **GET** /sapi/v1/capital/config/getall | All Coins' Information (USER_DATA)
[**sapi_v1_capital_deposit_address_get**](WalletApi.md#sapi_v1_capital_deposit_address_get) | **GET** /sapi/v1/capital/deposit/address | Deposit Address (supporting network) (USER_DATA)
[**sapi_v1_capital_deposit_hisrec_get**](WalletApi.md#sapi_v1_capital_deposit_hisrec_get) | **GET** /sapi/v1/capital/deposit/hisrec | Deposit History???supporting network??? (USER_DATA)
[**sapi_v1_capital_withdraw_apply_post**](WalletApi.md#sapi_v1_capital_withdraw_apply_post) | **POST** /sapi/v1/capital/withdraw/apply | Withdraw (USER_DATA)
[**sapi_v1_capital_withdraw_history_get**](WalletApi.md#sapi_v1_capital_withdraw_history_get) | **GET** /sapi/v1/capital/withdraw/history | Withdraw History (supporting network) (USER_DATA)
[**sapi_v1_system_status_get**](WalletApi.md#sapi_v1_system_status_get) | **GET** /sapi/v1/system/status | System Status (System)
[**sapi_v3_asset_get_user_asset_post**](WalletApi.md#sapi_v3_asset_get_user_asset_post) | **POST** /sapi/v3/asset/getUserAsset | User Asset (USER_DATA)



## sapi_v1_account_api_restrictions_get

> crate::models::SapiV1AccountApiRestrictionsGet200Response sapi_v1_account_api_restrictions_get(timestamp, signature, recv_window)
Get API Key Permission (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1AccountApiRestrictionsGet200Response**](_sapi_v1_account_apiRestrictions_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_account_api_trading_status_get

> crate::models::SapiV1AccountApiTradingStatusGet200Response sapi_v1_account_api_trading_status_get(timestamp, signature, recv_window)
Account API Trading Status (USER_DATA)

Fetch account API trading status with details.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1AccountApiTradingStatusGet200Response**](_sapi_v1_account_apiTradingStatus_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_account_disable_fast_withdraw_switch_post

> serde_json::Value sapi_v1_account_disable_fast_withdraw_switch_post(timestamp, signature, recv_window)
Disable Fast Withdraw Switch (USER_DATA)

- This request will disable fastwithdraw switch under your account. - You need to enable \"trade\" option for the api key which requests this endpoint.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## sapi_v1_account_enable_fast_withdraw_switch_post

> serde_json::Value sapi_v1_account_enable_fast_withdraw_switch_post(timestamp, signature, recv_window)
Enable Fast Withdraw Switch (USER_DATA)

- This request will enable fastwithdraw switch under your account. You need to enable \"trade\" option for the api key which requests this endpoint. - When Fast Withdraw Switch is on, transferring funds to a Binance account will be done instantly. There is no on-chain transaction, no transaction ID and no withdrawal fee.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## sapi_v1_account_snapshot_get

> crate::models::SapiV1AccountSnapshotGet200Response sapi_v1_account_snapshot_get(r#type, timestamp, signature, start_time, end_time, limit, recv_window)
Daily Account Snapshot (USER_DATA)

- The query time period must be less than 30 days - Support query within the last one month only - If startTimeand endTime not sent, return records of the last 7 days by default  Weight(IP): 2400

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> |  |  |[default to 7]
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1AccountSnapshotGet200Response**](_sapi_v1_accountSnapshot_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_account_status_get

> crate::models::SapiV1AccountStatusGet200Response sapi_v1_account_status_get(timestamp, signature, recv_window)
Account Status (USER_DATA)

Fetch account status detail.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1AccountStatusGet200Response**](_sapi_v1_account_status_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_asset_asset_detail_get

> crate::models::SapiV1AssetAssetDetailGet200Response sapi_v1_asset_asset_detail_get(timestamp, signature, asset, recv_window)
Asset Detail (USER_DATA)

Fetch details of assets supported on Binance.  - Please get network and other deposit or withdraw details from `GET /sapi/v1/capital/config/getall`.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1AssetAssetDetailGet200Response**](_sapi_v1_asset_assetDetail_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_asset_asset_dividend_get

> crate::models::SapiV1AssetAssetDividendGet200Response sapi_v1_asset_asset_dividend_get(timestamp, signature, asset, start_time, end_time, limit, recv_window)
Asset Dividend Record (USER_DATA)

Query asset Dividend Record  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> |  |  |[default to 20]
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1AssetAssetDividendGet200Response**](_sapi_v1_asset_assetDividend_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_asset_convert_transfer_post

> crate::models::SapiV1AssetConvertTransferPost200Response sapi_v1_asset_convert_transfer_post(client_tran_id, asset, amount, target_asset, timestamp, signature, recv_window)
Convert Transfer (USER_DATA)

Convert transfer, convert between BUSD and stablecoins. If the clientId has been used before, will not do the convert transfer, the original transfer will be returned.  Weight(UID): 5

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_tran_id** | **String** | The unique flag, the min length is 20 | [required] |
**asset** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**target_asset** | **String** | Target asset you want to convert | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1AssetConvertTransferPost200Response**](_sapi_v1_asset_convert_transfer_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_asset_convert_transfer_query_by_page_post

> crate::models::SapiV1AssetConvertTransferQueryByPagePost200Response sapi_v1_asset_convert_transfer_query_by_page_post(start_time, end_time, timestamp, signature, tran_id, asset, account_type, current, size, recv_window)
Query Convert Transfer (USER_DATA)

Weight(UID): 5

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_time** | **i64** | UTC timestamp in ms | [required] |
**end_time** | **i64** | UTC timestamp in ms | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**tran_id** | Option<**i64**> | The transaction id |  |
**asset** | Option<**String**> | If it is blank, we will match deducted asset and target asset. |  |
**account_type** | Option<**String**> | MAIN: main account. CARD: funding account. If it is blank, we will query spot and card wallet, otherwise, we just query the corresponding wallet |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1AssetConvertTransferQueryByPagePost200Response**](_sapi_v1_asset_convert_transfer_queryByPage_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_asset_dribblet_get

> crate::models::SapiV1AssetDribbletGet200Response sapi_v1_asset_dribblet_get(timestamp, signature, start_time, end_time, recv_window)
DustLog(USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1AssetDribbletGet200Response**](_sapi_v1_asset_dribblet_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_asset_dust_btc_post

> crate::models::SapiV1AssetDustBtcPost200Response sapi_v1_asset_dust_btc_post(timestamp, signature, recv_window)
Get Assets That Can Be Converted Into BNB (USER_DATA)

Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1AssetDustBtcPost200Response**](_sapi_v1_asset_dust_btc_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_asset_dust_post

> crate::models::SapiV1AssetDustPost200Response sapi_v1_asset_dust_post(asset, timestamp, signature, recv_window)
Dust Transfer (USER_DATA)

Convert dust assets to BNB.  Weight(UID): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | [**Vec<String>**](String.md) | The asset being converted. For example, asset=BTC&asset=USDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1AssetDustPost200Response**](_sapi_v1_asset_dust_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_asset_get_funding_asset_post

> Vec<crate::models::SapiV1AssetGetFundingAssetPost200ResponseInner> sapi_v1_asset_get_funding_asset_post(timestamp, signature, asset, need_btc_valuation, recv_window)
Funding Wallet (USER_DATA)

- Currently supports querying the following business assets???Binance Pay, Binance Card, Binance Gift Card, Stock Token  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**need_btc_valuation** | Option<**String**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1AssetGetFundingAssetPost200ResponseInner>**](_sapi_v1_asset_get_funding_asset_post_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_asset_trade_fee_get

> Vec<crate::models::SapiV1AssetTradeFeeGet200ResponseInner> sapi_v1_asset_trade_fee_get(timestamp, signature, symbol, recv_window)
Trade Fee (USER_DATA)

Fetch trade fee  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**symbol** | Option<**String**> | Trading symbol, e.g. BNBUSDT |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1AssetTradeFeeGet200ResponseInner>**](_sapi_v1_asset_tradeFee_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_asset_transfer_get

> crate::models::SapiV1AssetTransferGet200Response sapi_v1_asset_transfer_get(r#type, timestamp, signature, start_time, end_time, current, size, from_symbol, to_symbol, recv_window)
Query User Universal Transfer History (USER_DATA)

- `fromSymbol` must be sent when type are ISOLATEDMARGIN_MARGIN and ISOLATEDMARGIN_ISOLATEDMARGIN - `toSymbol` must be sent when type are MARGIN_ISOLATEDMARGIN and ISOLATEDMARGIN_ISOLATEDMARGIN - Support query within the last 6 months only - If `startTime` and `endTime` not sent, return records of the last 7 days by default  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | Universal transfer type | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**current** | Option<**i32**> | Current querying page. Start from 1. Default:1 |  |
**size** | Option<**i32**> | Default:10 Max:100 |  |
**from_symbol** | Option<**String**> | Must be sent when type are ISOLATEDMARGIN_MARGIN and ISOLATEDMARGIN_ISOLATEDMARGIN |  |
**to_symbol** | Option<**String**> | Must be sent when type are MARGIN_ISOLATEDMARGIN and ISOLATEDMARGIN_ISOLATEDMARGIN |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1AssetTransferGet200Response**](_sapi_v1_asset_transfer_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_asset_transfer_post

> crate::models::SapiV1AssetTransferGet200Response1 sapi_v1_asset_transfer_post(r#type, asset, amount, timestamp, signature, from_symbol, to_symbol, recv_window)
User Universal Transfer (USER_DATA)

You need to enable `Permits Universal Transfer` option for the api key which requests this endpoint.  - `fromSymbol` must be sent when type are ISOLATEDMARGIN_MARGIN and ISOLATEDMARGIN_ISOLATEDMARGIN - `toSymbol` must be sent when type are MARGIN_ISOLATEDMARGIN and ISOLATEDMARGIN_ISOLATEDMARGIN  ENUM of transfer types: - MAIN_UMFUTURE Spot account transfer to USD???-M Futures account - MAIN_CMFUTURE Spot account transfer to COIN-M Futures account - MAIN_MARGIN Spot account transfer to Margin???cross???account - UMFUTURE_MAIN USD???-M Futures account transfer to Spot account - UMFUTURE_MARGIN USD???-M Futures account transfer to Margin???cross???account - CMFUTURE_MAIN COIN-M Futures account transfer to Spot account - CMFUTURE_MARGIN COIN-M Futures account transfer to Margin(cross) account - MARGIN_MAIN Margin???cross???account transfer to Spot account - MARGIN_UMFUTURE Margin???cross???account transfer to USD???-M Futures - MARGIN_CMFUTURE Margin???cross???account transfer to COIN-M Futures - ISOLATEDMARGIN_MARGIN Isolated margin account transfer to Margin(cross) account - MARGIN_ISOLATEDMARGIN Margin(cross) account transfer to Isolated margin account - ISOLATEDMARGIN_ISOLATEDMARGIN Isolated margin account transfer to Isolated margin account - MAIN_FUNDING Spot account transfer to Funding account - FUNDING_MAIN Funding account transfer to Spot account - FUNDING_UMFUTURE Funding account transfer to UMFUTURE account - UMFUTURE_FUNDING UMFUTURE account transfer to Funding account - MARGIN_FUNDING MARGIN account transfer to Funding account - FUNDING_MARGIN Funding account transfer to Margin account - FUNDING_CMFUTURE Funding account transfer to CMFUTURE account - CMFUTURE_FUNDING CMFUTURE account transfer to Funding account  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | Universal transfer type | [required] |
**asset** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**from_symbol** | Option<**String**> | Must be sent when type are ISOLATEDMARGIN_MARGIN and ISOLATEDMARGIN_ISOLATEDMARGIN |  |
**to_symbol** | Option<**String**> | Must be sent when type are MARGIN_ISOLATEDMARGIN and ISOLATEDMARGIN_ISOLATEDMARGIN |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1AssetTransferGet200Response1**](_sapi_v1_asset_transfer_get_200_response_1.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_capital_config_getall_get

> Vec<crate::models::SapiV1CapitalConfigGetallGet200ResponseInner> sapi_v1_capital_config_getall_get(timestamp, signature, recv_window)
All Coins' Information (USER_DATA)

Get information of coins (available for deposit and withdraw) for user.  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1CapitalConfigGetallGet200ResponseInner>**](_sapi_v1_capital_config_getall_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_capital_deposit_address_get

> crate::models::SapiV1CapitalDepositAddressGet200Response sapi_v1_capital_deposit_address_get(coin, timestamp, signature, network, recv_window)
Deposit Address (supporting network) (USER_DATA)

Fetch deposit address with network.  - If network is not send, return with default network of the coin. - You can get network and isDefault in networkList in the response of Get /sapi/v1/capital/config/getall (HMAC SHA256).  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coin** | **String** | Coin name | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**network** | Option<**String**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1CapitalDepositAddressGet200Response**](_sapi_v1_capital_deposit_address_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_capital_deposit_hisrec_get

> Vec<crate::models::SapiV1CapitalDepositHisrecGet200ResponseInner> sapi_v1_capital_deposit_hisrec_get(timestamp, signature, coin, status, start_time, end_time, offset, limit, recv_window)
Deposit History???supporting network??? (USER_DATA)

Fetch deposit history.  - Please notice the default `startTime` and `endTime` to make sure that time interval is within 0-90 days. - If both `startTime` and `endTime` are sent, time between `startTime` and `endTime` must be less than 90 days.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**coin** | Option<**String**> | Coin name |  |
**status** | Option<**i32**> | * `0` - pending * `6` - credited but cannot withdraw * `1` - success |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**offset** | Option<**i32**> |  |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1CapitalDepositHisrecGet200ResponseInner>**](_sapi_v1_capital_deposit_hisrec_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_capital_withdraw_apply_post

> crate::models::SapiV1CapitalWithdrawApplyPost200Response sapi_v1_capital_withdraw_apply_post(coin, address, amount, timestamp, signature, withdraw_order_id, network, address_tag, transaction_fee_flag, name, wallet_type, recv_window)
Withdraw (USER_DATA)

Submit a withdraw request.  - If `network` not send, return with default network of the coin. - You can get `network` and `isDefault` in `networkList` of a coin in the response of `Get /sapi/v1/capital/config/getall (HMAC SHA256)`.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coin** | **String** | Coin name | [required] |
**address** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**withdraw_order_id** | Option<**String**> | Client id for withdraw |  |
**network** | Option<**String**> | Get the value from `GET /sapi/v1/capital/config/getall` |  |
**address_tag** | Option<**String**> | Secondary address identifier for coins like XRP,XMR etc. |  |
**transaction_fee_flag** | Option<**bool**> | When making internal transfer - `true` ->  returning the fee to the destination account; - `false` -> returning the fee back to the departure account. |  |[default to false]
**name** | Option<**String**> |  |  |
**wallet_type** | Option<**i32**> | The wallet type for withdraw???0-Spot wallet, 1- Funding wallet. Default is Spot wallet |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::SapiV1CapitalWithdrawApplyPost200Response**](_sapi_v1_capital_withdraw_apply_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_capital_withdraw_history_get

> Vec<crate::models::SapiV1CapitalWithdrawHistoryGet200ResponseInner> sapi_v1_capital_withdraw_history_get(timestamp, signature, coin, withdraw_order_id, status, start_time, end_time, offset, limit, recv_window)
Withdraw History (supporting network) (USER_DATA)

Fetch withdraw history.  - `network` may not be in the response for old withdraw. - Please notice the default `startTime` and `endTime` to make sure that time interval is within 0-90 days. - If both `startTime` and `endTime` are sent, time between `startTime` and `endTime` must be less than 90 days  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**coin** | Option<**String**> | Coin name |  |
**withdraw_order_id** | Option<**String**> |  |  |
**status** | Option<**i32**> | * `0` - Email Sent * `1` - Cancelled * `2` - Awaiting Approval * `3` - Rejected * `4` - Processing * `5` - Failure * `6` - Completed |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**offset** | Option<**i32**> |  |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV1CapitalWithdrawHistoryGet200ResponseInner>**](_sapi_v1_capital_withdraw_history_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_system_status_get

> crate::models::SapiV1SystemStatusGet200Response sapi_v1_system_status_get()
System Status (System)

Fetch system status.  Weight(IP): 1

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SapiV1SystemStatusGet200Response**](_sapi_v1_system_status_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v3_asset_get_user_asset_post

> Vec<crate::models::SapiV3AssetGetUserAssetPost200ResponseInner> sapi_v3_asset_get_user_asset_post(timestamp, signature, asset, need_btc_valuation, recv_window)
User Asset (USER_DATA)

Get user assets, just for positive data.  Weight(IP): 5

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**asset** | Option<**String**> |  |  |
**need_btc_valuation** | Option<**String**> |  |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::SapiV3AssetGetUserAssetPost200ResponseInner>**](_sapi_v3_asset_getUserAsset_post_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

