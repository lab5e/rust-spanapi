# \CollectionsApi

All URIs are relative to *https://api.lab5e.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_collection**](CollectionsApi.md#create_collection) | **POST** /span/collections | Create collection
[**delete_collection**](CollectionsApi.md#delete_collection) | **DELETE** /span/collections/{collectionId} | Delete collection
[**list_collection_data**](CollectionsApi.md#list_collection_data) | **GET** /span/collections/{collectionId}/data | Retrieve data from devices
[**list_collections**](CollectionsApi.md#list_collections) | **GET** /span/collections | List collections
[**retrieve_collection**](CollectionsApi.md#retrieve_collection) | **GET** /span/collections/{collectionId} | Retrieve collection
[**retrieve_collection_stats**](CollectionsApi.md#retrieve_collection_stats) | **GET** /span/collections/{collectionId}/stats | Retrieve collection statistics
[**update_collection**](CollectionsApi.md#update_collection) | **PATCH** /span/collections/{collectionId} | Update collection



## create_collection

> crate::models::Collection create_collection(body)
Create collection

Create a new collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateCollectionRequest**](CreateCollectionRequest.md) | Request object when creating a collection. The collect ID is assigned by the service. | [required] |

### Return type

[**crate::models::Collection**](Collection.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_collection

> crate::models::Collection delete_collection(collection_id)
Delete collection

Remove the collection. Devices, firmware images, outputs and all other related resources must be removed from the collection before it can be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | The ID of the collection you want to delete | [required] |

### Return type

[**crate::models::Collection**](Collection.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_collection_data

> crate::models::ListDataResponse list_collection_data(collection_id, limit, start, end, offset)
Retrieve data from devices

Retrieve data sent by the devices in the collection. The maximum number of data points is 100.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | The collection ID requested. This is included in the request path. | [required] |
**limit** | Option<**i32**> | Limit the number of payloads to return. The default is 512. |  |
**start** | Option<**String**> | Start of time range. The default is 24 hours ago. Value is in milliseconds since epoch. |  |
**end** | Option<**String**> | End of time range. The default is the current time stamp. Value is in milliseconds since epoch. |  |
**offset** | Option<**String**> | The message offset based on the message ID. This parameter can't be combined with the start and end parameters. If no parameter is set the first N messages will be returned. If this parameter is set the next N messages (from newest to oldest) with message ID less than the offset will be returned. |  |

### Return type

[**crate::models::ListDataResponse**](ListDataResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_collections

> crate::models::ListCollectionResponse list_collections()
List collections

Lists all the collections that one of your teams owns. The collections returned includes only the data on the collection and not the summary information

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListCollectionResponse**](ListCollectionResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_collection

> crate::models::Collection retrieve_collection(collection_id, upstream, downstream)
Retrieve collection

Retrieve collection information. This includes a list of the most recent messages in the inbox. The upstream and downstream parameters are optional and if set to true will include the timestamps for up to 100 messages up- and downstream for the last hour.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | The collection ID of the collection you are requesting | [required] |
**upstream** | Option<**bool**> |  |  |
**downstream** | Option<**bool**> |  |  |

### Return type

[**crate::models::Collection**](Collection.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_collection_stats

> crate::models::CollectionStats retrieve_collection_stats(collection_id)
Retrieve collection statistics

Retrieve statistics for the collection. This is the aggregated metrics for devices, outputs, firmware images, blobs and gateways in the collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | The collection ID of the collection you are requesting | [required] |

### Return type

[**crate::models::CollectionStats**](CollectionStats.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_collection

> crate::models::Collection update_collection(collection_id, body)
Update collection

Update a collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | The ID of the collection. This is assigned by the backend. | [required] |
**body** | [**UpdateCollectionRequest**](UpdateCollectionRequest.md) |  | [required] |

### Return type

[**crate::models::Collection**](Collection.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

