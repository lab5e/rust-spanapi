# \DevicesApi

All URIs are relative to *https://api.lab5e.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_downstream_message**](DevicesApi.md#add_downstream_message) | **POST** /span/collections/{collectionId}/devices/{deviceId}/outbox | Add message to oubox
[**create_device**](DevicesApi.md#create_device) | **POST** /span/collections/{collectionId}/devices | Create device
[**delete_device**](DevicesApi.md#delete_device) | **DELETE** /span/collections/{collectionId}/devices/{deviceId} | Remove device.
[**delete_downstream_message**](DevicesApi.md#delete_downstream_message) | **DELETE** /span/collections/{collectionId}/devices/{deviceId}/outbox/{messageId} | Delete outgoing message
[**device_certificate**](DevicesApi.md#device_certificate) | **GET** /span/collections/{collectionId}/devices/{deviceId}/certs | Get issued certificate(s) for device
[**list_device_data**](DevicesApi.md#list_device_data) | **GET** /span/collections/{collectionId}/devices/{deviceId}/data | Retrieve data from device
[**list_devices**](DevicesApi.md#list_devices) | **GET** /span/collections/{collectionId}/devices | List devices in collection.
[**list_downstream_messages**](DevicesApi.md#list_downstream_messages) | **GET** /span/collections/{collectionId}/devices/{deviceId}/outbox | List the messages in the outbox
[**list_upstream_messages**](DevicesApi.md#list_upstream_messages) | **GET** /span/collections/{collectionId}/devices/{deviceId}/inbox | List incoming messages
[**retrieve_device**](DevicesApi.md#retrieve_device) | **GET** /span/collections/{collectionId}/devices/{deviceId} | Retrieve device
[**retrieve_device_stats**](DevicesApi.md#retrieve_device_stats) | **GET** /span/collections/{collectionId}/devices/{deviceId}/stats | Retrieve device statistics
[**update_device**](DevicesApi.md#update_device) | **PATCH** /span/collections/{existingCollectionId}/devices/{deviceId} | Update device



## add_downstream_message

> crate::models::MessageDownstream add_downstream_message(collection_id, device_id, body)
Add message to oubox

Add a new message in the outgoing queue to the device. If there is other messages in the outbox these messages will be sent first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**device_id** | **String** |  | [required] |
**body** | [**AddDownstreamMessageRequest**](AddDownstreamMessageRequest.md) |  | [required] |

### Return type

[**crate::models::MessageDownstream**](MessageDownstream.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_device

> crate::models::Device create_device(collection_id, body)
Create device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | This is the containing collection | [required] |
**body** | [**CreateDeviceRequest**](CreateDeviceRequest.md) |  | [required] |

### Return type

[**crate::models::Device**](Device.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_device

> crate::models::Device delete_device(collection_id, device_id)
Remove device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | This is the containing collection | [required] |
**device_id** | **String** | The device ID is assigned by the backend. | [required] |

### Return type

[**crate::models::Device**](Device.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_downstream_message

> crate::models::DeleteDownstreamMessageResponse delete_downstream_message(collection_id, device_id, message_id)
Delete outgoing message

Delete an outgoing (ie downstream) message from the outbox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**device_id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |

### Return type

[**crate::models::DeleteDownstreamMessageResponse**](DeleteDownstreamMessageResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## device_certificate

> crate::models::DeviceCertificateResponse device_certificate(collection_id, device_id)
Get issued certificate(s) for device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**device_id** | **String** |  | [required] |

### Return type

[**crate::models::DeviceCertificateResponse**](DeviceCertificateResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_device_data

> crate::models::ListDataResponse list_device_data(collection_id, device_id, limit, start, end, offset)
Retrieve data from device

List the data received from the device. Use the query parameters to control what data you retrieve. The maximum number of data points is 100.

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


## list_devices

> crate::models::ListDevicesResponse list_devices(collection_id)
List devices in collection.

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


## list_downstream_messages

> crate::models::ListDownstreamMessagesResponse list_downstream_messages(collection_id, device_id, limit, start, end, offset)
List the messages in the outbox

List messages that should be sent to the device when it connects to the service. The messages are sent to the device when it connects to the service and either sends a message (via UDP or CoAP) or requests a message via CoAP GET request.option

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**device_id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |
**start** | Option<**String**> | Start of time range. The default is 24 hours ago. Value is in milliseconds since epoch. |  |
**end** | Option<**String**> | End of time range. The default is the current time stamp. Value is in milliseconds since epoch. |  |
**offset** | Option<**String**> | The message offset based on the message ID. This parameter can't be combined with the start and end parameters. If no parameter is set the first N messages will be returned. If this parameter is set the next N messages (from newest to oldest) with message ID less than the offset will be returned. |  |

### Return type

[**crate::models::ListDownstreamMessagesResponse**](ListDownstreamMessagesResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_upstream_messages

> crate::models::ListUpstreamMessagesResponse list_upstream_messages(collection_id, device_id, limit, start, end, offset)
List incoming messages

Retrieve a list of incoming (ie upstream) messages, ie messages sent from the device to the service. These messages are buffered in the service until they expire.  Use the query parameters to limit the number of messages to return. If no limit is specified the default limit of 250 is used.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**device_id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |
**start** | Option<**String**> | Start of time range. The default is 24 hours ago. Value is in milliseconds since epoch. |  |
**end** | Option<**String**> | End of time range. The default is the current time stamp. Value is in milliseconds since epoch. |  |
**offset** | Option<**String**> | The message offset based on the message ID. This parameter can't be combined with the start and end parameters. If no parameter is set the first N messages will be returned. If this parameter is set the next N messages (from newest to oldest) with message ID less than the offset will be returned. |  |

### Return type

[**crate::models::ListUpstreamMessagesResponse**](ListUpstreamMessagesResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_device

> crate::models::Device retrieve_device(collection_id, device_id)
Retrieve device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | This is the containing collection | [required] |
**device_id** | **String** | The device identifier | [required] |

### Return type

[**crate::models::Device**](Device.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_device_stats

> crate::models::DeviceStats retrieve_device_stats(collection_id, device_id)
Retrieve device statistics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | This is the containing collection | [required] |
**device_id** | **String** | The device identifier | [required] |

### Return type

[**crate::models::DeviceStats**](DeviceStats.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_device

> crate::models::Device update_device(existing_collection_id, device_id, body)
Update device

The device can be moved from one collection to another by setting the collection ID field to the new collection. You must have administrative access to both collections. A note on gateway configurations: Empty gateway configuration blocks are deleted. If the configuration block contains a gateway ID it will be updated  with the new values. All values must be submitted in the request. If a device is moved out of the collection and it references a gateway in the configuration the operation will fail. Devices that are moved from one collection to another and references gateway configurations must be updated before they are moved.

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

