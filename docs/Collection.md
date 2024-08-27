# Collection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection_id** | Option<**String**> | The ID of the collection. This is assigned by the backend. | [optional]
**team_id** | Option<**String**> | The team ID that owns the collection. This field is required. When you create new collections the default is to use your private team ID. | [optional]
**firmware** | Option<[**crate::models::CollectionFirmware**](CollectionFirmware.md)> |  | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Tags for the collection. Tags are metadata fields that you can assign to the collection. | [optional]
**upstream_timestamps** | Option<**Vec<String>**> |  | [optional]
**downstream_timestamps** | Option<**Vec<String>**> |  | [optional]
**enabled** | Option<**bool**> | Disabled flag for collection. If the collection is disabled it is in effect read only and inactive. You can't update a disabled collection. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


