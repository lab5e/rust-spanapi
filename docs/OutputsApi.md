# \OutputsApi

All URIs are relative to *https://api.lab5e.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_output**](OutputsApi.md#create_output) | **POST** /span/collections/{collectionId}/outputs | Create output
[**delete_output**](OutputsApi.md#delete_output) | **DELETE** /span/collections/{collectionId}/outputs/{outputId} | Delete output
[**list_outputs**](OutputsApi.md#list_outputs) | **GET** /span/collections/{collectionId}/outputs | List outputs
[**logs**](OutputsApi.md#logs) | **GET** /span/collections/{collectionId}/outputs/{outputId}/logs | Output logs
[**retrieve_output**](OutputsApi.md#retrieve_output) | **GET** /span/collections/{collectionId}/outputs/{outputId} | Retrieve output
[**retrieve_output_stats**](OutputsApi.md#retrieve_output_stats) | **GET** /span/collections/{collectionId}/outputs/{outputId}/stats | Retrieve output statistics
[**status**](OutputsApi.md#status) | **GET** /span/collections/{collectionId}/outputs/{outputId}/status | Output status
[**update_output**](OutputsApi.md#update_output) | **PATCH** /span/collections/{existingCollectionId}/outputs/{outputId} | Update output



## create_output

> crate::models::Output create_output(collection_id, body)
Create output

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**body** | [**CreateOutputBody**](CreateOutputBody.md) |  | [required] |

### Return type

[**crate::models::Output**](Output.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_output

> crate::models::Output delete_output(collection_id, output_id)
Delete output

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**output_id** | **String** |  | [required] |

### Return type

[**crate::models::Output**](Output.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_outputs

> crate::models::ListOutputResponse list_outputs(collection_id)
List outputs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |

### Return type

[**crate::models::ListOutputResponse**](ListOutputResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logs

> crate::models::OutputLogResponse logs(collection_id, output_id)
Output logs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**output_id** | **String** |  | [required] |

### Return type

[**crate::models::OutputLogResponse**](OutputLogResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_output

> crate::models::Output retrieve_output(collection_id, output_id)
Retrieve output

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**output_id** | **String** |  | [required] |

### Return type

[**crate::models::Output**](Output.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_output_stats

> crate::models::OutputStats retrieve_output_stats(collection_id, output_id)
Retrieve output statistics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**output_id** | **String** |  | [required] |

### Return type

[**crate::models::OutputStats**](OutputStats.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## status

> crate::models::OutputStatusResponse status(collection_id, output_id)
Output status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**output_id** | **String** |  | [required] |

### Return type

[**crate::models::OutputStatusResponse**](OutputStatusResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_output

> crate::models::Output update_output(existing_collection_id, output_id, body)
Update output

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**existing_collection_id** | **String** |  | [required] |
**output_id** | **String** |  | [required] |
**body** | [**UpdateOutputBody**](UpdateOutputBody.md) |  | [required] |

### Return type

[**crate::models::Output**](Output.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

