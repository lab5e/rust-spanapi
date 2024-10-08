# UpdateCollectionBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**team_id** | Option<**String**> | The team ID that owns the collection. This field is required. When you create new collections the default is to use your private team ID. | [optional]
**firmware** | Option<[**crate::models::CollectionFirmware**](CollectionFirmware.md)> |  | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Tags for the collection. Tags are metadata fields that you can assign to the collection. | [optional]
**enabled** | Option<**bool**> | Enabled flag for the collection. A collection may be disabled or enabled to save time. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


