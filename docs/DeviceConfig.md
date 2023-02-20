# DeviceConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ciot** | Option<[**crate::models::CellularIoTConfig**](CellularIoTConfig.md)> |  | [optional]
**inet** | Option<[**serde_json::Value**](.md)> | This is the configuration for an internet-connected device. There are currently no configuration options for internet devices; the device is identified via the clientcertificate.  This is empty since there's no configuration required for the internet  gateway | [optional]
**gateway** | Option<[**::std::collections::HashMap<String, crate::models::GatewayDeviceConfig>**](GatewayDeviceConfig.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


