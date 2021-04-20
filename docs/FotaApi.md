# \FotaApi

All URIs are relative to *https://api.lab5e.com/span*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clear_firmware_error**](FotaApi.md#clear_firmware_error) | **delete** /collections/{collectionId}/devices/{deviceId}/fwerror | Clear FOTA error
[**create_firmware**](FotaApi.md#create_firmware) | **post** /collections/{collectionId}/firmware | Create firmware
[**delete_firmware**](FotaApi.md#delete_firmware) | **delete** /collections/{collectionId}/firmware/{imageId} | Delete firmware
[**firmware_usage**](FotaApi.md#firmware_usage) | **patch** /collections/{collectionId}/firmware/{imageId}/usage | Firmware usage
[**list_firmware**](FotaApi.md#list_firmware) | **get** /collections/{collectionId}/firmware | List firmware
[**retrieve_firmware**](FotaApi.md#retrieve_firmware) | **get** /collections/{collectionId}/firmware/{imageId} | Retrieve firmware
[**update_firmware**](FotaApi.md#update_firmware) | **patch** /collections/{collectionId}/firmware/{imageId} | Update firmware



## clear_firmware_error

> crate::models::ClearFirmwareErrorResponse clear_firmware_error(collection_id, device_id)
Clear FOTA error

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**device_id** | **String** |  | [required] |

### Return type

[**crate::models::ClearFirmwareErrorResponse**](ClearFirmwareErrorResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_firmware

> crate::models::Firmware create_firmware(collection_id, body)
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


## delete_firmware

> crate::models::Firmware delete_firmware(collection_id, image_id)
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


## firmware_usage

> crate::models::FirmwareUsageResponse firmware_usage(collection_id, image_id)
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


## list_firmware

> crate::models::ListFirmwareResponse list_firmware(collection_id)
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


## retrieve_firmware

> crate::models::Firmware retrieve_firmware(collection_id, image_id)
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


## update_firmware

> crate::models::Firmware update_firmware(collection_id, image_id, body)
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

