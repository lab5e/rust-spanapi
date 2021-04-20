# UpdateDeviceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**existing_collection_id** | Option<**String**> |  | [optional]
**device_id** | Option<**String**> |  | [optional]
**collection_id** | Option<**String**> | The collection id for the device. This field is optional and can be omitted if the collection id isn't changed. When changing to a new collection you must be an owner of the other collection, ie an administrator of the team that owns the new collection. | [optional]
**imsi** | Option<**String**> | The IMSI is the unique ID for the (e|nu|whatever)SIM card on your device. This is the primary identifier for your device on the network. | [optional]
**imei** | Option<**String**> | The IMEI number is the unique ID for your hardware as seen by the network. Obviously you might have a completely different view on things. | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Tags are metadata for the device that you can set. These are just strings. | [optional]
**firmware** | Option<[**crate::models::FirmwareMetadata**](FirmwareMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


