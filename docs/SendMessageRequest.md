# SendMessageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection_id** | Option<**String**> |  | [optional]
**device_id** | Option<**String**> |  | [optional]
**port** | Option<**i32**> |  | [optional]
**payload** | Option<**String**> |  | [optional]
**transport** | Option<**String**> | Valid transports are \"udp\", \"coap\", \"coap-pull\", \"udp-pull\", \"coap-push\", \"udp-push\". \"udp\" is equivalent to \"udp-push\" and \"coap\" is equivalent to \"coap-push\". Push messages are sent unsolicited to the device wheil pull messages are sent whenever the device wither sends data upstream (for UDP) or does a CoAP request to the CoAP service in span. | [optional]
**coap_path** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


