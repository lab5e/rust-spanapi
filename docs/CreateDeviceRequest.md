# CreateDeviceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tags** | Option<**::std::collections::HashMap<String, String>**> | Tags are metadata for the device that you can set. These are just strings. | [optional]
**firmware** | Option<[**crate::models::FirmwareMetadata**](FirmwareMetadata.md)> |  | [optional]
**config** | Option<[**crate::models::DeviceConfig**](DeviceConfig.md)> |  | [optional]
**metadata** | Option<[**crate::models::DeviceMetadata**](DeviceMetadata.md)> |  | [optional]
**imsi** | Option<**String**> | The IMSI is the unique ID for the (e|nu|whatever)SIM card on your device. This is the primary identifier for your device on the network.  Deprecated: The IMSI is replaced by CellularIoTMetadata | [optional]
**imei** | Option<**String**> | The IMEI number is the unique ID for your hardware as seen by the network. Obviously you might have a completely different view on things.  Deprecated: The IMEI is replaced by CellularIoTMetadata | [optional]
**network** | Option<[**crate::models::NetworkMetadata**](NetworkMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


