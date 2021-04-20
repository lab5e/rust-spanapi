# \FotaApi

All URIs are relative to *https://api.lab5e.com/span*

Method | HTTP request | Description
------------- | ------------- | -------------
[**span_clear_firmware_error**](FotaApi.md#span_clear_firmware_error) | **delete** /collections/{collectionId}/devices/{deviceId}/fwerror | Clear FOTA error
[**span_create_firmware**](FotaApi.md#span_create_firmware) | **post** /collections/{collectionId}/firmware | Create firmware
[**span_delete_firmware**](FotaApi.md#span_delete_firmware) | **delete** /collections/{collectionId}/firmware/{imageId} | Delete firmware
[**span_firmware_usage**](FotaApi.md#span_firmware_usage) | **patch** /collections/{collectionId}/firmware/{imageId}/usage | Firmware usage
[**span_list_firmware**](FotaApi.md#span_list_firmware) | **get** /collections/{collectionId}/firmware | List firmware
[**span_retrieve_firmware**](FotaApi.md#span_retrieve_firmware) | **get** /collections/{collectionId}/firmware/{imageId} | Retrieve firmware
[**span_update_firmware**](FotaApi.md#span_update_firmware) | **patch** /collections/{collectionId}/firmware/{imageId} | Update firmware



## span_clear_firmware_error

> serde_json::Value span_clear_firmware_error(collection_id, device_id)
Clear FOTA error

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**device_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## span_create_firmware

> crate::models::Firmware span_create_firmware(collection_id, body)
Create firmware

Create a new firmware image. This is also invoked by the custom HTTP uploader if the POST uses multipart/formdata for the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**body** | [**CreateFirmwareRequest**](CreateFirmwareRequest.md) |  | [required] |

### Return type

[**crate::models::Firmware**](Firmware.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## span_delete_firmware

> crate::models::Firmware span_delete_firmware(collection_id, image_id)
Delete firmware

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**image_id** | **String** |  | [required] |

### Return type

[**crate::models::Firmware**](Firmware.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## span_firmware_usage

> crate::models::FirmwareUsageResponse span_firmware_usage(collection_id, image_id)
Firmware usage

Shows the firmware usage for devices in the collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**image_id** | **String** |  | [required] |

### Return type

[**crate::models::FirmwareUsageResponse**](FirmwareUsageResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## span_list_firmware

> crate::models::ListFirmwareResponse span_list_firmware(collection_id)
List firmware

Lists available firmware images in collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |

### Return type

[**crate::models::ListFirmwareResponse**](ListFirmwareResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## span_retrieve_firmware

> crate::models::Firmware span_retrieve_firmware(collection_id, image_id)
Retrieve firmware

Retrieve information on a firmware image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**image_id** | **String** |  | [required] |

### Return type

[**crate::models::Firmware**](Firmware.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## span_update_firmware

> crate::models::Firmware span_update_firmware(collection_id, image_id, body)
Update firmware

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | Collection ID  Collection ID for the collection owning the firmware image. | [required] |
**image_id** | **String** | Firmware image ID | [required] |
**body** | [**Firmware**](Firmware.md) |  | [required] |

### Return type

[**crate::models::Firmware**](Firmware.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

