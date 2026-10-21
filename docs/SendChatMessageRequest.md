# SendChatMessageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conversation_token** | Option<**String**> | Optional conversation token. | [optional]
**encoded_message_create_event** | **String** | Base64-encoded Thrift MessageCreateEvent containing encrypted message contents. | 
**encoded_message_event_signature** | Option<**String**> | Base64-encoded Thrift MessageEventSignature for message verification. | [optional]
**message_id** | **String** | Unique identifier for this message. | 
**xdk_version** | Option<**String**> | Version of the Chat XDK that produced the payload. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


