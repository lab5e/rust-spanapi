# Device

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device_id** | Option<**String**> | The device ID is assigned by the backend. | [optional]
**collection_id** | Option<**String**> |  | [optional]
**imsi** | Option<**String**> | The IMSI is the unique ID for the (e|nu|whatever)SIM card on your device. This is the primary identifier for your device on the network. | [optional]
**imei** | Option<**String**> | The IMEI number is the unique ID for your hardware as seen by the network. Obviously you might have a completely different view on things. | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Tags are metadata for the device that you can set. These are just strings. | [optional]
**network** | Option<[**crate::models::NetworkMetadata**](NetworkMetadata.md)> |  | [optional]
**firmware** | Option<[**crate::models::FirmwareMetadata**](FirmwareMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

