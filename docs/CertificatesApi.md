# \CertificatesApi

All URIs are relative to *https://api.lab5e.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_certificate**](CertificatesApi.md#create_certificate) | **POST** /span/collections/{collectionId}/certificates/create | Create certificate
[**retrieve_certificate_chain**](CertificatesApi.md#retrieve_certificate_chain) | **GET** /span/collections/{collectionId}/certificates | Get certificate chain
[**sign_certificate**](CertificatesApi.md#sign_certificate) | **POST** /span/collections/{collectionId}/certificates/sign | Sign certificate
[**verify_certificate**](CertificatesApi.md#verify_certificate) | **POST** /span/collections/{collectionId}/certificates/verify | Verify certificate



## create_certificate

> crate::models::CreateCertificateResponse create_certificate(collection_id, body)
Create certificate

Create a new device or gateway (client) certificate for an internet-connected device. The devices will use this client certificate to authenticate when sending data via the Internet endpoint. This will create a X509 client certificate with an ECC public key. The key is not stored by the service so keep it in a secure place once it is downloaded. The returned certificate will be valid for 14 days. The key for the certificate is your own responsibility. The client certificate is used in both the TLS, DTLS and gRPC service endpoints.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**body** | [**CreateCertificateRequest**](CreateCertificateRequest.md) |  | [required] |

### Return type

[**crate::models::CreateCertificateResponse**](CreateCertificateResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_certificate_chain

> crate::models::CertificateChainResponse retrieve_certificate_chain(collection_id, gateway_id, device_id)
Get certificate chain

Get the certificate chain for the root CA and intermediate certificates used by the device, gateway and server certificates. It is highly recommended to verify the server certificate when sending data to avoid any man-in-the-middle attacks. This chain will contain all required certificates needed to verify the client certificate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**gateway_id** | Option<**String**> |  |  |
**device_id** | Option<**String**> |  |  |

### Return type

[**crate::models::CertificateChainResponse**](CertificateChainResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sign_certificate

> crate::models::SignCertificateResponse sign_certificate(collection_id, body)
Sign certificate

Sign a device or gateway (aka client) certificate. The certificate is a X509 Certificate signing request PEM encoded. The certificate will be valid for 14 days and must be renewed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**body** | [**SignCertificateRequest**](SignCertificateRequest.md) |  | [required] |

### Return type

[**crate::models::SignCertificateResponse**](SignCertificateResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_certificate

> crate::models::VerifyCertificateResponse verify_certificate(collection_id, body)
Verify certificate

Verify client certificate. If a client certificate fails it can be tricky to pinpoint exactly *why* the certificate isn't accepted. This resource validates the client certificate and returns the error in plain text.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** |  | [required] |
**body** | [**VerifyCertificateRequest**](VerifyCertificateRequest.md) |  | [required] |

### Return type

[**crate::models::VerifyCertificateResponse**](VerifyCertificateResponse.md)

### Authorization

[APIToken](../README.md#APIToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

