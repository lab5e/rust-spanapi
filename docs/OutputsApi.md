# \OutputsApi

All URIs are relative to *https://api.lab5e.com/span*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_output**](OutputsApi.md#create_output) | **post** /collections/{collectionId}/outputs | Create output
[**delete_output**](OutputsApi.md#delete_output) | **delete** /collections/{collectionId}/outputs/{outputId} | Delete output
[**list_outputs**](OutputsApi.md#list_outputs) | **get** /collections/{collectionId}/outputs | List outputs
[**logs**](OutputsApi.md#logs) | **get** /collections/{collectionId}/outputs/{outputId}/logs | Output logs
[**retrieve_output**](OutputsApi.md#retrieve_output) | **get** /collections/{collectionId}/outputs/{outputId} | Retrieve output
[**status**](OutputsApi.md#status) | **get** /collections/{collectionId}/outputs/{outputId}/status | Output status
[**update_output**](OutputsApi.md#update_output) | **patch** /collections/{collectionId}/outputs/{outputId} | Update output



## create_output

> crate::models::Output create_output(collection_id, body)
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

> crate::models::Output update_output(collection_id, output_id, body)
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

