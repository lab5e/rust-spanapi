# \FotaApi

All URIs are relative to *https://api.lab5e.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clear_firmware_error**](FotaApi.md#clear_firmware_error) | **DELETE** /span/collections/{collectionId}/devices/{deviceId}/fwerror | Clear FOTA error
[**create_firmware**](FotaApi.md#create_firmware) | **POST** /span/collections/{collectionId}/firmware | Create firmware
[**delete_firmware**](FotaApi.md#delete_firmware) | **DELETE** /span/collections/{collectionId}/firmware/{imageId} | Delete firmware
[**firmware_usage**](FotaApi.md#firmware_usage) | **GET** /span/collections/{collectionId}/firmware/{imageId}/usage | Firmware usage
[**list_firmware**](FotaApi.md#list_firmware) | **GET** /span/collections/{collectionId}/firmware | List firmware
[**retrieve_firmware**](FotaApi.md#retrieve_firmware) | **GET** /span/collections/{collectionId}/firmware/{imageId} | Retrieve firmware
[**update_firmware**](FotaApi.md#update_firmware) | **PATCH** /span/collections/{existingCollectionId}/firmware/{imageId} | Update firmware



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

Firmware images must have unique version numbers and have an unique checksum. The checksum is calculated when the firmware image is uploaded.

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

> crate::models::Firmware update_firmware(existing_collection_id, image_id, body)
Update firmware

Only the version and tags fields can be updated. The other fields will be ignored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**existing_collection_id** | **String** |  | [required] |
**image_id** | **String** |  | [required] |
**body** | [**UpdateFirmwareRequest**](UpdateFirmwareRequest.md) |  | [required] |

### Return type

[**crate::models::Firmware**](Firmware.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

