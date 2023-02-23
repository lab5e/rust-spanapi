# Rust API client for spanapi

API for device, collection, output and firmware management

For more information, please visit [https://lab5e.com](https://lab5e.com)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 4.4.1 busy-janay
- Package version: 4.4.1
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `spanapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
spanapi = { path = "./spanapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.lab5e.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*BlobsApi* | [**delete_blob**](docs/BlobsApi.md#delete_blob) | **DELETE** /span/collections/{collectionId}/blobs/{blobId} | Remove a blob stored on a collection
*BlobsApi* | [**list_blobs**](docs/BlobsApi.md#list_blobs) | **GET** /span/collections/{collectionId}/blobs | List the blobs for a collection
*CertificatesApi* | [**create_certificate**](docs/CertificatesApi.md#create_certificate) | **POST** /span/collections/{collectionId}/certificates/create | Create certificate
*CertificatesApi* | [**retrieve_certificate_chain**](docs/CertificatesApi.md#retrieve_certificate_chain) | **GET** /span/collections/{collectionId}/certificates | Get certificate chain
*CertificatesApi* | [**sign_certificate**](docs/CertificatesApi.md#sign_certificate) | **POST** /span/collections/{collectionId}/certificates/sign | Sign certificate
*CertificatesApi* | [**verify_certificate**](docs/CertificatesApi.md#verify_certificate) | **POST** /span/collections/{collectionId}/certificates/verify | Verify certificate
*CollectionsApi* | [**create_collection**](docs/CollectionsApi.md#create_collection) | **POST** /span/collections | Create collection
*CollectionsApi* | [**delete_collection**](docs/CollectionsApi.md#delete_collection) | **DELETE** /span/collections/{collectionId} | Delete collection
*CollectionsApi* | [**list_collection_data**](docs/CollectionsApi.md#list_collection_data) | **GET** /span/collections/{collectionId}/data | Retrieve data from devices
*CollectionsApi* | [**list_collections**](docs/CollectionsApi.md#list_collections) | **GET** /span/collections | List collections
*CollectionsApi* | [**retrieve_collection**](docs/CollectionsApi.md#retrieve_collection) | **GET** /span/collections/{collectionId} | Retrieve collection
*CollectionsApi* | [**update_collection**](docs/CollectionsApi.md#update_collection) | **PATCH** /span/collections/{collectionId} | Update collection
*DevicesApi* | [**add_downstream_message**](docs/DevicesApi.md#add_downstream_message) | **POST** /span/collections/{collectionId}/devices/{deviceId}/outbox | Add message to oubox
*DevicesApi* | [**create_device**](docs/DevicesApi.md#create_device) | **POST** /span/collections/{collectionId}/devices | Create device
*DevicesApi* | [**delete_device**](docs/DevicesApi.md#delete_device) | **DELETE** /span/collections/{collectionId}/devices/{deviceId} | Remove device.
*DevicesApi* | [**delete_downstream_message**](docs/DevicesApi.md#delete_downstream_message) | **DELETE** /span/collections/{collectionId}/devices/{deviceId}/outbox/{messageId} | Delete outgoing message
*DevicesApi* | [**device_certificate**](docs/DevicesApi.md#device_certificate) | **GET** /span/collections/{collectionId}/devices/{deviceId}/certs | Get issued certificate(s) for device
*DevicesApi* | [**list_device_data**](docs/DevicesApi.md#list_device_data) | **GET** /span/collections/{collectionId}/devices/{deviceId}/data | Retrieve data from device
*DevicesApi* | [**list_devices**](docs/DevicesApi.md#list_devices) | **GET** /span/collections/{collectionId}/devices | List devices in collection.
*DevicesApi* | [**list_downstream_messages**](docs/DevicesApi.md#list_downstream_messages) | **GET** /span/collections/{collectionId}/devices/{deviceId}/outbox | List the messages in the outbox
*DevicesApi* | [**list_upstream_messages**](docs/DevicesApi.md#list_upstream_messages) | **GET** /span/collections/{collectionId}/devices/{deviceId}/inbox | List incoming messages
*DevicesApi* | [**retrieve_device**](docs/DevicesApi.md#retrieve_device) | **GET** /span/collections/{collectionId}/devices/{deviceId} | Retrieve device
*DevicesApi* | [**update_device**](docs/DevicesApi.md#update_device) | **PATCH** /span/collections/{existingCollectionId}/devices/{deviceId} | Update device
*FotaApi* | [**clear_firmware_error**](docs/FotaApi.md#clear_firmware_error) | **DELETE** /span/collections/{collectionId}/devices/{deviceId}/fwerror | Clear FOTA error
*FotaApi* | [**create_firmware**](docs/FotaApi.md#create_firmware) | **POST** /span/collections/{collectionId}/firmware | Create firmware
*FotaApi* | [**delete_firmware**](docs/FotaApi.md#delete_firmware) | **DELETE** /span/collections/{collectionId}/firmware/{imageId} | Delete firmware
*FotaApi* | [**firmware_usage**](docs/FotaApi.md#firmware_usage) | **GET** /span/collections/{collectionId}/firmware/{imageId}/usage | Firmware usage
*FotaApi* | [**list_firmware**](docs/FotaApi.md#list_firmware) | **GET** /span/collections/{collectionId}/firmware | List firmware
*FotaApi* | [**retrieve_firmware**](docs/FotaApi.md#retrieve_firmware) | **GET** /span/collections/{collectionId}/firmware/{imageId} | Retrieve firmware
*FotaApi* | [**update_firmware**](docs/FotaApi.md#update_firmware) | **PATCH** /span/collections/{existingCollectionId}/firmware/{imageId} | Update firmware
*GatewaysApi* | [**create_gateway**](docs/GatewaysApi.md#create_gateway) | **POST** /span/collections/{collectionId}/gateways | Create gateway
*GatewaysApi* | [**delete_gateway**](docs/GatewaysApi.md#delete_gateway) | **DELETE** /span/collections/{collectionId}/gateways/{gatewayId} | Delete gateway
*GatewaysApi* | [**gateway_certificates**](docs/GatewaysApi.md#gateway_certificates) | **GET** /span/collections/{collectionId}/gateways/{gatewayId}/certs | Get issued certificate(s) for gateway
*GatewaysApi* | [**list_gateways**](docs/GatewaysApi.md#list_gateways) | **GET** /span/collections/{collectionId}/gateways | List gateways
*GatewaysApi* | [**retrieve_gateway**](docs/GatewaysApi.md#retrieve_gateway) | **GET** /span/collections/{collectionId}/gateways/{gatewayId} | Retrieve gateway
*GatewaysApi* | [**update_gateway**](docs/GatewaysApi.md#update_gateway) | **PATCH** /span/collections/{existingCollectionId}/gateways/{gatewayId} | Update gateway
*OutputsApi* | [**create_output**](docs/OutputsApi.md#create_output) | **POST** /span/collections/{collectionId}/outputs | Create output
*OutputsApi* | [**delete_output**](docs/OutputsApi.md#delete_output) | **DELETE** /span/collections/{collectionId}/outputs/{outputId} | Delete output
*OutputsApi* | [**list_outputs**](docs/OutputsApi.md#list_outputs) | **GET** /span/collections/{collectionId}/outputs | List outputs
*OutputsApi* | [**logs**](docs/OutputsApi.md#logs) | **GET** /span/collections/{collectionId}/outputs/{outputId}/logs | Output logs
*OutputsApi* | [**retrieve_output**](docs/OutputsApi.md#retrieve_output) | **GET** /span/collections/{collectionId}/outputs/{outputId} | Retrieve output
*OutputsApi* | [**status**](docs/OutputsApi.md#status) | **GET** /span/collections/{collectionId}/outputs/{outputId}/status | Output status
*OutputsApi* | [**update_output**](docs/OutputsApi.md#update_output) | **PATCH** /span/collections/{existingCollectionId}/outputs/{outputId} | Update output
*SpanApi* | [**get_system_info**](docs/SpanApi.md#get_system_info) | **GET** /span/system | System information


## Documentation For Models

 - [AddDownstreamMessageRequest](docs/AddDownstreamMessageRequest.md)
 - [Any](docs/Any.md)
 - [Blob](docs/Blob.md)
 - [CellularIoTConfig](docs/CellularIoTConfig.md)
 - [CellularIoTMetadata](docs/CellularIoTMetadata.md)
 - [CertificateChainResponse](docs/CertificateChainResponse.md)
 - [CertificateInfo](docs/CertificateInfo.md)
 - [ClearFirmwareErrorResponse](docs/ClearFirmwareErrorResponse.md)
 - [CoApMetadata](docs/CoApMetadata.md)
 - [Collection](docs/Collection.md)
 - [CollectionFirmware](docs/CollectionFirmware.md)
 - [CreateCertificateRequest](docs/CreateCertificateRequest.md)
 - [CreateCertificateResponse](docs/CreateCertificateResponse.md)
 - [CreateCollectionRequest](docs/CreateCollectionRequest.md)
 - [CreateDeviceRequest](docs/CreateDeviceRequest.md)
 - [CreateFirmwareRequest](docs/CreateFirmwareRequest.md)
 - [CreateOutputRequest](docs/CreateOutputRequest.md)
 - [DeleteDownstreamMessageResponse](docs/DeleteDownstreamMessageResponse.md)
 - [Device](docs/Device.md)
 - [DeviceCertificateResponse](docs/DeviceCertificateResponse.md)
 - [DeviceConfig](docs/DeviceConfig.md)
 - [DeviceMetadata](docs/DeviceMetadata.md)
 - [Firmware](docs/Firmware.md)
 - [FirmwareManagement](docs/FirmwareManagement.md)
 - [FirmwareMetadata](docs/FirmwareMetadata.md)
 - [FirmwareUsageResponse](docs/FirmwareUsageResponse.md)
 - [Gateway](docs/Gateway.md)
 - [GatewayCertificateResponse](docs/GatewayCertificateResponse.md)
 - [GatewayCioTConfig](docs/GatewayCioTConfig.md)
 - [GatewayConfig](docs/GatewayConfig.md)
 - [GatewayCustomConfig](docs/GatewayCustomConfig.md)
 - [GatewayDeviceConfig](docs/GatewayDeviceConfig.md)
 - [GatewayDeviceMetadata](docs/GatewayDeviceMetadata.md)
 - [GatewayInetConfig](docs/GatewayInetConfig.md)
 - [GatewayMetadata](docs/GatewayMetadata.md)
 - [GatewayStatus](docs/GatewayStatus.md)
 - [GatewayType](docs/GatewayType.md)
 - [InetMetadata](docs/InetMetadata.md)
 - [InlineObject](docs/InlineObject.md)
 - [InlineObject1](docs/InlineObject1.md)
 - [ListBlobResponse](docs/ListBlobResponse.md)
 - [ListCollectionResponse](docs/ListCollectionResponse.md)
 - [ListDataResponse](docs/ListDataResponse.md)
 - [ListDevicesResponse](docs/ListDevicesResponse.md)
 - [ListDownstreamMessagesResponse](docs/ListDownstreamMessagesResponse.md)
 - [ListFirmwareResponse](docs/ListFirmwareResponse.md)
 - [ListGatewayResponse](docs/ListGatewayResponse.md)
 - [ListOutputResponse](docs/ListOutputResponse.md)
 - [ListUpstreamMessagesResponse](docs/ListUpstreamMessagesResponse.md)
 - [MessageDownstream](docs/MessageDownstream.md)
 - [MessageState](docs/MessageState.md)
 - [MessageTransport](docs/MessageTransport.md)
 - [MessageUpstream](docs/MessageUpstream.md)
 - [MqttMetadata](docs/MqttMetadata.md)
 - [NetworkMetadata](docs/NetworkMetadata.md)
 - [NetworkOperator](docs/NetworkOperator.md)
 - [Output](docs/Output.md)
 - [OutputConfig](docs/OutputConfig.md)
 - [OutputDataMessage](docs/OutputDataMessage.md)
 - [OutputLogEntry](docs/OutputLogEntry.md)
 - [OutputLogResponse](docs/OutputLogResponse.md)
 - [OutputMessageType](docs/OutputMessageType.md)
 - [OutputStatusResponse](docs/OutputStatusResponse.md)
 - [OutputType](docs/OutputType.md)
 - [RetrieveBlobResponse](docs/RetrieveBlobResponse.md)
 - [SignCertificateRequest](docs/SignCertificateRequest.md)
 - [SignCertificateResponse](docs/SignCertificateResponse.md)
 - [Status](docs/Status.md)
 - [SystemInfoResponse](docs/SystemInfoResponse.md)
 - [UdpMetadata](docs/UdpMetadata.md)
 - [UpdateCollectionRequest](docs/UpdateCollectionRequest.md)
 - [UpdateDeviceRequest](docs/UpdateDeviceRequest.md)
 - [UpdateFirmwareRequest](docs/UpdateFirmwareRequest.md)
 - [UpdateOutputRequest](docs/UpdateOutputRequest.md)
 - [VerifyCertificateRequest](docs/VerifyCertificateRequest.md)
 - [VerifyCertificateResponse](docs/VerifyCertificateResponse.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

dev@lab5e.com

