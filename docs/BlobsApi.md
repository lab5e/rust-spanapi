# \BlobsApi

All URIs are relative to *https://api.lab5e.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_blob**](BlobsApi.md#delete_blob) | **DELETE** /span/collections/{collectionId}/blobs/{blobId} | Remove a blob stored on a collection
[**list_blobs**](BlobsApi.md#list_blobs) | **GET** /span/collections/{collectionId}/blobs | List the blobs for a collection



## delete_blob

> serde_json::Value delete_blob(collection_id, blob_id)
Remove a blob stored on a collection

Remove a blob stored on the collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**blob_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_blobs

> crate::models::ListBlobResponse list_blobs(collection_id, limit)
List the blobs for a collection

Retrieve a list of all the blobs stored on this collection. Blobs are uploaded by the devices through one of the blob endpoints.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |

### Return type

[**crate::models::ListBlobResponse**](ListBlobResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

