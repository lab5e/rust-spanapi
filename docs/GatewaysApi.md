# \GatewaysApi

All URIs are relative to *https://api.lab5e.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_gateways**](GatewaysApi.md#list_gateways) | **GET** /span/networks/{networkId}/gateways | List gateways
[**list_networks**](GatewaysApi.md#list_networks) | **GET** /span/networks | List networks
[**retrieve_gateway**](GatewaysApi.md#retrieve_gateway) | **GET** /span/networks/{networkId}/gateways/{gatewayId} | Retrieve gateway



## list_gateways

> crate::models::ListGatewayResponse list_gateways(network_id)
List gateways

List the gatways for the network. Some of the gatways are built into Span and can't be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | **String** |  | [required] |

### Return type

[**crate::models::ListGatewayResponse**](ListGatewayResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_networks

> crate::models::ListNetworkResponse list_networks()
List networks

List networks available to the collection. This will include the built-in networks in Span.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListNetworkResponse**](ListNetworkResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_gateway

> crate::models::Gateway retrieve_gateway(network_id, gateway_id)
Retrieve gateway

Get gateway information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | **String** |  | [required] |
**gateway_id** | **String** |  | [required] |

### Return type

[**crate::models::Gateway**](Gateway.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

