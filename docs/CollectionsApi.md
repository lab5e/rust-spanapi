# \CollectionsApi

All URIs are relative to *https://api.lab5e.com/span*

Method | HTTP request | Description
------------- | ------------- | -------------
[**span_broadcast_message**](CollectionsApi.md#span_broadcast_message) | **post** /collections/{collectionId}/to | Broadcast message
[**span_create_collection**](CollectionsApi.md#span_create_collection) | **post** /collections | Create collection
[**span_delete_collection**](CollectionsApi.md#span_delete_collection) | **delete** /collections/{collectionId} | Delete collection
[**span_list_collection_data**](CollectionsApi.md#span_list_collection_data) | **get** /collections/{collectionId}/data | Get payloads
[**span_list_collections**](CollectionsApi.md#span_list_collections) | **get** /collections | List collections
[**span_retrieve_collection**](CollectionsApi.md#span_retrieve_collection) | **get** /collections/{collectionId} | Retrieve collection
[**span_update_collection**](CollectionsApi.md#span_update_collection) | **patch** /collections/{collectionId} | Update collection



## span_broadcast_message

> crate::models::MultiSendMessageResponse span_broadcast_message(collection_id, body)
Broadcast message

Broadcast a message to all devices in the collection. This request will always succeed if the collection exists, even if there are one or more send errors. Individual errors are returned as an array of error messages in the response. Use equivalent to resource for devices to send a message to single device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**body** | [**BroadcastMessageRequest**](BroadcastMessageRequest.md) |  | [required] |

### Return type

[**crate::models::MultiSendMessageResponse**](MultiSendMessageResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## span_create_collection

> crate::models::Collection span_create_collection(body)
Create collection

The returned collection is the collection stored in the backend. Defaults have been set. There are no required fields in a collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Collection**](Collection.md) |  | [required] |

### Return type

[**crate::models::Collection**](Collection.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## span_delete_collection

> crate::models::Collection span_delete_collection(collection_id)
Delete collection

You must have write access to the collection

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


## span_list_collection_data

> crate::models::ListDataResponse span_list_collection_data(collection_id, limit, start, end, offset)
Get payloads

List the data received from all the devices in the collection.

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


## span_list_collections

> crate::models::ListCollectionResponse span_list_collections()
List collections

Lists all the collections that one of your teams owns.

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


## span_retrieve_collection

> crate::models::Collection span_retrieve_collection(collection_id)
Retrieve collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | The collection ID of the collection you are requesting | [required] |

### Return type

[**crate::models::Collection**](Collection.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## span_update_collection

> crate::models::Collection span_update_collection(collection_id, body)
Update collection

You must have write access to the collection, ie. you must administer it

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | The ID of the collection. This is assigned by the backend. | [required] |
**body** | [**Collection**](Collection.md) |  | [required] |

### Return type

[**crate::models::Collection**](Collection.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

