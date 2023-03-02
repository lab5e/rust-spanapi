# Device

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device_id** | Option<**String**> | The device ID is assigned by the backend. | [optional]
**collection_id** | Option<**String**> |  | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Tags are metadata for the device that you can set. These are just strings. | [optional]
**firmware** | Option<[**crate::models::FirmwareMetadata**](FirmwareMetadata.md)> |  | [optional]
**config** | Option<[**crate::models::DeviceConfig**](DeviceConfig.md)> |  | [optional]
**metadata** | Option<[**crate::models::DeviceMetadata**](DeviceMetadata.md)> |  | [optional]
**last_gateway_id** | Option<**String**> |  | [optional]
**last_transport** | Option<[**crate::models::MessageTransport**](MessageTransport.md)> |  | [optional]
**last_received** | Option<**String**> |  | [optional]
**last_payload** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


