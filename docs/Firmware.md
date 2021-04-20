# Firmware

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**image_id** | Option<**String**> |  | [optional]
**version** | Option<**String**> | Make sure that the firmware image reports this version. If the version reported by this image is different the FOTA process won't be reported as successful since the device will report a version different from this. | [optional]
**filename** | Option<**String**> | This is just for internal house keeping, making it potentially easier to identify the firmware image. | [optional]
**sha256** | Option<**String**> | To ensure each image is unique the SHA-256 hash for all images in a collection must be unqique | [optional]
**length** | Option<**i32**> |  | [optional]
**collection_id** | Option<**String**> | Collection ID for the collection owning the firmware image. | [optional]
**created** | Option<**String**> |  | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Tags for firmware image. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


