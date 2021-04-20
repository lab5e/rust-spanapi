# \DevicesApi

All URIs are relative to *https://api.lab5e.com/span*

Method | HTTP request | Description
------------- | ------------- | -------------
[**span_create_device**](DevicesApi.md#span_create_device) | **post** /collections/{collectionId}/devices | Create device
[**span_delete_device**](DevicesApi.md#span_delete_device) | **delete** /collections/{collectionId}/devices/{deviceId} | Remove device
[**span_list_device_data**](DevicesApi.md#span_list_device_data) | **get** /collections/{collectionId}/devices/{deviceId}/data | Get payloads
[**span_list_devices**](DevicesApi.md#span_list_devices) | **get** /collections/{collectionId}/devices | List devices
[**span_retrieve_device**](DevicesApi.md#span_retrieve_device) | **get** /collections/{collectionId}/devices/{deviceId} | Retrieve device
[**span_send_message**](DevicesApi.md#span_send_message) | **post** /collections/{collectionId}/devices/{deviceId}/to | Send message
[**span_update_device**](DevicesApi.md#span_update_device) | **patch** /collections/{existingCollectionId}/devices/{deviceId} | Update device



## span_create_device

> crate::models::Device span_create_device(collection_id, body)
Create device

Create a new device. This will add a device to the collection. You must have write access to the collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | This is the containing collection | [required] |
**body** | [**Device**](Device.md) |  | [required] |

### Return type

[**crate::models::Device**](Device.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## span_delete_device

> crate::models::Device span_delete_device(collection_id, device_id)
Remove device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**device_id** | **String** |  | [required] |

### Return type

[**crate::models::Device**](Device.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## span_list_device_data

> crate::models::ListDataResponse span_list_device_data(collection_id, device_id, limit, start, end, offset)
Get payloads

List the data received from the device. Use the query parameters to control what data you retrieve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | The collection ID. This is included in the request path. | [required] |
**device_id** | **String** | The device ID. This is included in the request path. | [required] |
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


## span_list_devices

> crate::models::ListDevicesResponse span_list_devices(collection_id)
List devices

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |

### Return type

[**crate::models::ListDevicesResponse**](ListDevicesResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## span_retrieve_device

> crate::models::Device span_retrieve_device(collection_id, device_id)
Retrieve device

Retrieve a single device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**device_id** | **String** |  | [required] |

### Return type

[**crate::models::Device**](Device.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## span_send_message

> serde_json::Value span_send_message(collection_id, device_id, body)
Send message

Send a message to the device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**device_id** | **String** |  | [required] |
**body** | [**SendMessageRequest**](SendMessageRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## span_update_device

> crate::models::Device span_update_device(existing_collection_id, device_id, body)
Update device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**existing_collection_id** | **String** |  | [required] |
**device_id** | **String** |  | [required] |
**body** | [**UpdateDeviceRequest**](UpdateDeviceRequest.md) |  | [required] |

### Return type

[**crate::models::Device**](Device.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

