# \DatadumpApi

All URIs are relative to *https://api.lab5e.com/span*

Method | HTTP request | Description
------------- | ------------- | -------------
[**data_dump**](DatadumpApi.md#data_dump) | **post** /datadump | Data dump



## data_dump

> crate::models::DataDumpResponse data_dump(body)
Data dump

Do a complete data dump of your data, devices, outputs and collections.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DataDumpRequest**](DataDumpRequest.md) |  | [required] |

### Return type

[**crate::models::DataDumpResponse**](DataDumpResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

