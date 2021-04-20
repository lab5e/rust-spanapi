# \OutputsApi

All URIs are relative to *https://api.lab5e.com/span*

Method | HTTP request | Description
------------- | ------------- | -------------
[**span_create_output**](OutputsApi.md#span_create_output) | **post** /collections/{collectionId}/outputs | Create output
[**span_delete_output**](OutputsApi.md#span_delete_output) | **delete** /collections/{collectionId}/outputs/{outputId} | Delete output
[**span_list_outputs**](OutputsApi.md#span_list_outputs) | **get** /collections/{collectionId}/outputs | List outputs
[**span_logs**](OutputsApi.md#span_logs) | **get** /collections/{collectionId}/outputs/{outputId}/logs | Output logs
[**span_retrieve_output**](OutputsApi.md#span_retrieve_output) | **get** /collections/{collectionId}/outputs/{outputId} | Retrieve output
[**span_status**](OutputsApi.md#span_status) | **get** /collections/{collectionId}/outputs/{outputId}/status | Output status
[**span_update_output**](OutputsApi.md#span_update_output) | **patch** /collections/{collectionId}/outputs/{outputId} | Update output



## span_create_output

> crate::models::Output span_create_output(collection_id, body)
Create output

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**body** | [**Output**](Output.md) |  | [required] |

### Return type

[**crate::models::Output**](Output.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## span_delete_output

> crate::models::Output span_delete_output(collection_id, output_id)
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


## span_list_outputs

> crate::models::ListOutputResponse span_list_outputs(collection_id)
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


## span_logs

> crate::models::OutputLogResponse span_logs(collection_id, output_id)
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


## span_retrieve_output

> crate::models::Output span_retrieve_output(collection_id, output_id)
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


## span_status

> crate::models::OutputStatusResponse span_status(collection_id, output_id)
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


## span_update_output

> crate::models::Output span_update_output(collection_id, output_id, body)
Update output

Running outputs will be restarted if required. Note that the collection ID can't be changed on an existing output.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**output_id** | **String** |  | [required] |
**body** | [**Output**](Output.md) |  | [required] |

### Return type

[**crate::models::Output**](Output.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

