# \DatadumpApi

All URIs are relative to *https://api.lab5e.com/span*

Method | HTTP request | Description
------------- | ------------- | -------------
[**span_data_dump**](DatadumpApi.md#span_data_dump) | **post** /datadump | Data dump



## span_data_dump

> crate::models::DataDumpResponse span_data_dump(body)
Data dump

Do a complete data dump of your data, devices, outputs and collections.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::DataDumpResponse**](DataDumpResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

