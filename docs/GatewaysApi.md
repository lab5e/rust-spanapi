# \GatewaysApi

All URIs are relative to *https://api.lab5e.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_gateway**](GatewaysApi.md#create_gateway) | **POST** /span/collections/{collectionId}/gateways | Create gateway
[**delete_gateway**](GatewaysApi.md#delete_gateway) | **DELETE** /span/collections/{collectionId}/gateways/{gatewayId} | Delete gateway
[**gateway_certificates**](GatewaysApi.md#gateway_certificates) | **GET** /span/collections/{collectionId}/gateways/{gatewayId}/certs | Get issued certificate(s) for gateway
[**list_gateways**](GatewaysApi.md#list_gateways) | **GET** /span/collections/{collectionId}/gateways | List gateways
[**retrieve_gateway**](GatewaysApi.md#retrieve_gateway) | **GET** /span/collections/{collectionId}/gateways/{gatewayId} | Retrieve gateway
[**retrieve_gateway_stats**](GatewaysApi.md#retrieve_gateway_stats) | **GET** /span/collections/{collectionId}/gateways/{gatewayId}/stats | Retrieve gateway statistics
[**update_gateway**](GatewaysApi.md#update_gateway) | **PATCH** /span/collections/{existingCollectionId}/gateways/{gatewayId} | Update gateway



## create_gateway

> crate::models::Gateway create_gateway(collection_id, body)
Create gateway

Create a new gateway.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**body** | [**CreateGatewayBody**](CreateGatewayBody.md) |  | [required] |

### Return type

[**crate::models::Gateway**](Gateway.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_gateway

> crate::models::Gateway delete_gateway(collection_id, gateway_id)
Delete gateway

Remove a gateway from Span.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**gateway_id** | **String** |  | [required] |

### Return type

[**crate::models::Gateway**](Gateway.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gateway_certificates

> crate::models::GatewayCertificateResponse gateway_certificates(collection_id, gateway_id)
Get issued certificate(s) for gateway

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**gateway_id** | **String** |  | [required] |

### Return type

[**crate::models::GatewayCertificateResponse**](GatewayCertificateResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gateways

> crate::models::ListGatewayResponse list_gateways(collection_id)
List gateways

List the user's gatways, including built-in gateways.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |

### Return type

[**crate::models::ListGatewayResponse**](ListGatewayResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_gateway

> crate::models::Gateway retrieve_gateway(collection_id, gateway_id)
Retrieve gateway

Get gateway information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**gateway_id** | **String** |  | [required] |

### Return type

[**crate::models::Gateway**](Gateway.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_gateway_stats

> crate::models::GatewayStats retrieve_gateway_stats(collection_id, gateway_id)
Retrieve gateway statistics

Get statistics for gateway

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**gateway_id** | **String** |  | [required] |

### Return type

[**crate::models::GatewayStats**](GatewayStats.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_gateway

> crate::models::Gateway update_gateway(existing_collection_id, gateway_id, body)
Update gateway

Update configuration for a gateway. If you want to remove or move a gateway from the collection the devices in the collection must not have any gateway configuration. The certificates for the gateway are unchanged when the gateway is moved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**existing_collection_id** | **String** |  | [required] |
**gateway_id** | **String** |  | [required] |
**body** | [**UpdateGatewayBody**](UpdateGatewayBody.md) |  | [required] |

### Return type

[**crate::models::Gateway**](Gateway.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

