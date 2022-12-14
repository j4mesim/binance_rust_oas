# \TradeApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v3_account_get**](TradeApi.md#api_v3_account_get) | **GET** /api/v3/account | Account Information (USER_DATA)
[**api_v3_all_order_list_get**](TradeApi.md#api_v3_all_order_list_get) | **GET** /api/v3/allOrderList | Query all OCO (USER_DATA)
[**api_v3_all_orders_get**](TradeApi.md#api_v3_all_orders_get) | **GET** /api/v3/allOrders | All Orders (USER_DATA)
[**api_v3_my_trades_get**](TradeApi.md#api_v3_my_trades_get) | **GET** /api/v3/myTrades | Account Trade List (USER_DATA)
[**api_v3_open_order_list_get**](TradeApi.md#api_v3_open_order_list_get) | **GET** /api/v3/openOrderList | Query Open OCO (USER_DATA)
[**api_v3_open_orders_delete**](TradeApi.md#api_v3_open_orders_delete) | **DELETE** /api/v3/openOrders | Cancel all Open Orders on a Symbol (TRADE)
[**api_v3_open_orders_get**](TradeApi.md#api_v3_open_orders_get) | **GET** /api/v3/openOrders | Current Open Orders (USER_DATA)
[**api_v3_order_cancel_replace_post**](TradeApi.md#api_v3_order_cancel_replace_post) | **POST** /api/v3/order/cancelReplace | Cancel an Existing Order and Send a New Order (Trade)
[**api_v3_order_delete**](TradeApi.md#api_v3_order_delete) | **DELETE** /api/v3/order | Cancel Order (TRADE)
[**api_v3_order_get**](TradeApi.md#api_v3_order_get) | **GET** /api/v3/order | Query Order (USER_DATA)
[**api_v3_order_list_delete**](TradeApi.md#api_v3_order_list_delete) | **DELETE** /api/v3/orderList | Cancel OCO (TRADE)
[**api_v3_order_list_get**](TradeApi.md#api_v3_order_list_get) | **GET** /api/v3/orderList | Query OCO (USER_DATA)
[**api_v3_order_oco_post**](TradeApi.md#api_v3_order_oco_post) | **POST** /api/v3/order/oco | New OCO (TRADE)
[**api_v3_order_post**](TradeApi.md#api_v3_order_post) | **POST** /api/v3/order | New Order (TRADE)
[**api_v3_order_test_post**](TradeApi.md#api_v3_order_test_post) | **POST** /api/v3/order/test | Test New Order (TRADE)
[**api_v3_rate_limit_order_get**](TradeApi.md#api_v3_rate_limit_order_get) | **GET** /api/v3/rateLimit/order | Query Current Order Count Usage (TRADE)



## api_v3_account_get

> crate::models::Account api_v3_account_get(timestamp, signature, recv_window)
Account Information (USER_DATA)

Get current account information.  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::Account**](account.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_all_order_list_get

> Vec<crate::models::ApiV3AllOrderListGet200ResponseInner> api_v3_all_order_list_get(timestamp, signature, from_id, start_time, end_time, limit, recv_window)
Query all OCO (USER_DATA)

Retrieves all OCO based on provided optional parameters  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**from_id** | Option<**i64**> | Trade id to fetch from. Default gets most recent trades. |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::ApiV3AllOrderListGet200ResponseInner>**](_api_v3_allOrderList_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_all_orders_get

> Vec<crate::models::OrderDetails> api_v3_all_orders_get(symbol, timestamp, signature, order_id, start_time, end_time, limit, recv_window)
All Orders (USER_DATA)

Get all account orders; active, canceled, or filled..  - If `orderId` is set, it will get orders >= that `orderId`. Otherwise most recent orders are returned. - For some historical orders `cummulativeQuoteQty` will be < 0, meaning the data is not available at this time. - If `startTime` and/or `endTime` provided, `orderId` is not required  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_id** | Option<**i64**> | Order id |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::OrderDetails>**](orderDetails.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_my_trades_get

> Vec<crate::models::MyTrade> api_v3_my_trades_get(symbol, timestamp, signature, order_id, start_time, end_time, from_id, limit, recv_window)
Account Trade List (USER_DATA)

Get trades for a specific account and symbol.  If `fromId` is set, it will get id >= that `fromId`. Otherwise most recent orders are returned.  Weight(IP): 10

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_id** | Option<**i64**> | This can only be used in combination with symbol. |  |
**start_time** | Option<**i64**> | UTC timestamp in ms |  |
**end_time** | Option<**i64**> | UTC timestamp in ms |  |
**from_id** | Option<**i64**> | Trade id to fetch from. Default gets most recent trades. |  |
**limit** | Option<**i32**> | Default 500; max 1000. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::MyTrade>**](myTrade.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_open_order_list_get

> Vec<crate::models::ApiV3OpenOrderListGet200ResponseInner> api_v3_open_order_list_get(timestamp, signature, recv_window)
Query Open OCO (USER_DATA)

Weight(IP): 3

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::ApiV3OpenOrderListGet200ResponseInner>**](_api_v3_openOrderList_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_open_orders_delete

> Vec<crate::models::ApiV3OpenOrdersDelete200ResponseInner> api_v3_open_orders_delete(symbol, timestamp, signature, recv_window)
Cancel all Open Orders on a Symbol (TRADE)

Cancels all active orders on a symbol. This includes OCO orders.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::ApiV3OpenOrdersDelete200ResponseInner>**](_api_v3_openOrders_delete_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_open_orders_get

> Vec<crate::models::OrderDetails> api_v3_open_orders_get(timestamp, signature, symbol, recv_window)
Current Open Orders (USER_DATA)

Get all open orders on a symbol. Careful when accessing this with no symbol.  Weight(IP): - `3` for a single symbol; - `40` when the symbol parameter is omitted;

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**symbol** | Option<**String**> | Trading symbol, e.g. BNBUSDT |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::OrderDetails>**](orderDetails.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_order_cancel_replace_post

> crate::models::ApiV3OrderCancelReplacePost200Response api_v3_order_cancel_replace_post(symbol, side, r#type, cancel_replace_mode, timestamp, signature, time_in_force, quantity, quote_order_qty, price, cancel_new_client_order_id, cancel_orig_client_order_id, cancel_order_id, new_client_order_id, strategy_id, strategy_type, stop_price, trailing_delta, iceberg_qty, new_order_resp_type, recv_window)
Cancel an Existing Order and Send a New Order (Trade)

Cancels an existing order and places a new order on the same symbol.  Filters are evaluated before the cancel order is placed.  If the new order placement is successfully sent to the engine, the order count will increase by 1.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**side** | **String** |  | [required] |
**r#type** | **String** | Order type | [required] |
**cancel_replace_mode** | **String** | - `STOP_ON_FAILURE` If the cancel request fails, the new order placement will not be attempted. - `ALLOW_FAILURES` If new order placement will be attempted even if cancel request fails. | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**time_in_force** | Option<**String**> | Order time in force |  |
**quantity** | Option<**f64**> | Order quantity |  |
**quote_order_qty** | Option<**f64**> | Quote quantity |  |
**price** | Option<**f64**> | Order price |  |
**cancel_new_client_order_id** | Option<**String**> | Used to uniquely identify this cancel. Automatically generated by default |  |
**cancel_orig_client_order_id** | Option<**String**> | Either the cancelOrigClientOrderId or cancelOrderId must be provided. If both are provided, cancelOrderId takes precedence. |  |
**cancel_order_id** | Option<**i64**> | Either the cancelOrigClientOrderId or cancelOrderId must be provided. If both are provided, cancelOrderId takes precedence. |  |
**new_client_order_id** | Option<**String**> | Used to uniquely identify this cancel. Automatically generated by default |  |
**strategy_id** | Option<**i64**> |  |  |
**strategy_type** | Option<**i64**> | The value cannot be less than 1000000. |  |
**stop_price** | Option<**f64**> | Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders. |  |
**trailing_delta** | Option<**f64**> | Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders. |  |
**iceberg_qty** | Option<**f64**> | Used with LIMIT, STOP_LOSS_LIMIT, and TAKE_PROFIT_LIMIT to create an iceberg order. |  |
**new_order_resp_type** | Option<**String**> | Set the response JSON. MARKET and LIMIT order types default to FULL, all other orders default to ACK. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::ApiV3OrderCancelReplacePost200Response**](_api_v3_order_cancelReplace_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_order_delete

> crate::models::Order api_v3_order_delete(symbol, timestamp, signature, order_id, orig_client_order_id, new_client_order_id, recv_window)
Cancel Order (TRADE)

Cancel an active order.  Either `orderId` or `origClientOrderId` must be sent.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_id** | Option<**i64**> | Order id |  |
**orig_client_order_id** | Option<**String**> | Order id from client |  |
**new_client_order_id** | Option<**String**> | Used to uniquely identify this cancel. Automatically generated by default |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::Order**](order.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_order_get

> crate::models::OrderDetails api_v3_order_get(symbol, timestamp, signature, order_id, orig_client_order_id, recv_window)
Query Order (USER_DATA)

Check an order's status.  - Either `orderId` or `origClientOrderId` must be sent. - For some historical orders `cummulativeQuoteQty` will be < 0, meaning the data is not available at this time.  Weight(IP): 2

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_id** | Option<**i64**> | Order id |  |
**orig_client_order_id** | Option<**String**> | Order id from client |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::OrderDetails**](orderDetails.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_order_list_delete

> crate::models::OcoOrder api_v3_order_list_delete(symbol, timestamp, signature, order_list_id, list_client_order_id, new_client_order_id, recv_window)
Cancel OCO (TRADE)

Cancel an entire Order List  Canceling an individual leg will cancel the entire OCO  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_list_id** | Option<**i64**> | Order list id |  |
**list_client_order_id** | Option<**String**> | A unique Id for the entire orderList |  |
**new_client_order_id** | Option<**String**> | Used to uniquely identify this cancel. Automatically generated by default |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::OcoOrder**](ocoOrder.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_order_list_get

> crate::models::ApiV3OrderListDelete200Response api_v3_order_list_get(timestamp, signature, order_list_id, orig_client_order_id, recv_window)
Query OCO (USER_DATA)

Retrieves a specific OCO based on provided optional parameters  Weight(IP): 2

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**order_list_id** | Option<**i64**> | Order list id |  |
**orig_client_order_id** | Option<**String**> | Order id from client |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::ApiV3OrderListDelete200Response**](_api_v3_orderList_delete_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_order_oco_post

> crate::models::ApiV3OrderOcoPost200Response api_v3_order_oco_post(symbol, side, quantity, price, stop_price, timestamp, signature, list_client_order_id, limit_client_order_id, limit_strategy_id, limit_strategy_type, limit_iceberg_qty, trailing_delta, stop_client_order_id, stop_strategy_id, stop_strategy_type, stop_limit_price, stop_iceberg_qty, stop_limit_time_in_force, new_order_resp_type, recv_window)
New OCO (TRADE)

Send in a new OCO  - Price Restrictions:   - `SELL`: Limit Price > Last Price > Stop Price   - `BUY`: Limit Price < Last Price < Stop Price - Quantity Restrictions:     - Both legs must have the same quantity     - `ICEBERG` quantities however do not have to be the same - Order Rate Limit     - `OCO` counts as 2 orders against the order rate limit.      Weight(IP): 1

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
**list_client_order_id** | Option<**String**> | A unique Id for the entire orderList |  |
**limit_client_order_id** | Option<**String**> | A unique Id for the limit order |  |
**limit_strategy_id** | Option<**i64**> |  |  |
**limit_strategy_type** | Option<**i64**> | The value cannot be less than 1000000. |  |
**limit_iceberg_qty** | Option<**f64**> |  |  |
**trailing_delta** | Option<**f64**> |  |  |
**stop_client_order_id** | Option<**String**> | A unique Id for the stop loss/stop loss limit leg |  |
**stop_strategy_id** | Option<**i64**> |  |  |
**stop_strategy_type** | Option<**i64**> |  |  |
**stop_limit_price** | Option<**f64**> | If provided, stopLimitTimeInForce is required. |  |
**stop_iceberg_qty** | Option<**f64**> |  |  |
**stop_limit_time_in_force** | Option<**String**> |  |  |
**new_order_resp_type** | Option<**String**> | Set the response JSON. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::ApiV3OrderOcoPost200Response**](_api_v3_order_oco_post_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_order_post

> crate::models::ApiV3OrderDelete200Response api_v3_order_post(symbol, side, r#type, timestamp, signature, time_in_force, quantity, quote_order_qty, price, new_client_order_id, strategy_id, strategy_type, stop_price, trailing_delta, iceberg_qty, new_order_resp_type, recv_window)
New Order (TRADE)

Send in a new order.  - `LIMIT_MAKER` are `LIMIT` orders that will be rejected if they would immediately match and trade as a taker. - `STOP_LOSS` and `TAKE_PROFIT` will execute a `MARKET` order when the `stopPrice` is reached. - Any `LIMIT` or `LIMIT_MAKER` type order can be made an iceberg order by sending an `icebergQty`. - Any order with an `icebergQty` MUST have `timeInForce` set to `GTC`. - `MARKET` orders using `quantity` specifies how much a user wants to buy or sell based on the market price. - `MARKET` orders using `quoteOrderQty` specifies the amount the user wants to spend (when buying) or receive (when selling) of the quote asset; the correct quantity will be determined based on the market liquidity and `quoteOrderQty`. - `MARKET` orders using `quoteOrderQty` will not break `LOT_SIZE` filter rules; the order will execute a quantity that will have the notional value as close as possible to `quoteOrderQty`. - same `newClientOrderId` can be accepted only when the previous one is filled, otherwise the order will be rejected.  Trigger order price rules against market price for both `MARKET` and `LIMIT` versions:  - Price above market price: `STOP_LOSS` `BUY`, `TAKE_PROFIT` `SELL` - Price below market price: `STOP_LOSS` `SELL`, `TAKE_PROFIT` `BUY`   Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**side** | **String** |  | [required] |
**r#type** | **String** | Order type | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**time_in_force** | Option<**String**> | Order time in force |  |
**quantity** | Option<**f64**> | Order quantity |  |
**quote_order_qty** | Option<**f64**> | Quote quantity |  |
**price** | Option<**f64**> | Order price |  |
**new_client_order_id** | Option<**String**> | Used to uniquely identify this cancel. Automatically generated by default |  |
**strategy_id** | Option<**i64**> |  |  |
**strategy_type** | Option<**i64**> | The value cannot be less than 1000000. |  |
**stop_price** | Option<**f64**> | Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders. |  |
**trailing_delta** | Option<**f64**> | Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders. |  |
**iceberg_qty** | Option<**f64**> | Used with LIMIT, STOP_LOSS_LIMIT, and TAKE_PROFIT_LIMIT to create an iceberg order. |  |
**new_order_resp_type** | Option<**String**> | Set the response JSON. MARKET and LIMIT order types default to FULL, all other orders default to ACK. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**crate::models::ApiV3OrderDelete200Response**](_api_v3_order_delete_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_order_test_post

> serde_json::Value api_v3_order_test_post(symbol, side, r#type, timestamp, signature, time_in_force, quantity, quote_order_qty, price, new_client_order_id, strategy_id, strategy_type, stop_price, trailing_delta, iceberg_qty, new_order_resp_type, recv_window)
Test New Order (TRADE)

Test new order creation and signature/recvWindow long. Creates and validates a new order but does not send it into the matching engine.  Weight(IP): 1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Trading symbol, e.g. BNBUSDT | [required] |
**side** | **String** |  | [required] |
**r#type** | **String** | Order type | [required] |
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**time_in_force** | Option<**String**> | Order time in force |  |
**quantity** | Option<**f64**> | Order quantity |  |
**quote_order_qty** | Option<**f64**> | Quote quantity |  |
**price** | Option<**f64**> | Order price |  |
**new_client_order_id** | Option<**String**> | Used to uniquely identify this cancel. Automatically generated by default |  |
**strategy_id** | Option<**i64**> |  |  |
**strategy_type** | Option<**i64**> | The value cannot be less than 1000000. |  |
**stop_price** | Option<**f64**> | Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders. |  |
**trailing_delta** | Option<**f64**> | Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders. |  |
**iceberg_qty** | Option<**f64**> | Used with LIMIT, STOP_LOSS_LIMIT, and TAKE_PROFIT_LIMIT to create an iceberg order. |  |
**new_order_resp_type** | Option<**String**> | Set the response JSON. MARKET and LIMIT order types default to FULL, all other orders default to ACK. |  |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v3_rate_limit_order_get

> Vec<crate::models::ApiV3RateLimitOrderGet200ResponseInner> api_v3_rate_limit_order_get(timestamp, signature, recv_window)
Query Current Order Count Usage (TRADE)

Displays the user's current order count usage for all intervals.  Weight(IP): 20

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**Vec<crate::models::ApiV3RateLimitOrderGet200ResponseInner>**](_api_v3_rateLimit_order_get_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

