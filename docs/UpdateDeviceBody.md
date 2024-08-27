# UpdateDeviceBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection_id** | Option<**String**> | The collection id for the device. This field is optional and can be omitted if the collection id isn't changed. When changing to a new collection you must be an owner of the other collection, ie an administrator of the team that owns the new collection. | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Tags are metadata for the device that you can set. These are just strings. | [optional]
**firmware** | Option<[**crate::models::FirmwareMetadata**](FirmwareMetadata.md)> |  | [optional]
**config** | Option<[**crate::models::DeviceConfig**](DeviceConfig.md)> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


